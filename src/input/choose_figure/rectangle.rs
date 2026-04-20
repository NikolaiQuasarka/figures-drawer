use crate::{drawing::basic, figures::rectangle, input};

pub fn get_rectangle() -> rectangle::Rectangle {
    let size = input::input::<basic::Size, _>(
        "Введите размер в формате: ширина высота",
        "Не меньше 1x1, только целые числа",
        |size| size.0 > 0 && size.1 > 0,
    );

    rectangle::Rectangle::from(size).expect("Не удалось создать прямоугольник")
}
