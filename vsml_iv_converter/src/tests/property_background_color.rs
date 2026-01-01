use super::common::*;
use std::collections::HashMap;
use vsml_ast::vsml::{Content, Element, Meta, VSML};
use vsml_ast::vss::{VSSItem, VSSSelector, VSSSelectorTree};
use vsml_core::schemas::{Color, ObjectData};

#[test]
fn background_color_property_hex() {
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
        rules: vec![create_rule("background-color", "#ff0000")],
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
        background_color, ..
    } = children[0]
    else {
        panic!("Expected Element");
    };
    assert_eq!(
        background_color,
        Some(Color {
            r: 255,
            g: 0,
            b: 0,
            a: 255
        })
    );
}

#[test]
fn background_color_property_rgb() {
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
        rules: vec![create_rule("background-color", "rgb(0, 255, 0)")],
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
        background_color, ..
    } = children[0]
    else {
        panic!("Expected Element");
    };
    assert_eq!(
        background_color,
        Some(Color {
            r: 0,
            g: 255,
            b: 0,
            a: 255
        })
    );
}

#[test]
fn background_color_property_rgba() {
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
        rules: vec![create_rule("background-color", "rgba(100, 150, 200, 128)")],
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
        background_color, ..
    } = children[0]
    else {
        panic!("Expected Element");
    };
    assert_eq!(
        background_color,
        Some(Color {
            r: 100,
            g: 150,
            b: 200,
            a: 128
        })
    );
}
