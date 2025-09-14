use super::*;
use std::collections::HashMap;
use vsml_ast::vsml::Element;
use vsml_ast::vss::{Rule, VSSItem, VSSSelector, VSSSelectorTree};

fn create_element(name: &str, class: Option<&str>, id: Option<&str>) -> Element {
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

fn create_rule(property: &str, value: &str) -> Rule {
    Rule {
        property: property.to_string(),
        value: value.to_string(),
    }
}

fn create_descendant_selector(parent_class: &str, child_class: &str) -> VSSSelectorTree {
    VSSSelectorTree::Descendant(
        vec![VSSSelector::Class(parent_class.to_string())],
        Box::new(VSSSelectorTree::Selectors(vec![VSSSelector::Class(
            child_class.to_string(),
        )])),
    )
}

mod scan_tests {
    use super::*;

    #[test]
    fn matches_simple_descendant_selector() {
        let parent = create_element("div", Some("parent"), None);
        let child = create_element("div", Some("child"), None);
        let elements = vec![parent];
        let child_elements = vec![child];

        let rule = create_rule("color", "red");
        let vss_items = vec![VSSItem {
            selectors: vec![create_descendant_selector("parent", "child")],
            rules: vec![rule],
        }];

        let mut scanner = VssScanner::new(&vss_items);
        scanner.traverse_stack = vec![&elements, &child_elements];

        assert_eq!(scanner.scan().count(), 1);
    }

    #[test]
    fn no_match_for_non_existent_class() {
        let parent = create_element("div", Some("parent"), None);
        let child = create_element("div", Some("other"), None);
        let elements = vec![parent];
        let child_elements = vec![child];

        let rule = create_rule("color", "red");
        let vss_items = vec![VSSItem {
            selectors: vec![create_descendant_selector("parent", "child")],
            rules: vec![rule],
        }];

        let mut scanner = VssScanner::new(&vss_items);
        scanner.traverse_stack = vec![&elements, &child_elements];

        assert_eq!(scanner.scan().count(), 0);
    }

    #[test]
    fn matches_multiple_rules() {
        let parent = create_element("div", Some("parent"), None);
        let child = create_element("div", Some("child"), None);
        let elements = vec![parent];
        let child_elements = vec![child];

        let vss_items = vec![VSSItem {
            selectors: vec![create_descendant_selector("parent", "child")],
            rules: vec![
                create_rule("color", "red"),
                create_rule("background", "blue"),
            ],
        }];

        let mut scanner = VssScanner::new(&vss_items);
        scanner.traverse_stack = vec![&elements, &child_elements];

        assert_eq!(scanner.scan().count(), 2);
    }

    #[test]
    fn no_match_for_incorrect_hierarchy() {
        // <tag class="a">
        //   <tag class="b">
        //     <tag class="c">
        //     </tag>
        //   </tag>
        // </tag>
        let root = create_element("tag", Some("a"), None);
        let middle = create_element("tag", Some("b"), None);
        let leaf = create_element("tag", Some("c"), None);

        let root_elements = vec![root];
        let middle_elements = vec![middle];
        let leaf_elements = vec![leaf];

        // セレクタ: .b .a .c
        let vss_items = vec![VSSItem {
            selectors: vec![VSSSelectorTree::Descendant(
                vec![VSSSelector::Class("b".to_string())],
                Box::new(VSSSelectorTree::Descendant(
                    vec![VSSSelector::Class("a".to_string())],
                    Box::new(VSSSelectorTree::Selectors(vec![VSSSelector::Class(
                        "c".to_string(),
                    )])),
                )),
            )],
            rules: vec![create_rule("color", "red")],
        }];

        let mut scanner = VssScanner::new(&vss_items);
        scanner.traverse_stack = vec![&root_elements, &middle_elements, &leaf_elements];

        // .b .a .c はマッチしてはいけない（.aは.bの子孫ではないため）
        assert_eq!(scanner.scan().count(), 0);
    }
}
