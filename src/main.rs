use rectangle::{drawing::basic, input};

fn main() {
    app();
}

fn app() {
    let mut grid = input::get_grid::get_grid();

    println!("Сетка создана!");

    let drawing = input::choose_figure::get_drawing()
        .create_drawing()
        .unwrap();

    println!("Рисунок создан");

    grid.draw(drawing, basic::Point::default());

    println!("{}", grid.to_string())
}
