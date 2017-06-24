extern crate esprit;
extern crate easter;
extern crate joker;
extern crate polydoc_core;


use polydoc_core::{DeclItem, DeclType, DocumentedItem};


pub fn generate<S>(source: S) -> Vec<DocumentedItem>
where
    S: AsRef<str>
{
    use polydoc_core::{docparsing, merge};

    let docs = docparsing::extract_docs(&source);
    let decls = extract_declarations(&source);
    merge::merge_docs_with_decls(&docs, &decls)
}


fn extract_declarations<S>(source: S) -> Vec<DeclItem>
where
    S: AsRef<str>
{
    use esprit;
    use easter::stmt::StmtListItem;
    use easter::decl::Decl;
    use joker::word::Name;

    let mut items = Vec::new();
    let script = esprit::script(source.as_ref()).expect("Failed to parse javascript.");

    for item in &script.body
    {
        match item
        {
            &StmtListItem::Decl(Decl::Fun(ref f)) =>
            {
                let line = f.location.expect("Missing location").start.line as u64;
                let ref name = f.id.as_ref().expect("Expected function name.").name;
                if let &Name::String(ref s) = name
                {
                    let doc = DeclItem
                    {
                        line,
                        name: s.clone(),
                        data: DeclType::Function
                    };
                    items.push(doc);
                }
            },
            _ => unimplemented!()
        }
    }

    items
}


#[cfg(test)]
mod tests
{
    use super::*;

    #[test]
    fn print_and_fail()
    {
        let docs = extract_declarations(r#"
            // This is a function which returns the number 2.
            // Use it wisely.
            function two() {
                return 2;
            }

            /**
              This is a function which returns the number 3.
              Use it any way you want I guess.
            */
            function three() {
                return 3;
            }
        "#);
        println!("{:?}", docs);
        assert!(false);
    }
}