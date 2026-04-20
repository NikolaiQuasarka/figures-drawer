use crate::drawing::{
    basic::{Point, Size},
    matrix::Matrix,
};

#[derive(Debug)]
pub struct Grid {
    field: Matrix,
}

impl Grid {
    pub fn from(size: Size) -> Result<Self, ()> {
        let Ok(matrix) = Matrix::from(size) else {
            return Err(());
        };

        Ok(Grid { field: matrix })
    }

    pub fn draw(&mut self, drawing: Matrix, offset: Point) {
        let (x_center, y_center) = self.field.get_center();

        let Size(columns, rows) = drawing.get_size();

        //Вычисляем естественное смещение, чтобы рисовать в центре
        let natural_x_offset = (columns / 2) as i64;
        //Для вертикали также отнимаем 1, так как без этого центр например будет отдаваться приоритет нижней стороне
        let natural_y_offset = (rows / 2) as i64;

        for (y, row) in drawing.get_rows().into_iter().enumerate() {
            for (x, drawing_cell) in row.into_iter().enumerate() {
                if let Some(grid_cell) = self.field.cell_mut((
                    //В качестве отправной точки используем центр матрицы
                    (x_center as i64)
                        //К центру добавляем офсет. Если +5, то рисуем првее, если -5, то левее
                        .saturating_add(offset.0 as i64)
                        //Добавляем номер столбца рисунка.
                        // Очевидно, что точка x+1 должна быть правее, чем x
                        .saturating_add(x as i64)
                        //Добавляем натуральный офсет для центрирования
                        .saturating_sub(natural_x_offset) as u32,
                    (y_center as i64)
                        //Используем sub, так как нумерация строк и столбцов идет слева/сверху вправо/вниз
                        // Это полностью противоположно тому, что нужно сделать
                        .saturating_sub(offset.1 as i64)
                        .saturating_sub(y as i64)
                        .saturating_add(natural_y_offset) as u32,
                )) {
                    if let Some(cell) = drawing_cell {
                        let _ = grid_cell.insert(cell.clone());
                    }
                };
                continue;
            }
        }
    }

    pub fn get_size(&self) -> Size {
        self.field.get_size()
    }
}

impl ToString for Grid {
    fn to_string(&self) -> String {
        self.field
            .get_rows()
            .iter()
            .map(|row| {
                row.iter()
                    .map(|cell| cell.unwrap_or(' '))
                    .collect::<String>()
            })
            .collect::<Vec<_>>()
            .join("\n")
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
            let rectangle = Rectangle::from(Size(4, 4)).unwrap();

            let mut grid = Grid::from(Size(4, 4)).unwrap();

            grid.draw(rectangle.create_drawing().unwrap(), Point::default());

            assert_eq!(rectangle.create_drawing().unwrap(), grid.field)
        }
    }
}
