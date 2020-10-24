use crate::ast::hir;

pub fn generate(document: &hir::Document<'_>) -> String {
    let mut ouput = r#"<div class="writer-doc">"#.to_string();

    let mut statements = document.statements.iter();
    while let Some(statement) = statements.next() {
        match statement {
            hir::Statement::Header(hir::HeaderStatement { header_type, text }) => {
                let htag = match header_type {
                    hir::HeaderType::H1 => "h1",
                    hir::HeaderType::H2 => "h2",
                    hir::HeaderType::H3 => "h3",
                    hir::HeaderType::H4 => "h4",
                    hir::HeaderType::H5 => "h5",
                    hir::HeaderType::H6 => "h6",
                };

                let mut text_output = String::new();
                for segment in &text.segments {
                    generate_text(segment, &mut text_output);
                }

                ouput += &format!("<{htag}>{text}</{htag}>", htag = htag, text = text_output);
            }
            hir::Statement::Paragraph(hir::ParagraphStatement { text }) => {
                let mut text_output = String::new();
                for segment in &text.segments {
                    generate_text(segment, &mut text_output);
                }

                ouput += &format!("<p>{}</p>", text_output);
            }
        }
    }

    ouput += "</div>";

    ouput
}

fn generate_text(segment: &hir::TextSegment<'_>, output: &mut String) {
    match segment {
        hir::TextSegment::Text(text) => *output += &html_escape::encode_text(text),
        hir::TextSegment::Break => *output += "<br />",
        hir::TextSegment::Emphasised { emphasis, inner } => {
            let (tag_opening, tag_closing) = match emphasis {
                hir::Emphasis::Bold => ("<b>", "</b>"),
                hir::Emphasis::Italic => ("<i>", "</i>"),
                hir::Emphasis::Strikethrough => ("<s>", "</s>"),
            };
            *output += tag_opening;
            for segment in inner {
                generate_text(segment, output);
            }
            *output += tag_closing;
        }
    }
}
