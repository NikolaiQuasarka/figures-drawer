use crate::{
    drawing::{basic::Size, matrix::Matrix},
    figures::Drawable,
};

pub struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    pub fn from(Size(width, height): Size) -> Result<Self, ()> {
        if width < 1 || height < 1 {
            return Err(());
        }

        Ok(Self { width, height })
    }
}

impl Drawable for Rectangle {
    fn create_drawing(&self) -> Result<Matrix<char>, ()> {
        let mut matrix = Matrix::from(Size(self.width, self.height))?;
        for y in 0..self.height {
            for x in 0..self.width {
                let mut cell_char = ' ';
                if y == 0 || y == self.height - 1 || x == 0 || x == self.width - 1 {
                    cell_char = '*';
                }
                let Some(cell) = matrix.cell_mut((x, y)) else {
                    return Err(());
                };
                *cell = cell_char;
            }
        }

        Ok(matrix)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    mod from {
        use super::*;

        #[test]
        fn zero_size() {
            let rectangle = Rectangle::from(Size(0, 0));

            assert!(rectangle.is_err())
        }

        #[test]
        fn width_wrong() {
            let rectangle = Rectangle::from(Size(0, 4));

            assert!(rectangle.is_err())
        }

        #[test]
        fn height_wrong() {
            let rectangle = Rectangle::from(Size(7, 0));

            assert!(rectangle.is_err())
        }

        #[test]
        fn correct_size() {
            let rectangle = Rectangle::from(Size(7, 8));

            assert!(rectangle.is_ok())
        }
    }

    mod create_drawing {
        use super::*;

        fn matrix_to_string(matrix: Matrix<char>) -> String {
            matrix
                .get_rows()
                .iter()
                .map(|row| row.iter().collect::<String>())
                .collect()
        }

        #[test]
        fn five_x_five() {
            let rectangle = Rectangle::from(Size(5, 5))
                .unwrap()
                .create_drawing()
                .unwrap();

            let rectangle_string = "******   **   **   ******";

            assert_eq!(rectangle_string, matrix_to_string(rectangle))
        }

        #[test]
        fn one_x_one() {
            let rectangle = Rectangle::from(Size(1, 1))
                .unwrap()
                .create_drawing()
                .unwrap();

            let rectangle_string = "*";

            assert_eq!(rectangle_string, matrix_to_string(rectangle))
        }

        #[test]
        fn ten_x_ten() {
            let rectangle = Rectangle::from(Size(10, 10))
                .unwrap()
                .create_drawing()
                .unwrap();

            let rectangle_string = "***********        **        **        **        **        **        **        **        ***********";

            assert_eq!(rectangle_string, matrix_to_string(rectangle))
        }
    }
}
