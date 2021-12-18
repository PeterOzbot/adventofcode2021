#[derive(Debug)]
pub struct Package {
    pub version: u64,
    pub id: PackageType,
    pub value: PackageValue,
}

#[derive(Debug)]
pub enum PackageValue {
    LiteralValue(u64),
    Packages(Vec<Package>),
}

#[derive(Debug)]
pub enum PackageType {
    Sum,          //0
    Product,      //1
    Minimum,      //2
    Maximum,      //3
    LiteralValue, //4
    GreaterThan,  //5
    LessThan,     //6
    EqualTo,      //7
}

#[derive(Debug)]
pub enum OperatorMode{
    Bits(u64),
    Packages(u64)
}