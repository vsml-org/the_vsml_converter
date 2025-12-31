use super::common::*;
use std::collections::HashMap;
use vsml_ast::vsml::{Content, Element, Meta, VSML};
use vsml_ast::vss::{VSSItem, VSSSelector, VSSSelectorTree};
use vsml_core::schemas::ObjectData;

#[test]
fn layer_mode_property_single() {
    let elements = vec![
        Element::Tag {
            name: "mock".to_string(),
            attributes: HashMap::new(),
            children: vec![],
        },
        Element::Tag {
            name: "mock".to_string(),
            attributes: HashMap::new(),
            children: vec![],
        },
    ];

    let vss_items = vec![VSSItem {
        selectors: vec![VSSSelectorTree::Selectors(vec![VSSSelector::Tag(
            "cont".to_string(),
        )])],
        rules: vec![
            create_rule("order", "parallel"),
            create_rule("layer-mode", "single"),
        ],
    }];

    let vsml = VSML {
        meta: Meta { vss_items },
        content: Content {
            width: 1920,
            height: 1080,
            fps: Some(60),
            sampling_rate: Some(48000),
            elements,
        },
    };

    let provider = TestObjectProcessorProvider::new();
    let result = crate::convert(&vsml, &provider);

    if let ObjectData::Element {
        element_rect,
        children,
        ..
    } = result.object
    {
        assert_eq!(children.len(), 2);
        assert_eq!(element_rect.width, 200.0);
        assert_eq!(element_rect.height, 100.0);
    } else {
        panic!("Expected Element");
    }
}

#[test]
fn layer_mode_property_multi() {
    let elements = vec![
        Element::Tag {
            name: "mock".to_string(),
            attributes: HashMap::new(),
            children: vec![],
        },
        Element::Tag {
            name: "mock".to_string(),
            attributes: HashMap::new(),
            children: vec![],
        },
    ];

    let vss_items = vec![VSSItem {
        selectors: vec![VSSSelectorTree::Selectors(vec![VSSSelector::Tag(
            "cont".to_string(),
        )])],
        rules: vec![
            create_rule("order", "parallel"),
            create_rule("layer-mode", "multi"),
        ],
    }];

    let vsml = VSML {
        meta: Meta { vss_items },
        content: Content {
            width: 1920,
            height: 1080,
            fps: Some(60),
            sampling_rate: Some(48000),
            elements,
        },
    };

    let provider = TestObjectProcessorProvider::new();
    let result = crate::convert(&vsml, &provider);

    if let ObjectData::Element {
        element_rect,
        children,
        ..
    } = result.object
    {
        assert_eq!(children.len(), 2);
        assert_eq!(element_rect.width, 100.0);
        assert_eq!(element_rect.height, 100.0);
    } else {
        panic!("Expected Element");
    }
}
