use std::collections::HashMap;
use std::sync::Arc;
use vsml_ast::vss::Rule;
use vsml_core::schemas::{
    Duration, FontColor, LayerMode, Order, RectSize, TextStyleData, parse_font_family,
};

/// VSS ルールから設定されるスタイル情報と計算結果を統合した構造体
pub struct ConsolidatedStyle {
    // VSSルールから直接設定される値
    pub rule_duration: Option<f64>,
    pub order: Order,
    pub layer_mode: LayerMode,
    pub text_style: TextStyleData,

    // 計算によって求められる値（子要素処理後）
    pub calculated_duration: f64,
    pub size: RectSize,

    // 子要素処理用の作業変数
    pub start_offset: f64,
    pub children_offset_position: (f32, f32),
    pub has_infinite_child: bool,
}

impl ConsolidatedStyle {
    pub fn new(
        default_duration: f64,
        default_size: RectSize,
        default_order: Order,
        default_layer_mode: LayerMode,
        parent_text_style: Option<TextStyleData>,
    ) -> Self {
        Self {
            rule_duration: None,
            order: default_order,
            layer_mode: default_layer_mode,
            text_style: parent_text_style.unwrap_or(TextStyleData {
                color: None,
                // TODO: OSのデフォルトのfont-familyを別箇所で設定する
                font_family: vec!["Meiryo".to_string()],
            }),
            calculated_duration: default_duration,
            size: default_size,
            start_offset: 0.0,
            children_offset_position: (0.0, 0.0),
            has_infinite_child: false,
        }
    }

    pub fn final_duration(&self) -> f64 {
        self.rule_duration.unwrap_or(self.calculated_duration)
    }
}

/// VSS プロパティを処理するための trait
pub trait StylePropertyProcessor: Send + Sync {
    /// このprocessorが処理するプロパティ名
    fn property_name(&self) -> &str;

    /// ルールを適用してスタイルを更新
    fn apply(&self, rule_value: &str, style: &mut ConsolidatedStyle) -> Result<(), String>;
}

/// Order プロパティのプロセッサ
pub struct OrderProcessor;

impl StylePropertyProcessor for OrderProcessor {
    fn property_name(&self) -> &str {
        "order"
    }

    fn apply(&self, rule_value: &str, style: &mut ConsolidatedStyle) -> Result<(), String> {
        style.order = match rule_value {
            "sequence" => Order::Sequence,
            "parallel" => Order::Parallel,
            _ => return Err(format!("Invalid order value: {}", rule_value)),
        };
        Ok(())
    }
}

/// LayerMode プロパティのプロセッサ
pub struct LayerModeProcessor;

impl StylePropertyProcessor for LayerModeProcessor {
    fn property_name(&self) -> &str {
        "layer-mode"
    }

    fn apply(&self, rule_value: &str, style: &mut ConsolidatedStyle) -> Result<(), String> {
        style.layer_mode = match rule_value {
            "single" => LayerMode::Single,
            "multi" => LayerMode::Multi,
            _ => return Err(format!("Invalid layer-mode value: {}", rule_value)),
        };
        Ok(())
    }
}

/// Duration プロパティのプロセッサ
pub struct DurationProcessor;

impl StylePropertyProcessor for DurationProcessor {
    fn property_name(&self) -> &str {
        "duration"
    }

    fn apply(&self, rule_value: &str, style: &mut ConsolidatedStyle) -> Result<(), String> {
        let duration: Duration = rule_value
            .parse()
            .map_err(|_| format!("Invalid duration value: {}", rule_value))?;

        match duration {
            Duration::Percent(_) => {
                todo!("Percent duration not yet implemented")
            }
            Duration::Frame(_) => {
                todo!("Frame duration not yet implemented")
            }
            Duration::Second(duration) => {
                style.rule_duration = Some(duration);
            }
            Duration::Fit => {
                style.rule_duration = Some(f64::INFINITY);
            }
        }

        Ok(())
    }
}

/// FontColor プロパティのプロセッサ
pub struct FontColorProcessor;

impl StylePropertyProcessor for FontColorProcessor {
    fn property_name(&self) -> &str {
        "font-color"
    }

    fn apply(&self, rule_value: &str, style: &mut ConsolidatedStyle) -> Result<(), String> {
        let font_color: FontColor = rule_value
            .parse()
            .map_err(|_| format!("Invalid font-color value: {}", rule_value))?;

        style.text_style.color = Some(font_color.value());
        Ok(())
    }
}

/// FontFamily プロパティのプロセッサ
pub struct FontFamilyProcessor;

impl StylePropertyProcessor for FontFamilyProcessor {
    fn property_name(&self) -> &str {
        "font-family"
    }

    fn apply(&self, rule_value: &str, style: &mut ConsolidatedStyle) -> Result<(), String> {
        let mut font_family = parse_font_family(rule_value);
        // 新しいfont-familyを先頭が来るようにする
        font_family.append(&mut style.text_style.font_family);
        style.text_style.font_family = font_family;
        Ok(())
    }
}

/// スタイルプロセッサのレジストリ
pub struct StyleProcessorRegistry {
    processors: HashMap<String, Arc<dyn StylePropertyProcessor>>,
}

impl StyleProcessorRegistry {
    pub fn new() -> Self {
        let mut registry = StyleProcessorRegistry {
            processors: HashMap::new(),
        };

        // 標準のprocessorを登録
        registry.register(Arc::new(OrderProcessor));
        registry.register(Arc::new(LayerModeProcessor));
        registry.register(Arc::new(DurationProcessor));
        registry.register(Arc::new(FontColorProcessor));
        registry.register(Arc::new(FontFamilyProcessor));

        registry
    }

    pub fn register(&mut self, processor: Arc<dyn StylePropertyProcessor>) {
        self.processors
            .insert(processor.property_name().to_string(), processor);
    }

    pub fn process_rule(&self, rule: &Rule, style: &mut ConsolidatedStyle) -> Result<(), String> {
        if let Some(processor) = self.processors.get(rule.property.as_str()) {
            processor.apply(&rule.value, style)?;
        }
        // 未知のプロパティは無視（将来の拡張のため）
        Ok(())
    }
}
