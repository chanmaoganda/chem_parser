use std::collections::HashMap;

use lazy_static::lazy_static;

lazy_static! {
    pub static ref ELEMENT_MAP: HashMap<u8, ElementSymbol> = HashMap::from([
        (1, ElementSymbol::H),
        (2, ElementSymbol::He),
        (3, ElementSymbol::Li),
        (4, ElementSymbol::Be),
        (5, ElementSymbol::B),
        (6, ElementSymbol::C),
        (7, ElementSymbol::N),
        (8, ElementSymbol::O),
        (9, ElementSymbol::F),
        (10, ElementSymbol::Ne),
        (11, ElementSymbol::Na),
        (12, ElementSymbol::Mg),
        (13, ElementSymbol::Al),
        (14, ElementSymbol::Si),
        (15, ElementSymbol::P),
        (16, ElementSymbol::S),
        (17, ElementSymbol::Cl),
        (18, ElementSymbol::Ar),
        (19, ElementSymbol::K),
        (20, ElementSymbol::Ca),
        (21, ElementSymbol::Sc),
        (22, ElementSymbol::Ti),
        (23, ElementSymbol::V),
        (24, ElementSymbol::Cr),
        (25, ElementSymbol::Mn),
        (26, ElementSymbol::Fe),
        (27, ElementSymbol::Co),
        (28, ElementSymbol::Ni),
        (29, ElementSymbol::Cu),
        (30, ElementSymbol::Zn),
        (31, ElementSymbol::Ga),
        (32, ElementSymbol::Ge),
        (33, ElementSymbol::As),
        (34, ElementSymbol::Se),
        (35, ElementSymbol::Br),
        (36, ElementSymbol::Kr),
        (37, ElementSymbol::Rb),
        (38, ElementSymbol::Sr),
        (39, ElementSymbol::Y),
        (40, ElementSymbol::Zr),
        (41, ElementSymbol::Nb),
        (42, ElementSymbol::Mo),
        (43, ElementSymbol::Tc),
        (44, ElementSymbol::Ru),
        (45, ElementSymbol::Rh),
        (46, ElementSymbol::Pd),
        (47, ElementSymbol::Ag),
        (48, ElementSymbol::Cd),
        (49, ElementSymbol::In),
        (50, ElementSymbol::Sn),
        (51, ElementSymbol::Sb),
        (52, ElementSymbol::Te),
        (53, ElementSymbol::I),
        (54, ElementSymbol::Xe),
        (55, ElementSymbol::Cs),
        (56, ElementSymbol::Ba),
        (57, ElementSymbol::La),
        (58, ElementSymbol::Ce),
        (59, ElementSymbol::Pr),
        (60, ElementSymbol::Nd),
        (61, ElementSymbol::Pm),
        (62, ElementSymbol::Sm),
        (63, ElementSymbol::Eu),
        (64, ElementSymbol::Gd),
        (65, ElementSymbol::Tb),
        (66, ElementSymbol::Dy),
        (67, ElementSymbol::Ho),
        (68, ElementSymbol::Er),
        (69, ElementSymbol::Tm),
        (70, ElementSymbol::Yb),
        (71, ElementSymbol::Lu),
        (72, ElementSymbol::Hf),
        (73, ElementSymbol::Ta),
        (74, ElementSymbol::W),
        (75, ElementSymbol::Re),
        (76, ElementSymbol::Os),
        (77, ElementSymbol::Ir),
        (78, ElementSymbol::Pt),
        (79, ElementSymbol::Au),
        (80, ElementSymbol::Hg),
        (81, ElementSymbol::Tl),
        (82, ElementSymbol::Pb),
        (83, ElementSymbol::Bi),
        (84, ElementSymbol::Po),
        (85, ElementSymbol::At),
        (86, ElementSymbol::Rn),
        (87, ElementSymbol::Fr),
        (88, ElementSymbol::Ra),
        (89, ElementSymbol::Ac),
        (90, ElementSymbol::Th),
        (91, ElementSymbol::Pa),
        (92, ElementSymbol::U),
        (93, ElementSymbol::Np),
        (94, ElementSymbol::Pu),
        (95, ElementSymbol::Am),
        (96, ElementSymbol::Cm),
        (97, ElementSymbol::Bk),
        (98, ElementSymbol::Cf),
        (99, ElementSymbol::Es),
        (100, ElementSymbol::Fm),
        (101, ElementSymbol::Md),
        (102, ElementSymbol::No),
        (103, ElementSymbol::Lr),
        (104, ElementSymbol::Rf),
        (105, ElementSymbol::Db),
        (106, ElementSymbol::Sg),
        (107, ElementSymbol::Bh),
        (108, ElementSymbol::Hs),
        (109, ElementSymbol::Mt),
        (110, ElementSymbol::Ds),
        (111, ElementSymbol::Rg),
        (112, ElementSymbol::Cn),
        (113, ElementSymbol::Nh),
        (114, ElementSymbol::Fl),
        (115, ElementSymbol::Mc),
        (116, ElementSymbol::Lv),
        (117, ElementSymbol::Ts),
        (118, ElementSymbol::Og),
    ]);

    pub static ref ELEMENT_STR_MAP: HashMap<ElementSymbol, &'static str> = HashMap::from([
        (ElementSymbol::H, "H"),
        (ElementSymbol::He, "He"),
        (ElementSymbol::Li, "Li"),
        (ElementSymbol::Be, "Be"),
        (ElementSymbol::B, "B"),
        (ElementSymbol::C, "C"),
        (ElementSymbol::N, "N"),
        (ElementSymbol::O, "O"),
        (ElementSymbol::F, "F"),
        (ElementSymbol::Ne, "Ne"),
        (ElementSymbol::Na, "Na"),
        (ElementSymbol::Mg, "Mg"),
        (ElementSymbol::Al, "Al"),
        (ElementSymbol::Si, "Si"),
        (ElementSymbol::P, "P"),
        (ElementSymbol::S, "S"),
        (ElementSymbol::Cl, "Cl"),
        (ElementSymbol::Ar, "Ar"),
        (ElementSymbol::K, "K"),
        (ElementSymbol::Ca, "Ca"),
        (ElementSymbol::Sc, "Sc"),
        (ElementSymbol::Ti, "Ti"),
        (ElementSymbol::V, "V"),
        (ElementSymbol::Cr, "Cr"),
        (ElementSymbol::Mn, "Mn"),
        (ElementSymbol::Fe, "Fe"),
        (ElementSymbol::Co, "Co"),
        (ElementSymbol::Ni, "Ni"),
        (ElementSymbol::Cu, "Cu"),
        (ElementSymbol::Zn, "Zn"),
        (ElementSymbol::Ga, "Ga"),
        (ElementSymbol::Ge, "Ge"),
        (ElementSymbol::As, "As"),
        (ElementSymbol::Se, "Se"),
        (ElementSymbol::Br, "Br"),
        (ElementSymbol::Kr, "Kr"),
        (ElementSymbol::Rb, "Rb"),
        (ElementSymbol::Sr, "Sr"),
        (ElementSymbol::Y, "Y"),
        (ElementSymbol::Zr, "Zr"),
        (ElementSymbol::Nb, "Nb"),
        (ElementSymbol::Mo, "Mo"),
        (ElementSymbol::Tc, "Tc"),
        (ElementSymbol::Ru, "Ru"),
        (ElementSymbol::Rh, "Rh"),
        (ElementSymbol::Pd, "Pd"),
        (ElementSymbol::Ag, "Ag"),
        (ElementSymbol::Cd, "Cd"),
        (ElementSymbol::In, "In"),
        (ElementSymbol::Sn, "Sn"),
        (ElementSymbol::Sb, "Sb"),
        (ElementSymbol::Te, "Te"),
        (ElementSymbol::I, "I"),
        (ElementSymbol::Xe, "Xe"),
        (ElementSymbol::Cs, "Cs"),
        (ElementSymbol::Ba, "Ba"),
        (ElementSymbol::La, "La"),
        (ElementSymbol::Ce, "Ce"),
        (ElementSymbol::Pr, "Pr"),
        (ElementSymbol::Nd, "Nd"),
        (ElementSymbol::Pm, "Pm"),
        (ElementSymbol::Sm, "Sm"),
        (ElementSymbol::Eu, "Eu"),
        (ElementSymbol::Gd, "Gd"),
        (ElementSymbol::Tb, "Tb"),
        (ElementSymbol::Dy, "Dy"),
        (ElementSymbol::Ho, "Ho"),
        (ElementSymbol::Er, "Er"),
        (ElementSymbol::Tm, "Tm"),
        (ElementSymbol::Yb, "Yb"),
        (ElementSymbol::Lu, "Lu"),
        (ElementSymbol::Hf, "Hf"),
        (ElementSymbol::Ta, "Ta"),
        (ElementSymbol::W, "W"),
        (ElementSymbol::Re, "Re"),
        (ElementSymbol::Os, "Os"),
        (ElementSymbol::Ir, "Ir"),
        (ElementSymbol::Pt, "Pt"),
        (ElementSymbol::Au, "Au"),
        (ElementSymbol::Hg, "Hg"),
        (ElementSymbol::Tl, "Tl"),
        (ElementSymbol::Pb, "Pb"),
        (ElementSymbol::Bi, "Bi"),
        (ElementSymbol::Po, "Po"),
        (ElementSymbol::At, "At"),
        (ElementSymbol::Rn, "Rn"),
        (ElementSymbol::Fr, "Fr"),
        (ElementSymbol::Ra, "Ra"),
        (ElementSymbol::Ac, "Ac"),
        (ElementSymbol::Th, "Th"),
        (ElementSymbol::Pa, "Pa"),
        (ElementSymbol::U, "U"),
        (ElementSymbol::Np, "Np"),
        (ElementSymbol::Pu, "Pu"),
        (ElementSymbol::Am, "Am"),
        (ElementSymbol::Cm, "Cm"),
        (ElementSymbol::Bk, "Bk"),
        (ElementSymbol::Cf, "Cf"),
        (ElementSymbol::Es, "Es"),
        (ElementSymbol::Fm, "Fm"),
        (ElementSymbol::Md, "Md"),
        (ElementSymbol::No, "No"),
        (ElementSymbol::Lr, "Lr"),
        (ElementSymbol::Rf, "Rf"),
        (ElementSymbol::Db, "Db"),
        (ElementSymbol::Sg, "Sg"),
        (ElementSymbol::Bh, "Bh"),
        (ElementSymbol::Hs, "Hs"),
        (ElementSymbol::Mt, "Mt"),
        (ElementSymbol::Ds, "Ds"),
        (ElementSymbol::Rg, "Rg"),
        (ElementSymbol::Cn, "Cn"),
        (ElementSymbol::Nh, "Nh"),
        (ElementSymbol::Fl, "Fl"),
        (ElementSymbol::Mc, "Mc"),
        (ElementSymbol::Lv, "Lv"),
        (ElementSymbol::Ts, "Ts"),
        (ElementSymbol::Og, "Og"),
    ]);
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub enum ElementSymbol {
    Unknown,
    H,  // Hydrogen
    He, // Helium
    Li, // Lithium
    Be, // Beryllium
    B,  // Boron
    C,  // Carbon
    N,  // Nitrogen
    O,  // Oxygen
    F,  // Fluorine
    Ne, // Neon
    Na, // Sodium
    Mg, // Magnesium
    Al, // Aluminum
    Si, // Silicon
    P,  // Phosphorus
    S,  // Sulfur
    Cl, // Chlorine
    Ar, // Argon
    K,  // Potassium
    Ca, // Calcium
    Sc, // Scandium
    Ti, // Titanium
    V,  // Vanadium
    Cr, // Chromium
    Mn, // Manganese
    Fe, // Iron
    Co, // Cobalt
    Ni, // Nickel
    Cu, // Copper
    Zn, // Zinc
    Ga, // Gallium
    Ge, // Germanium
    As, // Arsenic
    Se, // Selenium
    Br, // Bromine
    Kr, // Krypton
    Rb, // Rubidium
    Sr, // Strontium
    Y,  // Yttrium
    Zr, // Zirconium
    Nb, // Niobium
    Mo, // Molybdenum
    Tc, // Technetium
    Ru, // Ruthenium
    Rh, // Rhodium
    Pd, // Palladium
    Ag, // Silver
    Cd, // Cadmium
    In, // Indium
    Sn, // Tin
    Sb, // Antimony
    Te, // Tellurium
    I,  // Iodine
    Xe, // Xenon
    Cs, // Cesium
    Ba, // Barium
    La, // Lanthanum
    Ce, // Cerium
    Pr, // Praseodymium
    Nd, // Neodymium
    Pm, // Promethium
    Sm, // Samarium
    Eu, // Europium
    Gd, // Gadolinium
    Tb, // Terbium
    Dy, // Dysprosium
    Ho, // Holmium
    Er, // Erbium
    Tm, // Thulium
    Yb, // Ytterbium
    Lu, // Lutetium
    Hf, // Hafnium
    Ta, // Tantalum
    W,  // Tungsten
    Re, // Rhenium
    Os, // Osmium
    Ir, // Iridium
    Pt, // Platinum
    Au, // Gold
    Hg, // Mercury
    Tl, // Thallium
    Pb, // Lead
    Bi, // Bismuth
    Po, // Polonium
    At, // Astatine
    Rn, // Radon
    Fr, // Francium
    Ra, // Radium
    Ac, // Actinium
    Th, // Thorium
    Pa, // Protactinium
    U,  // Uranium
    Np, // Neptunium
    Pu, // Plutonium
    Am, // Americium
    Cm, // Curium
    Bk, // Berkelium
    Cf, // Californium
    Es, // Einsteinium
    Fm, // Fermium
    Md, // Mendelevium
    No, // Nobelium
    Lr, // Lawrencium
    Rf, // Rutherfordium
    Db, // Dubnium
    Sg, // Seaborgium
    Bh, // Bohrium
    Hs, // Hassium
    Mt, // Meitnerium
    Ds, // Darmstadtium
    Rg, // Roentgenium
    Cn, // Copernicium
    Nh, // Nihonium
    Fl, // Flerovium
    Mc, // Moscovium
    Lv, // Livermorium
    Ts, // Tennessine
    Og, // Oganesson
}