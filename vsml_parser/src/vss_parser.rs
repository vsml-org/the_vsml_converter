use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{newline, space1};
use nom::combinator::{all_consuming, iterator, map, peek, success};
use nom::multi::{many0, many1};
use nom::sequence::{terminated, tuple};
use nom::IResult;
use regex::Regex;
use std::sync::LazyLock;
use thiserror::Error;

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

#[derive(Debug, Error, PartialEq)]
pub enum VSSParseError {}

pub fn parse(vss: &str) -> Result<Vec<VSSItem>, VSSParseError> {
    match parse_vss_item_list(vss) {
        Ok((_, result)) => Ok(result),
        Err(e) => {
            todo!()
        }
    }
}

fn parse_vss_item_list(input: &str) -> IResult<&str, Vec<VSSItem>> {
    let (input, _) = skip_comment_or_whitespace(input)?;
    all_consuming(many0(terminated(
        parse_vss_item,
        skip_comment_or_whitespace,
    )))(input)
}

fn parse_vss_item(input: &str) -> IResult<&str, VSSItem> {
    let mut iter = iterator(
        input,
        terminated(
            parse_vss_selector,
            tuple((tag(","), skip_comment_or_whitespace)),
        ),
    );
    let mut selectors = iter.collect::<Vec<_>>();
    let (mut input, _) = iter.finish()?;
    if let Ok((i, selector)) = parse_vss_selector(input) {
        input = i;
        selectors.push(selector);
    }
    let (input, _) = skip_comment_or_whitespace(input)?;
    let (input, _) = tag("{")(input)?;
    let (input, _) = skip_comment_or_whitespace(input)?;
    let mut iter = iterator(
        input,
        terminated(
            parse_vss_rule,
            tuple((tag(";"), skip_comment_or_whitespace)),
        ),
    );
    let mut rules = iter.collect::<Vec<_>>();
    let (mut input, _) = iter.finish()?;
    if let Ok((i, rule)) = parse_vss_rule(input) {
        input = i;
        rules.push(rule);
    }
    let (input, _) = skip_comment_or_whitespace(input)?;
    let (input, _) = tag("}")(input)?;
    let (input, _) = skip_comment_or_whitespace(input)?;

    Ok((input, VSSItem { selectors, rules }))
}

fn parse_vss_selector(input: &str) -> IResult<&str, VSSSelectorTree> {
    enum Operator {
        Descendant,
        Child,
        Sibling,
        AdjSibling,
    }
    let (mut i, _) = skip_comment_or_whitespace(input)?;
    let mut buffer = Vec::new();
    loop {
        let (input, selectors) = parse_vss_selector_selectors(i)?;
        let (input, op) = alt((tag("+"), tag(">"), tag("~"), success("")))(input)?;
        let op = match op {
            ">" => Operator::Child,
            "+" => Operator::Sibling,
            "~" => Operator::AdjSibling,
            "" => {
                if peek::<_, _, (), _>(alt((tag(","), tag("{"))))(input).is_ok() {
                    return Ok((
                        input,
                        buffer.into_iter().rev().fold(
                            VSSSelectorTree::Selectors(selectors),
                            |acc, (selectors, op)| match op {
                                Operator::Descendant => {
                                    VSSSelectorTree::Descendant(selectors, Box::new(acc))
                                }
                                Operator::Child => VSSSelectorTree::Child(selectors, Box::new(acc)),
                                Operator::Sibling => {
                                    VSSSelectorTree::Sibling(selectors, Box::new(acc))
                                }
                                Operator::AdjSibling => {
                                    VSSSelectorTree::AdjSibling(selectors, Box::new(acc))
                                }
                            },
                        ),
                    ));
                }
                Operator::Descendant
            }
            _ => unreachable!(),
        };
        buffer.push((selectors, op));
        let (input, ()) = skip_comment_or_whitespace(input)?;
        i = input;
    }
}

fn parse_vss_selector_selectors(input: &str) -> IResult<&str, Vec<VSSSelector>> {
    macro_rules! make_identifier_rule {
        ($head:literal) => {
            concat!("^", $head, "(?:[_a-zA-Z]|-[a-zA-Z_-])[-_0-9a-zA-Z]*")
        };
    }
    // TODO: Attributeに対応する
    static TAG: LazyLock<Regex> = LazyLock::new(|| Regex::new(make_identifier_rule!("")).unwrap());
    static CLASS: LazyLock<Regex> =
        LazyLock::new(|| Regex::new(make_identifier_rule!("\\.")).unwrap());
    static ID: LazyLock<Regex> = LazyLock::new(|| Regex::new(make_identifier_rule!("#")).unwrap());
    static PSEUDO_CLASS: LazyLock<Regex> =
        LazyLock::new(|| Regex::new(make_identifier_rule!(":{1,2}")).unwrap());
    terminated(
        many1(map(
            terminated(
                alt((
                    tag("*"),
                    regex_matches(&TAG),
                    regex_matches(&CLASS),
                    regex_matches(&ID),
                    regex_matches(&PSEUDO_CLASS),
                )),
                skip_comments,
            ),
            |selector| {
                if selector == "*" {
                    VSSSelector::All
                } else if let Some(class_name) = selector.strip_prefix(".") {
                    VSSSelector::Class(class_name.to_owned())
                } else if let Some(id) = selector.strip_prefix("#") {
                    VSSSelector::Id(id.to_owned())
                } else if let Some(pseudo_class) = selector.strip_prefix(":") {
                    VSSSelector::PseudoClass(pseudo_class.to_owned())
                } else {
                    VSSSelector::Tag(selector.to_owned())
                }
            },
        )),
        skip_comment_or_whitespace,
    )(input)
}

