use crate::coord::Coord;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Line {
    from: Coord,
    to: Coord,
}

impl Line {
    pub fn new<C1, C2>(from: C1, to: C2) -> Self
    where
        C1: Into<Coord>,
        C2: Into<Coord>,
    {
        Self {
            from: from.into(),
            to: to.into(),
        }
    }

    /// Traces Bresenham's line algorithm between `from` and `to`.
    pub fn iter(&self) -> impl Iterator<Item = Coord> {
        let delta = self.to - self.from;
        let x_step = Coord::new(delta.x.signum(), 0);
        let y_step = Coord::new(0, delta.y.signum());
        let x_is_major = delta.x.abs() > delta.y.abs();

        let (major_step, minor_step) = if x_is_major {
            (x_step, y_step)
        } else {
            (y_step, x_step)
        };

        let (major_fault, minor_fault) = if x_is_major {
            (delta.x.abs(), delta.y.abs())
        } else {
            (delta.y.abs(), delta.x.abs())
        };

        LineIter {
            end_coord: self.to,
            next_coord: self.from,
            major_step,
            minor_step,
            fault: major_fault as f32 / 2.0,
            major_fault,
            minor_fault,
            is_finished: false,
        }
    }
}

pub struct LineIter {
    end_coord: Coord,
    next_coord: Coord,
    // Added to the coordinate every iteration.
    major_step: Coord,
    // Added to the coordinate when `fault` falls below zero.
    minor_step: Coord,
    fault: f32,
    // Amount to add to `fault` when it falls below zero.
    major_fault: i32,
    // Amount to subtract from `fault` every iteration.
    minor_fault: i32,
    is_finished: bool,
}

impl Iterator for LineIter {
    type Item = Coord;

    fn next(&mut self) -> Option<Coord> {
        if self.is_finished {
            return None;
        }
        if self.next_coord == self.end_coord {
            self.is_finished = true;
            return Some(self.end_coord);
        }

        // We return the coordinate computed on the previous iteration.
        let return_coord = self.next_coord;

        self.next_coord += self.major_step;

        self.fault -= self.minor_fault as f32;
        // The choice of < over <= here seems arbitrary. The step patterns they
        // produce are mirror images of each other, for example:
        //  < 0.0 -- 3-4-4-5-4-3
        // <= 0.0 -- 3-4-5-4-4-3
        if self.fault < 0.0 {
            self.fault += self.major_fault as f32;
            self.next_coord += self.minor_step;
        }

        Some(return_coord)
    }
}
