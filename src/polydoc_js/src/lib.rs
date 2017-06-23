extern crate esprit;
extern crate easter;
extern crate joker;
extern crate polydoc_core;


use polydoc_core::{SourceItem, ItemType};


pub fn generate<S>(source: S) -> Vec<SourceItem>
where
    S: AsRef<str>
{
    // This should parse docs and tie results together
    extract_declarations(source)
}


fn extract_declarations<S>(source: S) -> Vec<SourceItem>
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
                    let doc = SourceItem
                    {
                        line,
                        name: s.clone(),
                        data: ItemType::Function
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