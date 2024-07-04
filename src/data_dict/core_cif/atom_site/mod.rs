use chemrust_core::data::atom::CoreAtomData;
use chemrust_core::data::geom::coordinates::CoordData::{Cartesian, Fractional};

use self::adp_type::AdpType;
use self::label_symbol::{TypeSymbol, TypeSymbolCode};
use self::symmetry_multiplicity::SymMultiplicity;
use crate::data_dict::{DataLabel, LoopData, LoopDataEntry, LoopDataLabel, NumValue};

mod adp_type;
mod label_symbol;
mod symmetry_multiplicity;

pub type AtomSiteData = LoopDataEntry<AtomSiteLoopItem>;
pub type LoopAtomSiteData = LoopData<AtomSiteLoopItem>;

#[allow(clippy::upper_case_acronyms, non_camel_case_types)]
#[derive(Debug, Clone)]
pub enum AtomSiteLoopItem {
    Adp_type(AdpType),
    Aniso_B_11,
    Aniso_B_12,
    Aniso_B_13,
    Aniso_B_22,
    Aniso_B_23,
    Aniso_B_33,
    Aniso_label,
    Aniso_ratio,
    Aniso_type_symbol(TypeSymbol),
    Aniso_U_11,
    Aniso_U_12,
    Aniso_U_13,
    Aniso_U_22,
    Aniso_U_23,
    Aniso_U_33,
    Attached_hydrogens,
    B_equiv_geom_mean,
    B_iso_or_equiv,
    Calc_attached_atom,
    Calc_flag,
    Cartn_x(NumValue<f64>),
    Cartn_y(NumValue<f64>),
    Cartn_z(NumValue<f64>),
    Chemical_conn_number,
    Constraints,
    Description,
    Disorder_assembly,
    Disorder_group,
    Fract_x(NumValue<f64>),
    Fract_y(NumValue<f64>),
    Fract_z(NumValue<f64>),
    Label(String),
    Label_component_0,
    Label_component_1,
    Label_component_2,
    Label_component_3,
    Label_component_4,
    Label_component_5,
    Label_component_6,
    Occupancy(f64),
    Refinement_flags,
    Refinement_flags_adp,
    Refinement_flags_occupancy,
    Refinement_flags_posn,
    Restraints,
    Site_symmetry_multiplicity(SymMultiplicity),
    Site_symmetry_order,
    Symmetry_multiplicity(SymMultiplicity),
    Thermal_displace_type,
    Type_symbol(TypeSymbol),
    U_equiv_geom_mean,
    U_iso_or_equiv(f64),
    Wyckoff_symbol,
}

