use crate::ast::hir;

#[derive(Debug)]
pub struct HtmlBackend;

impl super::Backend for HtmlBackend {
    type Output = String;
    type Error = std::convert::Infallible;

    fn compile_hir(document: &hir::Document<'_>) -> Result<Self::Output, Self::Error> {
        let mut output = r#"<div class="writer4-doc">"#.to_string();

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

                    output += &format!("<{htag}>{text}</{htag}>", htag = htag, text = text_output);
                }
                hir::Statement::Paragraph(hir::ParagraphStatement { text }) => {
                    let mut text_output = String::new();
                    for segment in &text.segments {
                        generate_text(segment, &mut text_output);
                    }

                    output += &format!("<p>{}</p>", text_output);
                }
                hir::Statement::List(hir::ListStatement { list }) => {
                    generate_list(list, &mut output);
                }
                hir::Statement::HorizontalRule(_) => output += "<hr>",
            }
        }

        output += "</div>";

        Ok(output)
    }
}

fn generate_list(list: &hir::List<'_>, output: &mut String) {
    *output += "<ul>";
    for list_item in &list.items {
        generate_list_item(list_item, output);
    }
    *output += "</ul>";
}

fn generate_list_item(list_item: &hir::ListItem<'_>, output: &mut String) {
    *output += "<li>";
    for segment in &list_item.text.segments {
        generate_text(segment, output);
    }
    if let Some(child) = &list_item.child {
        generate_list(child, output);
    }
    *output += "</li>";
}

fn generate_text(segment: &hir::TextSegment<'_>, output: &mut String) {
    match segment {
        hir::TextSegment::Text(text) => *output += &html_escape::encode_text(text),
        hir::TextSegment::Break => *output += "<br>",
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
