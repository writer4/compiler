mod ast;
mod gen;
mod parser;

pub fn compile(source_code: String) -> String {
    match parser::parse(&source_code) {
        Ok(document) => gen::generate(&document),
        Err(e) => format!("<code>{:#?}</code>", e),
    }
}
