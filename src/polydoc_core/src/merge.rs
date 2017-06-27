use {DocItem, DeclItem, DocumentedItem, DeclType};


pub fn merge_docs_with_decls(docs: &[DocItem], decls: &[DeclItem]) -> Vec<DocumentedItem>
{
    let mut documented_items = Vec::new();

    for decl in decls
    {
        for doc in docs
        {
            if doc.end_line == decl.line - 1
            {
                let documented = match decl.data
                {
                    DeclType::Function => DocumentedItem::Function
                    {
                        name: decl.name.clone(),
                        description: doc.text.clone()
                    }
                };
                documented_items.push(documented);
                break;
            }
        }
    }

    documented_items
}


#[cfg(test)]
mod tests
{
    use super::*;


    fn docitem(start_line: u64, end_line: u64, text: &str) -> DocItem
    {
        DocItem
        {
            start_line,
            end_line,
            text: text.to_owned()
        }
    }

    fn funcdecl(start_line: u64, name: &str) -> DeclItem
    {
        DeclItem
        {
            line: start_line,
            name: name.to_owned(),
            data: DeclType::Function
        }
    }

    fn docfunc(name: &str, description: &str) -> DocumentedItem
    {
        DocumentedItem::Function
        {
            name: name.to_owned(),
            description: description.to_owned()
        }
    }


    fn test_merge(docs: Vec<DocItem>, decls: Vec<DeclItem>, expected: Vec<DocumentedItem>)
    {
        let result = merge_docs_with_decls(&docs, &decls);
        assert_eq!(result, expected);
    }

    #[test]
    fn merge_empty_gives_empty()
    {
        test_merge(vec![], vec![], vec![]);
    }

    #[test]
    fn merge_1_doc_1_decl()
    {
        test_merge(vec![docitem(0, 0, "Doc")], vec![funcdecl(1, "Func")],
            vec![docfunc("Func", "Doc")]);
    }

    #[test]
    fn merge_1_multiline_doc_1_decl()
    {
        test_merge(vec![docitem(0, 2, "Doc")], vec![funcdecl(3, "Func")],
            vec![docfunc("Func", "Doc")]);
    }

    #[test]
    fn merge_1_doc_1_decl_not_connected()
    {
        test_merge(vec![docitem(0, 0, "Doc")], vec![funcdecl(4, "Func")],
            vec![]);
    }

    #[test]
    fn merge_2_doc_1_decl()
    {
        test_merge(vec![docitem(0, 0, "Doc"), docitem(2, 2, "Doc2")], vec![funcdecl(3, "Func")],
            vec![docfunc("Func", "Doc2")]);
    }

    #[test]
    fn merge_1_doc_2_decl()
    {
        test_merge(vec![docitem(0, 0, "Doc")], vec![funcdecl(1, "Func"), funcdecl(2, "Func2")],
            vec![docfunc("Func", "Doc")]);
    }

    #[test]
    fn merge_2_doc_2_decl()
    {
        test_merge(
            vec![docitem(0, 0, "Doc0"), docitem(5, 5, "Doc1")],
            vec![funcdecl(1, "Func0"), funcdecl(6, "Func1")],
            vec![docfunc("Func0", "Doc0"), docfunc("Func1", "Doc1"), ]);
    }

}