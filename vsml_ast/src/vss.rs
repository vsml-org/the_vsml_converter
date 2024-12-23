#[derive(Debug, PartialEq)]
pub enum VSSSelectorAttributeValue {
    None,
    Equal(String),
    Contain(String),
    StartWith(String),
    EndWith(String),
    Include(String),
}

#[derive(Debug, PartialEq)]
pub enum VSSSelector {
    All,
    Tag(String),
    Class(String),
    Id(String),
    PseudoClass(String),
    Attribute(String, VSSSelectorAttributeValue),
}

#[derive(Debug, PartialEq)]
pub enum VSSSelectorTree {
    // .selector1.selector2
    Selectors(Vec<VSSSelector>),
    // .selector .selector
    Descendant(Vec<VSSSelector>, Box<VSSSelectorTree>),
    // .selector > .selector
    Child(Vec<VSSSelector>, Box<VSSSelectorTree>),
    // .selector + .selector
    Sibling(Vec<VSSSelector>, Box<VSSSelectorTree>),
    // .selector ~ .selector
    AdjSibling(Vec<VSSSelector>, Box<VSSSelectorTree>),
}

#[derive(Debug, PartialEq)]
pub struct Rule {
    pub property: String,
    pub value: String,
}

#[derive(Debug, PartialEq)]
pub struct VSSItem {
    // <SelectorTree>, <SelectorTree>
    pub selectors: Vec<VSSSelectorTree>,
    pub rules: Vec<Rule>,
}
