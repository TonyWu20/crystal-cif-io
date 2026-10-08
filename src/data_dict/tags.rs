//! Central tag registry for the implemented `CifTerm` tag strings.
//!
//! The registry is a plain list, organized by data-dictionary
//! category. Each tier appends its own category section. The tag
//! strings carry no leading underscore. They match the `CifTerm`
//! tag output and the inventory item names in
//! `tests/dict_inventory.json`.
//!
//! `tests/dict_inventory.rs` checks every registered tag against
//! the recorded dictionary inventory. A tag that is absent from
//! the inventory and not listed there as dictionary drift fails the
//! test.

/// `atom_site` category tags (54).
pub mod atom_site {
    pub const TAGS: &[&str] = &[
        "atom_site_adp_type",
        "atom_site_aniso_B_11",
        "atom_site_aniso_B_12",
        "atom_site_aniso_B_13",
        "atom_site_aniso_B_22",
        "atom_site_aniso_B_23",
        "atom_site_aniso_B_33",
        "atom_site_aniso_label",
        "atom_site_aniso_ratio",
        "atom_site_aniso_type_symbol",
        "atom_site_aniso_U_11",
        "atom_site_aniso_U_12",
        "atom_site_aniso_U_13",
        "atom_site_aniso_U_22",
        "atom_site_aniso_U_23",
        "atom_site_aniso_U_33",
        "atom_site_attached_hydrogens",
        "atom_site_b_equiv_geom_mean",
        "atom_site_b_iso_or_equiv",
        "atom_site_calc_attached_atom",
        "atom_site_calc_flag",
        "atom_site_cartn_x",
        "atom_site_cartn_y",
        "atom_site_cartn_z",
        "atom_site_chemical_conn_number",
        "atom_site_constraints",
        "atom_site_description",
        "atom_site_disorder_assembly",
        "atom_site_disorder_group",
        "atom_site_fract_x",
        "atom_site_fract_y",
        "atom_site_fract_z",
        "atom_site_label",
        "atom_site_label_component_0",
        "atom_site_label_component_1",
        "atom_site_label_component_2",
        "atom_site_label_component_3",
        "atom_site_label_component_4",
        "atom_site_label_component_5",
        "atom_site_label_component_6",
        "atom_site_occupancy",
        "atom_site_refinement_flags",
        "atom_site_refinement_flags_adp",
        "atom_site_refinement_flags_occupancy",
        "atom_site_refinement_flags_posn",
        "atom_site_restraints",
        "atom_site_site_symmetry_multiplicity",
        "atom_site_site_symmetry_order",
        "atom_site_symmetry_multiplicity",
        "atom_site_thermal_displace_type",
        "atom_site_type_symbol",
        "atom_site_u_equiv_geom_mean",
        "atom_site_u_iso_or_equiv",
        "atom_site_wyckoff_symbol",
    ];
}

/// `audit` category tags (5).
pub mod audit {
    pub const TAGS: &[&str] = &[
        "audit_block_code",
        "audit_block_doi",
        "audit_creation_date",
        "audit_creation_method",
        "audit_update_record",
    ];
}

/// `cell` category tags (22).
pub mod cell {
    pub const TAGS: &[&str] = &[
        "cell_angle_alpha",
        "cell_angle_beta",
        "cell_angle_gamma",
        "cell_formula_units_z",
        "cell_length_a",
        "cell_length_b",
        "cell_length_c",
        "cell_measurement_pressure",
        "cell_measurement_radiation",
        "cell_measurement_reflns_used",
        "cell_measurement_temperature",
        "cell_measurement_theta_max",
        "cell_measurement_theta_min",
        "cell_measurement_wavelength",
        "cell_reciprocal_angle_alpha",
        "cell_reciprocal_angle_beta",
        "cell_reciprocal_angle_gamma",
        "cell_reciprocal_length_a",
        "cell_reciprocal_length_b",
        "cell_reciprocal_length_c",
        "cell_special_details",
        "cell_volume",
    ];
}

