use rectangle::{
    drawing::basic::{self, Size},
    figures::{Drawable, rectangle::Rectangle},
    input,
};

fn main() {
    app();
}

fn app() {
    let mut grid = input::get_grid::get_grid();

    // let grid = Grid::from(rectangle::drawing::basic::Size(grid.0 + 1, grid.1 + 1));

    println!("Сетка создана!");

    let drawings = input::choose_figure::get_drawings();

    println!("Рисунки созданы");

    grid.draw(
        Rectangle::from(Size(grid.get_size().0, grid.get_size().1))
            .unwrap()
            .create_drawing()
            .unwrap(),
        basic::Point::default(),
    );

    drawings.iter().for_each(|drawing| {
        let drawing = drawing.create_drawing().unwrap();

        let offset = input::get_offset::get_offset();

        grid.draw(drawing, offset);
    });

    println!("{}", grid.to_string())
}
