use std::fmt::Display;

use winnow::{
    combinator::{preceded, repeat},
    Parser,
};

use crate::{
    grammar::{tags_values::Value, whitespace_comments::WhiteSpace, SyntacticUnit},
    LoopColumn,
};

pub use self::{heading::DataBlockHeading, members::DataBlockMember};

use super::SingleLineData;

mod heading;
mod members;

#[derive(Debug, Clone)]
pub struct DataBlock {
    heading: DataBlockHeading,
    members: Vec<DataBlockMember>,
}

impl DataBlock {
    pub fn from_heading_members(heading_member: (DataBlockHeading, Vec<DataBlockMember>)) -> Self {
        let (heading, members) = heading_member;
        Self { heading, members }
    }

    pub fn find_loop_column_by_tag<T: AsRef<str>>(&self, tag: T) -> Option<LoopColumn> {
        self.members.iter().find_map(|member| {
            if let DataBlockMember::DataItems(data_item) = member {
                data_item.get_loop_column_values_by_tag(&tag)
            } else {
                None
            }
        })
    }

    pub fn find_single_value_by_tag<T: AsRef<str>>(&self, tag: T) -> Option<&SingleLineData> {
        self.members.iter().find_map(|member| {
            if let DataBlockMember::DataItems(data_item) = member {
                data_item.get_single_value_by_tag(&tag)
            } else {
                None
            }
        })
    }

    pub fn heading(&self) -> &str {
        self.heading.as_ref()
    }
}

impl SyntacticUnit for DataBlock {
    type ParseResult = Self;

    type FormatOutput = String;

    fn parser(input: &mut &str) -> winnow::prelude::PResult<Self::ParseResult> {
        (
            DataBlockHeading::parser,
            repeat(0.., preceded(WhiteSpace::parser, DataBlockMember::parser)),
        )
            .map(Self::from_heading_members)
            .parse_next(input)
    }

    fn formatted_output(&self) -> Self::FormatOutput {
        let members = self
            .members
            .iter()
            .map(|m| m.to_string())
            .collect::<Vec<String>>()
            .join("\n");
        [self.heading.to_string(), members].join("\n")
    }
}

impl Display for DataBlock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.formatted_output())
    }
}

#[cfg(test)]
mod test {
    use winnow::{
        combinator::{peek, preceded},
        Parser,
    };

    use crate::grammar::{
        structures::{
            data_block::members::DataBlockMember, data_items::DataItems,
            tag_value_line::SingleLineData,
        },
        tags_values::Tag,
        whitespace_comments::WhiteSpace,
        SyntacticUnit,
    };

    use super::DataBlock;