/// `cell_measurement_refln` category tags (4).
pub mod cell_measurement_refln {
    pub const TAGS: &[&str] = &[
        "cell_measurement_refln_index_h",
        "cell_measurement_refln_index_k",
        "cell_measurement_refln_index_l",
        "cell_measurement_refln_theta",
    ];
}

/// `space_group` category tags (5).
pub mod space_group {
    pub const TAGS: &[&str] = &[
        "space_group_crystal_system",
        "space_group_id",
        "space_group_IT_number",
        "space_group_name_H-M_alt",
        "space_group_name_Hall",
    ];
}

/// `space_group_symop` category tags (3).
pub mod space_group_symop {
    pub const TAGS: &[&str] = &[
        "space_group_symop_id",
        "space_group_symop_operation_xyz",
        "space_group_symop_sg_id",
    ];
}

/// `atom_sites` category tags (29).
pub mod atom_sites {
    pub const TAGS: &[&str] = &[
        "atom_sites_Cartn_tran_matrix_11",
        "atom_sites_Cartn_tran_matrix_12",
        "atom_sites_Cartn_tran_matrix_13",
        "atom_sites_Cartn_tran_matrix_21",
        "atom_sites_Cartn_tran_matrix_22",
        "atom_sites_Cartn_tran_matrix_23",
        "atom_sites_Cartn_tran_matrix_31",
        "atom_sites_Cartn_tran_matrix_32",
        "atom_sites_Cartn_tran_matrix_33",
        "atom_sites_Cartn_tran_vector_1",
        "atom_sites_Cartn_tran_vector_2",
        "atom_sites_Cartn_tran_vector_3",
        "atom_sites_Cartn_transform_axes",
        "atom_sites_fract_tran_matrix_11",
        "atom_sites_fract_tran_matrix_12",
        "atom_sites_fract_tran_matrix_13",
        "atom_sites_fract_tran_matrix_21",
        "atom_sites_fract_tran_matrix_22",
        "atom_sites_fract_tran_matrix_23",
        "atom_sites_fract_tran_matrix_31",
        "atom_sites_fract_tran_matrix_32",
        "atom_sites_fract_tran_matrix_33",
        "atom_sites_fract_tran_vector_1",
        "atom_sites_fract_tran_vector_2",
        "atom_sites_fract_tran_vector_3",
        "atom_sites_solution_hydrogens",
        "atom_sites_solution_primary",
        "atom_sites_solution_secondary",
        "atom_sites_special_details",
    ];
}

/// `atom_type` category tags (22).
pub mod atom_type {
    pub const TAGS: &[&str] = &[
        "atom_type_analytical_mass_%",
        "atom_type_description",
        "atom_type_number_in_cell",
        "atom_type_oxidation_number",
        "atom_type_radius_bond",
        "atom_type_radius_contact",
        "atom_type_scat_Cromer_Mann_a1",
        "atom_type_scat_Cromer_Mann_a2",
        "atom_type_scat_Cromer_Mann_a3",
        "atom_type_scat_Cromer_Mann_a4",
        "atom_type_scat_Cromer_Mann_b1",
        "atom_type_scat_Cromer_Mann_b2",
        "atom_type_scat_Cromer_Mann_b3",
        "atom_type_scat_Cromer_Mann_b4",
        "atom_type_scat_Cromer_Mann_c",
        "atom_type_scat_dispersion_imag",
        "atom_type_scat_dispersion_real",
        "atom_type_scat_dispersion_source",
        "atom_type_scat_length_neutron",
        "atom_type_scat_source",
        "atom_type_scat_versus_stol_list",
        "atom_type_symbol",
    ];
}

