use rectangle::{drawing::basic, input};

fn main() {
    app();
}

fn app() {
    let mut grid = input::get_grid::get_grid();

    println!("Сетка создана!");

    let drawings = input::choose_figure::get_drawings();

    println!("Рисунки созданы");

    drawings.iter().for_each(|drawing| {
        let drawing = drawing.create_drawing().unwrap();

        grid.draw(drawing, basic::Point::default());
    });

    println!("{}", grid.to_string())
}
