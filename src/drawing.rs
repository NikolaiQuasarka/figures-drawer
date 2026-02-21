use crate::utils::{i32_to_u32, i32_to_usize};

pub struct Size(pub u32, pub u32);

#[derive(Debug)]
pub struct Point(pub i32, pub i32);

#[derive(Debug)]
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
        Size(self.data[0][0] as u32, self.data.len() as u32)
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

#[derive(Debug)]
pub struct Grid {
    field: Box<[Box<[char]>]>,
    center: Point,
}

impl Grid {
    pub fn new(size: Size, center: Point) -> Self {
        let field = (0..size.1)
            .map(|_| vec![' '; size.0 as usize].into_boxed_slice())
            .collect::<Vec<_>>()
            .into_boxed_slice();
        Grid { field, center }
    }
}
