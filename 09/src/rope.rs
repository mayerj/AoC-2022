use core::fmt;
use std::ops;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Rope {
    pub head_location: Point,
    pub tail_location: Point,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct Point {
    x: i32,
    y: i32,
}
impl fmt::Debug for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        return f
            .debug_tuple("point")
            .field(&self.x)
            .field(&self.y)
            .finish();
    }
}

impl Rope {
    pub fn new(x: i32, y: i32) -> Self {
        return Rope {
            head_location: Point::new(x, y),
            tail_location: Point::new(x, y),
        };
    }
    pub fn new_with_tail(head: (i32, i32), tail: (i32, i32)) -> Self {
        return Rope {
            head_location: Point::new(head.0, head.1),
            tail_location: Point::new(tail.0, tail.1),
        };
    }

    pub fn move_head(self: Self, direction: char) -> Rope {
        let new_head_location = self.head_location + direction;

        let new_tail_location: Point;
        if !new_head_location.is_touching(self.tail_location) {
            new_tail_location = compute_new_tail(&new_head_location, &self.tail_location);

            log::debug!(
                "{:?} - {:?} - {:?} - {:?} - {:?}",
                self.head_location,
                direction,
                new_head_location,
                self.tail_location,
                new_tail_location
            );

            assert!(new_head_location.is_touching(new_tail_location));
        } else {
            new_tail_location = self.tail_location.clone();
        }

        return Rope {
            head_location: new_head_location,
            tail_location: new_tail_location,
        };
    }
}

impl Point {
    pub fn new(x: i32, y: i32) -> Self {
        return Point { x: x, y: y };
    }

    pub fn manhattan(self: Self, other: Point) -> u32 {
        return self.x.abs_diff(other.x) + self.y.abs_diff(other.y);
    }

    pub fn is_touching(self: Self, other: Point) -> bool {
        let distance = f32::floor(f32::sqrt(
            (other.x - self.x).pow(2) as f32 + (other.y - self.y).pow(2) as f32,
        ));

        log::debug!(
            "{:?} is_touching {:?} == {} ({})",
            self,
            other,
            distance,
            1f32 >= distance
        );

        return 1f32 >= distance;
    }
}

impl ops::Add<char> for Point {
    type Output = Point;

    fn add(self, rhs: char) -> Point {
        return match rhs {
            'R' => Point::new(self.x + 1, self.y),
            'L' => Point::new(self.x - 1, self.y),
            'D' => Point::new(self.x, self.y - 1),
            'U' => Point::new(self.x, self.y + 1),
            _ => panic!("unknown direction {}", rhs),
        };
    }
}

fn compute_new_tail(head: &Point, tail: &Point) -> Point {
    if head.x == tail.x {
        if tail.y > head.y {
            return Point {
                x: tail.x,
                y: tail.y - 1,
            };
        } else {
            return Point {
                x: tail.x,
                y: tail.y + 1,
            };
        }
    } else if head.y == tail.y {
        if tail.x > head.x {
            return Point {
                x: tail.x - 1,
                y: tail.y,
            };
        } else {
            return Point {
                x: tail.x + 1,
                y: tail.y,
            };
        }
    } else {
        if head.x > tail.x && head.y > tail.y {
            return Point {
                x: tail.x + 1,
                y: tail.y + 1,
            };
        } else if head.x > tail.x && head.y < tail.y {
            return Point {
                x: tail.x + 1,
                y: tail.y - 1,
            };
        } else if head.x < tail.x && head.y > tail.y {
            return Point {
                x: tail.x - 1,
                y: tail.y + 1,
            };
        } else if head.x < tail.x && head.y < tail.y {
            return Point {
                x: tail.x - 1,
                y: tail.y - 1,
            };
        }
    }
    panic!("???")
}
