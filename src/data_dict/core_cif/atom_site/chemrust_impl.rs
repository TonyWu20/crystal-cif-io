use chemrust_core::data::{atom::CoreAtomData, geom::coordinates::CoordData::Fractional};

use crate::{
    data_dict::{core_cif::atom_site::adp_type::AdpType, LoopValueTerm},
    grammar::{CharString, DataItems, Float, LoopUnit, Numeric, UnquotedString},
};

use super::{
    label_symbol::{TypeSymbol, TypeSymbolCode},
    AtomSiteLoopItem,
};

pub(crate) fn basic_atom_site_data<T: CoreAtomData>(atom_data: &T) -> DataItems {
    let (labels, symbols): (Vec<CharString>, Vec<TypeSymbol>) = atom_data
        .symbols_repr()
        .iter()
        .enumerate()
        .map(|(i, symbol)| {
            let label = CharString::Unquoted(UnquotedString::new(format!("{}{}", symbol, i)));
            let symbol = TypeSymbol::new(TypeSymbolCode::ElementSymbol(*symbol), None);
            (label, symbol)
        })
        .unzip();
    let labels = AtomSiteLoopItem::Label(labels);
    let symbols = AtomSiteLoopItem::Type_symbol(symbols);
    let is_frac = matches!(atom_data.coords_repr()[0], Fractional(_));
    let (x, y, z) = if is_frac {
        let (x, (y, z)) = atom_data
            .coords_repr()
            .iter()
            .map(|cd| {
                let f = cd.raw_data();
                (Numeric::from(f.x), (Numeric::from(f.y), Numeric::from(f.z)))
            })
            .unzip();
        (
            AtomSiteLoopItem::Fract_x(x),
            AtomSiteLoopItem::Fract_y(y),
            AtomSiteLoopItem::Fract_z(z),
        )
    } else {
        let (x, (y, z)) = atom_data
            .coords_repr()
            .iter()
            .map(|cd| {
                let c = cd.raw_data();
                (Numeric::from(c.x), (Numeric::from(c.y), Numeric::from(c.z)))
            })
            .unzip();
        (
            AtomSiteLoopItem::Cartn_x(x),
            AtomSiteLoopItem::Cartn_y(y),
            AtomSiteLoopItem::Cartn_z(z),
        )
    };
    let u_iso = AtomSiteLoopItem::U_iso_or_equiv(vec![Float(0.0); atom_data.coords_repr().len()]);
    let adp_type = AtomSiteLoopItem::Adp_type(vec![AdpType::Uiso; atom_data.coords_repr().len()]);
    let occupancy = AtomSiteLoopItem::Occupancy(vec![Float(1.0); atom_data.coords_repr().len()]);
    let columns =
        [labels, symbols, x, y, z, u_iso, adp_type, occupancy].map(|item| item.to_loop_column());
    let loop_unit = LoopUnit::builder()
        .with_value_columns(columns.to_vec())
        .build();
    DataItems::MultiValues(loop_unit.into())
}
