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
        }
    }

    Ok(hir::Document { statements })
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

fn parse_text_segments<'a>(
    segments_lir: &[lir::TextSegment<'a>],
) -> Result<Vec<hir::TextSegment<'a>>> {
    let mut segments = Vec::new();

    let mut idx = 0;
    while idx < segments_lir.len() {
        match &segments_lir[idx] {
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
                            lir::TextSegment::Emphasis(emph_end) => emph == emph_end,
                            _ => false,
                        });

                match offset {
                    Some(offset) => {
                        segments.push(hir::TextSegment::Emphasised {
                            emphasis: *emph,
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
