use super::common::*;
use crate::VssScanner;
use vsml_ast::vss::{VSSItem, VSSSelector, VSSSelectorTree};

#[test]
fn matches_simple_selector() {
    let element = create_element("div", Some("test"), None);
    let elements = vec![element];

    let rule = create_rule("color", "red");
    let vss_items = vec![VSSItem {
        selectors: vec![VSSSelectorTree::Selectors(vec![VSSSelector::Class(
            "test".to_string(),
        )])],
        rules: vec![rule],
    }];

    let mut scanner = VssScanner::new(&vss_items);
    scanner.traverse_stack = vec![&elements];

    assert_eq!(scanner.scan().count(), 1);
}
