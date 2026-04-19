use crate::{figures::rectangle, input};

pub fn get_rectangle() -> rectangle::Rectangle {
    let width = input::input(
        "Введите ширину:",
        "Ширина должна быть больше 0 и целым числом",
        |&width| width > 0,
    );
    let height = input::input(
        "Введите высоту:",
        "Ширина должна быть больше 0 и целым числом",
        |&height| height > 0,
    );

    rectangle::Rectangle::from(width, height).expect("Не удалось создать прямоугольник")
}
