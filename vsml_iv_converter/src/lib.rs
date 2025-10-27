#[cfg(test)]
mod tests;

use std::collections::{HashMap, HashSet};
use std::sync::Arc;
use vsml_ast::vsml::{Content, Element, Meta, VSML};
use vsml_ast::vss::{Rule, VSSItem, VSSSelector, VSSSelectorTree};
use vsml_core::ElementRect;
use vsml_core::schemas::{
    Duration, FontColor, IVData, LayerMode, ObjectData, ObjectProcessor, ObjectType, Order,
    RectSize, StyleData, TextData, TextStyleData,
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
        let Element::Tag {
            name,
            attributes,
            children,
        } = &cont_element_list[0]
        else {
            unreachable!()
        };
        convert_tag_element(
            scanner,
            0.0,
            (0.0, 0.0),
            name,
            attributes,
            children,
            object_processor_provider,
            None,
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
    fn scan(&mut self) -> impl Iterator<Item = &Rule> + '_ {
        return self
            .vss_items
            .iter()
            .filter(|vss_item| {
                vss_item.selectors.iter().any(|selector| {
                    SelectorTreeMatchChecker::new(&self.traverse_stack).is_match(selector)
                })
            })
            .flat_map(|vss_item| &vss_item.rules);
        struct SelectorTreeMatchChecker<'a> {
            target_stack: &'a [&'a [Element]],
        }
        impl SelectorTreeMatchChecker<'_> {
            fn new<'a>(target_stack: &'a [&'a [Element]]) -> SelectorTreeMatchChecker<'a> {
                SelectorTreeMatchChecker { target_stack }
            }

            fn is_match(&mut self, selector_tree: &VSSSelectorTree) -> bool {
                match selector_tree {
                    VSSSelectorTree::Selectors(selectors) => {
                        let Some((tail, head)) = self.target_stack.split_last() else {
                            return false;
                        };
                        self.target_stack = head;
                        selector_is_match(selectors, tail)
                    }
                    VSSSelectorTree::Descendant(parent_selectors, child_tree) => {
                        if !self.is_match(child_tree) {
                            return false;
                        }
                        // この時点で、child_treeにマッチする部分はself.target_stackから消されているはず
                        while let Some((tail, head)) = self.target_stack.split_last() {
                            self.target_stack = head;
                            if selector_is_match(parent_selectors, tail) {
                                return true;
                            }
                        }
                        false
                    }
                    VSSSelectorTree::Child(parent_selectors, child_tree) => {
                        if !self.is_match(child_tree) {
                            return false;
                        }
                        let Some((tail, head)) = self.target_stack.split_last() else {
                            return false;
                        };
                        self.target_stack = head;
                        selector_is_match(parent_selectors, tail)
                    }
                    // TODO: 他のセレクタも実装
                    _ => todo!(),
                }
            }
        }
    }

    fn traverse<R>(&mut self, element: &'a [Element], f: impl FnOnce(&mut Self) -> R) -> R {
        self.traverse_stack.push(element);
        let result = f(self);
        self.traverse_stack.pop();
        result
    }
}
fn selector_is_match(selectors: &[VSSSelector], element: &[Element]) -> bool {
    let element_tag;
    let element_id;
    let element_classes;
    match element.last().unwrap() {
        Element::Tag {
            name, attributes, ..
        } => {
            element_tag = name.as_str();
            element_id = attributes.get("id").map(String::as_str);
            element_classes = attributes
                .get("class")
                .map_or_else(HashSet::new, |classes| classes.split_whitespace().collect());
        }
        Element::Text(_) => return false,
    };
    selectors
        .iter()
        .all(|single_selector| match single_selector {
            VSSSelector::All => true,
            VSSSelector::Tag(tag) => element_tag == tag.as_str(),
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

pub trait ObjectProcessorProvider<I, A> {
    fn get_processor(&self, name: &str) -> Option<Arc<dyn ObjectProcessor<I, A>>>;
}

impl<I, A> ObjectProcessorProvider<I, A> for HashMap<String, Arc<dyn ObjectProcessor<I, A>>> {
    fn get_processor(&self, name: &str) -> Option<Arc<dyn ObjectProcessor<I, A>>> {
        self.get(name).cloned()
    }
}

/// font-familyのパース用のutil関数
fn parse_font_family(input: &str) -> Vec<String> {
    let mut result = Vec::new();
    let mut current = String::new();
    let mut in_quotes = false;
    let mut escaped = false;

    for c in input.chars() {
        match c {
            '"' | '\'' if !escaped => {
                // クォートの開始/終了
                in_quotes = !in_quotes;
                // クォートは保存しない
            }
            '\\' if !escaped => {
                // エスケープ文字
                escaped = true;
            }
            ',' if !in_quotes => {
                // カンマによる区切り（クォート外の場合のみ）
                if !current.trim().is_empty() {
                    result.push(current.trim().to_string());
                }
                current.clear();
            }
            _ => {
                // エスケープされたクォートの場合は、バックスラッシュを除去してクォートのみ追加
                if !escaped || c == '"' || c == '\'' {
                    current.push(c);
                }
                escaped = false;
            }
        }
    }

    // 最後の要素を追加
    if !current.trim().is_empty() {
        result.push(current.trim().to_string());
    }

    result
}

// TODO: 引数多すぎ警告を修正する
#[allow(clippy::too_many_arguments)]
fn convert_tag_element<'a, I, A>(
    vss_scanner: &mut VssScanner<'a>,
    offset_start_time: f64,
    offset_position: (f32, f32),
    name: &str,
    attributes: &HashMap<String, String>,
    children: &'a [Element],
    object_processor_provider: &impl ObjectProcessorProvider<I, A>,
    parent_text_style: Option<TextStyleData>,
) -> ObjectData<I, A> {
    // スタイル情報
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
    let mut text_style = parent_text_style.unwrap_or(TextStyleData {
        color: None,
        // TODO: OSのデフォルトのfont-familyを別箇所で設定する
        font_family: vec!["Meiryo".to_string()],
    });

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
            "font-color" => {
                let color = rule.value.as_str();
                text_style.color = Some(color.parse::<FontColor>().unwrap().to_rgba().unwrap());
            }
            "font-family" => {
                let font_family = parse_font_family(rule.value.as_str());
                // 新しいfont-familyを先頭に持ってくる
                text_style.font_family = font_family
                    .into_iter()
                    .chain(text_style.font_family.into_iter())
                    .collect();
            }
            _ => {}
        }
    }
    let mut object_data_children = vec![];
    let mut start_offset = 0.0;
    let mut children_offset_position = (0.0, 0.0);
    let mut has_infinite_child = false;

    for (i, element) in children.iter().enumerate() {
        let child_object_data = vss_scanner.traverse(&children[..=i], |scanner| match element {
            Element::Tag {
                name,
                attributes,
                children,
                ..
            } => convert_tag_element(
                scanner,
                start_offset,
                children_offset_position,
                name,
                attributes,
                children,
                object_processor_provider,
                Some(text_style.clone()),
            ),
            // TODO: VSSプロパティとしてwidth, heightが追加された場合、ここでwidth, heightも必要になる
            // 仮に横書きであれば水平方向に書いた描画範囲の幅がwidthを超える場合、改行して次の行に続ける必要がある
            // そのため、折り返しの判定をするために、width(縦書きの場合はheight)が必要になる
            // 現状は、textの描画サイズがそのままtxtタグの描画サイズになるため、width, heightは不要
            Element::Text(text) => convert_element_text(text, &text_style),
        });
        // 子要素によって親要素のstyleが変わる場合の処理
        match &child_object_data {
            &ObjectData::Element {
                duration,
                ref element_rect,
                ..
            } => {
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
                    // TODO: 並べる方向を決めるpropertyが来たらそれに従う
                    children_offset_position.0 += element_rect.width;
                    target_size.width += element_rect.width;
                    target_size.height = target_size.height.max(element_rect.height);
                } else {
                    target_size.width = target_size.width.max(element_rect.width);
                    target_size.height = target_size.height.max(element_rect.height);
                }
            }
            ObjectData::Text(data) => {
                // 親要素のprocessorを使ってテキストサイズを計算
                if let ObjectType::Other(processor) = &object_type {
                    let rect_size = processor.calculate_text_size(data);
                    // TODO: 並べる方向を決めるpropertyが来たらそれに従う
                    target_size.width += rect_size.width;
                    target_size.height = target_size.height.max(rect_size.height);
                }
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

fn convert_element_text<I, A>(text: &str, style: &TextStyleData) -> ObjectData<I, A> {
    let data = vec![TextData {
        text: text.to_owned(),
        style: style.clone(),
    }];

    // TODO: text内で部分色指定とかを対応する場合、textを分割して複数のTextDataを作る
    ObjectData::Text(data)
}
