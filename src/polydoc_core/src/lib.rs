extern crate regex;

#[macro_use]
extern crate serde_derive;


pub mod docparsing;
pub mod merge;


pub type DocParseFn<'a> = &'a Fn(&str) -> Vec<DocItem>;
pub type SourceParseFn<'a> = &'a Fn(&str) -> Vec<DeclItem>;
pub type MergeFn<'a> = &'a Fn(&[DocItem], &[DeclItem]) -> Vec<DocumentedItem>;


#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DeclItem
{
    pub line: u64,
    pub name: String,
    pub data: DeclType
}


#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum DeclType
{
    Function
}


#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DocItem
{
    pub start_line: u64,
    pub end_line: u64,
    pub text: String
}


#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum DocumentedItem
{
    Function
    {
        name: String,
        description: String
    }
}