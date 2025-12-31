use super::common::*;
use crate::VssScanner;
use vsml_ast::vss::{VSSItem, VSSSelector, VSSSelectorTree};

#[test]
fn matches_direct_child() {
    // <div class="parent">
    //   <div class="child">  <!-- .parent > .child としてマッチする -->
    //   </div>
    // </div>
    let parent = create_element("div", Some("parent"), None);
    let child = create_element("div", Some("child"), None);
    let parent_elements = vec![parent];
    let child_elements = vec![child];

    let vss_items = vec![VSSItem {
        selectors: vec![VSSSelectorTree::Child(
            vec![VSSSelector::Class("parent".to_string())],
            Box::new(VSSSelectorTree::Selectors(vec![VSSSelector::Class(
                "child".to_string(),
            )])),
        )],
        rules: vec![create_rule("color", "red")],
    }];

    let mut scanner = VssScanner::new(&vss_items);
    scanner.traverse_stack = vec![&parent_elements, &child_elements];

    assert_eq!(scanner.scan().count(), 1);
}

#[test]
fn no_match_for_indirect_child() {
    // <div class="parent">
    //   <div class="middle">
    //     <div class="child">  <!-- このchildはparentの直接の子ではないため、マッチしない -->
    //     </div>
    //   </div>
    // </div>
    let root = create_element("div", Some("parent"), None);
    let middle = create_element("div", Some("middle"), None);
    let leaf = create_element("div", Some("child"), None);
    let root_elements = vec![root];
    let middle_elements = vec![middle];
    let leaf_elements = vec![leaf];

    let vss_items = vec![VSSItem {
        selectors: vec![VSSSelectorTree::Child(
            vec![VSSSelector::Class("parent".to_string())],
            Box::new(VSSSelectorTree::Selectors(vec![VSSSelector::Class(
                "child".to_string(),
            )])),
        )],
        rules: vec![create_rule("color", "red")],
    }];

    let mut scanner = VssScanner::new(&vss_items);
    scanner.traverse_stack = vec![&root_elements, &middle_elements, &leaf_elements];

    assert_eq!(scanner.scan().count(), 0);
}

#[test]
fn matches_nested_child_selectors() {
    // <div class="parent">
    //   <div class="middle">
    //     <div class="child">  <!-- .parent > .middle > .child としてマッチする -->
    //     </div>
    //   </div>
    // </div>
    let root = create_element("div", Some("parent"), None);
    let middle = create_element("div", Some("middle"), None);
    let leaf = create_element("div", Some("child"), None);
    let root_elements = vec![root];
    let middle_elements = vec![middle];
    let leaf_elements = vec![leaf];

    let vss_items = vec![VSSItem {
        selectors: vec![VSSSelectorTree::Child(
            vec![VSSSelector::Class("parent".to_string())],
            Box::new(VSSSelectorTree::Child(
                vec![VSSSelector::Class("middle".to_string())],
                Box::new(VSSSelectorTree::Selectors(vec![VSSSelector::Class(
                    "child".to_string(),
                )])),
            )),
        )],
        rules: vec![create_rule("color", "red")],
    }];

    let mut scanner = VssScanner::new(&vss_items);
    scanner.traverse_stack = vec![&root_elements, &middle_elements, &leaf_elements];

    assert_eq!(scanner.scan().count(), 1);
}

#[test]
fn no_match_for_wrong_child_order() {
    // <div class="parent">
    //   <div class="child">     <!-- .middle > .child の順序が正しくないためマッチしない -->
    //     <div class="middle">
    //     </div>
    //   </div>
    // </div>
    let root = create_element("div", Some("parent"), None);
    let middle = create_element("div", Some("child"), None);
    let leaf = create_element("div", Some("middle"), None);
    let root_elements = vec![root];
    let middle_elements = vec![middle];
    let leaf_elements = vec![leaf];

    let vss_items = vec![VSSItem {
        selectors: vec![VSSSelectorTree::Child(
            vec![VSSSelector::Class("parent".to_string())],
            Box::new(VSSSelectorTree::Child(
                vec![VSSSelector::Class("middle".to_string())],
                Box::new(VSSSelectorTree::Selectors(vec![VSSSelector::Class(
                    "child".to_string(),
                )])),
            )),
        )],
        rules: vec![create_rule("color", "red")],
    }];

    let mut scanner = VssScanner::new(&vss_items);
    scanner.traverse_stack = vec![&root_elements, &middle_elements, &leaf_elements];

    assert_eq!(scanner.scan().count(), 0);
}

#[test]
fn mixed_child_and_descendant() {
    // <div class="parent">
    //   <div class="middle">
    //     <div class="other">
    //       <div class="child">  <!-- .parent > .middle .child としてマッチする -->
    //       </div>
    //     </div>
    //   </div>
    // </div>
    let root = create_element("div", Some("parent"), None);
    let middle = create_element("div", Some("middle"), None);
    let other = create_element("div", Some("other"), None);
    let child = create_element("div", Some("child"), None);
    let root_elements = vec![root];
    let middle_elements = vec![middle];
    let other_elements = vec![other];
    let child_elements = vec![child];

    let vss_items = vec![VSSItem {
        selectors: vec![VSSSelectorTree::Child(
            vec![VSSSelector::Class("parent".to_string())],
            Box::new(VSSSelectorTree::Descendant(
                vec![VSSSelector::Class("middle".to_string())],
                Box::new(VSSSelectorTree::Selectors(vec![VSSSelector::Class(
                    "child".to_string(),
                )])),
            )),
        )],
        rules: vec![create_rule("color", "red")],
    }];

    let mut scanner = VssScanner::new(&vss_items);
    scanner.traverse_stack = vec![
        &root_elements,
        &middle_elements,
        &other_elements,
        &child_elements,
    ];

    assert_eq!(scanner.scan().count(), 1);
}
