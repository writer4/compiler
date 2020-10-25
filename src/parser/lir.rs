use crate::ast::lir;
use pest::{iterators::Pair, prec_climber::PrecClimber, Parser};
use std::convert::TryFrom;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("parsing error")]
    Pest(#[from] pest::error::Error<Rule>),
}

pub fn parse(code: &str) -> Result<lir::Document<'_>> {
    // Parse code as document pair
    let document = WriterParser::parse(Rule::document, code)?.next().unwrap();

    // Create prec climber
    let prec = PrecClimber::new(vec![]);

    // Parse pair to ast
    Ok(lir::Document::parse(document, &prec)?)
}

#[derive(pest_derive::Parser)]
#[grammar = "writer.pest"]
struct WriterParser;

trait Parse<'a>: Sized {
    /// # Panics
    ///
    /// Panics on invalid `pair`.
    fn parse(pair: Pair<'a, Rule>, prec: &PrecClimber<Rule>) -> Result<Self>;
}

impl<'a, T> Parse<'a> for Vec<T>
where
    T: Parse<'a>,
{
    fn parse(pair: Pair<'a, Rule>, prec: &PrecClimber<Rule>) -> Result<Self> {
        let pairs = pair.into_inner();

        Ok(pairs
            .map(|pair| T::parse(pair, prec))
            .collect::<Result<_>>()?)
    }
}

impl<'a> Parse<'a> for lir::Document<'a> {
    fn parse(pair: Pair<'a, Rule>, prec: &PrecClimber<Rule>) -> Result<Self> {
        let document = pair.into_inner();

        let statements = document
            .filter(|pair| match pair.as_rule() {
                Rule::statement => true,
                Rule::EOI => false,
                _ => unreachable!(),
            })
            .map(|pair| lir::Statement::parse(pair, prec))
            .collect::<Result<_>>()?;

        Ok(lir::Document { statements })
    }
}

impl<'a> Parse<'a> for lir::Statement<'a> {
    fn parse(pair: Pair<'a, Rule>, prec: &PrecClimber<Rule>) -> Result<Self> {
        let statement = pair.into_inner().next().unwrap();

        let statement = match statement.as_rule() {
            Rule::empty_line_statement => lir::Statement::EmptyLine(lir::EmptyLineStatement),
            Rule::header_statement => {
                let mut header_statement = statement.into_inner();
                let mut header_type = 0;
                let text = loop {
                    let pair = header_statement.next().unwrap();
                    match pair.as_rule() {
                        Rule::number_sign => header_type += 1,
                        Rule::text => break lir::Text::parse(pair, prec)?,
                        _ => unreachable!(),
                    }
                };

                lir::Statement::Header(lir::HeaderStatement {
                    header_type: lir::HeaderType::try_from(header_type).unwrap(),
                    text,
                })
            }
            Rule::paragraph_statement => lir::Statement::Paragraph(lir::ParagraphStatement {
                text: lir::Text::parse(statement.into_inner().next().unwrap(), prec)?,
            }),
            Rule::list_item_statement => {
                let mut list_item_statement = statement.into_inner();

                let indentation = list_item_statement.next().unwrap();
                let text = match list_item_statement.next() {
                    Some(pair) if pair.as_rule() == Rule::text => lir::Text::parse(pair, prec)?,
                    _ => lir::Text {
                        segments: Vec::new(),
                    },
                };

                lir::Statement::ListItem(lir::ListItemStatement {
                    indentation: indentation.as_str().len(),
                    text,
                })
            }
            _ => unreachable!(),
        };

        Ok(statement)
    }
}

impl<'a> Parse<'a> for lir::Text<'a> {
    fn parse(pair: Pair<'a, Rule>, prec: &PrecClimber<Rule>) -> Result<Self> {
        assert!(pair.as_rule() == Rule::text);
        let text = pair.into_inner();

        let segments = text
            .map(|pair| match pair.as_rule() {
                Rule::emph_bold => lir::TextSegment::Emphasis(lir::Emphasis::Bold),
                Rule::emph_italic => lir::TextSegment::Emphasis(lir::Emphasis::Italic),
                Rule::emph_strikethrough => {
                    lir::TextSegment::Emphasis(lir::Emphasis::Strikethrough)
                }
                Rule::text_segment => lir::TextSegment::Text(pair.as_str()),
                _ => unreachable!(),
            })
            .collect();

        Ok(lir::Text { segments })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn prec() -> PrecClimber<Rule> {
        PrecClimber::new(vec![])
    }

    #[test]
    fn empty_line() {
        let code = r###" "###;
        let pair = WriterParser::parse(Rule::statement, code)
            .unwrap()
            .next()
            .unwrap();

        match Parse::parse(pair, &prec()).unwrap() {
            lir::Statement::EmptyLine(lir::EmptyLineStatement) => (),
            _ => panic!(),
        }
    }

    #[test]
    fn empty() {
        let code = r###""###;

        let _ = parse(code).unwrap();
    }

    #[test]
    fn simple() {
        let code = r###"# H1

lorem ipsum
alpha beta 123!

## h2
text"###;

        let _ = parse(code).unwrap();
    }
}
