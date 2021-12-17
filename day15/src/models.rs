use std::fmt;

pub struct Node {
    pub value: u64,
    pub visited: bool,
    pub distance:u64
}

impl fmt::Debug for Node {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let visited_str;
        if self.visited {
            visited_str = "+";
        } else {
            visited_str = "-";
        }

        write!(f, "{}{}", self.value, visited_str)
    }
}
