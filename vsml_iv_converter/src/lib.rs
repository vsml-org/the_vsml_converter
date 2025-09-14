use std::collections::{HashMap, HashSet};
use std::sync::Arc;
use vsml_ast::vsml::{Content, Element, Meta, VSML};
use vsml_ast::vss::{Rule, VSSItem, VSSSelector, VSSSelectorTree};
use vsml_core::ElementRect;
use vsml_core::schemas::{
    Duration, IVData, LayerMode, ObjectData, ObjectProcessor, ObjectType, Order, RectSize,
    StyleData,
};

pub fn convert<I, A>(
    vsml: &VSML,
    object_processor_provider: &impl ObjectProcessorProvider<I, A>,
) -> IVData<I, A> {
    let &VSML {
        meta: Meta { ref vss_items },
        content:
            Content {
                width,
                height,
                fps,
                sampling_rate,
                ref elements,
            },
    } = vsml;

    let mut vss_scanner = VssScanner::new(vss_items);
    let cont_element = Element::Tag {
        name: "cont".to_string(),
        attributes: HashMap::new(),
        children: elements.clone(),
    };
    let cont_element_list = vec![cont_element];
    let object = vss_scanner.traverse(&cont_element_list, |scanner| {
        recursive(
            &cont_element_list[0],
            scanner,
            0.0,
            (0.0, 0.0),
            object_processor_provider,
        )
    });

    IVData {
        resolution_x: width,
        resolution_y: height,
        fps: fps.unwrap_or(60),
        sampling_rate: sampling_rate.unwrap_or(48_000),
        object,
    }
}

struct VssScanner<'a> {
    vss_items: &'a [VSSItem],
    /// ルート要素からscan対象の要素までの要素のリスト
    traverse_stack: Vec<&'a [Element]>,
}

impl<'a> VssScanner<'a> {
    fn new(vss_items: &'a [VSSItem]) -> VssScanner<'a> {
        VssScanner {
            vss_items,
            traverse_stack: Vec::new(),
        }
    }

    /// traverse_stackに対して、selectorと一致するスタイルがないか絞り込み、一致するスタイルを取得している
    fn scan(&self) -> impl Iterator<Item = &Rule> + '_ {
        self.vss_items
            .iter()
            .filter(|vss_item| {
                vss_item.selectors.iter().any(|selector| {
                    self.selector_tree_is_match(selector, self.traverse_stack.len() - 1)
                })
            })
            .flat_map(|vss_item| &vss_item.rules)
    }

    fn selector_tree_is_match(
        &self,
        selector_tree: &VSSSelectorTree,
        element_index: usize,
    ) -> bool {
        match selector_tree {
            VSSSelectorTree::Selectors(selectors) => {
                self.selector_is_match(selectors, self.traverse_stack[element_index])
            }
            VSSSelectorTree::Descendant(parent_selectors, child_tree) => {
                // 現在の要素に対して子セレクタがマッチするか確認
                if !self.selector_tree_is_match(child_tree, element_index) {
                    return false;
                }

                // 親方向に遡って親セレクタとマッチするものを探す
                for i in (0..element_index).rev() {
                    if self.selector_is_match(parent_selectors, self.traverse_stack[i]) {
                        return true;
                    }
                }
                false
            }
            // TODO: 他のセレクタも実装
            _ => false,
        }
    }

    fn selector_is_match(&self, selectors: &[VSSSelector], element: &[Element]) -> bool {
        let element_tag;
        let element_id;
        let element_classes;
        match element.last().unwrap() {
            Element::Tag {
                name, attributes, ..
            } => {
                element_tag = Some(name.as_str());
                element_id = attributes.get("id").map(String::as_str);
                element_classes = attributes
                    .get("class")
                    .map_or_else(HashSet::new, |classes| classes.split_whitespace().collect());
            }
            Element::Text(_) => {
                element_tag = None;
                element_id = None;
                element_classes = HashSet::new();
            }
        };
        selectors
            .iter()
            .all(|single_selector| match single_selector {
                VSSSelector::All => true,
                VSSSelector::Tag(tag) => element_tag == Some(tag.as_str()),
                VSSSelector::Class(class_name) => element_classes.contains(class_name.as_str()),
                VSSSelector::Id(id_name) => element_id == Some(id_name),
                VSSSelector::PseudoClass(_) => {
                    todo!()
                }
                VSSSelector::Attribute(_, _) => {
                    todo!()
                }
            })
    }

    fn traverse<R>(&mut self, element: &'a [Element], f: impl FnOnce(&mut Self) -> R) -> R {
        self.traverse_stack.push(element);
        let result = f(self);
        self.traverse_stack.pop();
        result
    }
}

