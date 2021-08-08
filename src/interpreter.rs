#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Expr {}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Value {}

pub type Coord = (i64, i64);

pub type Picture = Vec<Coord>;
