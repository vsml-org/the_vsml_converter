use super::common::*;
use std::collections::HashMap;
use vsml_ast::vsml::{Content, Element, Meta, VSML};
use vsml_ast::vss::{VSSItem, VSSSelector, VSSSelectorTree};
use vsml_core::schemas::ObjectData;

#[test]
fn duration_property_seconds() {
    let elements = vec![Element::Tag {
        name: "mock".to_string(),
        attributes: {
            let mut attrs = HashMap::new();
            attrs.insert("class".to_string(), "target".to_string());
            attrs
        },
        children: vec![],
    }];

    let vss_items = vec![VSSItem {
        selectors: vec![VSSSelectorTree::Selectors(vec![VSSSelector::Class(
            "target".to_string(),
        )])],
        rules: vec![create_rule("duration", "5.5s")],
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

    let ObjectData::Element { children, .. } = result.object else {
        panic!("Expected Element");
    };
    let ObjectData::Element { duration, .. } = children[0] else {
        panic!("Expected Element");
    };
    assert_eq!(duration, 5.5);
}

#[test]
fn duration_property_frames() {
    let elements = vec![Element::Tag {
        name: "mock".to_string(),
        attributes: {
            let mut attrs = HashMap::new();
            attrs.insert("class".to_string(), "target".to_string());
            attrs
        },
        children: vec![],
    }];

    let vss_items = vec![VSSItem {
        selectors: vec![VSSSelectorTree::Selectors(vec![VSSSelector::Class(
            "target".to_string(),
        )])],
        rules: vec![create_rule("duration", "120f")],
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

    let ObjectData::Element { children, .. } = result.object else {
        panic!("Expected Element");
    };
    let ObjectData::Element { duration, .. } = children[0] else {
        panic!("Expected Element");
    };
    assert_eq!(duration, 2.0);
}

#[test]
fn duration_property_percent() {
    let elements = vec![Element::Tag {
        name: "cont".to_string(),
        attributes: {
            let mut attrs = HashMap::new();
            attrs.insert("class".to_string(), "parent".to_string());
            attrs
        },
        children: vec![Element::Tag {
            name: "mock".to_string(),
            attributes: {
                let mut attrs = HashMap::new();
                attrs.insert("class".to_string(), "child".to_string());
                attrs
            },
            children: vec![],
        }],
    }];

    let vss_items = vec![
        VSSItem {
            selectors: vec![VSSSelectorTree::Selectors(vec![VSSSelector::Class(
                "parent".to_string(),
            )])],
            rules: vec![create_rule("duration", "10s")],
        },
        VSSItem {
            selectors: vec![VSSSelectorTree::Selectors(vec![VSSSelector::Class(
                "child".to_string(),
            )])],
            rules: vec![create_rule("duration", "50%")],
        },
    ];

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

    let ObjectData::Element { children, .. } = result.object else {
        panic!("Expected Element");
    };
    let ObjectData::Element {
        children: inner_children,
        ..
    } = &children[0]
    else {
        panic!("Expected Element");
    };
    let ObjectData::Element { duration, .. } = inner_children[0] else {
        panic!("Expected Element");
    };
    assert_eq!(duration, 5.0);
}

#[test]
fn duration_property_fit() {
    let elements = vec![Element::Tag {
        name: "mock".to_string(),
        attributes: {
            let mut attrs = HashMap::new();
            attrs.insert("class".to_string(), "target".to_string());
            attrs
        },
        children: vec![],
    }];

    let vss_items = vec![VSSItem {
        selectors: vec![VSSSelectorTree::Selectors(vec![VSSSelector::Class(
            "target".to_string(),
        )])],
        rules: vec![create_rule("duration", "fit")],
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

    let ObjectData::Element { children, .. } = result.object else {
        panic!("Expected Element");
    };
    let ObjectData::Element { duration, .. } = children[0] else {
        panic!("Expected Element");
    };
    assert!(duration.is_infinite());
}
