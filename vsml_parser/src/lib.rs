use roxmltree::Document;
use std::error::Error;
use thiserror::Error;

mod vss_parser;

#[derive(Debug, Error)]
pub enum VSMLParseError<VSSError> {
    #[error("VSS load error: {0}")]
    VSSLoadError(#[from] VSSError),
}

pub fn parse<L>(vsml_string: &str, vss_loader: &L) -> Result<(), VSMLParseError<L::Err>>
where
    L: VSSLoader,
{
    // vsml文字列をXMLとして解釈し、DOMツリーを生成
    let doc = Document::parse(vsml_string).unwrap();
    // DOMツリーからrootタグを取得し、その子要素たち(metaタグ, contタグ)を取得する
    let root_children = doc
        .root_element()
        .children()
        .filter(|n| n.is_element())
        .collect::<Vec<_>>();
    // TODO: contタグのみの場合、0番目はcontになる、という想定を入れる
    // metaタグから子要素のstyleタグたちを取得する
    let meta_tag = root_children[0];
    let styles = meta_tag
        .children()
        .filter(|n| n.is_element())
        .collect::<Vec<_>>();

    // styleタグを使用してVSSItemの配列を作る
    let mut style_list = vec![];
    for style in styles {
        // srcにVSSのファイルパスがあればファイルを開きVSSItemにする
        let src_path_result = style.attribute("src");
        let vss_string_owned;
        let vss_string = if let Some(src) = src_path_result {
            vss_string_owned = vss_loader.load(src)?;
            vss_string_owned.as_str()
        } else {
            // srcが無ければその中のテキストを取得してVSSItemに変換する
            style.text().unwrap()
        };
        let vss = vss_parser::parse(vss_string);
        style_list.extend(vss);
    }

    // contのDOMツリーを取得する
    let _cont_tag = root_children[1];
    todo!()
}

pub trait VSSLoader {
    type Err: Error;

    fn load(&self, path: &str) -> Result<String, Self::Err>;
}