// <property>: <value>
fn parse_vss_rule(input: &str) -> IResult<&str, Rule> {
    let (input, _) = skip_comment_or_whitespace(input)?;
    let (input, property) = map(
        regex_matches(&Regex::new(r"^[a-zA-Z-]+").unwrap()),
        |s: &str| s.to_owned(),
    )(input)?;
    let (input, _) = skip_comment_or_whitespace(input)?;
    let (input, _) = tag(":")(input)?;
    let (input, _) = skip_comment_or_whitespace(input)?;
    static QUOTE_STRING: LazyLock<Regex> =
        LazyLock::new(|| Regex::new(r#"^"((?:[^\\"]+|\\.)*)""#).unwrap());
    static COMMON_VALUE: LazyLock<Regex> =
        LazyLock::new(|| Regex::new("^[a-zA-Z0-9-()%.#]").unwrap());
    static SPACES: LazyLock<Regex> = LazyLock::new(|| Regex::new(r#"^\s+"#).unwrap());
    let mut iter = iterator(
        input,
        terminated(
            alt((
                regex_matches(&QUOTE_STRING),
                regex_matches(&COMMON_VALUE),
                map(regex_matches(&SPACES), |_| " "),
            )),
            skip_comments,
        ),
    );
    let value = iter.fold(String::new(), |mut acc, s| {
        if s.is_empty() {
            return acc;
        }
        match s {
            " " => {
                if acc.chars().next_back().is_none_or(|c| c != ' ') {
                    acc.push_str(s);
                }
            }
            _ => {
                acc.push_str(s);
            }
        }
        acc
    });
    let (input, _) = iter.finish()?;

    Ok((input, Rule { property, value }))
}

fn skip_comment_or_whitespace(mut input: &str) -> IResult<&str, ()> {
    let mut len = input.len();
    loop {
        (input, _) = alt((
            map(space1, |_| ()),
            map(newline, |_| ()),
            skip_comments,
            success(()),
        ))(input)
        .unwrap();
        if input.len() == len {
            return Ok((input, ()));
        }
        len = input.len();
    }
}

fn skip_comments(input: &str) -> IResult<&str, ()> {
    let mut i = input;
    loop {
        let Ok((mut input, _)) = tag::<_, _, ()>("/*")(i) else {
            return Ok((i, ()));
        };
        loop {
            (input, _) = nom::bytes::complete::is_not("*")(input)?;
            let Ok((input, _)) = tag::<_, _, ()>("*/")(input) else {
                (input, _) = tag("*")(input)?;
                continue;
            };
            i = input;
            break;
        }
    }
}

fn regex_matches(regex: &Regex) -> impl for<'a> Fn(&'a str) -> IResult<&'a str, &'a str> + '_ {
    move |input| {
        if let Some(capture) = regex.find(input) {
            let matches_str = capture.as_str();
            Ok((&input[matches_str.len()..], matches_str))
        } else {
            Err(nom::Err::Error(nom::error::Error::new(
                input,
                nom::error::ErrorKind::RegexpMatch,
            )))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_skip_comment() {
        assert_eq!(skip_comments("/* comment */"), Ok(("", ())));
        assert_eq!(skip_comments("/* comment */hoge"), Ok(("hoge", ())));
        assert_eq!(skip_comments("/* comment *//* comment */"), Ok(("", ())));
        assert_eq!(skip_comments("/* 10 * 2 = 20 */hoge"), Ok(("hoge", ())));
        assert_eq!(
            skip_comments(" /* 10 * 2 = 20 */"),
            Ok((" /* 10 * 2 = 20 */", ()))
        );
        assert!(skip_comments("/* 10 * 2 = 20").is_err());
    }

    #[test]
    fn test_parse() {
        assert_eq!(
            parse(
                "
            seq {
              duration: 1s;
            }
            .subtitles-container txt {
              font-size: 20px;
              font-border-color: red;
            }
            #main-frame {
              width: 100rh;
            }",
            ),
            Ok(vec![
                VSSItem {
                    selectors: vec![VSSSelectorTree::Selectors(vec![VSSSelector::Tag(
                        "seq".to_string()
                    )])],
                    rules: vec![Rule {
                        property: "duration".to_string(),
                        value: "1s".to_string()
                    },]
                },
                VSSItem {
                    selectors: vec![VSSSelectorTree::Descendant(
                        vec![VSSSelector::Class("subtitles-container".to_string())],
                        Box::new(VSSSelectorTree::Selectors(vec![VSSSelector::Tag(
                            "txt".to_string()
                        )])),
                    )],
                    rules: vec![
                        Rule {
                            property: "font-size".to_string(),
                            value: "20px".to_string()
                        },
                        Rule {
                            property: "font-border-color".to_string(),
                            value: "red".to_string()
                        },
                    ]
                },
                VSSItem {
                    selectors: vec![VSSSelectorTree::Selectors(vec![VSSSelector::Id(
                        "main-frame".to_string()
                    )])],
                    rules: vec![Rule {
                        property: "width".to_string(),
                        value: "100rh".to_string()
                    },]
                }
            ])
        );
    }
}
