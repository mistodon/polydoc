extern crate regex;


pub mod docparsing;


#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SourceItem
{
    pub line: u64,
    pub name: String,
    pub data: ItemType
}


#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ItemType
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