use std::collections::HashMap;

#[derive(Debug)]
pub struct Data {
    pub polymer_template: Vec<char>,
    pub pair_insertions: HashMap<String, char>,
}
