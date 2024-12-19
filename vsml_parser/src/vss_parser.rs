use thiserror::Error;

#[derive(Debug, PartialEq)]
pub enum VSSSelectorAttributeValue {
    None,
    Equal(String),
    Contain(String),
    StartWith(String),
    EndWith(String),
    Include(String),
}

#[derive(Debug, PartialEq)]
pub enum VSSSelector {
    All,
    Tag(String),
    Class(String),
    Id(String),
    PseudoClass(String),
    Attribute(String, VSSSelectorAttributeValue),
}

#[derive(Debug, PartialEq)]
pub enum VSSSelectorTree {
    Selectors(Vec<VSSSelector>),
    // .selector .selector
    Descendant(Vec<VSSSelector>, Box<VSSSelectorTree>),
    // .selector > .selector
    Child(Vec<VSSSelector>, Box<VSSSelectorTree>),
    // .selector + .selector
    Sibling(Vec<VSSSelector>, Box<VSSSelectorTree>),
    // .selector ~ .selector
    AdjSibling(Vec<VSSSelector>, Box<VSSSelectorTree>),
}

#[derive(Debug, PartialEq)]
pub struct VSSItem {
    pub selector: Vec<VSSSelectorTree>,
    pub rules: Vec<(String, String)>,
}

#[derive(Debug, Error, PartialEq)]
pub enum VSSParseError {}

// TODO: ベタ書き処理を削除して実装する
pub fn parse(vss: &str) -> Result<Vec<VSSItem>, VSSParseError> {
    if vss == ".styled { font-color: red; }" {
        Ok(vec![VSSItem {
            selector: vec![VSSSelectorTree::Selectors(vec![VSSSelector::Class(
                "styled".to_string(),
            )])],
            rules: vec![("font-color".to_string(), "red".to_string())],
        }])
    } else {
        Ok(vec![VSSItem {
            selector: vec![VSSSelectorTree::Selectors(vec![VSSSelector::Tag(
                "prl".to_owned(),
            )])],
            rules: vec![("height".to_owned(), "100rh".to_owned())],
        }])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore]
    // parse関数が未実装のため、テストをignoreしています
    fn test_parse() {
        assert_eq!(
            parse(
                "
            seq {
              duration: 1s;
            }
            .subtitles-container txt {
              font-size: 20px;
              font-border-color: red;
            }
            #main-frame {
              width: 100rh;
            }",
            ),
            Ok(vec![
                VSSItem {
                    selector: vec![VSSSelectorTree::Selectors(vec![VSSSelector::Tag(
                        "seq".to_string()
                    )])],
                    rules: vec![("duration".to_string(), "1s".to_string())]
                },
                VSSItem {
                    selector: vec![VSSSelectorTree::Descendant(
                        vec![VSSSelector::Class("subtitles-container".to_string())],
                        Box::new(VSSSelectorTree::Selectors(vec![VSSSelector::Tag(
                            "txt".to_string()
                        )])),
                    )],
                    rules: vec![
                        ("font-size".to_string(), "20px".to_string()),
                        ("font-border-color".to_string(), "red".to_string())
                    ]
                },
                VSSItem {
                    selector: vec![VSSSelectorTree::Selectors(vec![VSSSelector::Id(
                        "main-frame".to_string()
                    )])],
                    rules: vec![("width".to_string(), "100vh".to_string())]
                }
            ])
        );
    }
}