impl DataLabel for AtomSiteLoopItem {
    fn tag(&self) -> String {
        match self {
            AtomSiteLoopItem::Adp_type(_) => "adp_type".to_string(),
            AtomSiteLoopItem::Aniso_B_11 => "aniso_B_11".to_string(),
            AtomSiteLoopItem::Aniso_B_12 => "aniso_B_12".to_string(),
            AtomSiteLoopItem::Aniso_B_13 => "aniso_B_13".to_string(),
            AtomSiteLoopItem::Aniso_B_22 => "aniso_B_22".to_string(),
            AtomSiteLoopItem::Aniso_B_23 => "aniso_B_23".to_string(),
            AtomSiteLoopItem::Aniso_B_33 => "aniso_B_33".to_string(),
            AtomSiteLoopItem::Aniso_label => "aniso_label".to_string(),
            AtomSiteLoopItem::Aniso_ratio => "aniso_ratio".to_string(),
            AtomSiteLoopItem::Aniso_type_symbol(_) => "aniso_type_symbol".to_string(),
            AtomSiteLoopItem::Aniso_U_11 => "aniso_U_11".to_string(),
            AtomSiteLoopItem::Aniso_U_12 => "aniso_U_12".to_string(),
            AtomSiteLoopItem::Aniso_U_13 => "aniso_U_13".to_string(),
            AtomSiteLoopItem::Aniso_U_22 => "aniso_U_22".to_string(),
            AtomSiteLoopItem::Aniso_U_23 => "aniso_U_23".to_string(),
            AtomSiteLoopItem::Aniso_U_33 => "aniso_U_33".to_string(),
            AtomSiteLoopItem::Attached_hydrogens => "attached_hydrogens".to_string(),
            AtomSiteLoopItem::B_equiv_geom_mean => "b_equiv_geom_mean".to_string(),
            AtomSiteLoopItem::B_iso_or_equiv => "b_iso_or_equiv".to_string(),
            AtomSiteLoopItem::Calc_attached_atom => "calc_attached_atom".to_string(),
            AtomSiteLoopItem::Calc_flag => "calc_flag".to_string(),
            AtomSiteLoopItem::Cartn_x(_) => "cartn_x".to_string(),
            AtomSiteLoopItem::Cartn_y(_) => "cartn_y".to_string(),
            AtomSiteLoopItem::Cartn_z(_) => "cartn_z".to_string(),
            AtomSiteLoopItem::Chemical_conn_number => "chemical_conn_number".to_string(),
            AtomSiteLoopItem::Constraints => "constraints".to_string(),
            AtomSiteLoopItem::Description => "description".to_string(),
            AtomSiteLoopItem::Disorder_assembly => "disorder_assembly".to_string(),
            AtomSiteLoopItem::Disorder_group => "disorder_group".to_string(),
            AtomSiteLoopItem::Fract_x(_) => "fract_x".to_string(),
            AtomSiteLoopItem::Fract_y(_) => "fract_y".to_string(),
            AtomSiteLoopItem::Fract_z(_) => "fract_z".to_string(),
            AtomSiteLoopItem::Label(_) => "label".to_string(),
            AtomSiteLoopItem::Label_component_0 => "label_component_0".to_string(),
            AtomSiteLoopItem::Label_component_1 => "label_component_1".to_string(),
            AtomSiteLoopItem::Label_component_2 => "label_component_2".to_string(),
            AtomSiteLoopItem::Label_component_3 => "label_component_3".to_string(),
            AtomSiteLoopItem::Label_component_4 => "label_component_4".to_string(),
            AtomSiteLoopItem::Label_component_5 => "label_component_5".to_string(),
            AtomSiteLoopItem::Label_component_6 => "label_component_6".to_string(),
            AtomSiteLoopItem::Occupancy(_) => "occupancy".to_string(),
            AtomSiteLoopItem::Refinement_flags => "refinement_flags".to_string(),
            AtomSiteLoopItem::Refinement_flags_adp => "refinement_flags_adp".to_string(),
            AtomSiteLoopItem::Refinement_flags_occupancy => {
                "refinement_flags_occupancy".to_string()
            }
            AtomSiteLoopItem::Refinement_flags_posn => "refinement_flags_posn".to_string(),
            AtomSiteLoopItem::Restraints => "restraints".to_string(),
            AtomSiteLoopItem::Site_symmetry_multiplicity(_) => {
                "site_symmetry_multiplicity".to_string()
            }
            AtomSiteLoopItem::Site_symmetry_order => "site_symmetry_order".to_string(),
            AtomSiteLoopItem::Symmetry_multiplicity(_) => "symmetry_multiplicity".to_string(),
            AtomSiteLoopItem::Thermal_displace_type => "thermal_displace_type".to_string(),
            AtomSiteLoopItem::Type_symbol(_) => "type_symbol".to_string(),
            AtomSiteLoopItem::U_equiv_geom_mean => "u_equiv_geom_mean".to_string(),
            AtomSiteLoopItem::U_iso_or_equiv(_) => "u_iso_or_equiv".to_string(),
            AtomSiteLoopItem::Wyckoff_symbol => "wyckoff_symbol".to_string(),
        }
    }

