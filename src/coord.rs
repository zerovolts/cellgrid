use std::{
    fmt,
    ops::{Add, AddAssign, Sub, SubAssign},
    str::FromStr,
};

/// The coordinate key to a specific [`Grid`](crate::grid::Grid) cell.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Coord {
    pub x: i32,
    pub y: i32,
}

#[derive(Debug, PartialEq)]
pub enum ParseCoordError {
    InvalidDimensions,
    InvalidDigit,
}

impl Coord {
    pub fn new(x: i32, y: i32) -> Self {
        Coord { x, y }
    }
}

impl Add<Coord> for Coord {
    type Output = Coord;

    fn add(self, rhs: Coord) -> Self::Output {
        Coord::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl AddAssign<Coord> for Coord {
    fn add_assign(&mut self, rhs: Coord) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl Sub<Coord> for Coord {
    type Output = Coord;

    fn sub(self, rhs: Coord) -> Self::Output {
        Coord::new(self.x - rhs.x, self.y - rhs.y)
    }
}

impl SubAssign<Coord> for Coord {
    fn sub_assign(&mut self, rhs: Coord) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}

impl From<(i32, i32)> for Coord {
    fn from((x, y): (i32, i32)) -> Self {
        Coord::new(x, y)
    }
}

impl From<Coord> for (i32, i32) {
    fn from(Coord { x, y }: Coord) -> Self {
        (x, y)
    }
}

impl FromStr for Coord {
    type Err = ParseCoordError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // Parse standard `(x, y)` format.
        let xy_vec = s
            .trim_matches(|p| p == '(' || p == ')')
            .split(',')
            .map(|s| s.trim())
            .collect::<Vec<_>>();

        if xy_vec.len() != 2 {
            return Err(ParseCoordError::InvalidDimensions);
        }

        let parsed_xy = xy_vec
            .iter()
            .map(|x| x.parse::<i32>().map_err(|_| ParseCoordError::InvalidDigit))
            .collect::<Result<Vec<_>, _>>()?;
        Ok(Coord::new(parsed_xy[0], parsed_xy[1]))
    }
}

impl fmt::Display for Coord {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn coord_parse() {
        let coord_str = "(0, 0)";
        // Ill-formatted but parseable coord strings
        // TODO: Restrict accepted coord format
        let bad_parens_coord_str = ")0, 0(";
        let none_parens_coord_str = "0, 0";
        assert_eq!(coord_str.parse(), Ok(Coord::new(0, 0)));
        assert_eq!(bad_parens_coord_str.parse(), Ok(Coord::new(0, 0)));
        assert_eq!(none_parens_coord_str.parse(), Ok(Coord::new(0, 0)));
    }

    #[test]
    fn neg_coord_parse() {
        let coord_str = "(-1, -1)";
        assert_eq!(coord_str.parse(), Ok(Coord::new(-1, -1)));
    }

    #[test]
    fn flexible_spacing_coord_parse() {
        let coord_str = "(0   , 0   )";
        assert_eq!(coord_str.parse(), Ok(Coord::new(0, 0)));
    }

    #[test]
    fn coord_parse_invalid_digit() {
        let coord_str = "(x, y)";
        let newline_coord_str = "(0, 0)\n";
        assert!(coord_str.parse::<Coord>() == Err(ParseCoordError::InvalidDigit));
        assert!(newline_coord_str.parse::<Coord>() == Err(ParseCoordError::InvalidDigit));
    }

    #[test]
    fn coord_parse_invalid_dimensions() {
        let insufficient_coord_str = "(0)";
        let excessive_coord_str = "(1, 2, 3)";
        assert!(insufficient_coord_str.parse::<Coord>() == Err(ParseCoordError::InvalidDimensions));
        assert!(excessive_coord_str.parse::<Coord>() == Err(ParseCoordError::InvalidDimensions));
    }
}
