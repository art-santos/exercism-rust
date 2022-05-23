use int_enum::IntEnum;
use enum_iterator::IntoEnumIterator;


#[repr(usize)]
#[derive(Clone, Copy, Debug, Eq, PartialEq, IntEnum, IntoEnumIterator)]
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

pub fn color_to_value(_color: ResistorColor) -> usize {
    return _color as usize
}

pub fn value_to_color_string(value: usize) -> String {
    return match ResistorColor::from_int(value) {
        Ok(resistor) => format!("{:?}", resistor),
        Err(_) => "value out of range".to_string()
    }
}

pub fn colors() -> Vec<ResistorColor> {
    return ResistorColor::into_enum_iter().collect()
}
