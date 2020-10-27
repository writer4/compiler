mod ast;
mod backend;
mod error;
mod parser;

pub use self::{
    backend::Backend,
    error::{Error, Result},
};

#[cfg(feature = "html-backend")]
pub use self::backend::HtmlBackend;

#[cfg(feature = "html-backend")]
pub fn compile_html(source_code: &str) -> Result<String, HtmlBackend> {
    Backend::compile(source_code)
}
