// 実際に変換するときに使用するStyleのtrait
pub trait Style: std::fmt::Debug {
    fn adapt_style(&self);
}

/// こういった感じにtraitを実装するstructを複数作っていく
#[derive(Debug)]
pub struct TimeMarginStart {}

impl Style for TimeMarginStart {
    fn adapt_style(&self) {}
}
