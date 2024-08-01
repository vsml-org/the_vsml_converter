use roxmltree::Node;

pub trait PreStyleFactory {
    fn check_property_name(&self, property_name: &str) -> bool;
    fn create_from_value(&self, value: &str) -> Box<dyn PreStyle>;
}

pub struct TimeMarginFactory {}

impl PreStyleFactory for TimeMarginFactory {
    fn check_property_name(&self, property_name: &str) -> bool {
        property_name == "time-margin"
    }
    // TODO: 実装
    fn create_from_value(&self, value: &str) -> Box<dyn PreStyle> {
        Box::new(TimeMargin {
            value: 10.0,
            unit: TimeUnit::Seconds,
        })
    }
}

pub struct DurationFactory {}

impl PreStyleFactory for DurationFactory {
    fn check_property_name(&self, property_name: &str) -> bool {
        property_name == "duration"
    }
    // TODO: 実装
    fn create_from_value(&self, value: &str) -> Box<dyn PreStyle> {
        Box::new(Duration {
            value: 10.0,
            unit: TimeUnit::Seconds,
        })
    }
}

// styleタグから持ってきたstyleのstruct
pub trait PreStyle {}

enum TimeUnit {
    Seconds,
    Frames,
    Samples,
}

pub struct Duration {
    value: f64,
    unit: TimeUnit,
}

impl PreStyle for Duration {}

pub struct TimeMargin {
    value: f64,
    unit: TimeUnit,
}

impl PreStyle for TimeMargin {}

#[derive(PartialEq)]
enum Combinator {
    Descendant,
    Child,
    NextSibling,
    SubsequentSibling,
}

struct SelectorPart {
    tag_name: Option<String>,
    class_names: Vec<String>,
    id_name: Option<String>,
    pub combinator: Combinator,
}

pub struct Selector {
    selector_list: Vec<SelectorPart>,
}

impl Selector {
    // TODO: 実装
    pub fn from_str(selector_str: &str) -> Result<Selector, &str> {
        let mut selector_list = vec![];
        let selector_parts = selector_str.split(' ').collect::<Vec<&str>>();

        for selector_part in selector_parts {
            if selector_part.is_empty() {
                continue;
            }
            match selector_part {
                "+" => {
                    let last_selector = selector_list.pop();
                    if last_selector.is_none() {
                        return Err("invalid selector");
                    }
                    // TODO: ここのis_someの確認方法を修正
                    if last_selector.is_some() {
                        let mut new_last_selector: SelectorPart = last_selector.unwrap();
                        if new_last_selector.combinator != Combinator::Descendant {
                            return Err("invalid selector");
                        }
                        new_last_selector.combinator = Combinator::NextSibling;
                        selector_list.push(new_last_selector);
                    }
                }
                ">" => {
                    let last_selector = selector_list.pop();
                    if last_selector.is_none() {
                        return Err("invalid selector");
                    }
                    // TODO: ここのis_someの確認方法を修正
                    if last_selector.is_some() {
                        let mut new_last_selector: SelectorPart = last_selector.unwrap();
                        if new_last_selector.combinator != Combinator::Descendant {
                            return Err("invalid selector");
                        }
                        new_last_selector.combinator = Combinator::Child;
                        selector_list.push(new_last_selector);
                    }
                }
                "~" => {
                    let last_selector = selector_list.pop();
                    if last_selector.is_none() {
                        return Err("invalid selector");
                    }
                    // TODO: ここのis_someの確認方法を修正
                    if last_selector.is_some() {
                        let mut new_last_selector: SelectorPart = last_selector.unwrap();
                        if new_last_selector.combinator != Combinator::Descendant {
                            return Err("invalid selector");
                        }
                        new_last_selector.combinator = Combinator::SubsequentSibling;
                        selector_list.push(new_last_selector);
                    }
                }
                _ => selector_list.push(SelectorPart {
                    tag_name: None,
                    class_names: vec![],
                    id_name: None,
                    combinator: Combinator::Descendant,
                }),
            }
        }
        Ok(Selector { selector_list })
    }
    // TODO: 実装
    pub fn is_target(&self, node: &Node) -> bool {
        // node.parent().unwrap().parent().unwrap().parent().unwrap();
        false
    }
}

pub struct VSSData {
    pub selector: Selector,
    pub pre_style_list: Vec<Box<dyn PreStyle>>,
}
