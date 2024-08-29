use std::collections::HashSet;

use super::ElementSymbol;


pub struct ElementElectronLayout {
    pub element: ElementSymbol,
    pub electron_layer: HashSet<ElectronLayer>,
}


pub struct ElectronLayer {
    pub layer_index: u8,
    pub electron_count: u8,
}