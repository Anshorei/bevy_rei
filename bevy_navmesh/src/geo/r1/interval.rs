/// An interval represents a continuous range of points
/// but can also be empty.
#[derive(Clone, Debug, PartialEq)]
pub enum Interval {
  Empty,
  Strict(StrictInterval),
}

/// A strict interval is an interval that is guaranteed to be non-empty
/// and therefore includes at the very least a single point.
#[derive(Clone, Debug, PartialEq)]
pub enum StrictInterval {
  Point(f64),
  Points { lo: f64, hi: f64 },
}

impl Interval {
  /// Returns an empty interval.
  pub fn empty() -> Interval {
    Interval::Empty
  }

  /// Returns an interval containing a single point.
  pub fn from_point(p: f64) -> Interval {
    Interval::Strict(StrictInterval::from_point(p))
  }

  /// Returns an interval spanning from lo to hi.
  pub fn from_points(lo: f64, hi: f64) -> Interval {
    StrictInterval::from_points(lo, hi)
      .map_or(Interval::Empty, |interval| Interval::Strict(interval))
  }

  /// Extend the interval to include the given point.
  pub fn extend(self, p: f64) -> Interval {
    match self {
      Interval::Empty => Interval::Strict(StrictInterval::from_point(p)),
      Interval::Strict(strict) => Interval::Strict(strict.extend(p)),
    }
  }

  pub fn intersection(self, other: Self) -> Interval {
    match (self, other) {
      (Interval::Empty, _) | (_, Interval::Empty) => Interval::Empty,
      (Interval::Strict(interval), Interval::Strict(other)) => interval.intersection(&other).map_or(Interval::Empty, |i| Interval::Strict(i)),
    }
  }

  /// Checks whether the interval is empty.
  pub fn is_empty(&self) -> bool {
    match self {
      Interval::Empty => true,
      _ => false,
    }
  }
}

impl StrictInterval {
  /// Returns an interval containing a single point.
  pub fn from_point(p: f64) -> Self {
    Self::Point(p)
  }

  /// Returns an interval containing the given range,
  /// returning none if the given range is invalid.
  pub fn from_points(lo: f64, hi: f64) -> Option<Self> {
    if hi < lo {
      return None;
    }

    Some(Self::Points { lo, hi })
  }

  /// Extend the interval to include the given point.
  pub fn extend(self, p: f64) -> Self {
    match self {
      Self::Point(o) => {
        if o < p {
          Self::Points { lo: o, hi: p }
        } else if o > p {
          Self::Points { lo: p, hi: o }
        } else {
          Self::Point(p)
        }
      }
      Self::Points { lo, hi } => {
        if p < lo {
          Self::Points { lo: p, hi }
        } else if p > hi {
          Self::Points { lo, hi: p }
        } else {
          Self::Points { lo, hi }
        }
      }
    }
  }

  /// Returns the center point of the interval.
  pub fn center(&self) -> f64 {
    match self {
      Self::Point(p) => *p,
      Self::Points { lo, hi } => *lo + (*hi - *lo) / 2.,
    }
  }

  /// Returns the lowest point of the interval.
  pub fn lo(&self) -> f64 {
    match self {
      Self::Point(p) => *p,
      Self::Points { lo, hi: _ } => *lo,
    }
  }

  /// Returns the highest point of the interval.
  pub fn hi(&self) -> f64 {
    match self {
      Self::Point(p) => *p,
      Self::Points { lo: _, hi } => *hi,
    }
  }

  /// Returns the point in the interval that lies
  /// closest to the given point.
  pub fn clamp_point(&self, p: f64) -> f64 {
    match self {
      Self::Point(o) => *o,
      Self::Points { lo, hi } => {
        if p > *hi {
          *hi
        } else if p < *lo {
          *lo
        } else {
          p
        }
      }
    }
  }

  /// Checks whether the interval contains the given point.
  pub fn contains(&self, p: f64) -> bool {
    match self {
      Self::Point(o) => p == *o,
      Self::Points { lo, hi } => p >= *lo && p <= *hi,
    }
  }

  /// Checks whether the interval is a superset of the given interval.
  pub fn contains_interval(&self, other: &Self) -> bool {
    match other {
      Self::Point(p) => self.contains(*p),
      Self::Points { lo, hi } => self.contains(*lo) && self.contains(*hi),
    }
  }

  /// Returns the interval that is the intersection of the two intervals.
  pub fn intersection(&self, other: &Self) -> Option<Self> {
    match self {
      Self::Point(p) => {
        if other.contains(*p) {
          Some(Self::from_point(*p))
        } else {
          None
        }
      }
      Self::Points { lo, hi } => Self::from_points(lo.max(other.lo()), hi.min(other.hi())),
    }
  }

  /// Returns the length of the interval
  pub fn length(&self) -> f64 {
    match self {
      Self::Point(_) => 0.,
      Self::Points { lo, hi } => *hi - *lo,
    }
  }
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn test_intersection() {
    let a = StrictInterval::from_points(1., 5.).unwrap();
    let b = StrictInterval::from_points(3., 7.).unwrap();

    assert_eq!(a.intersection(&b), StrictInterval::from_points(3., 5.));
    assert_eq!(
      Interval::Strict(a).intersection(Interval::Strict(b)),
      Interval::from_points(3., 5.)
    );
  }

  #[test]
  fn test_empty_intersection() {
    let a = StrictInterval::from_points(1., 3.).unwrap();
    let b = StrictInterval::from_points(5., 7.).unwrap();

    assert_eq!(a.intersection(&b), None);
    assert_eq!(
      Interval::Strict(a).intersection(Interval::Strict(b)),
      Interval::Empty
    );
  }
}
