use bevy::math::Vec3;
use float_cmp::approx_eq;

#[derive(Clone, Debug)]
pub struct Line {
  pub(super) point: Vec3,
  pub(super) vec:   Vec3,
}

impl Line {
  pub fn new(point: Vec3, vec: Vec3) -> Line {
    Line { point, vec }
  }

  pub fn contains(&self, point: Vec3) -> bool {
    approx_eq!(
      f32,
      self
        .vec
        .normalize()
        .dot((point - self.point).normalize())
        .abs(),
      1.
    )
  }

  pub fn parallel_to(&self, line: &Line) -> bool {
    approx_eq!(
      f32,
      self.vec.normalize().dot(line.vec.normalize()).abs(),
      1.
    )
  }

  pub fn intersection(&self, other: &Line) -> Option<LineIntersection> {
    if self.contains(other.point) || other.contains(self.point) {
      if self.parallel_to(&other) {
        return Some(LineIntersection::Line(other.clone()));
      }
      return Some(LineIntersection::Point(self.point));
    }

    if !approx_eq!(
      f32,
      self.vec.cross(other.vec).dot(self.point - other.point),
      0.
    ) {
      // Normal to plane containing both vectors is not
      // orthogonal to line between points of each vector
      return None;
    }

    let g = self.point - other.point;
    let h = self.vec.cross(g);
    let k = self.vec.cross(other.vec);
    if h.length() == 0. || k.length() == 0. {
      return None;
    }
    let l = self.vec * h.length() / k.length();

    let p = self.point + l * h.dot(k).signum();
    return Some(LineIntersection::Point(p));
  }
}

impl From<LineSegment> for Line {
  fn from(segment: LineSegment) -> Self {
    Line::new(segment.point, segment.vec)
  }
}

pub struct LineSegment {
  point: Vec3,
  vec:   Vec3,
}

impl LineSegment {
  pub fn length(&self) -> f32 {
    self.vec.length()
  }
}

pub enum LineIntersection {
  Point(Vec3),
  Line(Line),
}

impl LineIntersection {
  pub fn is_point(&self) -> bool {
    match self {
      Self::Point(_) => true,
      _ => false,
    }
  }
  pub fn point(&self) -> Option<Vec3> {
    match self {
      Self::Point(point) => Some(*point),
      _ => None,
    }
  }
}

pub enum LineSegmentIntersection {
  Point(Vec3),
  LineSegment(LineSegment),
}

#[cfg(test)]
mod test {
  use super::*;
  use bevy::math::Vec3;

  #[test]
  fn line_intersection() {
    let line = Line::new(Vec3::new(6., 8., 4.), Vec3::new(6., 7., 0.));
    let other = Line::new(Vec3::new(6., 8., 2.), Vec3::new(6., 7., 4.));

    assert_eq!(
      line.intersection(&other).unwrap().point().unwrap(),
      Vec3::new(9., 11.5, 4.)
    );
    assert_eq!(
      other.intersection(&line).unwrap().point().unwrap(),
      Vec3::new(9., 11.5, 4.)
    );
  }

  #[test]
  fn line_intersection2() {
    let line = Line::new(Vec3::new(3., -3., 0.), Vec3::new(-7., 0., 4.));
    let other = Line::new(Vec3::new(2., 5., 1.), Vec3::new(-6., -8., 3.));

    assert_eq!(
      line.intersection(&other).unwrap().point().unwrap(),
      Vec3::new(-4., -3., 4.)
    );
    assert_eq!(
      other.intersection(&line).unwrap().point().unwrap(),
      Vec3::new(-4., -3., 4.)
    );
  }

  #[test]
  fn line_no_intersection() {
    let line = Line::new(Vec3::new(3., -3., 1.), Vec3::new(-7., 0., 4.));
    let other = Line::new(Vec3::new(2., 5., 1.), Vec3::new(-6., -8., 3.));

    assert!(line.intersection(&other).is_none());
    assert!(other.intersection(&line).is_none());
  }

  #[test]
  fn parallel_to() {
    let line = Line::new(Vec3::new(3., -3., 1.), Vec3::new(-7., 0., 4.));
    let other = Line::new(Vec3::new(4., -3., 1.), Vec3::new(-14., 0., 8.));

    assert!(line.parallel_to(&other));
    assert!(other.parallel_to(&line));
  }
}
