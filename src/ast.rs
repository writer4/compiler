use std::convert::TryFrom;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Document<'a> {
    pub statements: Vec<Statement<'a>>,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Statement<'a> {
    EmptyLine(EmptyLineStatement),
    Header(HeaderStatement<'a>),
    Paragraph(ParagraphStatement<'a>),
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct EmptyLineStatement;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct HeaderStatement<'a> {
    pub header_type: HeaderType,
    pub text: &'a str,
}

#[derive(Debug, Clone, Eq, PartialEq)]
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

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct ParagraphStatement<'a> {
    pub text: &'a str,
}
