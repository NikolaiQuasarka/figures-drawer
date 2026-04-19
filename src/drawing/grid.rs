use crate::drawing::{
    basic::{Point, Size},
    matrix::Matrix,
};

#[derive(Debug)]
pub struct Grid {
    field: Matrix<char>,
    // center: Point,
}

impl Grid {
    pub fn from(size: Size) -> Result<Self, ()> {
        let Ok(matrix) = Matrix::from(size) else {
            return Err(());
        };

        Ok(Grid { field: matrix })
    }

    pub fn draw(&mut self, drawing: Matrix<char>, offset: Point) {
        for (y, row) in drawing.get_rows().into_iter().enumerate() {
            for (x, draw_cell) in row.into_iter().enumerate() {
                let Some(grid_cell) = self.field.cell_mut((
                    //Используем sub, так как нумерация строк и столбцов идет слева/сверху вправо/вниз
                    // Это полностью противоположно тому, что нужно сделать
                    (x as i64).saturating_add(offset.0 as i64) as u32,
                    (y as i64).saturating_sub(offset.1 as i64) as u32,
                )) else {
                    continue;
                };
                *grid_cell = *draw_cell;
            }
        }
    }
}

impl ToString for Grid {
    fn to_string(&self) -> String {
        self.field
            .get_rows()
            .iter()
            .map(|row| {
                let mut row = row.iter().collect::<String>();
                row.push('\n');
                row
            })
            .collect::<Vec<_>>()
            .join("")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod from {
        use super::*;

        #[test]
        fn positive_size_succeeds() {
            let grid = Grid::from(Size(5, 5));
            assert!(grid.is_ok());
        }

        #[test]
        fn zero_width_fails() {
            let size = Size(0, 5);
            let grid = Grid::from(size);
            assert!(grid.is_err());
        }

        #[test]
        fn zero_height_fails() {
            let size = Size(5, 0);
            let grid = Grid::from(size);
            assert!(grid.is_err());
        }

        #[test]
        fn zero_both_fails() {
            let size = Size(0, 0);
            let grid = Grid::from(size);
            assert!(grid.is_err());
        }

        #[test]
        fn max_value_succeeds() {
            let grid = Grid::from(Size(10001, 10000));
            assert!(grid.is_err());
        }
    }

    mod to_string {
        use super::*;

        #[test]
        fn empty_grid() {
            let grid = Grid::from(Size(5, 5)).unwrap();

            let string = (0..5)
                .map(|_| {
                    let mut string_vec = vec![' '; 5];
                    string_vec.push('\n');
                    string_vec.into_iter().collect::<String>()
                })
                .collect::<String>();

            assert_eq!(string, grid.to_string())
        }
    }

    mod draw {
        use super::*;
        use crate::figures::{Drawable, rectangle::Rectangle};

        #[test]
        fn rectangle() {
            let rectangle = Rectangle::from(4, 4).unwrap();

            let mut grid = Grid::from(Size(4, 4)).unwrap();

            grid.draw(rectangle.create_drawing().unwrap(), Point::default());

            assert_eq!(rectangle.create_drawing().unwrap(), grid.field)
        }
    }
}