    fn value_string(&self) -> String {
        match self {
            AtomSiteLoopItem::Adp_type(v) => format!("{v}"),
            AtomSiteLoopItem::Aniso_B_11 => todo!(),
            AtomSiteLoopItem::Aniso_B_12 => todo!(),
            AtomSiteLoopItem::Aniso_B_13 => todo!(),
            AtomSiteLoopItem::Aniso_B_22 => todo!(),
            AtomSiteLoopItem::Aniso_B_23 => todo!(),
            AtomSiteLoopItem::Aniso_B_33 => todo!(),
            AtomSiteLoopItem::Aniso_label => todo!(),
            AtomSiteLoopItem::Aniso_ratio => todo!(),
            AtomSiteLoopItem::Type_symbol(symbol) | AtomSiteLoopItem::Aniso_type_symbol(symbol) => {
                format!("{symbol}")
            }
            AtomSiteLoopItem::Aniso_U_11 => todo!(),
            AtomSiteLoopItem::Aniso_U_12 => todo!(),
            AtomSiteLoopItem::Aniso_U_13 => todo!(),
            AtomSiteLoopItem::Aniso_U_22 => todo!(),
            AtomSiteLoopItem::Aniso_U_23 => todo!(),
            AtomSiteLoopItem::Aniso_U_33 => todo!(),
            AtomSiteLoopItem::Attached_hydrogens => todo!(),
            AtomSiteLoopItem::B_equiv_geom_mean => todo!(),
            AtomSiteLoopItem::B_iso_or_equiv => todo!(),
            AtomSiteLoopItem::Calc_attached_atom => todo!(),
            AtomSiteLoopItem::Calc_flag => todo!(),
            AtomSiteLoopItem::Cartn_x(v)
            | AtomSiteLoopItem::Cartn_y(v)
            | AtomSiteLoopItem::Cartn_z(v)
            | AtomSiteLoopItem::Fract_x(v)
            | AtomSiteLoopItem::Fract_y(v)
            | AtomSiteLoopItem::Fract_z(v) => {
                format!("{v}")
            }
            AtomSiteLoopItem::Chemical_conn_number => todo!(),
            AtomSiteLoopItem::Constraints => todo!(),
            AtomSiteLoopItem::Description => todo!(),
            AtomSiteLoopItem::Disorder_assembly => todo!(),
            AtomSiteLoopItem::Disorder_group => todo!(),
            AtomSiteLoopItem::Label(s) => s.to_string(),
            AtomSiteLoopItem::Label_component_0 => todo!(),
            AtomSiteLoopItem::Label_component_1 => todo!(),
            AtomSiteLoopItem::Label_component_2 => todo!(),
            AtomSiteLoopItem::Label_component_3 => todo!(),
            AtomSiteLoopItem::Label_component_4 => todo!(),
            AtomSiteLoopItem::Label_component_5 => todo!(),
            AtomSiteLoopItem::Label_component_6 => todo!(),
            AtomSiteLoopItem::Occupancy(f) => format!("{}", f.clamp(0.0, 1.0)),
            AtomSiteLoopItem::Refinement_flags => todo!(),
            AtomSiteLoopItem::Refinement_flags_adp => todo!(),
            AtomSiteLoopItem::Refinement_flags_occupancy => todo!(),
            AtomSiteLoopItem::Refinement_flags_posn => todo!(),
            AtomSiteLoopItem::Restraints => todo!(),
            AtomSiteLoopItem::Site_symmetry_multiplicity(m) => format!("{m}"),
            AtomSiteLoopItem::Site_symmetry_order => todo!(),
            AtomSiteLoopItem::Symmetry_multiplicity(m) => format!("{m}"),
            AtomSiteLoopItem::Thermal_displace_type => todo!(),
            AtomSiteLoopItem::U_equiv_geom_mean => todo!(),
            AtomSiteLoopItem::U_iso_or_equiv(f) => format!("{f}"),
            AtomSiteLoopItem::Wyckoff_symbol => todo!(),
        }
    }