fn recursive<'a, I, A>(
    element: &'a Element,
    vss_scanner: &mut VssScanner<'a>,
    offset_start_time: f64,
    offset_position: (f32, f32),
    object_processor_provider: &impl ObjectProcessorProvider<I, A>,
) -> ObjectData<I, A> {
    match element {
        Element::Tag {
            name,
            attributes,
            children,
        } => convert_tag_element(
            vss_scanner,
            offset_start_time,
            offset_position,
            name,
            attributes,
            children,
            object_processor_provider,
        ),
        Element::Text(text) => ObjectData::Text(text.clone()),
    }
}

pub trait ObjectProcessorProvider<I, A> {
    fn get_processor(&self, name: &str) -> Option<Arc<dyn ObjectProcessor<I, A>>>;
}

impl<I, A> ObjectProcessorProvider<I, A> for HashMap<String, Arc<dyn ObjectProcessor<I, A>>> {
    fn get_processor(&self, name: &str) -> Option<Arc<dyn ObjectProcessor<I, A>>> {
        self.get(name).cloned()
    }
}

fn convert_tag_element<'a, I, A>(
    vss_scanner: &mut VssScanner<'a>,
    offset_start_time: f64,
    offset_position: (f32, f32),
    name: &str,
    attributes: &HashMap<String, String>,
    children: &'a [Element],
    object_processor_provider: &impl ObjectProcessorProvider<I, A>,
) -> ObjectData<I, A> {
    let object_type = match name {
        "cont" | "seq" | "prl" | "layer" => ObjectType::Wrap,
        name => ObjectType::Other(
            object_processor_provider
                .get_processor(name)
                .expect("Processor not found"),
        ),
    };
    let mut target_duration = match &object_type {
        ObjectType::Wrap => 0.0,
        ObjectType::Other(processor) => processor.default_duration(attributes),
    };
    let mut rule_target_duration = None;
    let mut target_size: RectSize = match &object_type {
        ObjectType::Wrap => RectSize::ZERO,
        ObjectType::Other(processor) => processor.default_image_size(attributes),
    };

    let mut order: Order = match name {
        "seq" | "cont" => Order::Sequence,
        "prl" | "layer" => Order::Parallel,
        _ => Order::Sequence,
    };
    let mut layer_mode: LayerMode = match name {
        "seq" | "prl" | "cont" => LayerMode::Multi,
        "layer" => LayerMode::Single,
        _ => LayerMode::Multi,
    };
    for rule in vss_scanner.scan() {
        match rule.property.as_str() {
            "order" => {
                order = match rule.value.as_str() {
                    "sequence" => Order::Sequence,
                    "parallel" => Order::Parallel,
                    _ => todo!("エラーを実装"),
                };
            }
            "layer-mode" => {
                layer_mode = match rule.value.as_str() {
                    "single" => LayerMode::Single,
                    "multi" => LayerMode::Multi,
                    _ => todo!(),
                };
            }
            "duration" => {
                let value = rule.value.as_str();
                let duration: Duration = value.parse().unwrap();
                match duration {
                    Duration::Percent(_) => {
                        todo!()
                    }
                    Duration::Frame(_) => {
                        todo!()
                    }
                    Duration::Second(duration) => {
                        rule_target_duration = Some(duration);
                    }
                    Duration::Fit => {
                        rule_target_duration = Some(f64::INFINITY);
                    }
                }
            }
            _ => {}
        }
    }
    let mut object_data_children = vec![];
    let mut start_offset = 0.0;
    let mut children_offset_position = (0.0, 0.0);
    let mut has_infinite_child = false;

    for (i, element) in children.iter().enumerate() {
        let child_object_data = vss_scanner.traverse(&children[..=i], |scanner| {
            recursive(
                element,
                scanner,
                start_offset,
                children_offset_position,
                object_processor_provider,
            )
        });
        if let &ObjectData::Element {
            duration,
            ref element_rect,
            ..
        } = &child_object_data
        {
            match order {
                Order::Sequence => {
                    start_offset += duration;
                    target_duration += duration;
                }
                Order::Parallel => {
                    if duration.is_finite() {
                        target_duration = target_duration.max(duration);
                    } else {
                        has_infinite_child = true;
                    }
                }
            }
            if layer_mode == LayerMode::Single {
                children_offset_position.0 += element_rect.width;
                target_size.width += element_rect.width;
                target_size.height = target_size.height.max(element_rect.height);
            } else {
                target_size.width = target_size.width.max(element_rect.width);
                target_size.height = target_size.height.max(element_rect.height);
            }
        }
        object_data_children.push(child_object_data);
    }
    if has_infinite_child && target_duration == 0.0 {
        target_duration = f64::INFINITY
    }

    ObjectData::Element {
        object_type,
        // time-margin, time-paddingとかが来たらここまでに計算する
        start_time: offset_start_time,
        duration: rule_target_duration.unwrap_or(target_duration),
        attributes: attributes.clone(),
        element_rect: ElementRect {
            alignment: Default::default(),
            parent_alignment: Default::default(),
            x: offset_position.0,
            y: offset_position.1,
            width: target_size.width,
            height: target_size.height,
        },
        styles: StyleData::default(),
        children: object_data_children,
    }
}
