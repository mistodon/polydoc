extern crate polydoc_core;


use polydoc_core::{Declaration, DeclType};


pub fn extract_declarations(source: &str) -> Vec<Declaration>
{
    vec![Declaration
    {
        start_line: 1,
        decl: DeclType::Function
        {
            name: "function".to_owned()
        }
    }]
}


#[cfg(test)]
mod tests
{
    use super::*;

}