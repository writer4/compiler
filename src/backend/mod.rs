#[cfg(feature = "html-backend")]
mod html;

#[cfg(feature = "html-backend")]
pub use self::html::HtmlBackend;

use crate::ast::hir;
use crate::parser;

pub trait Backend: Sized {
    type Output;
    type Error;

    fn compile_hir(document: &hir::Document<'_>) -> Result<Self::Output, Self::Error>;

    fn compile(source_code: &str) -> crate::Result<Self::Output, Self>
    where
        Self: std::fmt::Debug,
        Self::Error: std::error::Error,
    {
        let doc_lir = parser::lir::parse(source_code)?;
        let doc_hir = parser::hir::parse(&doc_lir)?;

        match Self::compile_hir(&doc_hir) {
            Ok(output) => Ok(output),
            Err(e) => Err(crate::Error::Backend(e)),
        }
    }
}
