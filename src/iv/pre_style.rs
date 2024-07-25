// styleタグから持ってきたstyleのstruct
pub trait PreStyle: std::fmt::Debug {}

struct Duration {
    value: f64,
}

impl PreStyle for Duration {}
