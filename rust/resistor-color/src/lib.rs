use int_enum::IntEnum;
use enum_iterator::{all, Sequence};

#[repr(u32)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, IntEnum, Sequence)]
pub enum ResistorColor {
    Black = 0,
    Blue = 1,
    Brown = 2,
    Green = 3,
    Grey = 4,
    Orange = 5,
    Red = 6,
    Violet = 7,
    White = 8,
    Yellow = 9,
}

pub fn color_to_value(_color: ResistorColor) -> u32 {
    _color.int_value()
}

pub fn value_to_color_string(value: u32) -> String {
    match ResistorColor::from_value(value) {
         Ok(color) if color == 0 => String::from("Black"),
         Ok(color) if color == 1 => String::from("Blue"),
         Ok(color) if color == 2 => String::from("Brown"),
         Ok(color) if color == 3 => String::from("Green"),
         Ok(color) if color == 4 => String::from("Grey"),
         Ok(color) if color == 5 => String::from("Orange"),
         Ok(color) if color == 6 => String::from("Red"),
         Ok(color) if color == 7 => String::from("Violet"),
         Ok(color) if color == 8 => String::from("White"),
         Ok(color) if color == 9 => String::from("Yellow"),
    }
}

pub fn colors() -> Vec<ResistorColor> {
    all::<ResistorColor>().collect::<Vec<ResistorColor>>()
}
