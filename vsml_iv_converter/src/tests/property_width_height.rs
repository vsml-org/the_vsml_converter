use super::common::*;
use std::collections::HashMap;
use vsml_ast::vsml::{Content, Element, Meta, VSML};
use vsml_ast::vss::{VSSItem, VSSSelector, VSSSelectorTree};
use vsml_core::schemas::ObjectData;

// Width tests
#[test]
fn width_property_pixel() {
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
        rules: vec![create_rule("width", "500px")],
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
        ref element_rect, ..
    } = children[0]
    else {
        panic!("Expected Element");
    };
    assert_eq!(element_rect.width, 500.0);
}

#[test]
fn width_property_resolution_width() {
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
        rules: vec![create_rule("width", "50rw")],
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
        ref element_rect, ..
    } = children[0]
    else {
        panic!("Expected Element");
    };
    assert_eq!(element_rect.width, 960.0); // 1920 * 50 / 100
}

#[test]
fn width_property_resolution_height() {
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
        rules: vec![create_rule("width", "50rh")],
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
        ref element_rect, ..
    } = children[0]
    else {
        panic!("Expected Element");
    };
    assert_eq!(element_rect.width, 540.0); // 1080 * 50 / 100
}

#[test]
fn width_property_percent() {
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
            rules: vec![create_rule("width", "800px")],
        },
        VSSItem {
            selectors: vec![VSSSelectorTree::Selectors(vec![VSSSelector::Class(
                "child".to_string(),
            )])],
            rules: vec![create_rule("width", "50%")],
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
        ref element_rect, ..
    } = inner_children[0]
    else {
        panic!("Expected Element");
    };
    assert_eq!(element_rect.width, 400.0); // 800 * 50 / 100
}

// Height tests
#[test]
fn height_property_pixel() {
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
        rules: vec![create_rule("height", "300px")],
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
        ref element_rect, ..
    } = children[0]
    else {
        panic!("Expected Element");
    };
    assert_eq!(element_rect.height, 300.0);
}

#[test]
fn height_property_resolution_width() {
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
        rules: vec![create_rule("height", "25rw")],
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
        ref element_rect, ..
    } = children[0]
    else {
        panic!("Expected Element");
    };
    assert_eq!(element_rect.height, 480.0); // 1920 * 25 / 100
}

#[test]
fn height_property_resolution_height() {
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
        rules: vec![create_rule("height", "75rh")],
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
        ref element_rect, ..
    } = children[0]
    else {
        panic!("Expected Element");
    };
    assert_eq!(element_rect.height, 810.0); // 1080 * 75 / 100
}

#[test]
fn height_property_percent() {
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
            rules: vec![create_rule("height", "600px")],
        },
        VSSItem {
            selectors: vec![VSSSelectorTree::Selectors(vec![VSSSelector::Class(
                "child".to_string(),
            )])],
            rules: vec![create_rule("height", "50%")],
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
        ref element_rect, ..
    } = inner_children[0]
    else {
        panic!("Expected Element");
    };
    assert_eq!(element_rect.height, 300.0); // 600 * 50 / 100
}

// Width and Height combination tests
#[test]
fn width_and_height_property_both_pixel() {
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
        rules: vec![
            create_rule("width", "640px"),
            create_rule("height", "480px"),
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

    let ObjectData::Element { children, .. } = result.object else {
        panic!("Expected Element");
    };
    let ObjectData::Element {
        ref element_rect, ..
    } = children[0]
    else {
        panic!("Expected Element");
    };
    assert_eq!(element_rect.width, 640.0);
    assert_eq!(element_rect.height, 480.0);
}

#[test]
fn width_property_with_aspect_ratio() {
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
        rules: vec![create_rule("width", "50px")],
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
        ref element_rect, ..
    } = children[0]
    else {
        panic!("Expected Element");
    };
    // default_image_size is 100x100, width is set to 50
    // height should maintain aspect ratio: 100 * 50 / 100 = 50
    assert_eq!(element_rect.width, 50.0);
    assert_eq!(element_rect.height, 50.0);
}

