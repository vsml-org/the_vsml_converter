use super::common::*;
use std::collections::HashMap;
use vsml_ast::vsml::{Content, Element, Meta, VSML};
use vsml_ast::vss::{VSSItem, VSSSelector, VSSSelectorTree};
use vsml_core::schemas::ObjectData;

#[test]
fn audio_volume_property_percent() {
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
        rules: vec![create_rule("audio-volume", "50%")],
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
    let ObjectData::Element { audio_volume, .. } = children[0] else {
        panic!("Expected Element");
    };
    assert_eq!(audio_volume, 0.5);
}

#[test]
fn audio_volume_property_over_100_percent() {
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
        rules: vec![create_rule("audio-volume", "150%")],
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
    let ObjectData::Element { audio_volume, .. } = children[0] else {
        panic!("Expected Element");
    };
    assert_eq!(audio_volume, 1.5);
}

#[test]
fn audio_volume_property_zero_percent() {
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
        rules: vec![create_rule("audio-volume", "0%")],
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
    let ObjectData::Element { audio_volume, .. } = children[0] else {
        panic!("Expected Element");
    };
    assert_eq!(audio_volume, 0.0);
}
