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

impl Circle {
    pub fn from(radius: u32) -> Self {
        Circle { radius }
    }

    fn calulate_distance_from_center_to_point(&self, basic::Point(x, y): basic::Point) -> u32 {
        let result = (x.pow(2) + y.pow(2)).isqrt() as u32;

        result
    }
}

impl Drawable for Circle {
    fn create_drawing(&self) -> Result<crate::drawing::matrix::Matrix, ()> {
        let size = Size(self.radius * 2 + 1, self.radius * 2 + 1);

        let mut matrix = Matrix::from(size)?;

        for row in (0..size.1).into_iter() {
            for column in (0..size.0).into_iter() {
                let point = matrix.absolute_to_relative((column, row)).ok_or(())?;

                let distance = self.calulate_distance_from_center_to_point(point);

                const MAX_DIFFERENCE: i32 = 0;

                let difference = ((self.radius.wrapping_sub(distance)) as i32).abs();

                if difference > MAX_DIFFERENCE {
                    continue;
                }

                let cell = matrix.cell_mut((column, row)).ok_or(())?;
                let _ = cell.insert('#');
            }
        }

        Ok(matrix)
    }
}
