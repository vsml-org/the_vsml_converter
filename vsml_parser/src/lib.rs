use crate::elements::{Content, Element, Meta, VSML};
use crate::vss_parser::VSSParseError;
use roxmltree::{Document, Node, NodeType};
use std::error::Error;
use thiserror::Error;

mod elements;
mod vss_parser;

#[derive(Debug, Error, PartialEq)]
pub enum VSMLParseError<VSSError> {
    #[error("XML Parse Error: {0}")]
    XMLParseError(#[from] roxmltree::Error),
    #[error("VSS load error: {0}")]
    VSSLoadError(VSSError),
    #[error("VSS parse error: {0}")]
    VSSParseError(#[from] VSSParseError),
    #[error("both src and row text were specified in the style tag")]
    BothSrcAndTextInStyleError,
    #[error("style tag must be specified as at least one of src or row text")]
    NoSrcAndTextInStyleError,
    #[error("multiple root nodes exist")]
    MultipleRootNodesError,
    #[error(r#"root node name was not "vsml""#)]
    RootNodeNameError,
    #[error(r#"multiple "meta" elements exist"#)]
    MultipleMetaElementsError,
    #[error(r#"multiple "cont" elements exist"#)]
    MultipleContentElementsError,
    #[error(r#""vsml" tag contains invalid element"#)]
    InvalidElementInVSMLError,
    #[error(r#""meta" tag contains invalid element"#)]
    InvalidElementInMetaError,
    #[error(r#""cont" tag is not found in "vsml" tag"#)]
    ContentElementNotFoundError,
    #[error("invalid resolution value: {0}")]
    InvalidResolutionValue(String),
    #[error(r#"invalid "fps" value: {0}"#)]
    InvalidFPSValue(String),
    #[error(r#"invalid "sample-rate" value: {0}"#)]
    InvalidSampleRateValue(String),
    #[error("resolution attribute not found")]
    ResolutionNotFound,
}

pub fn parse<L>(vsml_string: &str, vss_loader: &L) -> Result<VSML, VSMLParseError<L::Err>>
where
    L: VSSLoader,
{
    // vsml文字列をXMLとして解釈し、DOMツリーを生成
    let doc = Document::parse(vsml_string)?;
    let root = doc.root();
    let mut root_children = root.children();
    let vsml = root_children.next().unwrap();
    if root_children.next().is_some() {
        return Err(VSMLParseError::MultipleRootNodesError);
    }
    if !vsml.has_tag_name("vsml") {
        return Err(VSMLParseError::RootNodeNameError);
    }
    let mut meta = None;
    let mut content = None;
    for child in vsml.children() {
        match child.node_type() {
            NodeType::Root => unreachable!(),
            NodeType::Element => {}
            NodeType::PI | NodeType::Comment => continue,
            NodeType::Text => {
                if child.text().unwrap().trim().is_empty() {
                    continue;
                } else {
                    return Err(VSMLParseError::InvalidElementInVSMLError);
                }
            }
        }
        match child.tag_name().name() {
            "meta" => {
                if meta.is_some() {
                    return Err(VSMLParseError::MultipleMetaElementsError);
                }
                meta = Some(parse_meta(child, vss_loader)?);
            }
            "cont" => {
                if content.is_some() {
                    return Err(VSMLParseError::MultipleContentElementsError);
                }
                content = Some(parse_content(child)?);
            }
            _ => return Err(VSMLParseError::InvalidElementInVSMLError),
        }
    }
    if content.is_none() {
        return Err(VSMLParseError::ContentElementNotFoundError);
    }
    Ok(VSML {
        meta,
        content: content.unwrap(),
    })
}

fn parse_meta<L>(node: Node, vss_loader: &L) -> Result<Meta, VSMLParseError<L::Err>>
where
    L: VSSLoader,
{
    assert!(node.has_tag_name("meta"));
    let mut vss_items = vec![];
    for child in node.children() {
        match child.node_type() {
            NodeType::Root => unreachable!(),
            NodeType::Element => {}
            NodeType::PI | NodeType::Comment => continue,
            NodeType::Text => {
                if child.text().unwrap().trim().is_empty() {
                    continue;
                } else {
                    return Err(VSMLParseError::InvalidElementInMetaError);
                }
            }
        }
        match child.tag_name().name() {
            "style" => {
                let src_path_result = child.attribute("src");
                if let Some(src) = src_path_result {
                    if child.text().is_some() {
                        return Err(VSMLParseError::BothSrcAndTextInStyleError);
                    }
                    vss_items.extend(vss_parser::parse(
                        vss_loader
                            .load(src)
                            .map_err(VSMLParseError::VSSLoadError)?
                            .as_str(),
                    )?);
                } else if let Some(vss_text) = child.text() {
                    vss_items.extend(vss_parser::parse(vss_text)?);
                } else {
                    return Err(VSMLParseError::NoSrcAndTextInStyleError);
                }
            }
            _ => return Err(VSMLParseError::InvalidElementInMetaError),
        }
    }
    Ok(Meta { vss_items })
}

fn parse_content<L>(node: Node) -> Result<Content, VSMLParseError<L>> {
    assert!(node.has_tag_name("cont"));
    let resolution = node
        .attribute("resolution")
        .ok_or(VSMLParseError::ResolutionNotFound)?;
    let (width, height) = resolution
        .split_once('x')
        .ok_or_else(|| VSMLParseError::InvalidResolutionValue(resolution.to_owned()))?;
    let width = width
        .parse()
        .map_err(|_| VSMLParseError::InvalidResolutionValue(resolution.to_owned()))?;
    let height = height
        .parse()
        .map_err(|_| VSMLParseError::InvalidResolutionValue(resolution.to_owned()))?;
    let fps = node
        .attribute("fps")
        .map(|fps| {
            fps.parse()
                .map_err(|_| VSMLParseError::InvalidFPSValue(fps.to_owned()))
        })
        .transpose()?;
    let sampling_rate = node
        .attribute("sample-rate")
        .map(|sampling_rate| {
            sampling_rate
                .parse()
                .map_err(|_| VSMLParseError::InvalidSampleRateValue(sampling_rate.to_owned()))
        })
        .transpose()?;
    let elements = node.children().filter_map(parse_element).collect();
    Ok(Content {
        width,
        height,
        fps,
        sampling_rate,
        elements,
    })
}

fn parse_element(node: Node) -> Option<Element> {
    match node.node_type() {
        NodeType::Root => unreachable!(),
        NodeType::Element => {
            Some(Element::Tag {
                name: node.tag_name().name().to_owned(),
                attributes: node
                    .attributes()
                    .map(|attr| (attr.name().to_owned(), attr.value().to_owned()))
                    .collect(),
                children: node.children().filter_map(parse_element).collect(),
            })
        }
        NodeType::PI | NodeType::Comment => None,
        NodeType::Text => {
            let text = node.text().unwrap().trim();
            (!text.is_empty()).then(||Element::Text(text.to_owned()))
        }
    }
}

#[cfg_attr(test, mockall::automock(type Err=std::convert::Infallible;))]
pub trait VSSLoader {
    type Err: Error;

    fn load(&self, path: &str) -> Result<String, Self::Err>;
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::vss_parser::{VSSItem, VSSSelector, VSSSelectorTree};
    use std::collections::HashMap;

    #[test]
    fn test_parse_vsml() {
        let vsml = r#"<vsml>
  <meta>
    <style src="hoge.vss" />
    <style>
      prl {
        height: 100rh;
      }
    </style>
  </meta>
  <cont resolution="1920x1080" fps="30">
    <prl>
      <img src="yellow.jpg" />
      <layer>
        <txt class="styled">これは文章です</txt>
        <txt class="styled">これもまた文章です</txt>
      </layer>
    </prl>
  </cont>
</vsml>"#;
        let mut mock_vss_loader = MockVSSLoader::new();
        mock_vss_loader
            .expect_load()
            .times(1)
            .returning(|_| Ok(".styled { font-color: red; }".to_owned()));
        assert_eq!(
            parse(vsml, &mock_vss_loader),
            Ok(VSML {
                meta: Some(Meta {
                    vss_items: vec![
                        VSSItem {
                            selector: vec![VSSSelectorTree::Selectors(vec![VSSSelector::Class(
                                "styled".to_owned()
                            )])],
                            rules: vec![("font-color".to_owned(), "red".to_owned())],
                        },
                        VSSItem {
                            selector: vec![VSSSelectorTree::Selectors(vec![VSSSelector::Tag(
                                "prl".to_owned()
                            )])],
                            rules: vec![("height".to_owned(), "100rh".to_owned())],
                        },
                    ],
                }),
                content: Content {
                    width: 1920,
                    height: 1080,
                    fps: Some(30),
                    sampling_rate: None,
                    elements: vec![Element::Tag {
                        name: "prl".to_owned(),
                        attributes: HashMap::new(),
                        children: vec![
                            Element::Tag {
                                name: "img".to_owned(),
                                attributes: [("src".to_owned(), "yellow.jpg".to_owned())]
                                    .iter()
                                    .cloned()
                                    .collect(),
                                children: vec![],
                            },
                            Element::Tag {
                                name: "layer".to_owned(),
                                attributes: HashMap::new(),
                                children: vec![
                                    Element::Tag {
                                        name: "txt".to_owned(),
                                        attributes: [("class".to_owned(), "styled".to_owned())]
                                            .iter()
                                            .cloned()
                                            .collect(),
                                        children: vec![
                                            Element::Text("これは文章です".to_owned()),
                                        ],
                                    },
                                    Element::Tag {
                                        name: "txt".to_owned(),
                                        attributes: [("class".to_owned(), "styled".to_owned())]
                                            .iter()
                                            .cloned()
                                            .collect(),
                                        children: vec![
                                            Element::Text("これもまた文章です".to_owned()),
                                        ],
                                    },
                                ],
                            },
                        ],
                    },],
                },
            })
        );
    }
}
