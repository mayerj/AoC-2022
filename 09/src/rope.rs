use core::fmt;
use std::ops;

pub trait Ropelike {
    fn new(x: i32, y: i32) -> Self;
    fn move_head(&self, direction: char) -> Self;
    fn get_tail(&self) -> &Point;
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Rope {
    pub head_location: Point,
    pub tail_location: Point,
}

#[derive(Clone, Debug, PartialEq)]
pub struct LongRope {
    pub knots: Vec<Point>,
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
    pub fn new_with_tail(head: (i32, i32), tail: (i32, i32)) -> Self {
        return Rope {
            head_location: Point::new(head.0, head.1),
            tail_location: Point::new(tail.0, tail.1),
        };
    }
}

impl LongRope {}

impl Ropelike for Rope {
    fn new(x: i32, y: i32) -> Self {
        return Rope {
            head_location: Point::new(x, y),
            tail_location: Point::new(x, y),
        };
    }

    fn move_head(self: &Self, direction: char) -> Rope {
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

    fn get_tail(self: &Self) -> &Point {
        return &self.tail_location;
    }
}

impl Ropelike for LongRope {
    fn new(x: i32, y: i32) -> Self {
        return LongRope {
            knots: [Point::new(x, y); 10].to_vec(),
        };
    }
    fn move_head(self: &Self, direction: char) -> LongRope {
        let mut knots: Vec<Point> = Vec::new();
        knots.push(*self.knots.get(0).unwrap() + direction);

        for (index, knot) in self.knots[1..self.knots.len()].iter().enumerate() {
            let new_head_location = knots[index];
            let new_tail_location: Point;
            if !new_head_location.is_touching(*knot) {
                new_tail_location = compute_new_tail(&new_head_location, &knot);

                log::debug!(
                    "{:?} - {:?} - {:?} - {:?} - {:?}",
                    self.knots[index],
                    direction,
                    new_head_location,
                    knot,
                    new_tail_location
                );

                assert!(new_head_location.is_touching(new_tail_location));
            } else {
                new_tail_location = knot.clone();
            }

            knots.push(new_tail_location);
        }

        return LongRope { knots: knots };
    }

    fn get_tail(self: &Self) -> &Point {
        return self.knots.last().unwrap();
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
