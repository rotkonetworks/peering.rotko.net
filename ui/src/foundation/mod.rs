pub mod column;
pub mod row;

#[derive(Debug, Clone, PartialEq)]
pub enum Arrangement {
    Start,
    Center,
    Between,
    Around,
    Evenly,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Alignment {
    Start,
    Center,
    End,
}
