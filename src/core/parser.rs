use crate::core::{error::Error, language::Language};
use failure::Fallible;
use tree_sitter;

pub(crate) fn wast() -> Fallible<tree_sitter::Parser> {
    let language = unsafe { crate::tree_sitter_wast() };
    let mut parser = tree_sitter::Parser::new();
    parser.set_language(language).map_err(Error::TreeSitterLanguageError)?;
    Ok(parser)
}

pub(crate) fn wat() -> Fallible<tree_sitter::Parser> {
    let language = unsafe { crate::tree_sitter_wat() };
    let mut parser = tree_sitter::Parser::new();
    parser.set_language(language).map_err(Error::TreeSitterLanguageError)?;
    Ok(parser)
}

pub(crate) fn wit() -> Fallible<tree_sitter::Parser> {
    let language = unsafe { crate::tree_sitter_wit() };
    let mut parser = tree_sitter::Parser::new();
    parser.set_language(language).map_err(Error::TreeSitterLanguageError)?;
    Ok(parser)
}

pub(crate) fn witx() -> Fallible<tree_sitter::Parser> {
    let language = unsafe { crate::tree_sitter_witx() };
    let mut parser = tree_sitter::Parser::new();
    parser.set_language(language).map_err(Error::TreeSitterLanguageError)?;
    Ok(parser)
}

pub(crate) fn try_from(language: Language) -> Fallible<tree_sitter::Parser> {
    match language {
        Language::Wast => wast(),
        Language::Wat => wat(),
        Language::Wit => wit(),
        Language::Witx => witx(),
    }
}
