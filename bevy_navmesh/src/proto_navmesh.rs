use bevy::prelude::*;
use float_cmp::approx_eq;
use navmesh::NavMesh;

#[derive(Clone, Default)]
pub struct ProtoNavMesh {
  points:    Vec<Vec3>,
  triangles: Vec<(usize, usize, usize, Entity)>,
  dirty:     bool,
}

impl ProtoNavMesh {
  pub fn clean(&mut self) {
    self.dirty = false;
  }

  pub fn dirty(&mut self) {
    self.dirty = true;
  }

  pub fn is_clean(&self) -> bool {
    !self.dirty
  }

  pub fn is_dirty(&self) -> bool {
    self.dirty
  }

  fn get_index(&mut self, point: Vec3) -> usize {
    let position = self.points.iter().position(|p| {
      approx_eq!(f32, point.x, p.x)
        && approx_eq!(f32, point.y, p.y)
        && approx_eq!(f32, point.z, p.z)
    });

    if let Some(index) = position {
      return index;
    }
    self.points.push(point);
    return self.points.len() - 1;
  }

  pub fn add_triangle(&mut self, entity: Entity, triangle: (Vec3, Vec3, Vec3)) {
    let a = self.get_index(triangle.0);
    let b = self.get_index(triangle.1);
    let c = self.get_index(triangle.2);
    self.triangles.push((a, b, c, entity));
  }

  pub fn remove_entity(&mut self, entity: Entity) {
    self
      .triangles
      .retain(|(_, _, _, triangle_entity)| *triangle_entity != entity);
  }
}

impl Into<NavMesh> for ProtoNavMesh {
  fn into(self) -> NavMesh {
    let points = self.points.iter().map(|p| (p.x, p.y, p.z).into()).collect();
    let triangles = self
      .triangles
      .iter()
      .map(|t| (t.0 as u32, t.1 as u32, t.2 as u32).into())
      .collect();
    NavMesh::new(points, triangles).unwrap()
  }
}
