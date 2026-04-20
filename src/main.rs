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

    drawings.into_iter().for_each(|(drawing, offset)| {
        let drawing = drawing.create_drawing().unwrap();

        grid.draw(drawing, offset);
    });

    println!("{}", grid.to_string())
}
