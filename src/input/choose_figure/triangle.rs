use crate::{drawing::basic::Size, figures::triangle::Triangle, input};

pub fn get_gtiangle() -> Triangle {
    let size = input::input::<Size, _>(
        "Введите размер треугольника",
        "Размер не может быть меньше 1х1, только целые числа",
        |&size| size.0 > 0 && size.1 > 0,
    );

    Triangle::from(size, crate::figures::triangle::Location::LeftUp)
}
