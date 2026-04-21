use crate::{
    drawing::{
        basic::{self, Size},
        matrix::Matrix,
    },
    figures::Drawable,
};

pub struct Circle {
    radius: u32,
}

enum NumberState {
    Equal,
    Lower,
    Bigger,
}

impl Circle {
    pub fn from(radius: u32) -> Self {
        Circle { radius }
    }

    fn is_point_crossing_circle(&self, basic::Point(x, y): basic::Point) -> NumberState {
        let result = (x * 2 + y * 2).isqrt() as u32;

        if result > self.radius {
            NumberState::Bigger
        } else if result < self.radius {
            NumberState::Lower
        } else {
            NumberState::Equal
        }
    }
}

impl Drawable for Circle {
    fn create_drawing(&self) -> Result<crate::drawing::matrix::Matrix, ()> {
        let size = Size(self.radius, self.radius);

        let mut matrix = Matrix::from(size)?;

        for row in (0..size.1).into_iter() {
            for column in (0..size.0).into_iter() {
                let point = matrix.absolute_to_relative((column, row)).ok_or(())?;
                match self.is_point_crossing_circle(point) {
                    NumberState::Equal => {
                        let cell = matrix.cell_mut((column, row)).ok_or(())?;
                        let _ = cell.insert('#');
                    }
                    NumberState::Lower => continue,
                    NumberState::Bigger => continue,
                }
            }
        }

        Ok(matrix)
    }
}
