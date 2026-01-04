use core::f64;
use mockall::mock;
use std::collections::HashMap;
use std::sync::Arc;
use vsml_ast::vsml::Element;
use vsml_ast::vss::{Rule, VSSSelector, VSSSelectorTree};
use vsml_core::schemas::{ObjectProcessor, ProcessorInput, RectSize, TextData};

pub fn create_element(name: &str, class: Option<&str>, id: Option<&str>) -> Element {
    let mut attributes = HashMap::new();
    if let Some(class_name) = class {
        attributes.insert("class".to_string(), class_name.to_string());
    }
    if let Some(id_name) = id {
        attributes.insert("id".to_string(), id_name.to_string());
    }
    Element::Tag {
        name: name.to_string(),
        attributes,
        children: vec![],
    }
}

pub fn create_rule(property: &str, value: &str) -> Rule {
    Rule {
        property: property.to_string(),
        value: value.to_string(),
    }
}

pub fn create_descendant_selector(parent_class: &str, child_class: &str) -> VSSSelectorTree {
    VSSSelectorTree::Descendant(
        vec![VSSSelector::Class(parent_class.to_string())],
        Box::new(VSSSelectorTree::Selectors(vec![VSSSelector::Class(
            child_class.to_string(),
        )])),
    )
}

mock! {
    pub TestObjectProcessor {}

    impl ObjectProcessor<(), ()> for TestObjectProcessor {
        fn name(&self) -> &str;
        fn default_duration(&self, attributes: &HashMap<String, String>) -> f64;
        fn default_image_size(&self, attributes: &HashMap<String, String>) -> RectSize;
        fn calculate_text_size(&self, text_data: &[TextData]) -> RectSize;
        fn process_image(
            &self,
            render_sec: f64,
            attributes: &HashMap<String, String>,
            input: ProcessorInput<()>,
        ) -> Option<()>;
        fn process_audio(&self, attributes: &HashMap<String, String>, audio: Option<()>) -> Option<()>;
    }
}

pub struct TestObjectProcessorProperty {
    tag_name: String,
    default_duration: f64,
    default_image_size: RectSize,
    calculate_image_size: RectSize,
}

impl Default for TestObjectProcessorProperty {
    fn default() -> Self {
        TestObjectProcessorProperty {
            tag_name: "mock".to_string(),
            default_duration: f64::INFINITY,
            default_image_size: RectSize {
                width: 100.0,
                height: 100.0,
            },
            calculate_image_size: RectSize {
                width: 50.0,
                height: 20.0,
            },
        }
    }
}

impl TestObjectProcessorProperty {
    pub fn with_duration(mut self, duration: f64) -> Self {
        self.default_duration = duration;
        self
    }
    pub fn with_image_size(mut self, width: f32, height: f32) -> Self {
        self.default_image_size = RectSize { width, height };
        self
    }
    pub fn without_default_size(mut self) -> Self {
        self.default_image_size = RectSize::ZERO;
        self
    }
}

pub struct TestObjectProcessorProvider {
    processor: Arc<dyn ObjectProcessor<(), ()>>,
}

impl TestObjectProcessorProvider {
    pub fn with(
        TestObjectProcessorProperty {
            tag_name,
            default_duration,
            default_image_size,
            calculate_image_size,
        }: TestObjectProcessorProperty,
    ) -> Self {
        let mut mock = MockTestObjectProcessor::new();

        mock.expect_name().return_const(tag_name);

        mock.expect_default_duration()
            .return_const(default_duration);

        mock.expect_default_image_size()
            .return_const(default_image_size);

        mock.expect_calculate_text_size()
            .return_const(calculate_image_size);

        mock.expect_process_image().return_const(None::<()>);

        mock.expect_process_audio().return_const(None::<()>);

        TestObjectProcessorProvider {
            processor: Arc::new(mock),
        }
    }
    pub fn new() -> Self {
        TestObjectProcessorProvider::with(TestObjectProcessorProperty::default())
    }
}

impl crate::ObjectProcessorProvider<(), ()> for TestObjectProcessorProvider {
    fn get_processor(&self, _name: &str) -> Option<Arc<dyn ObjectProcessor<(), ()>>> {
        Some(self.processor.clone())
    }
}
