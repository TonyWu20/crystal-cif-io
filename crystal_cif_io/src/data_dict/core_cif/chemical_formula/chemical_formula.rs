//! The `chemical_formula` category of the typed document layer (roadmap D12).
//!
//! The `chemical_formula` category struct. One `Option` field per
//! registry item. `weight` and `weight_meas` keep the `FormulaWeight`
//! range guard.

use crate::core_cif::CifError;
use crate::data_dict::core_cif::chemical_formula::FormulaWeight;
use crate::data_dict::core_cif::{convert, CifCategory};
use crystal_cif_fmt::grammar::{CharString, Tag, Value};
use crystal_cif_fmt::{DataBlockMember, DataItems, SingleLineData};

#[derive(Debug, Clone, Default)]
/// The `chemical_formula` category struct. One field per registry item.
pub struct ChemicalFormula {
    /// The `_chemical_formula_analytical` item. The field holds the single value.
    ///
    /// Formula determined by standard chemical analysis including trace
    /// elements. See the _chemical_formula_[] category description for
    /// rules for writing chemical formulae. Parentheses are used only for
    /// standard uncertainties (e.s.d.'s).
    pub analytical: Option<CharString>,
    /// The `_chemical_formula_iupac` item. The field holds the single value.
    ///
    /// Formula expressed in conformance with IUPAC rules for inorganic and
    /// metal-organic compounds where these conflict with the rules for any
    /// other _chemical_formula_ entries. Typically used for formatting a
    /// formula in accordance with journal rules. This should appear in the
    /// data block in addition to the most appropriate of the other
    /// _chemical_formula_ data names. Ref: IUPAC (1990). Nomenclature of
    /// Inorganic Chemistry. Oxford: Blackwell Scientific Publications.
    pub iupac: Option<CharString>,
    /// The `_chemical_formula_moiety` item. The field holds the single value.
    ///
    /// Formula with each discrete bonded residue or ion shown as a separate
    /// moiety. See the _chemical_formula_[] category description for rules
    /// for writing chemical formulae. In addition to the general formulae
    /// requirements, the following rules apply: (1) Moieties are separated
    /// by commas ','. (2) The order of elements within a moiety follows
    /// general rule (5) in the _chemical_formula_[] category description.
    /// (3) Parentheses are not used within moieties but may surround a
    /// moiety. Parentheses may not be nested. (4) Charges should be placed
    /// at the end of the moiety. The charge '+' or '-' may be preceded by a
    /// numerical multiplier and should be separated from the last (element
    /// symbol + count) by a space. Pre- or post-multipliers may be used for
    /// individual moieties.
    pub moiety: Option<CharString>,
    /// The `_chemical_formula_structural` item. The field holds the single value.
    ///
    /// See the _chemical_formula_[] category description for the rules for
    /// writing chemical formulae for inorganics, organometallics, metal
    /// complexes etc., in which bonded groups are preserved as discrete
    /// entities within parentheses, with post-multipliers as required. The
    /// order of the elements should give as much information as possible
    /// about the chemical structure. Parentheses may be used and nested as
    /// required. This formula should correspond to the structure as
    /// actually reported, i.e. trace elements not included in atom-type and
    /// atom-site lists should not be included in this formula (see also
    /// _chemical_formula_analytical).
    pub structural: Option<CharString>,
    /// The `_chemical_formula_sum` item. The field holds the single value.
    ///
    /// See the _chemical_formula_[] category description for the rules for
    /// writing chemical formulae in which all discrete bonded residues and
    /// ions are summed over the constituent elements, following the
    /// ordering given in general rule (5) in the _chemical_formula_[]
    /// category description. Parentheses are not normally used.
    pub sum: Option<CharString>,
    /// The `_chemical_formula_weight` item. The field holds the single value.
    ///
    /// Formula mass in daltons. This mass should correspond to the formulae
    /// given under _chemical_formula_structural, *_iupac, *_moiety or *_sum
    /// and, together with the Z value and cell parameters, should yield the
    /// density given as _exptl_crystal_density_diffrn.
    /// Units: Da (dalton).
    pub weight: Option<FormulaWeight>,
    /// The `_chemical_formula_weight_meas` item. The field holds the single value.
    ///
    /// Formula mass in daltons measured by a non-diffraction experiment.
    /// Units: Da (dalton).
    pub weight_meas: Option<FormulaWeight>,
}

