use bevy::math::Vec3;

pub struct Triangle {
  a: Vec3,
  b: Vec3,
  c: Vec3,
}

impl Triangle {
  pub fn from_points(a: Vec3, b: Vec3, c: Vec3) -> Triangle {
    Triangle { a, b, c }
  }

  pub fn center(&self) -> Vec3 {
    (self.a + self.b + self.c) / 3.
  }

  pub fn area(&self) -> f32 {
    (self.b - self.a).cross(self.c - self.a).length() / 2.
  }

  pub fn perimeter(&self) -> f32 {
    self.a.length() + self.b.length() + self.c.length()
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use bevy::math::Vec3;

  #[test]
  fn triangle_center() {
    let triangle = Triangle::from_points(
      Vec3::new(-1., 0., 0.),
      Vec3::new(1., 0., 0.),
      Vec3::new(0., 2., 1.),
    );

    assert_eq!(triangle.center(), Vec3::new(0., 2. / 3., 1. / 3.));
  }

  #[test]
  fn triangle_area() {
    let triangle = Triangle::from_points(
      Vec3::new(0., 0., 0.),
      Vec3::new(1., 0., 0.),
      Vec3::new(0., 1., 0.),
    );

    assert_eq!(triangle.area(), 0.5);
  }
}
