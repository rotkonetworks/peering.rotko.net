pub mod column;
pub mod row;

#[derive(Debug, Clone, PartialEq)]
pub enum VerticalArrangement {
    Start,
    Center,
    Between,
    Around,
    Evenly,
}

#[derive(Debug, Clone, PartialEq)]
pub enum HorizontalAlignment {
    Start,
    Center,
    End,
}