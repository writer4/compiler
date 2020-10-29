pub use super::common::*;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Document<'a> {
    pub statements: Vec<Statement<'a>>,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Statement<'a> {
    EmptyLine(EmptyLineStatement),
    Comment(CommentStatement<'a>),
    Header(HeaderStatement<'a>),
    Paragraph(ParagraphStatement<'a>),
    ListItem(ListItemStatement<'a>),
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct EmptyLineStatement;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct CommentStatement<'a> {
    pub text: &'a str,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct HeaderStatement<'a> {
    pub header_type: HeaderType,
    pub text: Text<'a>,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct ParagraphStatement<'a> {
    pub text: Text<'a>,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct ListItemStatement<'a> {
    pub indentation: usize,
    pub text: Text<'a>,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Text<'a> {
    pub segments: Vec<TextSegment<'a>>,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum TextSegment<'a> {
    Text(&'a str),
    Break,
    Emphasis(Emphasis),
}
