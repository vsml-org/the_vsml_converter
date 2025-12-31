use super::common::*;
use std::collections::HashMap;
use vsml_ast::vsml::{Content, Element, Meta, VSML};
use vsml_ast::vss::{VSSItem, VSSSelector, VSSSelectorTree};
use vsml_core::schemas::ObjectData;

#[test]
fn font_family_property_single() {
    let elements = vec![Element::Tag {
        name: "mock".to_string(),
        attributes: {
            let mut attrs = HashMap::new();
            attrs.insert("class".to_string(), "target".to_string());
            attrs
        },
        children: vec![Element::Text("Hello".to_string())],
    }];

    let vss_items = vec![VSSItem {
        selectors: vec![VSSSelectorTree::Selectors(vec![VSSSelector::Class(
            "target".to_string(),
        )])],
        rules: vec![create_rule("font-family", "Arial")],
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
    let ObjectData::Element {
        children: inner_children,
        ..
    } = &children[0]
    else {
        panic!("Expected Element");
    };
    let ObjectData::Text(text_data) = &inner_children[0] else {
        panic!("Expected Text");
    };
    assert_eq!(text_data[0].style.font_family[0], "Arial");
}

#[test]
fn font_family_property_multiple() {
    let elements = vec![Element::Tag {
        name: "mock".to_string(),
        attributes: {
            let mut attrs = HashMap::new();
            attrs.insert("class".to_string(), "target".to_string());
            attrs
        },
        children: vec![Element::Text("Hello".to_string())],
    }];

    let vss_items = vec![VSSItem {
        selectors: vec![VSSSelectorTree::Selectors(vec![VSSSelector::Class(
            "target".to_string(),
        )])],
        rules: vec![create_rule("font-family", "Arial, 'MS Gothic', sans-serif")],
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
    let ObjectData::Element {
        children: inner_children,
        ..
    } = &children[0]
    else {
        panic!("Expected Element");
    };
    let ObjectData::Text(text_data) = &inner_children[0] else {
        panic!("Expected Text");
    };
    assert_eq!(text_data[0].style.font_family[0], "Arial");
    assert_eq!(text_data[0].style.font_family[1], "MS Gothic");
    assert_eq!(text_data[0].style.font_family[2], "sans-serif");
}

#[test]
fn font_family_property_nested_inheritance() {
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
            children: vec![Element::Text("Hello".to_string())],
        }],
    }];

    let vss_items = vec![
        VSSItem {
            selectors: vec![VSSSelectorTree::Selectors(vec![VSSSelector::Class(
                "parent".to_string(),
            )])],
            rules: vec![create_rule("font-family", "Georgia")],
        },
        VSSItem {
            selectors: vec![VSSSelectorTree::Selectors(vec![VSSSelector::Class(
                "child".to_string(),
            )])],
            rules: vec![create_rule("font-family", "Arial")],
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
    let ObjectData::Element {
        children: text_children,
        ..
    } = &inner_children[0]
    else {
        panic!("Expected Element");
    };
    let ObjectData::Text(text_data) = &text_children[0] else {
        panic!("Expected Text");
    };
    assert_eq!(text_data[0].style.font_family[0], "Arial");
    assert_eq!(text_data[0].style.font_family[1], "Georgia");
}
