use crate::drawing::basic::{Point, Size};

type Cell = Option<char>;

#[derive(Debug, PartialEq)]
pub struct Matrix {
    data: Box<[Box<[Cell]>]>,
}

impl Matrix {
    pub fn get_center(&self) -> (u32, u32) {
        let size = self.get_size();
        let center_width = size.0 / 2;
        let center_height = match size.1 % 2 {
            0 => size.1 / 2 - 1,
            _ => size.1 / 2,
        };

        (center_width, center_height)
    }

    pub fn from(size: Size) -> Result<Self, ()> {
        if size.0 < 1 || size.1 < 1 || size.0 > 10000 || size.1 > 10000 {
            return Err(());
        }
        Ok(Self {
            data: (0..size.1)
                .map(|_| vec![None; size.0 as usize].into_boxed_slice())
                .collect::<Vec<_>>()
                .into_boxed_slice(),
        })
    }

    pub fn get_size(&self) -> Size {
        Size(self.data[0].len() as u32, self.data.len() as u32)
    }

    pub fn relative_to_absolute(&self, position: Point) -> Option<(u32, u32)> {
        let center = self.get_center();

        let real_position = (center.0 as i32 + position.0, center.1 as i32 + position.1);

        let size = self.get_size();

        if real_position.0 < 0
            || real_position.0 >= size.0 as i32
            || real_position.1 < 0
            || real_position.1 >= size.1 as i32
        {
            return None;
        }

        let real_position = (real_position.0 as u32, real_position.1 as u32);

        self.cell(real_position).is_some().then_some(real_position)
    }

    pub fn cell<'a>(&'a self, (x, y): (u32, u32)) -> Option<&'a Cell> {
        let Some(row) = self.data.get(y as usize) else {
            return None;
        };

        let Some(cell) = row.get(x as usize) else {
            return None;
        };

        Some(cell)
    }
    pub fn cell_mut<'a>(&'a mut self, (x, y): (u32, u32)) -> Option<&'a mut Cell> {
        let Some(row) = self.data.get_mut(y as usize) else {
            return None;
        };

        let Some(cell) = row.get_mut(x as usize) else {
            return None;
        };

        Some(cell)
    }

    pub fn get_rows(&self) -> &Box<[Box<[Cell]>]> {
        &self.data
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod relative_to_real {
        use super::*;

        #[test]
        fn test1() {
            let matrix = Matrix::from(Size(100, 50)).expect("Ошибка создания");

            assert_eq!(
                (51, 25),
                matrix
                    .relative_to_absolute(Point(1, 1))
                    .expect("Ошибка извлечения позиции")
            )
        }

        #[test]
        fn test2() {
            let matrix = Matrix::from(Size(100, 50)).expect("Ошибка создания");

            assert_eq!(
                (40, 2),
                matrix
                    .relative_to_absolute(Point(-10, -22))
                    .expect("Ошибка извлечения позиции")
            )
        }

        #[test]
        fn test3() {
            let matrix = Matrix::from(Size(100, 50)).expect("Ошибка создания");

            assert_eq!(None, matrix.relative_to_absolute(Point(-100, -22)))
        }
    }

    mod from {
        use super::*;

        #[test]
        fn normal_value() {
            let matrix = Matrix::from(Size(10, 10));

            assert_eq!(Size(10, 10), matrix.unwrap().get_size())
        }

        #[test]
        fn wrong_value() {
            let matrix = Matrix::from(Size(0, 10));

            assert_eq!(Err(()), matrix)
        }
    }
    mod get_center {
        use super::*;
        #[test]
        fn center1() {
            let matrix = Matrix::from(Size(5, 5)).unwrap();

            assert_eq!((2, 2), matrix.get_center())
        }

        #[test]
        fn center2() {
            let matrix = Matrix::from(Size(6, 6)).unwrap();

            assert_eq!((3, 2), matrix.get_center())
        }

        #[test]
        fn center3() {
            let matrix = Matrix::from(Size(6, 5)).unwrap();

            assert_eq!((3, 2), matrix.get_center())
        }

        #[test]
        fn center4() {
            let matrix = Matrix::from(Size(6, 8)).unwrap();

            assert_eq!((3, 3), matrix.get_center())
        }

        #[test]
        fn center5() {
            let matrix = Matrix::from(Size(100, 50)).unwrap();

            assert_eq!((50, 24), matrix.get_center())
        }
    }
    mod cell {
        use super::*;

        #[test]
        fn get_existing_cell() {
            let mut matrix = Matrix::from(Size(10, 10)).unwrap();

            assert_eq!(' ', matrix.cell_mut((9, 9)).unwrap().unwrap())
        }

        #[test]
        fn get_not_existing_cell() {
            let mut matrix = Matrix::from(Size(10, 10)).unwrap();

            assert_eq!(None, matrix.cell_mut((10, 10)))
        }

        #[test]
        fn change_cell() {
            let mut matrix = Matrix::from(Size(10, 10)).unwrap();

            let cell = matrix.cell_mut((5, 5)).unwrap();
            *cell = Some('h');

            assert_eq!(Some(&Some('h')), matrix.cell((5, 5)))
        }
    }
}
