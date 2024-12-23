use crate::vss::VSSItem;
use std::collections::HashMap;

#[derive(Debug, PartialEq)]
pub struct VSML {
    pub meta: Meta,
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
#[derive(Debug, PartialEq, Clone)]
pub enum Element {
    Tag {
        name: String,
        attributes: HashMap<String, String>,
        children: Vec<Element>,
    },
    Text(String),
}
