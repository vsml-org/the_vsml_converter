use crate::vss::VSSItem;
use std::collections::HashMap;

/// VSMLファイル全体のAST構造体
#[derive(Debug, PartialEq)]
pub struct VSML {
    pub meta: Meta,
    pub content: Content,
}
/// metaタグ内のデータの構造体
#[derive(Debug, PartialEq)]
pub struct Meta {
    pub vss_items: Vec<VSSItem>,
}
/// contタグ内のデータの構造体
#[derive(Debug, PartialEq)]
pub struct Content {
    /// 生成される動画の幅
    pub width: u32,
    /// 生成される動画の高さ
    pub height: u32,
    /// 生成される動画のFPS
    pub fps: Option<u32>,
    /// 生成される動画のサンプリングレート
    pub sampling_rate: Option<u32>,
    /// contタグの子孫のElement
    pub elements: Vec<Element>,
}
/// タグやその中のテキストを表す構造体
#[derive(Debug, PartialEq, Clone)]
pub enum Element {
    Tag {
        name: String,
        attributes: HashMap<String, String>,
        children: Vec<Element>,
    },
    Text(String),
}
