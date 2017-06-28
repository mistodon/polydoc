extern crate polydoc_core;
extern crate polydoc_js;


mod polydoc;
pub use self::polydoc::{};


use polydoc_core::{Doc};


pub fn parse_from_source<S>(source: S) -> Vec<Doc>
where
    S: AsRef<str>
{
    let source = source.as_ref();
    let docs = polydoc_core::docparsing::extract_docs(source);
    let decls = polydoc_js::extract_declarations(source);
    let items = polydoc_core::merge::merge_docs_with_decls(docs.as_slice(), decls.as_slice());
    items
}
