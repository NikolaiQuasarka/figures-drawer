use crate::{figures::circle, input};

pub fn get_circle() -> circle::Circle {
    let radius = input::input::<u32, _>(
        "Введите радиус круга:",
        "Не меньше 1, только целые числа",
        |&radius| radius > 0,
    );

    circle::Circle::from(radius)
}
