extern crate regex;

#[macro_use]
extern crate serde_derive;


pub mod docparsing;
pub mod merge;


pub type DocParseFn<'a> = &'a Fn(&str) -> Vec<DocComment>;
pub type SourceParseFn<'a> = &'a Fn(&str) -> Vec<Declaration>;
pub type MergeFn<'a> = &'a Fn(&[DocComment], &[Declaration]) -> Vec<Doc>;


#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Declaration
{
    pub start_line: u64,
    pub decl: DeclType
}


#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum DeclType
{
    Function
    {
        name: String
    }
}


#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DocComment
{
    pub start_line: u64,
    pub end_line: u64,
    pub text: String
}


/// TODO:
///     - Make this a struct rather than an enum.
///     - Make all fields Options.
///     - Make an ItemType enum to be the type field.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum Doc
{
    Function
    {
        name: String,
        description: String
    }
}