    fn category_prefix(&self) -> String {
        "atom_site".to_string()
    }
}

impl LoopDataLabel for AtomSiteLoopItem {}

impl<T: CoreAtomData> From<&T> for LoopAtomSiteData {
    fn from(core_atom_data: &T) -> Self {
        let entries: Vec<AtomSiteData> = core_atom_data
            .symbols_repr()
            .iter()
            .zip(core_atom_data.coords_repr().iter())
            .enumerate()
            .map(|(i, (symbol, coord))| {
                let (x, y, z) = match coord {
                    Fractional(f) => (
                        AtomSiteLoopItem::Fract_x(NumValue::<f64>::new(f.x, None)),
                        AtomSiteLoopItem::Fract_y(NumValue::<f64>::new(f.y, None)),
                        AtomSiteLoopItem::Fract_z(NumValue::<f64>::new(f.z, None)),
                    ),
                    Cartesian(c) => (
                        AtomSiteLoopItem::Cartn_x(NumValue::<f64>::new(c.x, None)),
                        AtomSiteLoopItem::Cartn_y(NumValue::<f64>::new(c.y, None)),
                        AtomSiteLoopItem::Cartn_z(NumValue::<f64>::new(c.z, None)),
                    ),
                };
                AtomSiteData::init_builder()
                    .add_entry(AtomSiteLoopItem::Label(format!("{}{}", symbol, i)))
                    .add_entry(AtomSiteLoopItem::Type_symbol(TypeSymbol::new(
                        TypeSymbolCode::ElementSymbol(*symbol),
                        None,
                    )))
                    .add_entry(x)
                    .add_entry(y)
                    .add_entry(z)
                    .add_entry(AtomSiteLoopItem::U_iso_or_equiv(0.0))
                    .add_entry(AtomSiteLoopItem::Adp_type(AdpType::Uiso))
                    .add_entry(AtomSiteLoopItem::Occupancy(1.0))
                    .finish()
            })
            .collect();
        LoopAtomSiteData::new(entries)
    }
}

#[cfg(test)]
mod test {

    use std::{fs::read_to_string, path::Path};

    use castep_cell_io::CellParser;
    use castep_periodic_table::element::ElementSymbol;

    use crate::data_dict::{
        core_cif::atom_site::{
            label_symbol::{TypeSymbol, TypeSymbolCode},
            AtomSiteData, AtomSiteLoopItem,
        },
        NumValue,
    };

    use super::LoopAtomSiteData;

    #[test]
    fn atom_site_loop() {
        let atom_site = AtomSiteData::init_builder()
            .add_entry(AtomSiteLoopItem::Type_symbol(TypeSymbol::new(
                TypeSymbolCode::ElementSymbol(ElementSymbol::N),
                None,
            )))
            .add_entry(AtomSiteLoopItem::Fract_x(NumValue::<f64>::new(
                0.829,
                Some(3),
            )))
            .add_entry(AtomSiteLoopItem::Fract_y(NumValue::<f64>::new(
                0.820,
                Some(3),
            )))
            .add_entry(AtomSiteLoopItem::Fract_z(NumValue::<f64>::new(
                0.461,
                Some(3),
            )))
            .finish();
        println!("{}", atom_site);
    }
    #[test]
    fn atom_loop_with_cell() {
        let cell_path = Path::new(env!("CARGO_MANIFEST_DIR")).join("SAC_GDY_V_Co.cell");
        let cell_content = read_to_string(cell_path).expect("Read error");
        let cell_doc = CellParser::from(&cell_content)
            .parse()
            .expect("Parse error");
        let loop_atoms = LoopAtomSiteData::from(cell_doc.model_description().ionic_pos_block());
        println!("{}", loop_atoms);
    }
}
