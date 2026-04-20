use crate::{
    drawing::{basic, grid},
    input,
};

pub fn get_grid() -> grid::Grid {
    loop {
        let grid_size = get_grid_size();
        // Добавляем 1 для рамки
        let grid_size = basic::Size(grid_size.0 + 1, grid_size.1 + 1);

        let Ok(grid) = grid::Grid::from(grid_size) else {
            eprint!("Try again");
            continue;
        };

        break grid;
    }
}

fn get_grid_size() -> basic::Size {
    let size = input::input::<basic::Size, _>(
        "Введите размер сетки в формате: ширина высота",
        "Размер должен быть минимум 1x1, только целые числа",
        // |(width, height)| width > 0 && height > 0,
        |size| size.0 > 0 && size.1 > 0,
    );
    size
}
