use super::line::Line;
use bevy::prelude::*;
use float_cmp::approx_eq;

#[derive(Clone, Debug)]
pub struct Plane {
  point:  Vec3,
  normal: Vec3,
}

impl Plane {
  pub fn new(point: Vec3, normal: Vec3) -> Plane {
    Plane { point, normal }
  }

  pub fn from_points(a: Vec3, b: Vec3, c: Vec3) -> Plane {
    Plane {
      point:  a,
      normal: (a - b).cross(a - c),
    }
  }

  pub fn contains(&self, point: Vec3) -> bool {
    approx_eq!(f32, self.normal.dot(point - self.point), 0.)
  }

  pub fn contains_line(&self, line: &Line) -> bool {
    self.contains(line.point) && self.contains(line.point + line.vec)
  }

  pub fn parallel_to(&self, other: &Plane) -> bool {
    approx_eq!(
      f32,
      self.normal.normalize().dot(other.normal.normalize()).abs(),
      1.
    )
  }

  pub fn coplanar_with(&self, other: &Plane) -> bool {
    self.parallel_to(other) && self.contains(other.point)
  }

  pub fn intersection(&self, other: &Plane) -> Option<PlaneIntersection> {
    if self.coplanar_with(other) {
      return Some(PlaneIntersection::Plane(other.clone()));
    }

    let direction = self.normal.cross(other.normal);

    let w = self.normal.dot(other.normal);
    let divisor = 1. - w.powi(2);
    if divisor < f32::EPSILON.powi(2) {
      return None;
    }
    let origin = Vec3::new(0., 0., 0.) + self.normal * ((other.point * w - self.point) / divisor)
      - other.normal * ((other.point - self.point * w) / divisor);

    return Some(PlaneIntersection::Line(Line {
      point: origin,
      vec:   direction.normalize(),
    }));
  }
}

#[derive(Clone, Debug)]
pub enum PlaneIntersection {
  Line(Line),
  Plane(Plane),
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn contains_origin() {
    let plane = Plane::new(Vec3::new(0., 0., 0.), Vec3::new(5., 3., 2.));

    assert!(plane.contains(plane.point));
  }

  #[test]
  fn contains_point() {
    let point = Vec3::new(-2.0, 0.0, 5.0);
    let plane = Plane::new(Vec3::new(0., 0., 0.), Vec3::new(5., 3., 2.));

    assert!(plane.contains(point));
  }

  #[test]
  fn coplanar_to_self() {
    let plane = Plane::new(Vec3::new(0., 0., 0.), Vec3::new(5., 3., 2.));

    assert!(plane.coplanar_with(&plane.clone()));
  }

  #[test]
  fn not_coplanar_to_parallel() {
    let plane = Plane::new(Vec3::new(0., 0., 0.), Vec3::new(5., 3., 2.));
    let other = Plane::new(Vec3::new(1., 1., 1.), plane.normal.clone());

    assert!(!plane.coplanar_with(&other));
    assert!(!other.coplanar_with(&plane));
  }

  #[test]
  fn parallel_planes() {
    let plane = Plane::new(Vec3::new(0., 0., 0.), Vec3::new(5., 3., 2.));
    let other = Plane::new(Vec3::new(1., 2., 3.), Vec3::new(10., 6., 4.));

    assert!(plane.parallel_to(&other));
    assert!(other.parallel_to(&plane));
  }
}