    #[test]
    fn datablock_test() {
        let mut input = r#"data_I

# kfb29 (mar)

#=====================================================================
_vrf_PLAT_111_I
;
PROBLEM: _A ADDSYM Detects (Pseudo) Centre of Symmetry ...        100 Perc Fit
RESPONSE:  The acid used was enantiopure (S)-malic acid and the 
ADDSYM result is an artefact of the anion disorder.  ADDSYM reaches 
its conclusion having excluded all the atoms of the disordered 
anion, i.e. of the very component which is chiral. See Comment text.  
;
_vrf_PLAT_113_I
;
PROBLEM: _A ADDSYM Suggests Possible Pseudo/New Spacegroup       P-1          
RESPONSE: .See above
;


# 5. Chemical Data
_chemical_name_systematic
;
meso-5,5,7,12,12,14-Hexamethyl-4,11-diaza-1,8-diazoniacyclotetradecane 
(S)-malate(2-) methanol disolvate
;
_chemical_name_common             ?
_chemical_melting_point           ?
_chemical_formula_iupac          'C16 H38 N4 2+, C4 H4 O5 2-, 2C H4 O'
_chemical_formula_moiety         'C16 H38 N4 2+, C4 H4 O5 2-, 2C H4 O'
_chemical_formula_sum            'C22 H50 N4 O7'
_chemical_formula_weight          482.66
_chemical_compound_source        'synthesised by authors, see text'
_chemical_absolute_configuration  rm # known chiral centre

loop_
 _atom_type_symbol
 _atom_type_description
 _atom_type_scat_dispersion_real
 _atom_type_scat_dispersion_imag
 _atom_type_scat_source
 'C'  'C'   0.0033   0.0000
 'International Tables Vol C Tables 4.2.6.8 and 6.1.1.4'
 'H'  'H'   0.0000   0.0000
 'International Tables Vol C Tables 4.2.6.8 and 6.1.1.4'
 'N'  'N'   0.0061   0.0000
 'International Tables Vol C Tables 4.2.6.8 and 6.1.1.4'
 'O'  'O'   0.0106   0.0000
 'International Tables Vol C Tables 4.2.6.8 and 6.1.1.4'

_symmetry_cell_setting            triclinic
_symmetry_space_group_name_H-M    'P 1'
_symmetry_space_group_name_Hall   'P 1'

loop_
 _symmetry_equiv_pos_as_xyz
 'x, y, z'

_cell_length_a                    8.6559(9)
_cell_length_b                    9.3275(9)
_cell_length_c                    10.1044(12)
_cell_angle_alpha                 113.379(5)
_cell_angle_beta                  108.908(4)
_cell_angle_gamma                 100.345(4)
_cell_volume                      662.31(12)
_cell_formula_units_Z             1
_cell_measurement_temperature       150(1)
_cell_measurement_reflns_used        8456
_cell_measurement_theta_min          3.54
_cell_measurement_theta_max          27.61

_exptl_crystal_description        plate
_exptl_crystal_colour             colourless
_exptl_crystal_size_max           0.30
_exptl_crystal_size_mid           0.28
_exptl_crystal_size_min           0.10
_exptl_crystal_density_meas       ?
_exptl_crystal_density_diffrn     1.210
_exptl_crystal_density_method     'not measured'
_exptl_crystal_F_000              266
_exptl_absorpt_coefficient_mu     0.089
_exptl_absorpt_correction_type    Multi-scan
_exptl_absorpt_process_details    '(DENZO-SMN; Otwinowski & Minor, 1997)'
_exptl_absorpt_correction_T_min   0.950  
_exptl_absorpt_correction_T_max   0.988 

_exptl_special_details
;
?
;

_diffrn_ambient_temperature       150(1)
_diffrn_radiation_wavelength      0.71073
_diffrn_radiation_type            MoK\a
_diffrn_radiation_source          'fine-focus sealed X-ray tube'
_diffrn_radiation_monochromator   graphite
_diffrn_measurement_device_type   'Nonius KappaCCD'
_diffrn_measurement_method        '\f scans, and \w scans with \k offsets'
_diffrn_standards_decay_%         0
_diffrn_reflns_number             8456
_diffrn_reflns_av_R_equivalents   0.064
_diffrn_reflns_av_sigmaI/netI     0.0848
_diffrn_reflns_limit_h_min        -11
_diffrn_reflns_limit_h_max        11
_diffrn_reflns_limit_k_min        -12
_diffrn_reflns_limit_k_max        12
_diffrn_reflns_limit_l_min        -12
_diffrn_reflns_limit_l_max        13
_diffrn_reflns_theta_min          3.54
_diffrn_reflns_theta_max          27.61
_diffrn_reflns_theta_full              27.61
_diffrn_measured_fraction_theta_max    0.985
_diffrn_measured_fraction_theta_full   0.985
_reflns_number_total              3022
_reflns_number_gt                 1802
_reflns_threshold_expression      I>2\s(I)

_computing_data_collection        'KappaCCD Server Software (Nonius, 1997)'
_computing_cell_refinement        'DENZO-SMN (Otwinowski & Minor, 1997)'
_computing_data_reduction         'DENZO-SMN'
_computing_structure_solution     'SHELXS97 (Sheldrick, 1997)'
_computing_structure_refinement   'SHELXL97 (Sheldrick, 1997)'
_computing_molecular_graphics     'PLATON (Spek, 2003)'
_computing_publication_material   'SHELXL97 and PRPKAPPA (Ferguson, 1999)'

#=========================================================================

# 8. Refinement Data

_refine_ls_structure_factor_coef  Fsqd
_refine_ls_matrix_type            full
_refine_ls_weighting_scheme       calc
_refine_ls_weighting_details
 'w = 1/[\s^2^(Fo^2^)+(0.1019P)^2^+0.0709P] where P = (Fo^2^+2Fc^2^)/3'
_atom_sites_solution_primary      direct
_atom_sites_solution_secondary    difmap
_atom_sites_solution_hydrogens    geom
_refine_ls_hydrogen_treatment     constr
_refine_ls_extinction_method      SHELXL97
_refine_ls_extinction_coef        0.08(2)
_refine_ls_extinction_expression
 'Fc^*^=kFc[1+0.001xFc^2^\l^3^/sin(2\q)]^-1/4^'
_refine_ls_number_reflns          3022
_refine_ls_number_parameters      333
_refine_ls_number_restraints      23
_refine_ls_R_factor_all           0.1174
_refine_ls_R_factor_gt            0.0640
_refine_ls_wR_factor_ref          0.2015
_refine_ls_wR_factor_gt           0.1680
_refine_ls_goodness_of_fit_ref    1.033
_refine_ls_restrained_S_all       1.032
_refine_ls_shift/su_max           0.001
_refine_ls_shift/su_mean          0.000
_refine_diff_density_max          0.360
_refine_diff_density_min         -0.319

#=========================================================================
# 9. Atomic Coordinates and Displacement Parameters

loop_
 _atom_site_label
 _atom_site_type_symbol
 _atom_site_fract_x
 _atom_site_fract_y
 _atom_site_fract_z
 _atom_site_U_iso_or_equiv
 _atom_site_adp_type
 _atom_site_occupancy
 _atom_site_symmetry_multiplicity
 _atom_site_calc_flag
 _atom_site_refinement_flags
 _atom_site_disorder_assembly
 _atom_site_disorder_group
N1 N 0.3490(7) 0.5886(7) 0.6152(6) 0.0348(12) Uani 1 1 d . . .
C2 C 0.2646(9) 0.6456(9) 0.5044(8) 0.0417(17) Uani 1 1 d . . .
C3 C 0.2031(9) 0.5122(10) 0.3277(9) 0.0454(18) Uani 1 1 d . . .
N4 N 0.3509(7) 0.4851(7) 0.2942(6) 0.0351(13) Uani 1 1 d . . .
C5 C 0.3038(9) 0.3576(8) 0.1258(8) 0.0427(17) Uani 1 1 d . . .
C51 C 0.2029(11) 0.4108(11) 0.0063(8) 0.052(2) Uani 1 1 d . . .
C52 C 0.1843(9) 0.1879(9) 0.0882(9) 0.051(2) Uani 1 1 d . . .
C6 C 0.4699(9) 0.3468(8) 0.1112(7) 0.0409(17) Uani 1 1 d . . .
C7 C 0.5846(9) 0.2865(9) 0.2113(8) 0.0392(16) Uani 1 1 d . . .
C71 C 0.7346(11) 0.2657(11) 0.1687(10) 0.054(2) Uani 1 1 d . . .
N8 N 0.6494(8) 0.4112(7) 0.3859(6) 0.0406(14) Uani 1 1 d . . .
C9 C 0.7363(9) 0.3547(9) 0.5016(8) 0.0429(18) Uani 1 1 d . . .
C10 C 0.8003(8) 0.4854(9) 0.6700(8) 0.0396(16) Uani 1 1 d . . .
N11 N 0.6540(7) 0.5162(8) 0.7066(6) 0.0395(14) Uani 1 1 d . . .
C12 C 0.7037(8) 0.6402(9) 0.8750(8) 0.0406(17) Uani 1 1 d . . .
C121 C 0.7940(10) 0.5848(12) 0.9938(9) 0.057(2) Uani 1 1 d . . .
C122 C 0.8195(10) 0.8099(10) 0.9186(10) 0.057(2) Uani 1 1 d . . .
C13 C 0.5337(9) 0.6519(9) 0.8909(8) 0.0427(18) Uani 1 1 d . . .
C14 C 0.4196(9) 0.7124(9) 0.7900(8) 0.0404(16) Uani 1 1 d . . .
C141 C 0.2675(10) 0.7357(11) 0.8319(10) 0.055(2) Uani 1 1 d . . .
O21 O 0.8761(18) 0.6984(15) 0.4528(14) 0.055(3) Uani 0.702(8) 1 d PD A -1
O22 O 0.6487(12) 0.767(2) 0.352(2) 0.060(3) Uani 0.702(8) 1 d PD A -1
O23 O 1.1226(15) 1.2848(13) 0.5314(17) 0.053(2) Uani 0.702(8) 1 d PD A -1
O24 O 1.3506(8) 1.2325(8) 0.6507(9) 0.0422(17) Uani 0.702(8) 1 d PD A -1
O25 O 0.8473(7) 0.9921(7) 0.3250(7) 0.0543(16) Uani 0.702(8) 1 d P A -1
C21 C 0.8080(11) 0.7899(11) 0.4108(12) 0.042(3) Uani 0.702(8) 1 d PD A -1
C22 C 0.9351(13) 0.9296(15) 0.4161(15) 0.0670(17) Uani 0.702(8) 1 d PD A -1
C23 C 1.0659(14) 1.0562(15) 0.5800(15) 0.0670(17) Uani 0.702(8) 1 d PD A -1
C24 C 1.1900(8) 1.2041(11) 0.5920(12) 0.045(3) Uani 0.702(8) 1 d PD A -1
O41 O 0.4707(9) 0.9442(8) 0.2526(8) 0.0758(19) Uani 1 1 d . B -1
C42 C 0.3300(13) 0.8999(12) 0.2782(10) 0.069(2) Uani 1 1 d . B -1
O51 O 1.5287(9) 1.0569(8) 0.7487(8) 0.0758(18) Uani 1 1 d . . .
C53 C 1.6701(12) 1.0910(10) 0.7186(10) 0.063(2) Uani 1 1 d . . .
C31 C 0.829(3) 0.820(3) 0.461(3) 0.074(3) Uiso 0.298(8) 1 d PD A -2
C32 C 0.956(2) 0.995(3) 0.538(2) 0.074(3) Uiso 0.298(8) 1 d PD A -2
C33 C 1.062(3) 1.010(2) 0.450(2) 0.074(3) Uiso 0.298(8) 1 d PD A -2
C34 C 1.179(2) 1.186(2) 0.519(2) 0.074(3) Uiso 0.298(8) 1 d PD A -2
O31 O 0.891(6) 0.719(5) 0.490(5) 0.074(3) Uiso 0.298(8) 1 d PD A -2
O32 O 0.671(4) 0.782(6) 0.372(6) 0.074(3) Uiso 0.298(8) 1 d PD A -2
O33 O 1.127(5) 1.305(4) 0.531(4) 0.074(3) Uiso 0.298(8) 1 d PD A -2
O34 O 1.337(2) 1.203(2) 0.561(2) 0.074(3) Uiso 0.298(8) 1 d PD A -2
O35 O 1.100(2) 1.028(2) 0.679(2) 0.074(3) Uiso 0.298(8) 1 d PD A -2
H1A H 0.4394 0.5615 0.5957 0.042 Uiso 1 1 calc R . .
H1B H 0.2694 0.4927 0.5932 0.042 Uiso 1 1 calc R . .
H2A H 0.3479 0.7505 0.5285 0.050 Uiso 1 1 calc R . .
H2B H 0.1629 0.6693 0.5198 0.050 Uiso 1 1 calc R . .
H3A H 0.1248 0.4056 0.3055 0.054 Uiso 1 1 calc R . .
H3B H 0.1357 0.5488 0.2557 0.054 Uiso 1 1 calc R . .
H4 H 0.4233 0.5853 0.3166 0.042 Uiso 1 1 calc R . .
H51A H 0.0864 0.3997 0.0020 0.078 Uiso 1 1 calc R . .
H51B H 0.1917 0.3389 -0.1006 0.078 Uiso 1 1 calc R . .
H51C H 0.2669 0.5273 0.0421 0.078 Uiso 1 1 calc R . .
H52A H 0.2320 0.1680 0.1788 0.076 Uiso 1 1 calc R . .
H52B H 0.1787 0.0981 -0.0083 0.076 Uiso 1 1 calc R . .
H52C H 0.0667 0.1901 0.0702 0.076 Uiso 1 1 calc R . .
H6A H 0.5417 0.4591 0.1393 0.049 Uiso 1 1 calc R . .
H6B H 0.4379 0.2715 -0.0029 0.049 Uiso 1 1 calc R . .
H7 H 0.5120 0.1761 0.1891 0.047 Uiso 1 1 calc R . .
H71A H 0.8043 0.3725 0.1864 0.082 Uiso 1 1 calc R . .
H71B H 0.6883 0.1812 0.0558 0.082 Uiso 1 1 calc R . .
H71C H 0.8083 0.2301 0.2365 0.082 Uiso 1 1 calc R . .
H8A H 0.7276 0.5086 0.4090 0.049 Uiso 1 1 calc R . .
H8B H 0.5564 0.4351 0.4021 0.049 Uiso 1 1 calc R . .
H9A H 0.6518 0.2523 0.4807 0.052 Uiso 1 1 calc R . .
H9B H 0.8356 0.3269 0.4841 0.052 Uiso 1 1 calc R . .
H10A H 0.8688 0.4500 0.7427 0.048 Uiso 1 1 calc R . .
H10B H 0.8784 0.5903 0.6890 0.048 Uiso 1 1 calc R . .
H11 H 0.5790 0.4165 0.6825 0.047 Uiso 1 1 calc R . .
H12A H 0.9022 0.5736 0.9875 0.086 Uiso 1 1 calc R . .
H12B H 0.7162 0.4771 0.9670 0.086 Uiso 1 1 calc R . .
H12C H 0.8214 0.6684 1.1028 0.086 Uiso 1 1 calc R . .
H12D H 0.9325 0.8051 0.9210 0.086 Uiso 1 1 calc R . .
H12E H 0.8375 0.8932 1.0245 0.086 Uiso 1 1 calc R . .
H12F H 0.7634 0.8413 0.8386 0.086 Uiso 1 1 calc R . .
H13A H 0.4617 0.5399 0.8632 0.051 Uiso 1 1 calc R . .
H13B H 0.5661 0.7278 1.0049 0.051 Uiso 1 1 calc R . .
H14 H 0.4930 0.8220 0.8112 0.048 Uiso 1 1 calc R . .
H14A H 0.2035 0.7844 0.7728 0.082 Uiso 1 1 calc R . .
H14B H 0.3131 0.8105 0.9474 0.082 Uiso 1 1 calc R . .
H14C H 0.1886 0.6271 0.8022 0.082 Uiso 1 1 calc R . .
H25 H 0.9017 1.0949 0.3674 0.081 Uiso 0.702(8) 1 calc PR A -1
H22 H 1.0025 0.8756 0.3607 0.080 Uiso 0.702(8) 1 calc PR A -1
H23A H 1.1360 1.0012 0.6275 0.080 Uiso 0.702(8) 1 calc PR A -1
H23B H 1.0038 1.0989 0.6452 0.080 Uiso 0.702(8) 1 calc PR A -1
H41 H 0.5415 0.9003 0.2832 0.114 Uiso 1 1 calc R B -1
H42D H 0.3664 0.9442 0.3935 0.103 Uiso 1 1 calc R B -1
H42E H 0.2773 0.7778 0.2229 0.103 Uiso 1 1 calc R B -1
H42F H 0.2443 0.9455 0.2367 0.103 Uiso 1 1 calc R B -1
H51 H 1.4668 1.1114 0.7266 0.114 Uiso 1 1 calc R . .
H53A H 1.6645 1.1695 0.6769 0.095 Uiso 1 1 calc R . .
H53B H 1.6697 0.9873 0.6393 0.095 Uiso 1 1 calc R . .
H53C H 1.7776 1.1407 0.8182 0.095 Uiso 1 1 calc R . .
H32 H 0.8992 1.0809 0.5596 0.089 Uiso 0.298(8) 1 calc PR A -2
H33A H 1.1340 0.9393 0.4530 0.089 Uiso 0.298(8) 1 calc PR A -2
H33B H 0.9819 0.9676 0.3363 0.089 Uiso 0.298(8) 1 calc PR A -2
H35 H 1.0814 0.9467 0.6964 0.111 Uiso 0.298(8) 1 calc PRD A -2

loop_
 _atom_site_aniso_label
 _atom_site_aniso_U_11
 _atom_site_aniso_U_22
 _atom_site_aniso_U_33
 _atom_site_aniso_U_23
 _atom_site_aniso_U_13
 _atom_site_aniso_U_12
N1 0.026(3) 0.034(3) 0.038(3) 0.014(2) 0.012(2) 0.012(2)
C2 0.033(4) 0.044(4) 0.052(4) 0.026(4) 0.018(3) 0.016(3)
C3 0.035(4) 0.049(4) 0.045(4) 0.022(4) 0.012(3) 0.014(3)
N4 0.027(3) 0.031(3) 0.043(3) 0.017(3) 0.014(3) 0.006(2)
C5 0.046(4) 0.036(4) 0.047(4) 0.019(3) 0.023(4) 0.016(3)
C51 0.067(5) 0.057(5) 0.036(4) 0.023(4) 0.024(4) 0.027(4)
C52 0.042(4) 0.029(3) 0.063(5) 0.014(3) 0.019(4) -0.001(3)
C6 0.037(4) 0.034(4) 0.037(4) 0.011(3) 0.012(3) 0.006(3)
C7 0.038(4) 0.035(3) 0.042(4) 0.018(3) 0.017(3) 0.008(3)
C71 0.060(5) 0.065(5) 0.058(5) 0.037(5) 0.032(5) 0.038(5)
N8 0.041(3) 0.041(3) 0.045(3) 0.028(3) 0.017(3) 0.013(3)
C9 0.044(4) 0.050(4) 0.046(4) 0.031(4) 0.018(3) 0.024(3)
C10 0.026(4) 0.049(4) 0.047(4) 0.029(4) 0.013(3) 0.013(3)
N11 0.033(3) 0.041(3) 0.041(3) 0.022(3) 0.012(3) 0.010(2)
C12 0.027(3) 0.043(4) 0.038(4) 0.019(3) 0.002(3) 0.006(3)
C121 0.042(4) 0.074(5) 0.056(5) 0.039(4) 0.009(4) 0.027(4)
C122 0.044(5) 0.055(5) 0.062(5) 0.025(4) 0.016(4) 0.014(4)
C13 0.048(5) 0.050(4) 0.042(4) 0.032(4) 0.020(4) 0.021(3)
C14 0.039(4) 0.042(4) 0.044(4) 0.022(3) 0.018(3) 0.020(3)
C141 0.048(5) 0.073(6) 0.047(5) 0.029(4) 0.022(4) 0.026(4)
O21 0.039(4) 0.047(5) 0.075(7) 0.039(5) 0.017(5) 0.003(4)
O22 0.040(5) 0.039(5) 0.087(8) 0.029(6) 0.018(5) 0.007(4)
O23 0.030(4) 0.035(4) 0.082(5) 0.034(4) 0.010(3) 0.000(3)
O24 0.039(4) 0.029(3) 0.050(4) 0.015(3) 0.019(3) 0.006(3)
O25 0.044(3) 0.045(3) 0.058(3) 0.029(3) 0.005(2) 0.006(2)
C21 0.050(7) 0.025(5) 0.046(6) 0.018(5) 0.025(5) -0.002(4)
C22 0.047(3) 0.065(3) 0.064(3) 0.045(3) -0.003(2) -0.013(2)
C23 0.047(3) 0.065(3) 0.064(3) 0.045(3) -0.003(2) -0.013(2)
C24 0.027(5) 0.041(5) 0.045(6) 0.016(5) 0.002(4) 0.002(4)
O41 0.071(5) 0.068(4) 0.075(4) 0.041(4) 0.015(3) 0.017(3)
C42 0.068(6) 0.061(5) 0.059(5) 0.024(5) 0.017(5) 0.017(5)
O51 0.086(5) 0.071(4) 0.083(4) 0.048(4) 0.039(4) 0.025(4)
C53 0.083(7) 0.053(5) 0.069(6) 0.043(5) 0.034(5) 0.027(5)

#=========================================================================

# 10. Molecular Geometry

loop_
 _geom_bond_atom_site_label_1
 _geom_bond_atom_site_label_2
 _geom_bond_distance
 _geom_bond_site_symmetry_2
 _geom_bond_publ_flag
N1 C2 1.466(7) . no
N1 C14 1.502(8) . no
N1 H1A 0.92 . no
N1 H1B 0.92 . no
C2 C3 1.550(9) . no
C2 H2A 0.99 . no
C2 H2B 0.99 . no
C3 N4 1.465(8) . no
C3 H3A 0.99 . no
C3 H3B 0.99 . no
N4 C5 1.499(8) . no
N4 H4 0.92 . no
C5 C6 1.509(9) . no
C5 C52 1.549(9) . no
C5 C51 1.554(9) . no
C51 H51A 0.98 . no
C51 H51B 0.98 . no
C51 H51C 0.98 . no
C52 H52A 0.98 . no
C52 H52B 0.98 . no
C52 H52C 0.98 . no
C6 C7 1.525(9) . no
C6 H6A 0.99 . no
C6 H6B 0.99 . no
C7 N8 1.509(9) . no
C7 C71 1.515(10) . no
C7 H7 1.00 . no
C71 H71A 0.98 . no
C71 H71B 0.98 . no
C71 H71C 0.98 . no
N8 C9 1.507(7) . no
N8 H8A 0.92 . no
N8 H8B 0.92 . no
C9 C10 1.480(9) . no
C9 H9A 0.99 . no
C9 H9B 0.99 . no
C10 N11 1.474(8) . no
C10 H10A 0.99 . no
C10 H10B 0.99 . no
N11 C12 1.484(8) . no
N11 H11 0.92 . no
C12 C122 1.515(10) . no
C12 C121 1.531(8) . no
C12 C13 1.549(9) . no
C121 H12A 0.98 . no
C121 H12B 0.98 . no
C121 H12C 0.98 . no
C122 H12D 0.98 . no
C122 H12E 0.98 . no
C122 H12F 0.98 . no
C13 C14 1.529(9) . no
C13 H13A 0.99 . no
C13 H13B 0.99 . no
C14 C141 1.533(10) . no
C14 H14 1.00 . no
C141 H14A 0.98 . no
C141 H14B 0.98 . no
C141 H14C 0.98 . no
O21 C21 1.250(5) . no
O22 C21 1.250(5) . no
O23 C24 1.254(5) . no
O24 C24 1.250(5) . no
O25 C22 1.377(11) . no
O25 H25 0.84 . no
C21 C22 1.518(7) . no
C22 C23 1.474(7) . no
C22 H22 1.00 . no
C23 C24 1.524(8) . no
C23 H23A 0.99 . no
C23 H23B 0.99 . no
O41 C42 1.352(10) . no
O41 H41 0.84 . no
C42 H42D 0.98 . no
C42 H42E 0.98 . no
C42 H42F 0.98 . no
O51 C53 1.367(9) . no
O51 H51 0.84 . no
C53 H53A 0.98 . no
C53 H53B 0.98 . no
C53 H53C 0.98 . no
C31 O31 1.250(5) . no
C31 O32 1.251(5) . no
C31 C32 1.504(5) . no
C32 O35 1.432(10) . no
C32 C33 1.498(5) . no
C32 H32 1.00 . no
C33 C34 1.498(5) . no
C33 H33A 0.99 . no
C33 H33B 0.99 . no
C34 O33 1.249(5) . no
C34 O34 1.249(5) . no
O35 H35 0.84 . no

loop_
 _geom_angle_atom_site_label_1
 _geom_angle_atom_site_label_2
 _geom_angle_atom_site_label_3
 _geom_angle
 _geom_angle_site_symmetry_1
 _geom_angle_site_symmetry_3
 _geom_angle_publ_flag
C2 N1 C14 115.2(5) . . no
C2 N1 H1A 108.5 . . no
C14 N1 H1A 108.5 . . no
C2 N1 H1B 108.5 . . no
C14 N1 H1B 108.5 . . no
H1A N1 H1B 107.5 . . no
N1 C2 C3 111.2(5) . . no
N1 C2 H2A 109.4 . . no
C3 C2 H2A 109.4 . . no
N1 C2 H2B 109.4 . . no
C3 C2 H2B 109.4 . . no
H2A C2 H2B 108.0 . . no
N4 C3 C2 111.6(5) . . no
N4 C3 H3A 109.3 . . no
C2 C3 H3A 109.3 . . no
N4 C3 H3B 109.3 . . no
C2 C3 H3B 109.3 . . no
H3A C3 H3B 108.0 . . no
C3 N4 C5 115.4(5) . . no
C3 N4 H4 108.4 . . no
C5 N4 H4 108.4 . . no
N4 C5 C6 108.6(5) . . no
N4 C5 C52 108.7(5) . . no
C6 C5 C52 111.6(5) . . no
N4 C5 C51 109.9(5) . . no
C6 C5 C51 109.2(5) . . no
C52 C5 C51 108.8(6) . . no
C5 C51 H51A 109.5 . . no
C5 C51 H51B 109.5 . . no
H51A C51 H51B 109.5 . . no
C5 C51 H51C 109.5 . . no
H51A C51 H51C 109.5 . . no
H51B C51 H51C 109.5 . . no
C5 C52 H52A 109.5 . . no
C5 C52 H52B 109.5 . . no
H52A C52 H52B 109.5 . . no
C5 C52 H52C 109.5 . . no
H52A C52 H52C 109.5 . . no
H52B C52 H52C 109.5 . . no
C5 C6 C7 118.0(6) . . no
C5 C6 H6A 107.8 . . no
C7 C6 H6A 107.8 . . no
C5 C6 H6B 107.8 . . no
C7 C6 H6B 107.8 . . no
H6A C6 H6B 107.1 . . no
N8 C7 C71 111.5(6) . . no
N8 C7 C6 108.0(5) . . no
C71 C7 C6 110.9(5) . . no
N8 C7 H7 108.8 . . no
C71 C7 H7 108.8 . . no
C6 C7 H7 108.8 . . no
C7 C71 H71A 109.5 . . no
C7 C71 H71B 109.5 . . no
H71A C71 H71B 109.5 . . no
C7 C71 H71C 109.5 . . no
H71A C71 H71C 109.5 . . no
H71B C71 H71C 109.5 . . no
C9 N8 C7 114.8(5) . . no
C9 N8 H8A 108.6 . . no
C7 N8 H8A 108.6 . . no
C9 N8 H8B 108.6 . . no
C7 N8 H8B 108.6 . . no
H8A N8 H8B 107.5 . . no
C10 C9 N8 111.5(5) . . no
C10 C9 H9A 109.3 . . no
N8 C9 H9A 109.3 . . no
C10 C9 H9B 109.3 . . no
N8 C9 H9B 109.3 . . no
H9A C9 H9B 108.0 . . no
N11 C10 C9 111.1(5) . . no
N11 C10 H10A 109.4 . . no
C9 C10 H10A 109.4 . . no
N11 C10 H10B 109.4 . . no
C9 C10 H10B 109.4 . . no
H10A C10 H10B 108.0 . . no
C10 N11 C12 115.5(5) . . no
C10 N11 H11 108.4 . . no
C12 N11 H11 108.4 . . no
N11 C12 C122 110.7(6) . . no
N11 C12 C121 112.2(6) . . no
C122 C12 C121 110.2(6) . . no
N11 C12 C13 107.8(5) . . no
C122 C12 C13 109.8(6) . . no
C121 C12 C13 106.0(5) . . no
C12 C121 H12A 109.5 . . no
C12 C121 H12B 109.5 . . no
H12A C121 H12B 109.5 . . no
C12 C121 H12C 109.5 . . no
H12A C121 H12C 109.5 . . no
H12B C121 H12C 109.5 . . no
C12 C122 H12D 109.5 . . no
C12 C122 H12E 109.5 . . no
H12D C122 H12E 109.5 . . no
C12 C122 H12F 109.5 . . no
H12D C122 H12F 109.5 . . no
H12E C122 H12F 109.5 . . no
C14 C13 C12 117.5(5) . . no
C14 C13 H13A 107.9 . . no
C12 C13 H13A 107.9 . . no
C14 C13 H13B 107.9 . . no
C12 C13 H13B 107.9 . . no
H13A C13 H13B 107.2 . . no
N1 C14 C13 109.6(5) . . no
N1 C14 C141 109.5(6) . . no
C13 C14 C141 111.5(5) . . no
N1 C14 H14 108.7 . . no
C13 C14 H14 108.7 . . no
C141 C14 H14 108.7 . . no
C14 C141 H14A 109.5 . . no
C14 C141 H14B 109.5 . . no
H14A C141 H14B 109.5 . . no
C14 C141 H14C 109.5 . . no
H14A C141 H14C 109.5 . . no
H14B C141 H14C 109.5 . . no
O21 C21 O22 126.8(11) . . no
O21 C21 C22 114.1(10) . . no
O22 C21 C22 118.7(10) . . no
O25 C22 C23 115.1(7) . . no
O25 C22 C21 110.8(8) . . no
C23 C22 C21 113.2(7) . . no
O25 C22 H22 105.6 . . no
C23 C22 H22 105.6 . . no
C21 C22 H22 105.6 . . no
C22 C23 C24 115.1(7) . . no
C22 C23 H23A 108.5 . . no
C24 C23 H23A 108.5 . . no
C22 C23 H23B 108.5 . . no
C24 C23 H23B 108.5 . . no
H23A C23 H23B 107.5 . . no
O24 C24 O23 123.2(9) . . no
O24 C24 C23 119.4(8) . . no
O23 C24 C23 117.2(8) . . no
C53 O51 H51 109.5 . . no
O51 C53 H53A 109.5 . . no
O51 C53 H53B 109.5 . . no
H53A C53 H53B 109.5 . . no
O51 C53 H53C 109.5 . . no
H53A C53 H53C 109.5 . . no
H53B C53 H53C 109.5 . . no
O31 C31 O32 123(4) . . no
O31 C31 C32 116(3) . . no
O32 C31 C32 121(3) . . no
O35 C32 C33 95.1(15) . . no
O35 C32 C31 110(2) . . no
C33 C32 C31 112.7(5) . . no
O35 C32 H32 112.8 . . no
C33 C32 H32 112.8 . . no
C31 C32 H32 112.8 . . no
C34 C33 C32 113.1(5) . . no
C34 C33 H33A 109.0 . . no
C32 C33 H33A 109.0 . . no
C34 C33 H33B 109.0 . . no
C32 C33 H33B 109.0 . . no
H33A C33 H33B 107.8 . . no
O33 C34 O34 122(3) . . no
O33 C34 C33 124(3) . . no
O34 C34 C33 113.2(19) . . no
C32 O35 H35 109.5 . . no


loop_
 _geom_torsion_atom_site_label_1
 _geom_torsion_atom_site_label_2
 _geom_torsion_atom_site_label_3
 _geom_torsion_atom_site_label_4
 _geom_torsion
 _geom_torsion_site_symmetry_1
 _geom_torsion_site_symmetry_2
 _geom_torsion_site_symmetry_3
 _geom_torsion_site_symmetry_4
 _geom_torsion_publ_flag
N1 C2 C3 N4 -65.0(9) . . . . y
C2 C3 N4 C5 179.9(7) . . . . y
C3 N4 C5 C6 179.2(7) . . . . y
N4 C5 C6 C7 63.6(9) . . . . y
C5 C6 C7 N8 -63.6(9) . . . . y
C6 C7 N8 C9 168.9(7) . . . . y
C7 N8 C9 C10 178.4(7) . . . . y
O22 C21 C22 C23 119.7(14) . . . . y
C21 C22 C23 C24 -176.7(10) . . . . y
N8 C9 C10 N11 65.8(9) . . . . y
C9 C10 N11 C12 177.4(7) . . . . y
C10 N11 C12 C13 -178.5(7) . . . . y
N11 C12 C13 C14 -62.2(9) . . . . y
C12 C13 C14 N1 64.9(9) . . . . y
C13 C14 N1 C2 -169.1(7) . . . . y
C14 N1 C2 C3 177.7(7) . . . . y
O25 C22 C23 C24 -47.8(15) . . . . y
C22 C23 C24 O23 55.5(16) . . . . y



loop_
_geom_hbond_atom_site_label_D
_geom_hbond_atom_site_label_H
_geom_hbond_atom_site_label_A
_geom_hbond_distance_DH
_geom_hbond_distance_HA
_geom_hbond_distance_DA
_geom_hbond_angle_DHA
_geom_hbond_site_symmetry_A
_geom_hbond_publ_flag
#
# Hydrogen bonding scheme
# ======== ======= ======
#
# D   H   A    D-H  H...A  D...A     D-H...A  symm  publ
# -   -   -    ---  -----  -----     -------  ----  ----
  N1  H1A N11  0.92 2.05   2.799(8)   138      .     y
  N8  H8B N4   0.92 2.00   2.758(8)   138      .     y
  O25 H25 O23  0.84 1.98   2.721(12)  146      .     y
  O35 H35 O31  0.84 2.14   2.60(5)    114      .     n
  N4  H4  O22  0.92 2.17   3.046(15)  160      .     y
  N4  H4  O32  0.92 2.29   3.17(4)    159      .     n
  N8  H8A O21  0.92 1.78   2.702(14)  179      .     y
  N8  H8A O31  0.92 1.87   2.78(5)    171      .     n
  O41 H41 O22  0.84 1.89   2.708(16)  165      .     y
  O51 H51 O24  0.84 1.86   2.691(9)   168      .     y
  O51 H51 O34  0.84 2.25   3.005(18)  150      .     n
  N1  H1B O23  0.92 1.85   2.760(11)  172  1_445 y
  N1  H1B O33  0.92 1.68   2.60(4)    173  1_445 n
  N11 H11 O24  0.92 2.21   3.094(8)   161  1_445 y
  N11 H11 O34  0.92 2.18   3.06(2)    158  1_445 n
  C2  H2B O21  0.99 2.45   3.39(2)    159  1_455 n
####################################################################
#                         END of CIF
####################################################################
"#;
        let parse_result = DataBlock::parser(&mut input);
        match parse_result {
            Ok(d) => {
                println!(
                    "{}",
                    d.find_single_value_by_tag("chemical_formula_sum").unwrap()
                )
                // println!(
                //     "{:?}",
                //     preceded(WhiteSpace::parser, SingleLineData::parser).parse_next(&mut input)
                // );
                // println!("{:?}", input.lines().next())
            }
            Err(e) => {
                println!("{e}");
                println!(
                    "{:?}",
                    preceded(WhiteSpace::parser, SingleLineData::parser).parse_next(&mut input)
                );
            }
        }
    }
}
