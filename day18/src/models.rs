use std::fmt;

#[derive(Clone)]
pub enum SnailfishNumber {
    Pair(Box<SnailfishNumber>, Box<SnailfishNumber>),
    Number(u8),
}

impl fmt::Debug for SnailfishNumber {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self {
            SnailfishNumber::Number(value) => write!(f, "{}", value),
            SnailfishNumber::Pair(right_value, left_value) => {
                write!(f, "[{:?},{:?}]", right_value, left_value)
            }
        }
    }
}