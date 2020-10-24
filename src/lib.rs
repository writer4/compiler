mod ast;
mod gen;
mod parser;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("ir parsing error")]
    LirParser(#[from] parser::lir::Error),
    #[error("parsing error")]
    HirParser(#[from] parser::hir::Error),
}

pub fn compile(source_code: &str) -> Result<String> {
    let doc_lir = parser::lir::parse(source_code)?;
    let doc_hir = parser::hir::parse(&doc_lir)?;

    Ok(gen::generate(&doc_hir))
}
