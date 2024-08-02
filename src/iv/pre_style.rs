use roxmltree::Node;

// inheritやunsetなど共通の値の考慮がない
pub trait PreStyleFactory {
    fn check_property_name(&self, property_name: &str) -> bool;
    fn create_from_value(&self, property_name: &str, value: &str) -> Vec<Box<dyn PreStyle>>;
}

pub struct TimeMarginFactory {}

impl PreStyleFactory for TimeMarginFactory {
    fn check_property_name(&self, property_name: &str) -> bool {
        ["time-margin", "time-margin-start", "time-margin-end"].contains(&property_name)
    }
    // TODO: 実装
    fn create_from_value(&self, property_name: &str, value: &str) -> Vec<Box<dyn PreStyle>> {
        match property_name {
            "time-margin" => vec![
                Box::new(TimeMarginStart {
                    value: 10.0,
                    unit: TimeUnit::Seconds,
                }),
                Box::new(TimeMarginEnd {
                    value: 10.0,
                    unit: TimeUnit::Seconds,
                }),
            ],
            "time-margin-start" => vec![Box::new(TimeMarginStart {
                value: 10.0,
                unit: TimeUnit::Seconds,
            })],
            "time-margin-end" => vec![Box::new(TimeMarginEnd {
                value: 10.0,
                unit: TimeUnit::Seconds,
            })],
            _ => unreachable!(),
        }
    }
}

pub struct OrderFactory {}

impl PreStyleFactory for OrderFactory {
    fn check_property_name(&self, property_name: &str) -> bool {
        property_name == "order"
    }
    fn create_from_value(&self, _: &str, value: &str) -> Vec<Box<dyn PreStyle>> {
        let order = match value {
            "sequence" => Order::Sequence,
            "parallel" => Order::Parallel,
            _ => panic!(),
        };
        vec![Box::new(order)]
    }
}

pub struct LayerFactory {}

impl PreStyleFactory for LayerFactory {
    fn check_property_name(&self, property_name: &str) -> bool {
        property_name == "layer"
    }
    fn create_from_value(&self, _: &str, value: &str) -> Vec<Box<dyn PreStyle>> {
        let layer = match value {
            "single" => Layer::Single,
            "multi" => Layer::Multi,
            _ => panic!(),
        };
        vec![Box::new(layer)]
    }
}

pub struct DurationFactory {}

impl PreStyleFactory for DurationFactory {
    fn check_property_name(&self, property_name: &str) -> bool {
        property_name == "duration"
    }
    // TODO: 実装
    fn create_from_value(&self, _: &str, value: &str) -> Vec<Box<dyn PreStyle>> {
        vec![Box::new(Duration {
            value: 10.0,
            unit: TimeUnit::Seconds,
        })]
    }
}

// styleタグから持ってきたstyleのstruct
pub trait PreStyle {}

enum TimeUnit {
    Seconds,
    Frames,
    Samples,
}

enum Order {
    Sequence,
    Parallel,
}

impl PreStyle for Order {}

enum Layer {
    Single,
    Multi,
}

impl PreStyle for Layer {}

pub struct Duration {
    value: f64,
    unit: TimeUnit,
}

impl PreStyle for Duration {}

pub struct TimeMarginStart {
    value: f64,
    unit: TimeUnit,
}

impl PreStyle for TimeMarginStart {}

pub struct TimeMarginEnd {
    value: f64,
    unit: TimeUnit,
}

impl PreStyle for TimeMarginEnd {}

#[derive(PartialEq)]
enum Combinator {
    Descendant,
    Child,
    NextSibling,
    SubsequentSibling,
}

impl Combinator {
    pub fn from_str(combinator: &str) -> Combinator {
        match combinator {
            "+" => Combinator::NextSibling,
            ">" => Combinator::Child,
            "~" => Combinator::SubsequentSibling,
            _ => unreachable!(),
        }
    }
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
    pub fn from_str(selector_str: &str) -> Result<Selector, &str> {
        let mut selector_list = vec![];
        let selector_parts = selector_str.split(' ').collect::<Vec<&str>>();

        for selector_part in selector_parts {
            if selector_part.is_empty() {
                continue;
            }
            match selector_part {
                "+" | ">" | "~" => {
                    let last_selector = selector_list.pop();
                    let mut new_last_selector: SelectorPart = match last_selector {
                        Some(s) => s,
                        None => return Err("invalid selector"),
                    };
                    if new_last_selector.combinator != Combinator::Descendant {
                        return Err("invalid selector");
                    }
                    new_last_selector.combinator = Combinator::from_str(selector_part);
                    selector_list.push(new_last_selector);
                }
                // TODO: 実装
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
