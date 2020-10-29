use crate::ast::hir;

#[derive(Debug)]
pub struct PdfBackend;

impl super::Backend for PdfBackend {
    type Output = std::convert::Infallible;
    type Error = std::convert::Infallible;

    fn compile_hir(_: &hir::Document<'_>) -> Result<Self::Output, Self::Error> {
        todo!()
    }
}
