use crate::backend::Backend;
use crate::parser;

pub type Result<T, B> = std::result::Result<T, Error<B>>;

#[derive(Debug, thiserror::Error)]
pub enum Error<B>
where
    B: std::fmt::Debug + Backend,
    B::Error: std::error::Error,
{
    #[error("lir parsing error")]
    LirParser(#[from] parser::lir::Error),
    #[error("hir parsing error")]
    HirParser(#[from] parser::hir::Error),
    #[error("backend error")]
    Backend(B::Error),
}
