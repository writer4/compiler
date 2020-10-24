use std::convert::TryFrom;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Emphasis {
    Bold,
    Italic,
    Strikethrough,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum HeaderType {
    H1,
    H2,
    H3,
    H4,
    H5,
    H6,
}

impl TryFrom<u32> for HeaderType {
    type Error = &'static str;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(HeaderType::H1),
            2 => Ok(HeaderType::H2),
            3 => Ok(HeaderType::H3),
            4 => Ok(HeaderType::H4),
            5 => Ok(HeaderType::H5),
            6 => Ok(HeaderType::H6),
            _ => Err("invalid header"),
        }
    }
}
