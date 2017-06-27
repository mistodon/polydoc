extern crate polydoc_core;


use polydoc_core::{DocumentedItem, DocParseFn, SourceParseFn, MergeFn};


pub type SerializeFn<'a> = &'a Fn(&[DocumentedItem]) -> Option<String>;


pub fn polydoc<S>(
    source: S,
    doc_parse_func: DocParseFn,
    source_parse_func: SourceParseFn,
    merge_func: MergeFn,
    serialize_func: SerializeFn) -> Option<String>
where
    S: AsRef<str>
{
    let source = source.as_ref();
    let docs = doc_parse_func(source);
    let decls = source_parse_func(source);
    let items = merge_func(docs.as_slice(), decls.as_slice());
    serialize_func(items.as_slice())
}