use crate::{builder::FormulaBuilder, constants::ElementSymbol};

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Element {
    pub name: ElementSymbol,
    pub count: u16,
}

impl Element {
    pub fn new(name: ElementSymbol, count: u16) -> Self {
        Self { name, count }
    }

    pub fn unknown() -> Self {
        panic!("Constructing an Unknown element")
    }
}

#[derive(Debug, Clone, Default, Eq, PartialEq)]
pub struct Group {
    pub elements: Vec<Element>,
    pub count: u16,
}

impl Group {
    pub fn new<I: Iterator<Item = Element>>(elements: I, count: u16) -> Self {
        Self {
            elements: elements.collect(),
            count,
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum ChemUnit {
    Element(Element),
    Group(Group),
}

impl ChemUnit {
    pub fn new_element(name: ElementSymbol, count: u16) -> Self {
        Self::Element(Element { name, count })
    }

    pub fn new_group(elements: Vec<ChemUnit>, count: u16) -> Self {
        let elements = elements.into_iter()
            .map(|unit| {
                if let ChemUnit::Element(element) = unit {
                    return element;
                }
                Element::unknown()
            });
        Self::Group(Group::new(elements, count))
    }
}

#[derive(Debug, Clone, Default, Eq, PartialEq)]
pub struct Compound {
    pub elements: Vec<ChemUnit>,
}

#[derive(Debug, Clone, Default, Eq, PartialEq)]
pub struct Formula {
    pub left_compounds: Vec<Compound>,
    pub right_compounds: Vec<Compound>,
}

impl Formula {
    pub fn builder() -> FormulaBuilder {
        FormulaBuilder::new()
    }
}


#[test]
fn tester() {
    let formula = Formula::builder()
        .add_left_compound(Compound {
            elements: vec![
                ChemUnit::new_group(vec![
                    ChemUnit::new_element(ElementSymbol::N, 1),
                    ChemUnit::new_element(ElementSymbol::N, 1),
                ], 2),
    
                ChemUnit::new_element(ElementSymbol::S, 1),
                ChemUnit::new_element(ElementSymbol::O, 4),
            ],
        })
        .add_right_compound(Compound {
            elements: vec![
                ChemUnit::new_element(ElementSymbol::H, 2),
                ChemUnit::new_element(ElementSymbol::O, 1),
            ],
        })
        .build();

    println!("{:?}", formula);
}