/// `chemical` category tags (25).
pub mod chemical {
    pub const TAGS: &[&str] = &[
        "chemical_absolute_configuration",
        "chemical_compound_source",
        "chemical_enantioexcess_bulk",
        "chemical_enantioexcess_bulk_technique",
        "chemical_enantioexcess_crystal",
        "chemical_enantioexcess_crystal_technique",
        "chemical_identifier_inchi",
        "chemical_identifier_inchi_key",
        "chemical_identifier_inchi_version",
        "chemical_melting_point",
        "chemical_melting_point_gt",
        "chemical_melting_point_lt",
        "chemical_name_common",
        "chemical_name_mineral",
        "chemical_name_structure_type",
        "chemical_name_systematic",
        "chemical_optical_rotation",
        "chemical_properties_biological",
        "chemical_properties_physical",
        "chemical_temperature_decomposition",
        "chemical_temperature_decomposition_gt",
        "chemical_temperature_decomposition_lt",
        "chemical_temperature_sublimation",
        "chemical_temperature_sublimation_gt",
        "chemical_temperature_sublimation_lt",
    ];
}

/// `chemical_conn_atom` category tags (7).
pub mod chemical_conn_atom {
    pub const TAGS: &[&str] = &[
        "chemical_conn_atom_NCA",
        "chemical_conn_atom_NH",
        "chemical_conn_atom_charge",
        "chemical_conn_atom_display_x",
        "chemical_conn_atom_display_y",
        "chemical_conn_atom_number",
        "chemical_conn_atom_type_symbol",
    ];
}

/// `chemical_conn_bond` category tags (3).
pub mod chemical_conn_bond {
    pub const TAGS: &[&str] = &[
        "chemical_conn_bond_atom_1",
        "chemical_conn_bond_atom_2",
        "chemical_conn_bond_type",
    ];
}

/// `chemical_formula` category tags (7).
pub mod chemical_formula {
    pub const TAGS: &[&str] = &[
        "chemical_formula_analytical",
        "chemical_formula_iupac",
        "chemical_formula_moiety",
        "chemical_formula_structural",
        "chemical_formula_sum",
        "chemical_formula_weight",
        "chemical_formula_weight_meas",
    ];
}

/// `geom` category tags (1).
pub mod geom {
    pub const TAGS: &[&str] = &["geom_special_details"];
}

/// `geom_angle` category tags (8).
pub mod geom_angle {
    pub const TAGS: &[&str] = &[
        "geom_angle",
        "geom_angle_atom_site_label_1",
        "geom_angle_atom_site_label_2",
        "geom_angle_atom_site_label_3",
        "geom_angle_publ_flag",
        "geom_angle_site_symmetry_1",
        "geom_angle_site_symmetry_2",
        "geom_angle_site_symmetry_3",
    ];
}

/// `geom_bond` category tags (8).
pub mod geom_bond {
    pub const TAGS: &[&str] = &[
        "geom_bond_atom_site_label_1",
        "geom_bond_atom_site_label_2",
        "geom_bond_distance",
        "geom_bond_multiplicity",
        "geom_bond_publ_flag",
        "geom_bond_site_symmetry_1",
        "geom_bond_site_symmetry_2",
        "geom_bond_valence",
    ];
}

/// `geom_contact` category tags (6).
pub mod geom_contact {
    pub const TAGS: &[&str] = &[
        "geom_contact_atom_site_label_1",
        "geom_contact_atom_site_label_2",
        "geom_contact_distance",
        "geom_contact_publ_flag",
        "geom_contact_site_symmetry_1",
        "geom_contact_site_symmetry_2",
    ];
}

/// `geom_hbond` category tags (11).
pub mod geom_hbond {
    pub const TAGS: &[&str] = &[
        "geom_hbond_angle_DHA",
        "geom_hbond_atom_site_label_A",
        "geom_hbond_atom_site_label_D",
        "geom_hbond_atom_site_label_H",
        "geom_hbond_distance_DA",
        "geom_hbond_distance_DH",
        "geom_hbond_distance_HA",
        "geom_hbond_publ_flag",
        "geom_hbond_site_symmetry_A",
        "geom_hbond_site_symmetry_D",
        "geom_hbond_site_symmetry_H",
    ];
}

