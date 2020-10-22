use crate::ast;

pub fn generate(document: &ast::Document<'_>) -> String {
    let mut ouput = r#"<div class="writer-doc">"#.to_string();

    let mut statements = document.statements.iter().peekable();
    while let Some(statement) = statements.next() {
        match statement {
            ast::Statement::EmptyLine(ast::EmptyLineStatement) => (),
            ast::Statement::Header(ast::HeaderStatement { header_type, text }) => {
                let htag = match header_type {
                    ast::HeaderType::H1 => "h1",
                    ast::HeaderType::H2 => "h2",
                    ast::HeaderType::H3 => "h3",
                    ast::HeaderType::H4 => "h4",
                    ast::HeaderType::H5 => "h5",
                    ast::HeaderType::H6 => "h6",
                };

                ouput += &format!(
                    "<{htag}>{text}</{htag}>",
                    htag = htag,
                    text = html_escape::encode_text(text)
                );
            }
            ast::Statement::Paragraph(ast::ParagraphStatement { text }) => {
                let mut content = html_escape::encode_text(text);

                while let Some(ast::Statement::Paragraph(_)) = statements.peek() {
                    match statements.next().unwrap() {
                        ast::Statement::Paragraph(ast::ParagraphStatement { text }) => {
                            content += " ";
                            content += html_escape::encode_text(text);
                        }
                        _ => unreachable!(),
                    }
                }

                ouput += &format!("<p>{}</p>", content);
            }
        }
    }

    ouput += "</div>";

    ouput
}
