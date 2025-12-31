use super::common::*;
use std::collections::HashMap;
use vsml_ast::vsml::{Content, Element, Meta, VSML};
use vsml_ast::vss::{VSSItem, VSSSelector, VSSSelectorTree};
use vsml_core::schemas::ObjectData;

#[test]
fn order_property_sequence() {
    let elements = vec![
        Element::Tag {
            name: "mock".to_string(),
            attributes: {
                let mut attrs = HashMap::new();
                attrs.insert("class".to_string(), "item".to_string());
                attrs
            },
            children: vec![],
        },
        Element::Tag {
            name: "mock".to_string(),
            attributes: {
                let mut attrs = HashMap::new();
                attrs.insert("class".to_string(), "item".to_string());
                attrs
            },
            children: vec![],
        },
    ];

    let vss_items = vec![VSSItem {
        selectors: vec![VSSSelectorTree::Selectors(vec![VSSSelector::Tag(
            "cont".to_string(),
        )])],
        rules: vec![create_rule("order", "sequence")],
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

    let provider = TestObjectProcessorProvider::with(
        TestObjectProcessorProperty::default().with_duration(1.0),
    );
    let result = crate::convert(&vsml, &provider);

    let ObjectData::Element {
        children, duration, ..
    } = result.object
    else {
        panic!("Expected Element");
    };
    assert_eq!(duration, 2.0);
    assert_eq!(children.len(), 2);
    let ObjectData::Element { start_time, .. } = children[0] else {
        panic!("Expected Element");
    };
    assert_eq!(start_time, 0.0);
    let ObjectData::Element { start_time, .. } = children[1] else {
        panic!("Expected Element");
    };
    assert_eq!(start_time, 1.0);
}

#[test]
fn order_property_parallel() {
    let elements = vec![
        Element::Tag {
            name: "mock".to_string(),
            attributes: {
                let mut attrs = HashMap::new();
                attrs.insert("class".to_string(), "item".to_string());
                attrs
            },
            children: vec![],
        },
        Element::Tag {
            name: "mock".to_string(),
            attributes: {
                let mut attrs = HashMap::new();
                attrs.insert("class".to_string(), "item".to_string());
                attrs
            },
            children: vec![],
        },
    ];

    let vss_items = vec![VSSItem {
        selectors: vec![VSSSelectorTree::Selectors(vec![VSSSelector::Tag(
            "cont".to_string(),
        )])],
        rules: vec![create_rule("order", "parallel")],
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

    let provider = TestObjectProcessorProvider::with(
        TestObjectProcessorProperty::default().with_duration(1.0),
    );
    let result = crate::convert(&vsml, &provider);

    let ObjectData::Element {
        children, duration, ..
    } = result.object
    else {
        panic!("Expected Element");
    };
    assert_eq!(children.len(), 2);
    assert_eq!(duration, 1.0);
    let ObjectData::Element { start_time, .. } = children[0] else {
        panic!("Expected Element");
    };
    assert_eq!(start_time, 0.0);
    let ObjectData::Element { start_time, .. } = children[1] else {
        panic!("Expected Element");
    };
    assert_eq!(start_time, 0.0);
}
