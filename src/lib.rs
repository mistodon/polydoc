extern crate polydoc_core;


use polydoc_core::{Doc, SourceParseFn};


pub fn parse_from_source<S>(
    source: S,
    source_parse_fn: SourceParseFn) -> Vec<Doc>
where
    S: AsRef<str>
{
    let source = source.as_ref();
    let docs = polydoc_core::docparsing::extract_docs(source);
    let decls = source_parse_fn(source);
    let items = polydoc_core::merge::merge_docs_with_decls(docs.as_slice(), decls.as_slice());
    items
}
