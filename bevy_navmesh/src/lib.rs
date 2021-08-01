use bevy::prelude::*;
use navmesh::NavMesh;
use proto_navmesh::ProtoNavMesh;
use std::convert::*;

pub mod geo;
mod proto_navmesh;

/// The navmesh plugin adds mesh navigation
pub struct NavmeshPlugin;

impl Plugin for NavmeshPlugin {
  fn build(&self, app: &mut AppBuilder) {
    app
      .init_resource::<ProtoNavMesh>()
      .add_system_to_stage(CoreStage::First, update_navigation.system())
      .add_system_to_stage(CoreStage::First, update_navmesh.system())
      .add_system_to_stage(CoreStage::Last, clean_mesh.system());
  }
}

/// The navmesh debug plugin adds mesh navigation and renders
/// the navigation meshes. This bebug plugin requires the
/// bevy_prototype_lyon ShapePlugin in order to function.
#[cfg(feature = "debug")]
pub struct NavmeshDebugPlugin;

#[cfg(feature = "debug")]
impl Plugin for NavmeshDebugPlugin {
  fn build(&self, app: &mut AppBuilder) {
    app
      .add_plugin(NavmeshPlugin)
      .add_system_to_stage(CoreStage::PreUpdate, setup_navmesh_debug.system())
      .add_system_to_stage(CoreStage::PreUpdate, setup_navigation_debug.system())
      .add_system_to_stage(CoreStage::PostUpdate, cleanup_navmesh_debug.system())
      .add_system_to_stage(CoreStage::PostUpdate, cleanup_navigation_debug.system());
  }
}

#[derive(Clone, Debug, Default)]
pub struct Navmesh {
  pub points:    Vec<Vec3>,
  pub triangles: Vec<(Vec3, Vec3, Vec3)>,
}

#[derive(Default, Debug)]
pub struct Navigation {
  pub dest: Vec3,
  pub path: Vec<Vec3>,
}

impl Navigation {
  pub fn next(&self) -> Option<&Vec3> {
    self.path.first()
  }
}

fn update_navmesh(
  mut mesh: ResMut<ProtoNavMesh>,
  query: Query<
    (Entity, &Navmesh, &GlobalTransform),
    Or<(Changed<Navmesh>, Changed<GlobalTransform>)>,
  >,
) {
  for (entity, navmesh, global_transform) in query.iter() {
    mesh.remove_entity(entity);
    for (a, b, c) in navmesh.triangles.iter() {
      mesh.add_triangle(
        entity,
        (
          *a + global_transform.translation,
          *b + global_transform.translation,
          *c + global_transform.translation,
        ),
      );
      mesh.dirty()
    }
  }
}

fn update_navigation(
  mesh: ResMut<ProtoNavMesh>,
  mut query: Query<(
    &mut Navigation,
    &GlobalTransform,
    ChangeTrackers<Navigation>,
    ChangeTrackers<GlobalTransform>,
  )>,
) {
  let nav_mesh: NavMesh = mesh.clone().into();

  for (mut navigation, global_transform, navigation_tracker, global_transform_tracker) in
    query.iter_mut()
  {
    if mesh.is_clean() && !navigation_tracker.is_changed() && !global_transform_tracker.is_changed()
    {
      continue;
    }

    let nav_path = nav_mesh.find_path(
      global_transform.translation.into(),
      navigation.dest.into(),
      navmesh::NavQuery::Accuracy,
      navmesh::NavPathMode::MidPoints,
    );

    if let Some(points) = nav_path {
      navigation.path = points
        .iter()
        .map(|p| Into::<Vec3>::into(*p))
        .skip(1)
        .collect();
    }
  }
}

fn clean_mesh(mut mesh: ResMut<ProtoNavMesh>) {
  mesh.clean();
}

#[cfg(feature = "debug")]
pub struct NavmeshDebug {
  pub child: Entity,
}

#[cfg(feature = "debug")]
fn setup_navmesh_debug(mut commands: Commands, query: Query<(Entity, &Navmesh), Added<Navmesh>>) {
  use bevy_prototype_lyon::{entity::ShapeBundle, prelude::*};

  for (entity, navmesh) in query.iter() {
    for (a, b, c) in navmesh.triangles.iter() {
      let shape = shapes::Polygon {
        points: vec![a.truncate(), b.truncate(), c.truncate()],
        closed: true,
      };
      let child = commands
        .spawn_bundle(GeometryBuilder::build_as(
          &shape,
          ShapeColors::new(*Color::RED.set_a(0.2)),
          DrawMode::Outlined {
            fill_options:    FillOptions::default(),
            outline_options: StrokeOptions::default(),
          },
          Transform::default(),
        ))
        .id();

      commands
        .entity(entity)
        .push_children(&vec![child])
        .insert(NavmeshDebug { child });
    }
  }
}

#[cfg(feature = "debug")]
pub fn cleanup_navmesh_debug(
  mut commands: Commands,
  removed_meshes: RemovedComponents<Navmesh>,
  meshes: Query<(Entity, &NavmeshDebug)>,
) {
  for entity in removed_meshes.iter() {
    if let Ok(debug_navmesh) = meshes.get_component::<NavmeshDebug>(entity) {
      commands.entity(debug_navmesh.child).despawn();
    } else {
      warn!("Could not remove collision mesh debug from entity. Was already despawned?");
    }
  }
}

#[cfg(feature = "debug")]
pub struct NavigationDebug {
  pub debug_entity: Entity,
}

#[cfg(feature = "debug")]
fn setup_navigation_debug(
  mut commands: Commands,
  mesh: Res<ProtoNavMesh>,
  mut query: Query<(
    Entity,
    &mut Navigation,
    Option<&mut NavigationDebug>,
    &GlobalTransform,
    ChangeTrackers<Navigation>,
    ChangeTrackers<GlobalTransform>,
  )>,
) {
  use bevy_prototype_lyon::{entity::ShapeBundle, prelude::*};

  for (entity, mut navigation, mut debug, transform, navigation_tracker, transform_tracker) in
    query.iter_mut()
  {
    if !navigation_tracker.is_changed() && !transform_tracker.is_changed() {
      continue;
    }

    let mut path = PathBuilder::new();
    path.move_to(transform.translation.truncate());
    for p in navigation.path.iter() {
      path.line_to((*p).truncate());
    }

    let debug_entity = commands
      .spawn_bundle(ShapeBundle {
        path: path.build(),
        mode: DrawMode::Stroke(StrokeOptions::default()),
        ..ShapeBundle::default()
      })
      .id();

    if let Some(mut debug) = debug {
      commands.entity(debug.debug_entity).despawn();
      debug.debug_entity = debug_entity;
    } else {
      commands
        .entity(entity)
        .insert(NavigationDebug { debug_entity });
    }
  }
}

#[cfg(feature = "debug")]
pub fn cleanup_navigation_debug(
  mut commands: Commands,
  removed_meshes: RemovedComponents<Navigation>,
  meshes: Query<(Entity, &NavigationDebug)>,
) {
  for entity in removed_meshes.iter() {
    if let Ok(navigation_debug) = meshes.get_component::<NavigationDebug>(entity) {
      commands.entity(navigation_debug.debug_entity).despawn();
    } else {
      warn!("Could not remove collision mesh debug from entity. Was already despawned?");
    }
  }
}
