extern crate regex;


pub mod docparsing;
pub mod merge;


#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DeclItem
{
    pub line: u64,
    pub name: String,
    pub data: DeclType
}


#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DeclType
{
    Function
}


#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DocItem
{
    pub start_line: u64,
    pub end_line: u64,
    pub text: String
}


#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DocumentedItem
{
    Function
    {
        name: String,
        description: String
    }
}