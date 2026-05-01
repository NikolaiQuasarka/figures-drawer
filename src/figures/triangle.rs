use crate::{
    drawing::{
        basic::{Point, Size},
        matrix::Matrix,
    },
    figures::Drawable,
};

pub enum Location {
    LeftDown,
    LeftUp,
    RightDown,
    RigthUp,
}

pub struct Triangle {
    width: u32,
    height: u32,
    location: Location,
}

impl Triangle {
    pub fn from(size: Size, location: Location) -> Self {
        Self {
            width: size.0,
            height: size.1,
            location,
        }
    }
}

impl Drawable for Triangle {
    fn create_drawing(&self) -> Result<crate::drawing::matrix::Matrix, ()> {
        let mut matrix = Matrix::from(Size(self.width, self.height))?;

        let dx = self.width;
        let dy = self.height;

        let m: f64 = dy as f64 / dx as f64;

        //Рисуем гипотенузу
        for x in 0..self.width {
            let y = m * (x as f64 - 0.0) + 0.0;

            let _ = matrix.cell_mut((x, y as u32)).ok_or(())?.insert('*');
        }

        //Рисуем нижнюю границу
        for x in 0..self.width {
            let _ = matrix.cell_mut((x, self.height - 1)).ok_or(())?.insert('*');
        }

        //Рисуем бокакую границы
        for y in 0..self.height {
            let _ = matrix.cell_mut((0, y)).ok_or(())?.insert('*');
        }

        Ok(matrix)
    }
}
