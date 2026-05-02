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

        draw_line(&mut matrix, (0, self.height - 1), (self.width - 1, 0));

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

fn draw_line(matrix: &mut Matrix, start: (u32, u32), end: (u32, u32)) {
    let height = matrix.get_size().1 as f64;

    // Инвертируем Y на входе: математика → матрица (где 0 сверху)
    let (x0, y0) = (start.0 as f64, height - 1.0 - start.1 as f64);
    let (x1, y1) = (end.0 as f64, height - 1.0 - end.1 as f64);

    let dx = x1 - x0;
    let dy = y1 - y0;

    let n = (dx.abs().max(dy.abs())) as i32;
    if n == 0 {
        return;
    }

    let x_step = dx / n as f64;
    let y_step = dy / n as f64;

    for step in 0..=n {
        let x = (x0 + x_step * step as f64).round() as u32;
        let y = (y0 + y_step * step as f64).round() as u32;

        if let Some(cell) = matrix.cell_mut((x, y)) {
            let _ = cell.insert('*');
        }
    }
}