#[test]
fn height_property_with_aspect_ratio() {
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
        rules: vec![create_rule("height", "200px")],
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
        ref element_rect, ..
    } = children[0]
    else {
        panic!("Expected Element");
    };
    // default_image_size is 100x100, height is set to 200
    // height is expanded, not shrunk, so aspect ratio is not maintained
    assert_eq!(element_rect.width, 100.0);
    assert_eq!(element_rect.height, 200.0);
}

#[test]
fn width_and_height_property_mixed_units() {
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
        rules: vec![create_rule("width", "50rw"), create_rule("height", "50rh")],
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
        ref element_rect, ..
    } = children[0]
    else {
        panic!("Expected Element");
    };
    assert_eq!(element_rect.width, 960.0); // 1920 * 50 / 100
    assert_eq!(element_rect.height, 540.0); // 1080 * 50 / 100
}

// Tests without default_size (like text objects)
#[test]
fn width_property_without_default_size() {
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
        rules: vec![create_rule("width", "500px")],
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
        TestObjectProcessorProperty::default().without_default_size(),
    );
    let result = crate::convert(&vsml, &provider);

    let ObjectData::Element { children, .. } = result.object else {
        panic!("Expected Element");
    };
    let ObjectData::Element {
        ref element_rect, ..
    } = children[0]
    else {
        panic!("Expected Element");
    };
    assert_eq!(element_rect.width, 500.0);
    assert_eq!(element_rect.height, 0.0);
}

#[test]
fn height_property_without_default_size() {
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
        rules: vec![create_rule("height", "300px")],
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
        TestObjectProcessorProperty::default().without_default_size(),
    );
    let result = crate::convert(&vsml, &provider);

    let ObjectData::Element { children, .. } = result.object else {
        panic!("Expected Element");
    };
    let ObjectData::Element {
        ref element_rect, ..
    } = children[0]
    else {
        panic!("Expected Element");
    };
    assert_eq!(element_rect.width, 0.0);
    assert_eq!(element_rect.height, 300.0);
}

#[test]
fn width_and_height_property_without_default_size() {
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
        rules: vec![
            create_rule("width", "640px"),
            create_rule("height", "480px"),
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

    let provider = TestObjectProcessorProvider::with(
        TestObjectProcessorProperty::default().without_default_size(),
    );
    let result = crate::convert(&vsml, &provider);

    let ObjectData::Element { children, .. } = result.object else {
        panic!("Expected Element");
    };
    let ObjectData::Element {
        ref element_rect, ..
    } = children[0]
    else {
        panic!("Expected Element");
    };
    assert_eq!(element_rect.width, 640.0);
    assert_eq!(element_rect.height, 480.0);
}

#[test]
fn width_property_with_custom_default_size() {
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
        rules: vec![create_rule("width", "80px")],
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

    // default_image_size: 200x150のカスタムサイズ
    let provider = TestObjectProcessorProvider::with(
        TestObjectProcessorProperty::default().with_image_size(200.0, 150.0),
    );
    let result = crate::convert(&vsml, &provider);

    let ObjectData::Element { children, .. } = result.object else {
        panic!("Expected Element");
    };
    let ObjectData::Element {
        ref element_rect, ..
    } = children[0]
    else {
        panic!("Expected Element");
    };
    // width: 80, aspect ratio maintained: 150 * 80 / 200 = 60
    assert_eq!(element_rect.width, 80.0);
    assert_eq!(element_rect.height, 60.0);
}

#[test]
fn height_property_with_custom_default_size() {
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
        rules: vec![create_rule("height", "75px")],
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

    // default_image_size: 200x150のカスタムサイズ
    let provider = TestObjectProcessorProvider::with(
        TestObjectProcessorProperty::default().with_image_size(200.0, 150.0),
    );
    let result = crate::convert(&vsml, &provider);

    let ObjectData::Element { children, .. } = result.object else {
        panic!("Expected Element");
    };
    let ObjectData::Element {
        ref element_rect, ..
    } = children[0]
    else {
        panic!("Expected Element");
    };
    // height: 75, aspect ratio maintained: 200 * 75 / 150 = 100
    assert_eq!(element_rect.width, 100.0);
    assert_eq!(element_rect.height, 75.0);
}
