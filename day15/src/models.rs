use std::hash::{Hash, Hasher};

pub struct VisitedNode {
    pub x: usize,
    pub y: usize,
}

impl PartialEq for VisitedNode {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.y && self.y == other.y
    }
}
impl Eq for VisitedNode {}

impl Hash for VisitedNode {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.x.hash(state);
        self.y.hash(state);
    }
}