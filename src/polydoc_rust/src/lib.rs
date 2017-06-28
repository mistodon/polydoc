extern crate polydoc_core;

extern crate syntex_syntax;


use polydoc_core::{Declaration, DeclType};


fn which_line_at(byte_pos: u32, source: &str) -> u64
{
    let string_before = &source[0..(byte_pos as usize)];
    string_before.chars().filter(|&c| c == '\n').count() as u64
}


pub fn extract_declarations(source: &str) -> Vec<Declaration>
{
    use syntex_syntax::ast::{ItemKind};
    use syntex_syntax::codemap::{FilePathMapping};
    use syntex_syntax::parse::{self, ParseSess};

    let mut decls = Vec::new();

    let parse_sess = ParseSess::new(FilePathMapping::empty());

    let crate_data = parse::parse_crate_from_source_str(
        "name?".to_owned(),
        source.to_owned(),
        &parse_sess).unwrap();

    for item in &crate_data.module.items
    {
        let name = item.ident.name.to_string();
        let start_line = which_line_at(item.span.lo.0, source);
        match item.node
        {
            ItemKind::Fn(ref _decl, _unsafety, _constness, _abi, ref _generics, ref _block) =>
            {
                decls.push(Declaration
                {
                    start_line,
                    decl: DeclType::Function{ name }
                });
            }
            _ => unimplemented!()
        }
    }

    decls
}


#[cfg(test)]
mod tests
{
    use super::*;

    fn fundecl(name: &str, start_line: u64) -> Declaration
    {
        Declaration { start_line, decl: DeclType::Function { name: name.to_owned() } }
    }

    fn test_case(input: &str, expected: Vec<Declaration>)
    {
        let result = extract_declarations(input);
        assert_eq!(result, expected);
    }


    #[test]
    fn empty_input()
    {
        test_case("", vec![]);
    }

    #[test]
    fn small_function()
    {
        test_case("fn small() { }", vec![fundecl("small", 0)]);
    }

    #[test]
    fn two_functions()
    {
        test_case("fn one() { }\nfn two() { }", vec![fundecl("one", 0), fundecl("two", 1)]);
    }

    #[test]
    fn function_with_attribute()
    {
        test_case("#[test]\nfn attrd() { }", vec![fundecl("attrd", 1)]);
    }

}