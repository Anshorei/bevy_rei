use bevy::prelude::*;
#[cfg(feature = "debug")]
use bevy_navmesh::NavmeshDebugPlugin;
use bevy_navmesh::{Navigation, Navmesh};
use bevy_prototype_lyon::{entity::ShapeBundle, plugin::ShapePlugin, prelude::*};

const TILESIZE: f32 = 50.;

struct PlatformTimer(Timer);

fn main() {
  let mut app = App::build();
  app.add_plugins(DefaultPlugins).add_plugin(ShapePlugin);

  #[cfg(feature = "debug")]
  {
    app.add_plugin(NavmeshDebugPlugin);
  }

  app
    .add_startup_system(setup.system())
    .add_system(move_to_dest.system())
    .add_system(set_new_dest.system())
    .add_system(move_platform.system())
    .insert_resource(PlatformTimer(Timer::from_seconds(2., true)))
    .run();
}

fn get_map_triangles() -> Vec<(Vec3, Vec3, Vec3)> {
  let map = vec![
    1, 1, 1, 1, 1, 0, 0, 0, 1, 0, 1, 0, 1, 1, 1, 1, 1, 0, 1, 1, 1, 0, 0, 1, 1, 0, 0, 3, 0, 0, 0, 1,
    1, 1, 1, 4, 1, 1, 0, 1, 1, 0, 0, 2, 0, 1, 0, 1, 1, 0, 0, 1, 0, 1, 0, 1, 1, 1, 0, 1, 0, 1, 1, 1,
  ];

  let mut triangles = vec![];

  let index_to_triangles = |i, top, bot| {
    let x = (i % 8) as isize - 4;
    let y = (i / 8) as isize - 4;
    let a = Vec3::new(x as f32 * TILESIZE, y as f32 * TILESIZE, bot);
    let b = Vec3::new((x as f32 + 1.) * TILESIZE, y as f32 * TILESIZE, bot);
    let c = Vec3::new(x as f32 * TILESIZE, (y as f32 + 1.) * TILESIZE, top);
    let d = Vec3::new((x as f32 + 1.) * TILESIZE, (y as f32 + 1.) * TILESIZE, top);
    vec![(a.clone(), b, c.clone()), (b, c, d)]
  };

  for (i, v) in map.iter().enumerate() {
    if *v == 1 || *v == 4 {
      triangles.append(&mut index_to_triangles(i, 0., 0.));
    }
    if *v == 4 {
      triangles.append(&mut index_to_triangles(i, 1., 1.));
    }
    if *v == 2 {
      triangles.append(&mut index_to_triangles(i, 0., 1.));
    }
    if *v == 3 {
      triangles.append(&mut index_to_triangles(i, 1., 0.));
    }
  }

  return triangles;
}

pub struct Player;
pub struct Platform;

fn setup(mut commands: Commands) {
  info!("Setting up...");

  commands.spawn_bundle(OrthographicCameraBundle::new_2d());

  commands
    .spawn()
    .insert(Transform::default())
    .insert(GlobalTransform::default())
    .insert(Navmesh {
      triangles: get_map_triangles(),
      ..Navmesh::default()
    });

  commands
    .spawn()
    .insert(Platform {})
    .insert(Transform::default())
    .insert(GlobalTransform::default())
    .insert(Navmesh {
      triangles: vec![
        (
          Vec3::new(TILESIZE * -1., TILESIZE * 2., 0.),
          Vec3::new(TILESIZE * -2., TILESIZE * 2., 0.),
          Vec3::new(TILESIZE * -1., TILESIZE * 3., 0.),
        ),
        (
          Vec3::new(TILESIZE * -2., TILESIZE * 2., 0.),
          Vec3::new(TILESIZE * -1., TILESIZE * 3., 0.),
          Vec3::new(TILESIZE * -2., TILESIZE * 3., 0.),
        ),
      ],
      ..Navmesh::default()
    });

  commands
    .spawn()
    .insert(Player {})
    .insert_bundle(GeometryBuilder::build_as(
      &shapes::Circle {
        radius: TILESIZE / 4.,
        ..Default::default()
      },
      ShapeColors::new(Color::WHITE),
      DrawMode::Fill(FillOptions::default()),
      Transform {
        translation: Vec2::default().extend(1.),
        ..Default::default()
      },
    ))
    .insert(GlobalTransform::default())
    .insert(Navigation {
      dest: Vec3::new(100., 200., 0.),
      ..Default::default()
    });
}

fn move_platform(
  time: Res<Time>,
  mut timer: ResMut<PlatformTimer>,
  mut query: Query<(&Navmesh, &mut Transform), With<Platform>>,
) {
  if timer.0.tick(time.delta()).just_finished() {
    info!("Moving platform");
    for (mesh, mut transform) in query.iter_mut() {
      if transform.translation.y == 0. {
        transform.translation.y = TILESIZE;
      } else {
        transform.translation.y = 0.;
      }
    }
  }
}

fn move_to_dest(
  time: Res<Time>,
  mut query: Query<(&mut Transform, &GlobalTransform, &Navigation), With<Player>>,
) {
  for (mut transform, global_transform, nav) in query.iter_mut() {
    if let Some(next) = nav.next() {
      let path = *next - global_transform.translation;
      let travel_distance = time.delta_seconds() * 100.;
      if travel_distance > path.length() {
        transform.translation += path;
      } else {
        transform.translation += path.normalize() * travel_distance;
      }
    }
  }
}

fn set_new_dest(
  mouse_button_input: Res<Input<MouseButton>>,
  windows: Res<Windows>,
  mut query: Query<&mut Navigation>,
) {
  if !mouse_button_input.just_released(MouseButton::Left) {
    return;
  }

  for mut navigation in query.iter_mut() {
    if let Some(window) = windows.get_primary() {
      if let Some(position) = window.cursor_position() {
        let translation = Vec2::new(window.width(), window.height()) / 2.;
        navigation.dest = (position - translation).extend(0.);
      }
    }
  }
}