/// `geom_torsion` category tags (10).
pub mod geom_torsion {
    pub const TAGS: &[&str] = &[
        "geom_torsion",
        "geom_torsion_atom_site_label_1",
        "geom_torsion_atom_site_label_2",
        "geom_torsion_atom_site_label_3",
        "geom_torsion_atom_site_label_4",
        "geom_torsion_publ_flag",
        "geom_torsion_site_symmetry_1",
        "geom_torsion_site_symmetry_2",
        "geom_torsion_site_symmetry_3",
        "geom_torsion_site_symmetry_4",
    ];
}

/// `valence_param` category tags (9).
pub mod valence_param {
    pub const TAGS: &[&str] = &[
        "valence_param_B",
        "valence_param_Ro",
        "valence_param_atom_1",
        "valence_param_atom_1_valence",
        "valence_param_atom_2",
        "valence_param_atom_2_valence",
        "valence_param_details",
        "valence_param_id",
        "valence_param_ref_id",
    ];
}

/// `valence_ref` category tags (2).
pub mod valence_ref {
    pub const TAGS: &[&str] = &["valence_ref_id", "valence_ref_reference"];
}

/// All implemented tags, one entry per `CifTerm` tag string.
pub const ALL: &[&str] = &[
    // atom_site
    "atom_site_adp_type",
    "atom_site_aniso_B_11",
    "atom_site_aniso_B_12",
    "atom_site_aniso_B_13",
    "atom_site_aniso_B_22",
    "atom_site_aniso_B_23",
    "atom_site_aniso_B_33",
    "atom_site_aniso_label",
    "atom_site_aniso_ratio",
    "atom_site_aniso_type_symbol",
    "atom_site_aniso_U_11",
    "atom_site_aniso_U_12",
    "atom_site_aniso_U_13",
    "atom_site_aniso_U_22",
    "atom_site_aniso_U_23",
    "atom_site_aniso_U_33",
    "atom_site_attached_hydrogens",
    "atom_site_b_equiv_geom_mean",
    "atom_site_b_iso_or_equiv",
    "atom_site_calc_attached_atom",
    "atom_site_calc_flag",
    "atom_site_cartn_x",
    "atom_site_cartn_y",
    "atom_site_cartn_z",
    "atom_site_chemical_conn_number",
    "atom_site_constraints",
    "atom_site_description",
    "atom_site_disorder_assembly",
    "atom_site_disorder_group",
    "atom_site_fract_x",
    "atom_site_fract_y",
    "atom_site_fract_z",
    "atom_site_label",
    "atom_site_label_component_0",
    "atom_site_label_component_1",
    "atom_site_label_component_2",
    "atom_site_label_component_3",
    "atom_site_label_component_4",
    "atom_site_label_component_5",
    "atom_site_label_component_6",
    "atom_site_occupancy",
    "atom_site_refinement_flags",
    "atom_site_refinement_flags_adp",
    "atom_site_refinement_flags_occupancy",
    "atom_site_refinement_flags_posn",
    "atom_site_restraints",
    "atom_site_site_symmetry_multiplicity",
    "atom_site_site_symmetry_order",
    "atom_site_symmetry_multiplicity",
    "atom_site_thermal_displace_type",
    "atom_site_type_symbol",
    "atom_site_u_equiv_geom_mean",
    "atom_site_u_iso_or_equiv",
    "atom_site_wyckoff_symbol",
    // audit
    "audit_block_code",
    "audit_block_doi",
    "audit_creation_date",
    "audit_creation_method",
    "audit_update_record",
    // cell
    "cell_angle_alpha",
    "cell_angle_beta",
    "cell_angle_gamma",
    "cell_formula_units_z",
    "cell_length_a",
    "cell_length_b",
    "cell_length_c",
    "cell_measurement_pressure",
    "cell_measurement_radiation",
    "cell_measurement_reflns_used",
    "cell_measurement_temperature",
    "cell_measurement_theta_max",
    "cell_measurement_theta_min",
    "cell_measurement_wavelength",
    "cell_reciprocal_angle_alpha",
    "cell_reciprocal_angle_beta",
    "cell_reciprocal_angle_gamma",
    "cell_reciprocal_length_a",
    "cell_reciprocal_length_b",
    "cell_reciprocal_length_c",
    "cell_special_details",
    "cell_volume",
    // cell_measurement_refln
    "cell_measurement_refln_index_h",
    "cell_measurement_refln_index_k",
    "cell_measurement_refln_index_l",
    "cell_measurement_refln_theta",
    // space_group
    "space_group_crystal_system",
    "space_group_id",
    "space_group_IT_number",
    "space_group_name_H-M_alt",
    "space_group_name_Hall",
    // space_group_symop
    "space_group_symop_id",
    "space_group_symop_operation_xyz",
    "space_group_symop_sg_id",
    // atom_sites
    "atom_sites_Cartn_tran_matrix_11",
    "atom_sites_Cartn_tran_matrix_12",
    "atom_sites_Cartn_tran_matrix_13",
    "atom_sites_Cartn_tran_matrix_21",
    "atom_sites_Cartn_tran_matrix_22",
    "atom_sites_Cartn_tran_matrix_23",
    "atom_sites_Cartn_tran_matrix_31",
    "atom_sites_Cartn_tran_matrix_32",
    "atom_sites_Cartn_tran_matrix_33",
    "atom_sites_Cartn_tran_vector_1",
    "atom_sites_Cartn_tran_vector_2",
    "atom_sites_Cartn_tran_vector_3",
    "atom_sites_Cartn_transform_axes",
    "atom_sites_fract_tran_matrix_11",
    "atom_sites_fract_tran_matrix_12",
    "atom_sites_fract_tran_matrix_13",
    "atom_sites_fract_tran_matrix_21",
    "atom_sites_fract_tran_matrix_22",
    "atom_sites_fract_tran_matrix_23",
    "atom_sites_fract_tran_matrix_31",
    "atom_sites_fract_tran_matrix_32",
    "atom_sites_fract_tran_matrix_33",
    "atom_sites_fract_tran_vector_1",
    "atom_sites_fract_tran_vector_2",
    "atom_sites_fract_tran_vector_3",
    "atom_sites_solution_hydrogens",
    "atom_sites_solution_primary",
    "atom_sites_solution_secondary",
    "atom_sites_special_details",
    // atom_type
    "atom_type_analytical_mass_%",
    "atom_type_description",
    "atom_type_number_in_cell",
    "atom_type_oxidation_number",
    "atom_type_radius_bond",
    "atom_type_radius_contact",
    "atom_type_scat_Cromer_Mann_a1",
    "atom_type_scat_Cromer_Mann_a2",
    "atom_type_scat_Cromer_Mann_a3",
    "atom_type_scat_Cromer_Mann_a4",
    "atom_type_scat_Cromer_Mann_b1",
    "atom_type_scat_Cromer_Mann_b2",
    "atom_type_scat_Cromer_Mann_b3",
    "atom_type_scat_Cromer_Mann_b4",
    "atom_type_scat_Cromer_Mann_c",
    "atom_type_scat_dispersion_imag",
    "atom_type_scat_dispersion_real",
    "atom_type_scat_dispersion_source",
    "atom_type_scat_length_neutron",
    "atom_type_scat_source",
    "atom_type_scat_versus_stol_list",
    "atom_type_symbol",
    // chemical
    "chemical_absolute_configuration",
    "chemical_compound_source",
    "chemical_enantioexcess_bulk",
    "chemical_enantioexcess_bulk_technique",
    "chemical_enantioexcess_crystal",
    "chemical_enantioexcess_crystal_technique",
    "chemical_identifier_inchi",
    "chemical_identifier_inchi_key",
    "chemical_identifier_inchi_version",
    "chemical_melting_point",
    "chemical_melting_point_gt",
    "chemical_melting_point_lt",
    "chemical_name_common",
    "chemical_name_mineral",
    "chemical_name_structure_type",
    "chemical_name_systematic",
    "chemical_optical_rotation",
    "chemical_properties_biological",
    "chemical_properties_physical",
    "chemical_temperature_decomposition",
    "chemical_temperature_decomposition_gt",
    "chemical_temperature_decomposition_lt",
    "chemical_temperature_sublimation",
    "chemical_temperature_sublimation_gt",
    "chemical_temperature_sublimation_lt",
    // chemical_conn_atom
    "chemical_conn_atom_NCA",
    "chemical_conn_atom_NH",
    "chemical_conn_atom_charge",
    "chemical_conn_atom_display_x",
    "chemical_conn_atom_display_y",
    "chemical_conn_atom_number",
    "chemical_conn_atom_type_symbol",
    // chemical_conn_bond
    "chemical_conn_bond_atom_1",
    "chemical_conn_bond_atom_2",
    "chemical_conn_bond_type",
    // chemical_formula
    "chemical_formula_analytical",
    "chemical_formula_iupac",
    "chemical_formula_moiety",
    "chemical_formula_structural",
    "chemical_formula_sum",
    "chemical_formula_weight",
    "chemical_formula_weight_meas",
    // geom
    "geom_special_details",
    // geom_angle
    "geom_angle",
    "geom_angle_atom_site_label_1",
    "geom_angle_atom_site_label_2",
    "geom_angle_atom_site_label_3",
    "geom_angle_publ_flag",
    "geom_angle_site_symmetry_1",
    "geom_angle_site_symmetry_2",
    "geom_angle_site_symmetry_3",
    // geom_bond
    "geom_bond_atom_site_label_1",
    "geom_bond_atom_site_label_2",
    "geom_bond_distance",
    "geom_bond_multiplicity",
    "geom_bond_publ_flag",
    "geom_bond_site_symmetry_1",
    "geom_bond_site_symmetry_2",
    "geom_bond_valence",
    // geom_contact
    "geom_contact_atom_site_label_1",
    "geom_contact_atom_site_label_2",
    "geom_contact_distance",
    "geom_contact_publ_flag",
    "geom_contact_site_symmetry_1",
    "geom_contact_site_symmetry_2",
    // geom_hbond
    "geom_hbond_angle_DHA",
    "geom_hbond_atom_site_label_A",
    "geom_hbond_atom_site_label_D",
    "geom_hbond_atom_site_label_H",
    "geom_hbond_distance_DA",
    "geom_hbond_distance_DH",
    "geom_hbond_distance_HA",
    "geom_hbond_publ_flag",
    "geom_hbond_site_symmetry_A",
    "geom_hbond_site_symmetry_D",
    "geom_hbond_site_symmetry_H",
    // geom_torsion
    "geom_torsion",
    "geom_torsion_atom_site_label_1",
    "geom_torsion_atom_site_label_2",
    "geom_torsion_atom_site_label_3",
    "geom_torsion_atom_site_label_4",
    "geom_torsion_publ_flag",
    "geom_torsion_site_symmetry_1",
    "geom_torsion_site_symmetry_2",
    "geom_torsion_site_symmetry_3",
    "geom_torsion_site_symmetry_4",
    // valence_param
    "valence_param_B",
    "valence_param_Ro",
    "valence_param_atom_1",
    "valence_param_atom_1_valence",
    "valence_param_atom_2",
    "valence_param_atom_2_valence",
    "valence_param_details",
    "valence_param_id",
    "valence_param_ref_id",
    // valence_ref
    "valence_ref_id",
    "valence_ref_reference",
];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn category_sections_are_subsets_of_all() {
        for t in atom_site::TAGS {
            assert!(ALL.contains(t), "{t} missing from ALL");
        }
        for t in audit::TAGS {
            assert!(ALL.contains(t), "{t} missing from ALL");
        }
        for t in cell::TAGS {
            assert!(ALL.contains(t), "{t} missing from ALL");
        }
        for t in cell_measurement_refln::TAGS {
            assert!(ALL.contains(t), "{t} missing from ALL");
        }
        for t in space_group::TAGS {
            assert!(ALL.contains(t), "{t} missing from ALL");
        }
        for t in space_group_symop::TAGS {
            assert!(ALL.contains(t), "{t} missing from ALL");
        }
        for t in atom_sites::TAGS {
            assert!(ALL.contains(t), "{t} missing from ALL");
        }
        for t in atom_type::TAGS {
            assert!(ALL.contains(t), "{t} missing from ALL");
        }
        for t in chemical::TAGS {
            assert!(ALL.contains(t), "{t} missing from ALL");
        }
        for t in chemical_conn_atom::TAGS {
            assert!(ALL.contains(t), "{t} missing from ALL");
        }
        for t in chemical_conn_bond::TAGS {
            assert!(ALL.contains(t), "{t} missing from ALL");
        }
        for t in chemical_formula::TAGS {
            assert!(ALL.contains(t), "{t} missing from ALL");
        }
        for t in geom::TAGS {
            assert!(ALL.contains(t), "{t} missing from ALL");
        }
        for t in geom_angle::TAGS {
            assert!(ALL.contains(t), "{t} missing from ALL");
        }
        for t in geom_bond::TAGS {
            assert!(ALL.contains(t), "{t} missing from ALL");
        }
        for t in geom_contact::TAGS {
            assert!(ALL.contains(t), "{t} missing from ALL");
        }
        for t in geom_hbond::TAGS {
            assert!(ALL.contains(t), "{t} missing from ALL");
        }
        for t in geom_torsion::TAGS {
            assert!(ALL.contains(t), "{t} missing from ALL");
        }
        for t in valence_param::TAGS {
            assert!(ALL.contains(t), "{t} missing from ALL");
        }
        for t in valence_ref::TAGS {
            assert!(ALL.contains(t), "{t} missing from ALL");
        }
    }

    #[test]
    fn registry_counts() {
        assert_eq!(atom_site::TAGS.len(), 54);
        assert_eq!(audit::TAGS.len(), 5);
        assert_eq!(cell::TAGS.len(), 22);
        assert_eq!(cell_measurement_refln::TAGS.len(), 4);
        assert_eq!(space_group::TAGS.len(), 5);
        assert_eq!(space_group_symop::TAGS.len(), 3);
        assert_eq!(atom_sites::TAGS.len(), 29);
        assert_eq!(atom_type::TAGS.len(), 22);
        assert_eq!(chemical::TAGS.len(), 25);
        assert_eq!(chemical_conn_atom::TAGS.len(), 7);
        assert_eq!(chemical_conn_bond::TAGS.len(), 3);
        assert_eq!(chemical_formula::TAGS.len(), 7);
        assert_eq!(geom::TAGS.len(), 1);
        assert_eq!(geom_angle::TAGS.len(), 8);
        assert_eq!(geom_bond::TAGS.len(), 8);
        assert_eq!(geom_contact::TAGS.len(), 6);
        assert_eq!(geom_hbond::TAGS.len(), 11);
        assert_eq!(geom_torsion::TAGS.len(), 10);
        assert_eq!(valence_param::TAGS.len(), 9);
        assert_eq!(valence_ref::TAGS.len(), 2);
        assert_eq!(ALL.len(), 241);
        assert!(ALL.iter().all(|t| !t.starts_with('_')));
        assert_eq!(
            ALL.len(),
            ALL.iter().collect::<std::collections::HashSet<_>>().len()
        );
    }
}
