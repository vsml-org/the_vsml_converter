#[derive(Debug, PartialEq)]
pub enum VSSSelectorAttributeValue {
    /// `[src]` のようなアトリビュート名のみの指定
    None,
    /// `[src="video.mp4"]` のようなアトリビュートの値の完全一致の指定
    Equal(String),
    /// `[class~="hoge"]` のようなアトリビュートの値の空白区切りリストの1つとの一致の指定
    Contain(String),
    /// `[src^="video"]` のようなアトリビュートの値の先頭一致の指定
    StartWith(String),
    /// `[src$=".mp4"]` のようなアトリビュートの値の末尾一致の指定
    EndWith(String),
    /// `[class*="ho"]` のようなアトリビュートの値の部分一致の指定
    Include(String),
}

#[derive(Debug, PartialEq)]
pub enum VSSSelector {
    /// `*` のセレクタ
    All,
    /// `seq` のようなタグ名指定のセレクタ
    Tag(String),
    /// `.selector` のようなクラス名指定のセレクタ
    Class(String),
    /// `#id` のようなID名指定のセレクタ
    Id(String),
    /// `:after` のような擬似クラスセレクタ
    PseudoClass(String),
    /// `[src="video.mp4"]` のようなアトリビュートを指定するセレクタ
    Attribute(String, VSSSelectorAttributeValue),
}

#[derive(Debug, PartialEq)]
pub enum VSSSelectorTree {
    /// `.selector1.selector2` のような単一のエレメントを指すセレクタ
    Selectors(Vec<VSSSelector>),
    /// `.selector .selector` のような子孫のエレメントを指すセレクタ
    Descendant(Vec<VSSSelector>, Box<VSSSelectorTree>),
    /// `.selector > .selector` のような子エレメントを指すセレクタ
    Child(Vec<VSSSelector>, Box<VSSSelectorTree>),
    /// `.selector + .selector` のような後続の弟エレメントを指すセレクタ
    Sibling(Vec<VSSSelector>, Box<VSSSelectorTree>),
    /// `.selector ~ .selector` のような直後の弟エレメントを指すセレクタ
    AdjSibling(Vec<VSSSelector>, Box<VSSSelectorTree>),
}

/// `background-color: red` のような単一のルール
#[derive(Debug, PartialEq)]
pub struct Rule {
    pub property: String,
    pub value: String,
}

/// セレクタとそのルールのセット
#[derive(Debug, PartialEq)]
pub struct VSSItem {
    /// `<SelectorTree>, <SelectorTree>` のように複数のセレクタを表現するためVecとなっている
    pub selectors: Vec<VSSSelectorTree>,
    pub rules: Vec<Rule>,
}
