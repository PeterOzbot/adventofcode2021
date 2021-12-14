use std::fmt;

pub struct Coordinate {
    pub x: u32,
    pub y: u32,
}
pub enum FoldType {
    X,
    Y,
}
pub struct FoldInstruction {
    pub fold_type: FoldType,
    pub value: u32,
}
pub  struct Data {
    pub dots_coordinates: Vec<Coordinate>,
    pub fold_instructions: Vec<FoldInstruction>,
}

impl fmt::Debug for Data {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Coordinates: {:?} \n Instructions: {:?}",
            self.dots_coordinates, self.fold_instructions
        )
    }
}

impl fmt::Debug for Coordinate {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?},{:?}", self.x, self.y)
    }
}

impl fmt::Debug for FoldInstruction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let fold_type = match self.fold_type {
            FoldType::X => "X",
            FoldType::Y => "Y",
        };
        write!(f, "Fold along{:?}={:?}", fold_type, self.value)
    }
}
