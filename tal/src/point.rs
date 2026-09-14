use std::{
    fmt::Debug,
    ops::{Add, Mul, Neg, Sub},
};

/// A pair of coordinates representing a location in 2D space.
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct Point {
    pub x: i64,
    pub y: i64,
}

impl Debug for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "P({},{})", self.x, self.y)
    }
}

impl Add<Vector> for Point {
    type Output = Point;
    fn add(self, rhs: Vector) -> Point {
        Point {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Add<Point> for Vector {
    type Output = Point;
    fn add(self, rhs: Point) -> Point {
        Point {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Add<Direction> for Point {
    type Output = Point;
    fn add(self, rhs: Direction) -> Point {
        self + Vector::from(rhs)
    }
}

// Sub defaults to Sub<Self>, i.e. Sub<Point> here
impl Sub for Point {
    type Output = Vector;
    fn sub(self, rhs: Point) -> Self::Output {
        Vector {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl Sub<Vector> for Point {
    type Output = Point;
    fn sub(self, rhs: Vector) -> Self::Output {
        Point {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl Sub<Direction> for Point {
    type Output = Point;
    fn sub(self, rhs: Direction) -> Self::Output {
        self - Vector::from(rhs)
    }
}

/// A pair of coordinates representing a direction and magnitude in 2D space.
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct Vector {
    pub x: i64,
    pub y: i64,
}

impl Debug for Vector {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "V({},{})", self.x, self.y)
    }
}

impl Add for Vector {
    type Output = Vector;
    fn add(self, rhs: Vector) -> Vector {
        Vector {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Add<Direction> for Vector {
    type Output = Vector;
    fn add(self, rhs: Direction) -> Vector {
        self + Vector::from(rhs)
    }
}

impl Sub for Vector {
    type Output = Vector;
    fn sub(self, rhs: Vector) -> Vector {
        Vector {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl Sub<Direction> for Vector {
    type Output = Vector;
    fn sub(self, rhs: Direction) -> Vector {
        self - Vector::from(rhs)
    }
}

impl Mul<i64> for Vector {
    type Output = Vector;
    fn mul(self, rhs: i64) -> Self::Output {
        Vector {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

impl Neg for Vector {
    type Output = Vector;
    fn neg(self) -> Self::Output {
        Vector {
            x: -self.x,
            y: -self.y,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Direction {
    North,
    NorthEast,
    East,
    SouthEast,
    South,
    SouthWest,
    West,
    NorthWest,
}

impl Direction {
    pub const CARDINALS: [Direction; 4] = [
        Direction::North,
        Direction::East,
        Direction::South,
        Direction::West,
    ];
    pub const DIAGONALS: [Direction; 4] = [
        Direction::NorthEast,
        Direction::SouthEast,
        Direction::SouthWest,
        Direction::NorthWest,
    ];
    pub const ALL: [Direction; 8] = [
        Direction::North,
        Direction::NorthEast,
        Direction::East,
        Direction::SouthEast,
        Direction::South,
        Direction::SouthWest,
        Direction::West,
        Direction::NorthWest,
    ];
    pub fn cardinals() -> impl Iterator<Item = Direction> {
        Direction::CARDINALS.into_iter()
    }
    pub fn diagonals() -> impl Iterator<Item = Direction> {
        Direction::DIAGONALS.into_iter()
    }
    pub fn all() -> impl Iterator<Item = Direction> {
        Direction::ALL.into_iter()
    }
    pub const fn rotate_90_cw(&self) -> Direction {
        match self {
            Direction::North => Direction::East,
            Direction::NorthEast => Direction::SouthEast,
            Direction::East => Direction::South,
            Direction::SouthEast => Direction::SouthWest,
            Direction::South => Direction::West,
            Direction::SouthWest => Direction::NorthWest,
            Direction::West => Direction::North,
            Direction::NorthWest => Direction::NorthEast,
        }
    }
    /// Return the ORDINAL (0, 1, 2, 3) of this direction in the list of
    /// CARDINAL directions (North, South, East, West). PANICS if this
    /// is a non-cardinal direction.
    pub fn get_cardinal_ordinal(&self) -> usize {
        match self {
            Direction::North => 0,
            Direction::East => 1,
            Direction::South => 2,
            Direction::West => 3,
            _ => panic!("get_cardinal_ordinal called on non-cardinal direction {self:?}"),
        }
    }
}

impl From<Direction> for Vector {
    fn from(value: Direction) -> Self {
        match value {
            Direction::North => Vector { x: 0, y: -1 },
            Direction::NorthEast => Vector { x: 1, y: -1 },
            Direction::East => Vector { x: 1, y: 0 },
            Direction::SouthEast => Vector { x: 1, y: 1 },
            Direction::South => Vector { x: 0, y: 1 },
            Direction::SouthWest => Vector { x: -1, y: 1 },
            Direction::West => Vector { x: -1, y: 0 },
            Direction::NorthWest => Vector { x: -1, y: -1 },
        }
    }
}

impl Neg for Direction {
    type Output = Direction;
    fn neg(self) -> Self::Output {
        use Direction::*;
        match self {
            North => South,
            NorthEast => SouthWest,
            East => West,
            SouthEast => NorthWest,
            South => North,
            SouthWest => NorthEast,
            West => East,
            NorthWest => SouthEast,
        }
    }
}

impl Mul<i64> for Direction {
    type Output = Vector;
    fn mul(self, rhs: i64) -> Vector {
        Vector::from(self) * rhs
    }
}
