use super::ast;
use pest::{iterators::Pair, prec_climber::PrecClimber, Parser};
use std::convert::TryFrom;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("parsing error")]
    Pest(#[from] pest::error::Error<Rule>),
}

pub fn parse(code: &str) -> Result<ast::Document<'_>> {
    // Parse code as document pair
    let document = WriterParser::parse(Rule::document, code)?.next().unwrap();

    // Create prec climber
    let prec = PrecClimber::new(vec![]);

    // Parse pair to ast
    Ok(ast::Document::parse(document, &prec)?)
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

impl<'a> Parse<'a> for ast::Document<'a> {
    fn parse(pair: Pair<'a, Rule>, prec: &PrecClimber<Rule>) -> Result<Self> {
        let document = pair.into_inner();

        let statements = document
            .filter(|pair| match pair.as_rule() {
                Rule::statement => true,
                Rule::EOI => false,
                _ => unreachable!(),
            })
            .map(|pair| ast::Statement::parse(pair, prec))
            .collect::<Result<_>>()?;

        Ok(ast::Document { statements })
    }
}

impl<'a> Parse<'a> for ast::Statement<'a> {
    fn parse(pair: Pair<'a, Rule>, _prec: &PrecClimber<Rule>) -> Result<Self> {
        let statement = pair.into_inner().next().unwrap();

        let statement = match statement.as_rule() {
            Rule::empty_line_statement => ast::Statement::EmptyLine(ast::EmptyLineStatement),
            Rule::header_statement => {
                let mut header_statement = statement.into_inner();
                let mut header_type = 0;
                let text = loop {
                    let pair = header_statement.next().unwrap();
                    match pair.as_rule() {
                        Rule::number_sign => header_type += 1,
                        Rule::text => break pair.as_str(),
                        _ => unreachable!(),
                    }
                };

                ast::Statement::Header(ast::HeaderStatement {
                    header_type: ast::HeaderType::try_from(header_type).unwrap(),
                    text,
                })
            }
            Rule::paragraph_statement => {
                let mut paragraph_statement = statement.into_inner();
                let text = {
                    let pair = paragraph_statement.next().unwrap();
                    match pair.as_rule() {
                        Rule::text => pair.as_str(),
                        _ => unreachable!(),
                    }
                };

                ast::Statement::Paragraph(ast::ParagraphStatement { text })
            }
            _ => unreachable!(),
        };

        Ok(statement)
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
            ast::Statement::EmptyLine(ast::EmptyLineStatement) => (),
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
text
"###;

        let _ = parse(code).unwrap();
    }
}
