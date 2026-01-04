#[cfg(test)]
mod tests;

use std::collections::{HashMap, HashSet};
use std::sync::Arc;
use vsml_ast::vsml::{Content, Element, Meta, VSML};
use vsml_ast::vss::{Rule, VSSItem, VSSSelector, VSSSelectorTree};
use vsml_core::ElementRect;
use vsml_core::schemas::{
    AudioVolume, Duration, IVData, LayerMode, Length, ObjectData, ObjectProcessor, ObjectType,
    Order, RectSize, TextData, TextStyleData, parse_font_family,
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
    let fps = fps.unwrap_or(60);

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
            fps,
            RectSize {
                width: width as f32,
                height: height as f32,
            },
            None,
            None,
            None,
        )
    });

    IVData {
        resolution_x: width,
        resolution_y: height,
        fps,
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
    fps: u32,
    resolution: RectSize,
    parent_text_style: Option<TextStyleData>,
    parent_duration: Option<f64>,
    parent_size: Option<RectSize>,
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
    let mut audio_volume = 1.0;
    let mut background_color = None;
    let mut rule_target_width = None;
    let mut rule_target_height = None;

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
                let duration = value.parse().unwrap();
                match duration {
                    Duration::Percent(percent) => {
                        let parent_duration =
                            parent_duration.expect("no parent duration available");
                        if parent_duration.is_infinite() {
                            panic!("parent duration is infinite (fit)");
                        }
                        rule_target_duration = Some(parent_duration * (percent / 100.0));
                    }
                    Duration::Frame(frames) => {
                        let duration_seconds = frames as f64 / fps as f64;
                        rule_target_duration = Some(duration_seconds);
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
                let value = rule.value.as_str();
                let font_color = value.parse().unwrap();
                text_style.color = Some(font_color);
            }
            "background-color" => {
                let value = rule.value.as_str();
                let parsed_value = value.parse().unwrap();
                background_color = Some(parsed_value);
            }
            "font-family" => {
                let mut font_family = parse_font_family(rule.value.as_str());
                // 新しいfont-familyを先頭が来るようにする
                font_family.append(&mut text_style.font_family);
                text_style.font_family = font_family;
            }
            "audio-volume" => {
                let value = rule.value.as_str();
                let volume = value.parse().unwrap();
                match volume {
                    AudioVolume::Percent(percent) => {
                        audio_volume = percent / 100.0;
                    }
                }
            }
            "width" => {
                let value = rule.value.as_str();
                let length = value.parse().unwrap();
                match length {
                    Length::Pixel(px) => {
                        rule_target_width = Some(px);
                    }
                    Length::ResolutionWidth(rw) => {
                        rule_target_width = Some(resolution.width * (rw / 100.0));
                    }
                    Length::ResolutionHeight(rh) => {
                        rule_target_width = Some(resolution.height * (rh / 100.0));
                    }
                    Length::Percent(percent) => {
                        let parent_width = parent_size
                            .expect("no parent size available for percentage width")
                            .width;
                        rule_target_width = Some(parent_width * (percent / 100.0) as f32);
                    }
                }
            }
            "height" => {
                let value = rule.value.as_str();
                let length = value.parse().unwrap();
                match length {
                    Length::Pixel(px) => {
                        rule_target_height = Some(px);
                    }
                    Length::ResolutionWidth(rw) => {
                        rule_target_height = Some(resolution.width * (rw / 100.0));
                    }
                    Length::ResolutionHeight(rh) => {
                        rule_target_height = Some(resolution.height * (rh / 100.0));
                    }
                    Length::Percent(percent) => {
                        let parent_size = parent_size
                            .expect("no parent size available for percentage height")
                            .height;
                        rule_target_height = Some(parent_size * (percent / 100.0) as f32);
                    }
                }
            }
            _ => {}
        }
    }

    // 子要素に渡すdurationを決定（明示的に指定されている場合のみ）
    let duration_for_children = rule_target_duration.filter(|d| d.is_finite());

    // パーセントの基準となる、子要素に渡すサイズ情報を準備（アスペクト比を適用）
    // default_image_sizeを使って事前にアスペクト比を計算
    let size_for_children = match (rule_target_width, rule_target_height) {
        // 両方指定されている場合はそのまま使用
        (Some(width), Some(height)) => Some(RectSize { width, height }),
        // width/height一方のみ指定: アスペクト比を維持して縮小（拡大はしない）
        (Some(width), None) => {
            if target_size.width != 0.0 && width < target_size.width {
                Some(RectSize {
                    width,
                    height: target_size.height * width / target_size.width,
                })
            } else {
                Some(RectSize {
                    width,
                    height: target_size.height,
                })
            }
        }
        (None, Some(height)) => {
            if target_size.height != 0.0 && height < target_size.height {
                Some(RectSize {
                    width: target_size.width * height / target_size.height,
                    height,
                })
            } else {
                Some(RectSize {
                    width: target_size.width,
                    height,
                })
            }
        }
        // style指定がない場合は親のサイズは未確定
        (None, None) => None,
    };

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
                fps,
                resolution,
                Some(text_style.clone()),
                duration_for_children,
                size_for_children,
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
                if layer_mode == LayerMode::Single && order == Order::Parallel {
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

    // レイアウト用のサイズを計算
    // 一方だけ指定されていたらアス比を維持しつつ収まるように縮小
    // 子要素を加味したtarget_sizeが必要なのでsize_for_childrenとは別にもう一度計算している
    let (final_layout_width, final_layout_height) = match (rule_target_width, rule_target_height) {
        (Some(width), Some(height)) => (width, height),
        (Some(width), None) => {
            if target_size.width != 0.0 && width < target_size.width {
                (width, target_size.height * width / target_size.width)
            } else {
                (width, target_size.height)
            }
        }
        (None, Some(height)) => {
            if target_size.height != 0.0 && height < target_size.height {
                (target_size.width * height / target_size.height, height)
            } else {
                (target_size.width, height)
            }
        }
        (None, None) => (target_size.width, target_size.height),
    };

    ObjectData::Element {
        object_type,
        // time-margin, time-paddingとかが来たらここまでに計算する
        start_time: offset_start_time,
        duration: rule_target_duration.unwrap_or(target_duration),
        audio_volume,
        background_color,
        attributes: attributes.clone(),
        element_rect: ElementRect {
            alignment: Default::default(),
            parent_alignment: Default::default(),
            x: offset_position.0,
            y: offset_position.1,
            width: final_layout_width,
            height: final_layout_height,
        },
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
