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

    fn mid_point_alghorithm(&self) -> Matrix {
        let radius = self.radius;

        let size = Size(radius * 2 + 1, radius * 2 + 1);

        let mut matrix = Matrix::from(size).unwrap();

        let mut x = 0;
        let mut y = radius as i32;
        let mut d = 1i32.wrapping_sub(radius as i32);

        while x <= y {
            let points = [
                (x, y),
                (y, x),
                (-x, y),
                (-y, x),
                (x, -y),
                (y, -x),
                (-x, -y),
                (-y, -x),
            ];

            for (x, y) in points {
                let absolute_position = matrix.relative_to_absolute(basic::Point(x, y)).unwrap();

                let cell = matrix
                    .cell_mut(absolute_position)
                    .expect("Точка несуществует");

                let _ = cell.insert('*');
            }

            if d < 0 {
                d += 2 * x + 3;
            } else {
                d += 2 * (x - y) + 5;
                y -= 1;
            }

            x += 1;
        }

        matrix
    }
}

impl Drawable for Circle {
    fn create_drawing(&self) -> Result<crate::drawing::matrix::Matrix, ()> {
        let matrix = self.mid_point_alghorithm();
        Ok(matrix)
    }
}
