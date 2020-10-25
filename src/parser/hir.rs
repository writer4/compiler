use crate::ast::{hir, lir};

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, thiserror::Error)]
pub enum Error {}

pub fn parse<'a>(document: &lir::Document<'a>) -> Result<hir::Document<'a>> {
    let mut statements = Vec::new();

    let mut idx = 0;
    while idx < document.statements.len() {
        match &document.statements[idx] {
            lir::Statement::EmptyLine(_) => idx += 1,
            lir::Statement::Header(header_stmt) => {
                statements.push(hir::Statement::Header(parse_header_statement(header_stmt)?));
                idx += 1;
            }
            lir::Statement::Paragraph(paragraph_stmt) => {
                let mut paragraphs = vec![paragraph_stmt];
                idx += 1;

                while idx < document.statements.len() {
                    if let lir::Statement::Paragraph(paragraph_stmt) = &document.statements[idx] {
                        paragraphs.push(paragraph_stmt);
                        idx += 1;
                    } else {
                        break;
                    }
                }

                statements.push(hir::Statement::Paragraph(parse_paragraph_statement(
                    &paragraphs,
                )?));
            }
            lir::Statement::ListItem(list_item_stmt) => {
                let mut list_statements = vec![Either::Left(list_item_stmt)];
                idx += 1;

                while idx < document.statements.len() {
                    match &document.statements[idx] {
                        lir::Statement::ListItem(list_item_stmt) => {
                            list_statements.push(Either::Left(list_item_stmt));
                            idx += 1;
                        }
                        lir::Statement::Paragraph(paragraph_stmt) => {
                            list_statements.push(Either::Right(paragraph_stmt));
                            idx += 1;
                        }
                        _ => break,
                    }
                }

                statements.push(hir::Statement::List(hir::ListStatement {
                    list: parse_list(&list_statements)?,
                }));
            }
        }
    }

    Ok(hir::Document { statements })
}

#[derive(Debug)]
enum Either<L, R> {
    Left(L),
    Right(R),
}

fn parse_header_statement<'a>(
    header_statement: &lir::HeaderStatement<'a>,
) -> Result<hir::HeaderStatement<'a>> {
    Ok(hir::HeaderStatement {
        header_type: header_statement.header_type,
        text: hir::Text {
            segments: parse_text_segments(&header_statement.text.segments)?,
        },
    })
}

fn parse_paragraph_statement<'a>(
    paragraphs: &[&lir::ParagraphStatement<'a>],
) -> Result<hir::ParagraphStatement<'a>> {
    // Collect into one vec
    let segments = {
        let mut segments = Vec::new();

        for i in 0..paragraphs.len() {
            segments.extend(paragraphs[i].text.segments.clone());
            if i != paragraphs.len() - 1 {
                segments.push(lir::TextSegment::Break);
            }
        }

        segments
    };

    Ok(hir::ParagraphStatement {
        text: hir::Text {
            segments: parse_text_segments(&segments)?,
        },
    })
}

fn parse_list<'a>(
    statements: &[Either<&lir::ListItemStatement<'a>, &lir::ParagraphStatement<'a>>],
) -> Result<hir::List<'a>> {
    let mut items = vec![];

    let mut idx = 0;
    while idx < statements.len() {
        match statements[idx] {
            Either::Left(lir::ListItemStatement { indentation, text }) => {
                idx += 1;

                let mut text_segments = parse_text_segments(&text.segments)?;

                while let Some(Either::Right(lir::ParagraphStatement { text })) =
                    statements.get(idx)
                {
                    text_segments.push(hir::TextSegment::Break);
                    text_segments.extend(parse_text_segments(&text.segments)?);
                    idx += 1;
                }

                let child = match statements.get(idx) {
                    Some(Either::Left(list_item_stmt))
                        if list_item_stmt.indentation >= indentation + 2 =>
                    {
                        let start = idx;
                        idx += 1;

                        while let Some(stmt) = statements.get(idx) {
                            match stmt {
                                Either::Left(list_item_stmt_)
                                    if list_item_stmt_.indentation < list_item_stmt.indentation =>
                                {
                                    break;
                                }
                                _ => (),
                            }
                            idx += 1;
                        }

                        Some(parse_list(&statements[start..idx])?)
                    }
                    _ => None,
                };

                items.push(hir::ListItem {
                    text: hir::Text {
                        segments: text_segments,
                    },
                    child,
                });
            }
            Either::Right(lir::ParagraphStatement { .. }) => unreachable!(),
        }
    }

    Ok(hir::List { items })
}

fn parse_text_segments<'a>(
    segments_lir: &[lir::TextSegment<'a>],
) -> Result<Vec<hir::TextSegment<'a>>> {
    let mut segments = Vec::new();

    let mut idx = 0;
    while idx < segments_lir.len() {
        match segments_lir[idx] {
            lir::TextSegment::Text(text) => {
                segments.push(hir::TextSegment::Text(text));
                idx += 1;
            }
            lir::TextSegment::Break => {
                segments.push(hir::TextSegment::Break);
                idx += 1;
            }
            lir::TextSegment::Emphasis(emph) => {
                let offset =
                    &segments_lir[idx + 1..]
                        .into_iter()
                        .position(|segment| match segment {
                            lir::TextSegment::Emphasis(emph_end) => emph == *emph_end,
                            _ => false,
                        });

                match offset {
                    Some(offset) => {
                        segments.push(hir::TextSegment::Emphasised {
                            emphasis: emph,
                            inner: parse_text_segments(&segments_lir[idx + 1..idx + 1 + offset])?,
                        });
                        idx += offset + 2;
                    }
                    None => {
                        let text = match emph {
                            lir::Emphasis::Bold => "**",
                            lir::Emphasis::Italic => "__",
                            lir::Emphasis::Strikethrough => "~~",
                        };
                        segments.push(hir::TextSegment::Text(text));
                        idx += 1;
                    }
                }
            }
        }
    }

    Ok(segments)
}
