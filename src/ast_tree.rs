use crate::{builder::FormulaBuilder, constants::{ElementSymbol, ELEMENT_STR_MAP}};

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Element {
    pub symbol: ElementSymbol,
    pub count: u16,
}

impl Element {
    pub fn new(symbol: ElementSymbol, count: u16) -> Self {
        Self { symbol, count }
    }

    pub fn unknown() -> Self {
        log::error!("Constructing an Unknown element");
        Self {
            symbol: ElementSymbol::Unknown,
            count: 0,
        }
    }

    pub fn as_string(&self) -> String {
        let element_name = ELEMENT_STR_MAP.get(&self.symbol).unwrap();
        if self.count == 1 {
            return element_name.to_string();
        }
        format!("{}{}", element_name, self.count)
    }
}

#[derive(Debug, Clone, Default, Eq, PartialEq)]
pub struct Group {
    pub elements: Vec<Element>,
    pub count: u16,
}

impl Group {
    pub fn new(elements: Vec<Element>, count: u16) -> Self {
        assert!(!elements.is_empty());
        if elements.len() < 2 {
            panic!("A group must have at least 2 elements");
        }
        Self {
            elements,
            count,
        }
    }
}

/// A chemical unit can be either an element or a group of elements.
/// for example `"H"`, `"(NH4)"`, `"SO4"`, `"CH4"` can be treated as chemical unit.
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum ChemUnit {
    Element(Element),
    Group(Group),
}

impl ChemUnit {
    pub fn new_element(symbol: ElementSymbol, count: u16) -> Self {
        assert_ne!(ElementSymbol::Unknown, symbol);
        Self::Element(Element { symbol, count })
    }

    pub fn new_group(elements: Vec<ChemUnit>, count: u16) -> Self {
        let elements = elements
            .into_iter()
            .map(|unit| {
                if let ChemUnit::Element(element) = unit {
                    return element;
                }
                Element::unknown()
            })
            .collect();
        Self::Group(Group::new(elements, count))
    }

    pub fn as_string(&self) -> String {
        // where group_to_string is a helper function to convert a group to string.
            fn group_to_string(group: &Group) -> String {
                let mut string = String::new();
                group.elements.iter()
                    .for_each(|element| {
                        string.push_str(&element.as_string());
                    });
                if group.count == 1 {
                    return string;
                }
                format!("({}){}", string, group.count)
            }
        // end of helper function

        match self {
            ChemUnit::Element(element) => element.as_string(),
            ChemUnit::Group(group) => group_to_string(group),
        }
    }
}

#[derive(Debug, Clone, Default, Eq, PartialEq)]
pub struct Compound {
    pub chem_units: Vec<ChemUnit>,
}

impl Compound {
    pub fn as_string(&self) -> String {
        let mut string = String::new();
        self.chem_units.iter()
            .for_each(|unit| {
                string.push_str(&unit.as_string());
            });
        string
    }
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
            chem_units: vec![
                ChemUnit::new_group(
                    vec![
                        ChemUnit::new_element(ElementSymbol::N, 1),
                        ChemUnit::new_element(ElementSymbol::H, 4),
                    ],
                    2,
                ),
                ChemUnit::new_element(ElementSymbol::S, 1),
                ChemUnit::new_element(ElementSymbol::O, 4),
            ],
        })
        .add_right_compound(Compound {
            chem_units: vec![
                ChemUnit::new_group(
                    vec![
                        ChemUnit::new_element(ElementSymbol::N, 1),
                        ChemUnit::new_element(ElementSymbol::H, 4),
                    ],
                    1,
                ),
                ChemUnit::new_element(ElementSymbol::N, 1),
                ChemUnit::new_element(ElementSymbol::O, 3),
            ],
        })
        .build();

    println!("{:?}", formula);
    for compound in formula.left_compounds {
        println!("{:?}", compound.as_string());
    }
    for compound in formula.right_compounds {
        println!("{:?}", compound.as_string());
    }
}
