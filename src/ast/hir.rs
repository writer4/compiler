pub use super::common::*;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Document<'a> {
    pub statements: Vec<Statement<'a>>,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Statement<'a> {
    Header(HeaderStatement<'a>),
    Paragraph(ParagraphStatement<'a>),
    List(ListStatement<'a>),
    HorizontalRule(HorizontalRuleStatement),
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
pub struct ListStatement<'a> {
    pub list: List<'a>,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct HorizontalRuleStatement;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct List<'a> {
    pub items: Vec<ListItem<'a>>,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct ListItem<'a> {
    pub text: Text<'a>,
    pub child: Option<List<'a>>,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Text<'a> {
    pub segments: Vec<TextSegment<'a>>,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum TextSegment<'a> {
    Text(&'a str),
    Break,
    Emphasised {
        emphasis: Emphasis,
        inner: Vec<TextSegment<'a>>,
    },
}
