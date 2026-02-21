use crate::drawing::basic::Size;

#[derive(Debug, PartialEq)]
pub struct Matrix<T> {
    data: Box<[Box<[T]>]>,
}

impl Matrix<char> {
    pub fn from(size: Size) -> Result<Self, ()> {
        if size.0 < 1 || size.1 < 1 {
            return Err(());
        }
        Ok(Self {
            data: (0..size.1)
                .map(|_| vec![' '; size.0 as usize].into_boxed_slice())
                .collect::<Vec<_>>()
                .into_boxed_slice(),
        })
    }

    pub fn get_size(&self) -> Size {
        Size(self.data[0].len() as u32, self.data.len() as u32)
    }

    fn get_center(&self) -> (u32, u32) {
        let size = self.get_size();
        let center_width = size.0 / 2 + 1;
        let center_height = size.1 / 2 + 1;

        (center_width, center_height)
    }

    pub fn cell_mut<'a>(&'a mut self, (x, y): (u32, u32)) -> Option<&'a mut char> {
        let Some(row) = self.data.get_mut(y as usize) else {
            return None;
        };

        let Some(cell) = row.get_mut(x as usize) else {
            return None;
        };

        Some(cell)
        // unimplemented!()
    }
}

#[cfg(test)]
mod matrix_tests {
    use super::*;

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

            assert_eq!((3, 3), matrix.get_center())
        }

        #[test]
        fn center2() {
            let matrix = Matrix::from(Size(6, 6)).unwrap();

            assert_eq!((4, 4), matrix.get_center())
        }

        #[test]
        fn center3() {
            let matrix = Matrix::from(Size(6, 5)).unwrap();

            assert_eq!((4, 3), matrix.get_center())
        }

        #[test]
        fn center4() {
            let matrix = Matrix::from(Size(6, 8)).unwrap();

            assert_eq!((4, 5), matrix.get_center())
        }
    }
    mod cell {
        use super::*;

        #[test]
        fn get_existing_cell() {
            let mut matrix = Matrix::from(Size(10, 10)).unwrap();

            assert_eq!(&' ', matrix.cell_mut((9, 9)).unwrap())
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
            *cell = 'h';

            assert_eq!(&'h', matrix.cell_mut((5, 5)).unwrap())
        }
    }
}
