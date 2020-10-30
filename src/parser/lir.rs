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
    let document = Writer4Parser::parse(Rule::document, code)?.next().unwrap();

    // Create prec climber
    let prec = PrecClimber::new(vec![]);

    // Parse pair to ast
    Ok(lir::Document::parse(document, &prec)?)
}

#[derive(pest_derive::Parser)]
#[grammar = "writer4.pest"]
struct Writer4Parser;

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
            Rule::comment_statement => lir::Statement::Comment(lir::CommentStatement {
                text: statement.into_inner().next().unwrap().as_str(),
            }),
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
    fn parse(pair: Pair<'a, Rule>, _: &PrecClimber<Rule>) -> Result<Self> {
        assert!(pair.as_rule() == Rule::text);
        let text = pair.into_inner();

        let mut segments = text
            .map(|pair| match pair.as_rule() {
                Rule::emph_bold => lir::TextSegment::Emphasis(lir::Emphasis::Bold),
                Rule::emph_italic => lir::TextSegment::Emphasis(lir::Emphasis::Italic),
                Rule::emph_strikethrough => {
                    lir::TextSegment::Emphasis(lir::Emphasis::Strikethrough)
                }
                Rule::text_segment => lir::TextSegment::Text(pair.as_str()),
                _ => unreachable!(),
            })
            .collect::<Vec<_>>();

        if let Some(lir::TextSegment::Text(text)) = segments.last_mut() {
            let trailing_white_spaces = text
                .chars()
                .rev()
                .position(|c| c != ' ' && c != '\t')
                .unwrap_or(text.len());

            match trailing_white_spaces {
                0 => (),
                trailing_white_spaces if trailing_white_spaces == text.len() => {
                    segments.pop();
                }
                trailing_white_spaces => *text = &text[0..text.len() - trailing_white_spaces],
            }
        }

        Ok(lir::Text { segments })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn prec() -> PrecClimber<Rule> {
        PrecClimber::new(vec![])
    }

    fn statement_pair(code: &str) -> Pair<Rule> {
        Writer4Parser::parse(Rule::statement, code)
            .unwrap()
            .next()
            .unwrap()
    }

    #[test]
    fn empty_line() {
        let pair = statement_pair(r###""###);
        let expected = lir::Statement::EmptyLine(lir::EmptyLineStatement);
        assert_eq!(lir::Statement::parse(pair, &prec()).unwrap(), expected);

        let pair = statement_pair(r###"     "###);
        let expected = lir::Statement::EmptyLine(lir::EmptyLineStatement);
        assert_eq!(lir::Statement::parse(pair, &prec()).unwrap(), expected);

        let pair = statement_pair(r###"	"###);
        let expected = lir::Statement::EmptyLine(lir::EmptyLineStatement);
        assert_eq!(lir::Statement::parse(pair, &prec()).unwrap(), expected);

        let pair = statement_pair(r###" 		 "###);
        let expected = lir::Statement::EmptyLine(lir::EmptyLineStatement);
        assert_eq!(lir::Statement::parse(pair, &prec()).unwrap(), expected);
    }

    #[test]
    fn comment() {
        let pair = statement_pair(r###"//"###);
        let expected = lir::Statement::Comment(lir::CommentStatement { text: "" });
        assert_eq!(lir::Statement::parse(pair, &prec()).unwrap(), expected);

        let pair = statement_pair(r###"// Hello World"###);
        let expected = lir::Statement::Comment(lir::CommentStatement {
            text: " Hello World",
        });
        assert_eq!(lir::Statement::parse(pair, &prec()).unwrap(), expected);

        let pair = statement_pair(r###"  //xxx"###);
        let expected = lir::Statement::Comment(lir::CommentStatement { text: "xxx" });
        assert_eq!(lir::Statement::parse(pair, &prec()).unwrap(), expected);
    }

    #[test]
    fn header() {
        let pair = statement_pair(r###"# Hello World"###);
        let expected = lir::Statement::Header(lir::HeaderStatement {
            header_type: lir::HeaderType::H1,
            text: lir::Text {
                segments: vec![lir::TextSegment::Text("Hello World")],
            },
        });
        assert_eq!(lir::Statement::parse(pair, &prec()).unwrap(), expected);

        let pair = statement_pair(r###"##   Subtitle"###);
        let expected = lir::Statement::Header(lir::HeaderStatement {
            header_type: lir::HeaderType::H2,
            text: lir::Text {
                segments: vec![lir::TextSegment::Text("Subtitle")],
            },
        });
        assert_eq!(lir::Statement::parse(pair, &prec()).unwrap(), expected);

        let pair = statement_pair(r###"######   __%&%}[{~~__"###);
        let expected = lir::Statement::Header(lir::HeaderStatement {
            header_type: lir::HeaderType::H6,
            text: lir::Text {
                segments: vec![
                    lir::TextSegment::Emphasis(lir::Emphasis::Italic),
                    lir::TextSegment::Text("%&%}[{"),
                    lir::TextSegment::Emphasis(lir::Emphasis::Strikethrough),
                    lir::TextSegment::Emphasis(lir::Emphasis::Italic),
                ],
            },
        });
        assert_eq!(lir::Statement::parse(pair, &prec()).unwrap(), expected);

        let pair = statement_pair(r###"###  ~~Strikethrough~~  "###);
        let expected = lir::Statement::Header(lir::HeaderStatement {
            header_type: lir::HeaderType::H3,
            text: lir::Text {
                segments: vec![
                    lir::TextSegment::Emphasis(lir::Emphasis::Strikethrough),
                    lir::TextSegment::Text("Strikethrough"),
                    lir::TextSegment::Emphasis(lir::Emphasis::Strikethrough),
                ],
            },
        });
        assert_eq!(lir::Statement::parse(pair, &prec()).unwrap(), expected);
    }

    #[test]
    fn paragraph() {
        let pair = statement_pair(r###"lorem ipsum"###);
        let expected = lir::Statement::Paragraph(lir::ParagraphStatement {
            text: lir::Text {
                segments: vec![lir::TextSegment::Text("lorem ipsum")],
            },
        });
        assert_eq!(lir::Statement::parse(pair, &prec()).unwrap(), expected);

        let pair = statement_pair(r###"trailing tabs		"###);
        let expected = lir::Statement::Paragraph(lir::ParagraphStatement {
            text: lir::Text {
                segments: vec![lir::TextSegment::Text("trailing tabs")],
            },
        });
        assert_eq!(lir::Statement::parse(pair, &prec()).unwrap(), expected);

        let pair = statement_pair(r###"   lorem __**ipsum**__! "###);
        let expected = lir::Statement::Paragraph(lir::ParagraphStatement {
            text: lir::Text {
                segments: vec![
                    lir::TextSegment::Text("lorem "),
                    lir::TextSegment::Emphasis(lir::Emphasis::Italic),
                    lir::TextSegment::Emphasis(lir::Emphasis::Bold),
                    lir::TextSegment::Text("ipsum"),
                    lir::TextSegment::Emphasis(lir::Emphasis::Bold),
                    lir::TextSegment::Emphasis(lir::Emphasis::Italic),
                    lir::TextSegment::Text("!"),
                ],
            },
        });
        assert_eq!(lir::Statement::parse(pair, &prec()).unwrap(), expected);

        let pair = statement_pair(r###"#not a header"###);
        let expected = lir::Statement::Paragraph(lir::ParagraphStatement {
            text: lir::Text {
                segments: vec![lir::TextSegment::Text("#not a header")],
            },
        });
        assert_eq!(lir::Statement::parse(pair, &prec()).unwrap(), expected);

        let pair = statement_pair(r###"-not a list"###);
        let expected = lir::Statement::Paragraph(lir::ParagraphStatement {
            text: lir::Text {
                segments: vec![lir::TextSegment::Text("-not a list")],
            },
        });
        assert_eq!(lir::Statement::parse(pair, &prec()).unwrap(), expected);
    }

    #[test]
    fn list_item() {
        let pair = statement_pair(r###"- list item"###);
        let expected = lir::Statement::ListItem(lir::ListItemStatement {
            indentation: 0,
            text: lir::Text {
                segments: vec![lir::TextSegment::Text("list item")],
            },
        });
        assert_eq!(lir::Statement::parse(pair, &prec()).unwrap(), expected);

        let pair = statement_pair(r###"  -     list item"###);
        let expected = lir::Statement::ListItem(lir::ListItemStatement {
            indentation: 2,
            text: lir::Text {
                segments: vec![lir::TextSegment::Text("list item")],
            },
        });
        assert_eq!(lir::Statement::parse(pair, &prec()).unwrap(), expected);

        let pair = statement_pair(r###"   - list **item**"###);
        let expected = lir::Statement::ListItem(lir::ListItemStatement {
            indentation: 3,
            text: lir::Text {
                segments: vec![
                    lir::TextSegment::Text("list "),
                    lir::TextSegment::Emphasis(lir::Emphasis::Bold),
                    lir::TextSegment::Text("item"),
                    lir::TextSegment::Emphasis(lir::Emphasis::Bold),
                ],
            },
        });
        assert_eq!(lir::Statement::parse(pair, &prec()).unwrap(), expected);
    }

    #[test]
    fn empty() {
        let code = r###""###;
        let expected = lir::Document { statements: vec![] };
        assert_eq!(parse(code).unwrap(), expected);

        let code = r###" "###;
        let expected = lir::Document {
            statements: vec![lir::Statement::EmptyLine(lir::EmptyLineStatement)],
        };
        assert_eq!(parse(code).unwrap(), expected);
    }

    #[test]
    fn combined() {
        let code = r###"
# ~~Hello Wor__ld

// a random comment: 2 + 2 = 4
-a,b,c,d,e
lorem ipsum
alpha beta 123!
- A
- ~~B~~
  - C
  - D
  **12345**67890
  //
- E

- F

## h2
..."###;
        let expected = lir::Document {
            statements: vec![
                lir::Statement::EmptyLine(lir::EmptyLineStatement),
                lir::Statement::Header(lir::HeaderStatement {
                    header_type: lir::HeaderType::H1,
                    text: lir::Text {
                        segments: vec![
                            lir::TextSegment::Emphasis(lir::Emphasis::Strikethrough),
                            lir::TextSegment::Text("Hello Wor"),
                            lir::TextSegment::Emphasis(lir::Emphasis::Italic),
                            lir::TextSegment::Text("ld"),
                        ],
                    },
                }),
                lir::Statement::EmptyLine(lir::EmptyLineStatement),
                lir::Statement::Comment(lir::CommentStatement {
                    text: " a random comment: 2 + 2 = 4",
                }),
                lir::Statement::Paragraph(lir::ParagraphStatement {
                    text: lir::Text {
                        segments: vec![lir::TextSegment::Text("-a,b,c,d,e")],
                    },
                }),
                lir::Statement::Paragraph(lir::ParagraphStatement {
                    text: lir::Text {
                        segments: vec![lir::TextSegment::Text("lorem ipsum")],
                    },
                }),
                lir::Statement::Paragraph(lir::ParagraphStatement {
                    text: lir::Text {
                        segments: vec![lir::TextSegment::Text("alpha beta 123!")],
                    },
                }),
                lir::Statement::ListItem(lir::ListItemStatement {
                    indentation: 0,
                    text: lir::Text {
                        segments: vec![lir::TextSegment::Text("A")],
                    },
                }),
                lir::Statement::ListItem(lir::ListItemStatement {
                    indentation: 0,
                    text: lir::Text {
                        segments: vec![
                            lir::TextSegment::Emphasis(lir::Emphasis::Strikethrough),
                            lir::TextSegment::Text("B"),
                            lir::TextSegment::Emphasis(lir::Emphasis::Strikethrough),
                        ],
                    },
                }),
                lir::Statement::ListItem(lir::ListItemStatement {
                    indentation: 2,
                    text: lir::Text {
                        segments: vec![lir::TextSegment::Text("C")],
                    },
                }),
                lir::Statement::ListItem(lir::ListItemStatement {
                    indentation: 2,
                    text: lir::Text {
                        segments: vec![lir::TextSegment::Text("D")],
                    },
                }),
                lir::Statement::Paragraph(lir::ParagraphStatement {
                    text: lir::Text {
                        segments: vec![
                            lir::TextSegment::Emphasis(lir::Emphasis::Bold),
                            lir::TextSegment::Text("12345"),
                            lir::TextSegment::Emphasis(lir::Emphasis::Bold),
                            lir::TextSegment::Text("67890"),
                        ],
                    },
                }),
                lir::Statement::Comment(lir::CommentStatement { text: "" }),
                lir::Statement::ListItem(lir::ListItemStatement {
                    indentation: 0,
                    text: lir::Text {
                        segments: vec![lir::TextSegment::Text("E")],
                    },
                }),
                lir::Statement::EmptyLine(lir::EmptyLineStatement),
                lir::Statement::ListItem(lir::ListItemStatement {
                    indentation: 0,
                    text: lir::Text {
                        segments: vec![lir::TextSegment::Text("F")],
                    },
                }),
                lir::Statement::EmptyLine(lir::EmptyLineStatement),
                lir::Statement::Header(lir::HeaderStatement {
                    header_type: lir::HeaderType::H2,
                    text: lir::Text {
                        segments: vec![lir::TextSegment::Text("h2")],
                    },
                }),
                lir::Statement::Paragraph(lir::ParagraphStatement {
                    text: lir::Text {
                        segments: vec![lir::TextSegment::Text("...")],
                    },
                }),
            ],
        };
        assert_eq!(parse(code).unwrap(), expected);
    }
}
