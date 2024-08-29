use crate::ast_tree::{Compound, Formula};

pub struct FormulaBuilder {
    formula: Formula,
}

impl FormulaBuilder {
    pub fn new() -> Self {
        Self {
            formula: Formula {
                left_compounds: vec![],
                right_compounds: vec![],
            },
        }
    }

    pub fn add_left_compound(mut self, compound: Compound) -> Self {
        self.formula.left_compounds.push(compound);
        self
    }

    pub fn add_right_compound(mut self, compound: Compound) -> Self {
        self.formula.right_compounds.push(compound);
        self
    }

    pub fn build(self) -> Formula {
        self.formula
    }
}
