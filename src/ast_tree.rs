use crate::constants::ElementSymbol;

pub struct Element {
    pub name: ElementSymbol,
    pub count: u16,
}

pub struct Compound {
    pub elements: Vec<Element>,
}

pub struct Group {
    pub elements: Vec<Element>,
}