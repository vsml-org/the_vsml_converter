use crate::vss_parser::VSSItem;
use std::collections::HashMap;

#[derive(Debug, PartialEq)]
pub struct VSML {
    pub meta: Option<Meta>,
    pub content: Content,
}
#[derive(Debug, PartialEq)]
pub struct Meta {
    pub vss_items: Vec<VSSItem>,
}
#[derive(Debug, PartialEq)]
pub struct Content {
    pub width: u32,
    pub height: u32,
    pub fps: Option<u32>,
    pub sampling_rate: Option<u32>,
    pub elements: Vec<Element>,
}
#[derive(Debug, PartialEq)]
pub enum Element {
    Tag {
        name: String,
        attributes: HashMap<String, String>,
        children: Vec<Element>,
    },
    Text(String),
}