impl CifCategory for ChemicalFormula {
    fn to_members(&self) -> Vec<DataBlockMember> {
        let mut out = Vec::new();
        if let Some(v) = &self.analytical {
            out.push(DataBlockMember::DataItems(DataItems::SingleValue(
                SingleLineData::from_tag_value((
                    Tag::new("chemical_formula_analytical".to_string()),
                    Value::CharString(v.clone()),
                )),
            )));
        }
        if let Some(v) = &self.iupac {
            out.push(DataBlockMember::DataItems(DataItems::SingleValue(
                SingleLineData::from_tag_value((
                    Tag::new("chemical_formula_iupac".to_string()),
                    Value::CharString(v.clone()),
                )),
            )));
        }
        if let Some(v) = &self.moiety {
            out.push(DataBlockMember::DataItems(DataItems::SingleValue(
                SingleLineData::from_tag_value((
                    Tag::new("chemical_formula_moiety".to_string()),
                    Value::CharString(v.clone()),
                )),
            )));
        }
        if let Some(v) = &self.structural {
            out.push(DataBlockMember::DataItems(DataItems::SingleValue(
                SingleLineData::from_tag_value((
                    Tag::new("chemical_formula_structural".to_string()),
                    Value::CharString(v.clone()),
                )),
            )));
        }
        if let Some(v) = &self.sum {
            out.push(DataBlockMember::DataItems(DataItems::SingleValue(
                SingleLineData::from_tag_value((
                    Tag::new("chemical_formula_sum".to_string()),
                    Value::CharString(v.clone()),
                )),
            )));
        }
        if let Some(v) = &self.weight {
            out.push(DataBlockMember::DataItems(DataItems::SingleValue(
                SingleLineData::from_tag_value((
                    Tag::new("chemical_formula_weight".to_string()),
                    Value::from(*v),
                )),
            )));
        }
        if let Some(v) = &self.weight_meas {
            out.push(DataBlockMember::DataItems(DataItems::SingleValue(
                SingleLineData::from_tag_value((
                    Tag::new("chemical_formula_weight_meas".to_string()),
                    Value::from(*v),
                )),
            )));
        }
        out
    }

    fn from_members(
        &mut self,
        members: &[DataBlockMember],
    ) -> (Vec<DataBlockMember>, Vec<CifError>) {
        let mut residue = Vec::new();
        let mut errors = Vec::new();
        for m in members {
            if let DataBlockMember::DataItems(DataItems::SingleValue(sv)) = m {
                let tag = sv.tag().as_ref().to_lowercase();
                let value = sv.value();
                match tag.as_str() {
                    "chemical_formula_analytical" => {
                        match convert::read_char("chemical_formula_analytical", value) {
                            Ok(v) => self.analytical = v,
                            Err(e) => errors.push(e),
                        }
                    }
                    "chemical_formula_iupac" => {
                        match convert::read_char("chemical_formula_iupac", value) {
                            Ok(v) => self.iupac = v,
                            Err(e) => errors.push(e),
                        }
                    }
                    "chemical_formula_moiety" => {
                        match convert::read_char("chemical_formula_moiety", value) {
                            Ok(v) => self.moiety = v,
                            Err(e) => errors.push(e),
                        }
                    }
                    "chemical_formula_structural" => {
                        match convert::read_char("chemical_formula_structural", value) {
                            Ok(v) => self.structural = v,
                            Err(e) => errors.push(e),
                        }
                    }
                    "chemical_formula_sum" => {
                        match convert::read_char("chemical_formula_sum", value) {
                            Ok(v) => self.sum = v,
                            Err(e) => errors.push(e),
                        }
                    }
                    "chemical_formula_weight" => {
                        match convert::read_formula_weight("chemical_formula_weight", value) {
                            Ok(v) => self.weight = v,
                            Err(e) => errors.push(e),
                        }
                    }
                    "chemical_formula_weight_meas" => {
                        match convert::read_formula_weight("chemical_formula_weight_meas", value) {
                            Ok(v) => self.weight_meas = v,
                            Err(e) => errors.push(e),
                        }
                    }
                    _ => residue.push(m.clone()),
                }
            } else {
                residue.push(m.clone());
            }
        }
        (residue, errors)
    }
}
