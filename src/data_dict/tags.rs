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

/// `exptl` category tags (9).
pub mod exptl {
    pub const TAGS: &[&str] = &[
        "exptl_absorpt_coefficient_mu",
        "exptl_absorpt_correction_T_max",
        "exptl_absorpt_correction_T_min",
        "exptl_absorpt_correction_type",
        "exptl_absorpt_process_details",
        "exptl_crystals_number",
        "exptl_special_details",
        "exptl_transmission_factor_max",
        "exptl_transmission_factor_min",
    ];
}

/// `exptl_crystal` category tags (24).
pub mod exptl_crystal {
    pub const TAGS: &[&str] = &[
        "exptl_crystal_F_000",
        "exptl_crystal_colour",
        "exptl_crystal_colour_lustre",
        "exptl_crystal_colour_modifier",
        "exptl_crystal_colour_primary",
        "exptl_crystal_density_diffrn",
        "exptl_crystal_density_meas",
        "exptl_crystal_density_meas_gt",
        "exptl_crystal_density_meas_lt",
        "exptl_crystal_density_meas_temp",
        "exptl_crystal_density_meas_temp_gt",
        "exptl_crystal_density_meas_temp_lt",
        "exptl_crystal_density_method",
        "exptl_crystal_description",
        "exptl_crystal_id",
        "exptl_crystal_preparation",
        "exptl_crystal_pressure_history",
        "exptl_crystal_recrystallization_method",
        "exptl_crystal_size_length",
        "exptl_crystal_size_max",
        "exptl_crystal_size_mid",
        "exptl_crystal_size_min",
        "exptl_crystal_size_rad",
        "exptl_crystal_thermal_history",
    ];
}

/// `exptl_crystal_face` category tags (8).
pub mod exptl_crystal_face {
    pub const TAGS: &[&str] = &[
        "exptl_crystal_face_diffr_chi",
        "exptl_crystal_face_diffr_kappa",
        "exptl_crystal_face_diffr_phi",
        "exptl_crystal_face_diffr_psi",
        "exptl_crystal_face_index_h",
        "exptl_crystal_face_index_k",
        "exptl_crystal_face_index_l",
        "exptl_crystal_face_perp_dist",
    ];
}

/// `refine` category tags (46).
pub mod refine {
    pub const TAGS: &[&str] = &[
        "refine_diff_density_max",
        "refine_diff_density_min",
        "refine_diff_density_rms",
        "refine_ls_F_calc_details",
        "refine_ls_F_calc_formula",
        "refine_ls_F_calc_precision",
        "refine_ls_R_Fsqd_factor",
        "refine_ls_R_I_factor",
        "refine_ls_R_factor_all",
        "refine_ls_R_factor_gt",
        "refine_ls_R_factor_obs",
        "refine_ls_abs_structure_Flack",
        "refine_ls_abs_structure_Rogers",
        "refine_ls_abs_structure_details",
        "refine_ls_d_res_high",
        "refine_ls_d_res_low",
        "refine_ls_extinction_coef",
        "refine_ls_extinction_expression",
        "refine_ls_extinction_method",
        "refine_ls_goodness_of_fit_all",
        "refine_ls_goodness_of_fit_gt",
        "refine_ls_goodness_of_fit_obs",
        "refine_ls_goodness_of_fit_ref",
        "refine_ls_hydrogen_treatment",
        "refine_ls_matrix_type",
        "refine_ls_number_constraints",
        "refine_ls_number_parameters",
        "refine_ls_number_reflns",
        "refine_ls_number_restraints",
        "refine_ls_restrained_S_all",
        "refine_ls_restrained_S_gt",
        "refine_ls_restrained_S_obs",
        "refine_ls_shift/esd_max",
        "refine_ls_shift/esd_mean",
        "refine_ls_shift/su_max",
        "refine_ls_shift/su_max_lt",
        "refine_ls_shift/su_mean",
        "refine_ls_shift/su_mean_lt",
        "refine_ls_structure_factor_coef",
        "refine_ls_wR_factor_all",
        "refine_ls_wR_factor_gt",
        "refine_ls_wR_factor_obs",
        "refine_ls_wR_factor_ref",
        "refine_ls_weighting_details",
        "refine_ls_weighting_scheme",
        "refine_special_details",
    ];
}

/// `refine_ls_class` category tags (8).
pub mod refine_ls_class {
    pub const TAGS: &[&str] = &[
        "refine_ls_class_R_Fsqd_factor",
        "refine_ls_class_R_I_factor",
        "refine_ls_class_R_factor_all",
        "refine_ls_class_R_factor_gt",
        "refine_ls_class_code",
        "refine_ls_class_d_res_high",
        "refine_ls_class_d_res_low",
        "refine_ls_class_wR_factor_all",
    ];
}

/// `refln` category tags (31).
pub mod refln {
    pub const TAGS: &[&str] = &[
        "refln_A_calc",
        "refln_A_meas",
        "refln_B_calc",
        "refln_B_meas",
        "refln_F_calc",
        "refln_F_meas",
        "refln_F_sigma",
        "refln_F_squared_calc",
        "refln_F_squared_meas",
        "refln_F_squared_sigma",
        "refln_class_code",
        "refln_crystal_id",
        "refln_d_spacing",
        "refln_include_status",
        "refln_index_h",
        "refln_index_k",
        "refln_index_l",
        "refln_intensity_calc",
        "refln_intensity_meas",
        "refln_intensity_sigma",
        "refln_mean_path_length_tbar",
        "refln_observed_status",
        "refln_phase_calc",
        "refln_phase_meas",
        "refln_refinement_status",
        "refln_scale_group_code",
        "refln_sint/lambda",
        "refln_symmetry_epsilon",
        "refln_symmetry_multiplicity",
        "refln_wavelength",
        "refln_wavelength_id",
    ];
}

/// `reflns` category tags (17).
pub mod reflns {
    pub const TAGS: &[&str] = &[
        "reflns_Friedel_coverage",
        "reflns_Friedel_fraction_full",
        "reflns_Friedel_fraction_max",
        "reflns_d_resolution_high",
        "reflns_d_resolution_low",
        "reflns_limit_h_max",
        "reflns_limit_h_min",
        "reflns_limit_k_max",
        "reflns_limit_k_min",
        "reflns_limit_l_max",
        "reflns_limit_l_min",
        "reflns_number_gt",
        "reflns_number_observed",
        "reflns_number_total",
        "reflns_observed_criterion",
        "reflns_special_details",
        "reflns_threshold_expression",
    ];
}

/// `reflns_class` category tags (11).
pub mod reflns_class {
    pub const TAGS: &[&str] = &[
        "reflns_class_R_Fsqd_factor",
        "reflns_class_R_I_factor",
        "reflns_class_R_factor_all",
        "reflns_class_R_factor_gt",
        "reflns_class_code",
        "reflns_class_d_res_high",
        "reflns_class_d_res_low",
        "reflns_class_description",
        "reflns_class_number_gt",
        "reflns_class_number_total",
        "reflns_class_wR_factor_all",
    ];
}

/// `reflns_scale` category tags (4).
pub mod reflns_scale {
    pub const TAGS: &[&str] = &[
        "reflns_scale_group_code",
        "reflns_scale_meas_F",
        "reflns_scale_meas_F_squared",
        "reflns_scale_meas_intensity",
    ];
}

/// `reflns_shell` category tags (23).
pub mod reflns_shell {
    pub const TAGS: &[&str] = &[
        "reflns_shell_Rmerge_F_all",
        "reflns_shell_Rmerge_F_gt",
        "reflns_shell_Rmerge_F_obs",
        "reflns_shell_Rmerge_I_all",
        "reflns_shell_Rmerge_I_gt",
        "reflns_shell_Rmerge_I_obs",
        "reflns_shell_d_res_high",
        "reflns_shell_d_res_low",
        "reflns_shell_meanI_over_sigI_all",
        "reflns_shell_meanI_over_sigI_gt",
        "reflns_shell_meanI_over_sigI_obs",
        "reflns_shell_meanI_over_uI_all",
        "reflns_shell_meanI_over_uI_gt",
        "reflns_shell_number_measured_all",
        "reflns_shell_number_measured_gt",
        "reflns_shell_number_measured_obs",
        "reflns_shell_number_possible",
        "reflns_shell_number_unique_all",
        "reflns_shell_number_unique_gt",
        "reflns_shell_number_unique_obs",
        "reflns_shell_percent_possible_all",
        "reflns_shell_percent_possible_gt",
        "reflns_shell_percent_possible_obs",
    ];
}

/// `symmetry` category tags (4).
pub mod symmetry {
    pub const TAGS: &[&str] = &[
        "symmetry_Int_Tables_number",
        "symmetry_cell_setting",
        "symmetry_space_group_name_H-M",
        "symmetry_space_group_name_Hall",
    ];
}

/// `symmetry_equiv` category tags (2).
pub mod symmetry_equiv {
    pub const TAGS: &[&str] = &["symmetry_equiv_pos_as_xyz", "symmetry_equiv_pos_site_id"];
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
    // exptl
    "exptl_absorpt_coefficient_mu",
    "exptl_absorpt_correction_T_max",
    "exptl_absorpt_correction_T_min",
    "exptl_absorpt_correction_type",
    "exptl_absorpt_process_details",
    "exptl_crystals_number",
    "exptl_special_details",
    "exptl_transmission_factor_max",
    "exptl_transmission_factor_min",
    // exptl_crystal
    "exptl_crystal_F_000",
    "exptl_crystal_colour",
    "exptl_crystal_colour_lustre",
    "exptl_crystal_colour_modifier",
    "exptl_crystal_colour_primary",
    "exptl_crystal_density_diffrn",
    "exptl_crystal_density_meas",
    "exptl_crystal_density_meas_gt",
    "exptl_crystal_density_meas_lt",
    "exptl_crystal_density_meas_temp",
    "exptl_crystal_density_meas_temp_gt",
    "exptl_crystal_density_meas_temp_lt",
    "exptl_crystal_density_method",
    "exptl_crystal_description",
    "exptl_crystal_id",
    "exptl_crystal_preparation",
    "exptl_crystal_pressure_history",
    "exptl_crystal_recrystallization_method",
    "exptl_crystal_size_length",
    "exptl_crystal_size_max",
    "exptl_crystal_size_mid",
    "exptl_crystal_size_min",
    "exptl_crystal_size_rad",
    "exptl_crystal_thermal_history",
    // exptl_crystal_face
    "exptl_crystal_face_diffr_chi",
    "exptl_crystal_face_diffr_kappa",
    "exptl_crystal_face_diffr_phi",
    "exptl_crystal_face_diffr_psi",
    "exptl_crystal_face_index_h",
    "exptl_crystal_face_index_k",
    "exptl_crystal_face_index_l",
    "exptl_crystal_face_perp_dist",
    // refine
    "refine_diff_density_max",
    "refine_diff_density_min",
    "refine_diff_density_rms",
    "refine_ls_F_calc_details",
    "refine_ls_F_calc_formula",
    "refine_ls_F_calc_precision",
    "refine_ls_R_Fsqd_factor",
    "refine_ls_R_I_factor",
    "refine_ls_R_factor_all",
    "refine_ls_R_factor_gt",
    "refine_ls_R_factor_obs",
    "refine_ls_abs_structure_Flack",
    "refine_ls_abs_structure_Rogers",
    "refine_ls_abs_structure_details",
    "refine_ls_d_res_high",
    "refine_ls_d_res_low",
    "refine_ls_extinction_coef",
    "refine_ls_extinction_expression",
    "refine_ls_extinction_method",
    "refine_ls_goodness_of_fit_all",
    "refine_ls_goodness_of_fit_gt",
    "refine_ls_goodness_of_fit_obs",
    "refine_ls_goodness_of_fit_ref",
    "refine_ls_hydrogen_treatment",
    "refine_ls_matrix_type",
    "refine_ls_number_constraints",
    "refine_ls_number_parameters",
    "refine_ls_number_reflns",
    "refine_ls_number_restraints",
    "refine_ls_restrained_S_all",
    "refine_ls_restrained_S_gt",
    "refine_ls_restrained_S_obs",
    "refine_ls_shift/esd_max",
    "refine_ls_shift/esd_mean",
    "refine_ls_shift/su_max",
    "refine_ls_shift/su_max_lt",
    "refine_ls_shift/su_mean",
    "refine_ls_shift/su_mean_lt",
    "refine_ls_structure_factor_coef",
    "refine_ls_wR_factor_all",
    "refine_ls_wR_factor_gt",
    "refine_ls_wR_factor_obs",
    "refine_ls_wR_factor_ref",
    "refine_ls_weighting_details",
    "refine_ls_weighting_scheme",
    "refine_special_details",
    // refine_ls_class
    "refine_ls_class_R_Fsqd_factor",
    "refine_ls_class_R_I_factor",
    "refine_ls_class_R_factor_all",
    "refine_ls_class_R_factor_gt",
    "refine_ls_class_code",
    "refine_ls_class_d_res_high",
    "refine_ls_class_d_res_low",
    "refine_ls_class_wR_factor_all",
    // refln
    "refln_A_calc",
    "refln_A_meas",
    "refln_B_calc",
    "refln_B_meas",
    "refln_F_calc",
    "refln_F_meas",
    "refln_F_sigma",
    "refln_F_squared_calc",
    "refln_F_squared_meas",
    "refln_F_squared_sigma",
    "refln_class_code",
    "refln_crystal_id",
    "refln_d_spacing",
    "refln_include_status",
    "refln_index_h",
    "refln_index_k",
    "refln_index_l",
    "refln_intensity_calc",
    "refln_intensity_meas",
    "refln_intensity_sigma",
    "refln_mean_path_length_tbar",
    "refln_observed_status",
    "refln_phase_calc",
    "refln_phase_meas",
    "refln_refinement_status",
    "refln_scale_group_code",
    "refln_sint/lambda",
    "refln_symmetry_epsilon",
    "refln_symmetry_multiplicity",
    "refln_wavelength",
    "refln_wavelength_id",
    // reflns
    "reflns_Friedel_coverage",
    "reflns_Friedel_fraction_full",
    "reflns_Friedel_fraction_max",
    "reflns_d_resolution_high",
    "reflns_d_resolution_low",
    "reflns_limit_h_max",
    "reflns_limit_h_min",
    "reflns_limit_k_max",
    "reflns_limit_k_min",
    "reflns_limit_l_max",
    "reflns_limit_l_min",
    "reflns_number_gt",
    "reflns_number_observed",
    "reflns_number_total",
    "reflns_observed_criterion",
    "reflns_special_details",
    "reflns_threshold_expression",
    // reflns_class
    "reflns_class_R_Fsqd_factor",
    "reflns_class_R_I_factor",
    "reflns_class_R_factor_all",
    "reflns_class_R_factor_gt",
    "reflns_class_code",
    "reflns_class_d_res_high",
    "reflns_class_d_res_low",
    "reflns_class_description",
    "reflns_class_number_gt",
    "reflns_class_number_total",
    "reflns_class_wR_factor_all",
    // reflns_scale
    "reflns_scale_group_code",
    "reflns_scale_meas_F",
    "reflns_scale_meas_F_squared",
    "reflns_scale_meas_intensity",
    // reflns_shell
    "reflns_shell_Rmerge_F_all",
    "reflns_shell_Rmerge_F_gt",
    "reflns_shell_Rmerge_F_obs",
    "reflns_shell_Rmerge_I_all",
    "reflns_shell_Rmerge_I_gt",
    "reflns_shell_Rmerge_I_obs",
    "reflns_shell_d_res_high",
    "reflns_shell_d_res_low",
    "reflns_shell_meanI_over_sigI_all",
    "reflns_shell_meanI_over_sigI_gt",
    "reflns_shell_meanI_over_sigI_obs",
    "reflns_shell_meanI_over_uI_all",
    "reflns_shell_meanI_over_uI_gt",
    "reflns_shell_number_measured_all",
    "reflns_shell_number_measured_gt",
    "reflns_shell_number_measured_obs",
    "reflns_shell_number_possible",
    "reflns_shell_number_unique_all",
    "reflns_shell_number_unique_gt",
    "reflns_shell_number_unique_obs",
    "reflns_shell_percent_possible_all",
    "reflns_shell_percent_possible_gt",
    "reflns_shell_percent_possible_obs",
    // symmetry
    "symmetry_Int_Tables_number",
    "symmetry_cell_setting",
    "symmetry_space_group_name_H-M",
    "symmetry_space_group_name_Hall",
    // symmetry_equiv
    "symmetry_equiv_pos_as_xyz",
    "symmetry_equiv_pos_site_id",
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
        for t in exptl::TAGS {
            assert!(ALL.contains(t), "{t} missing from ALL");
        }
        for t in exptl_crystal::TAGS {
            assert!(ALL.contains(t), "{t} missing from ALL");
        }
        for t in exptl_crystal_face::TAGS {
            assert!(ALL.contains(t), "{t} missing from ALL");
        }
        for t in refine::TAGS {
            assert!(ALL.contains(t), "{t} missing from ALL");
        }
        for t in refine_ls_class::TAGS {
            assert!(ALL.contains(t), "{t} missing from ALL");
        }
        for t in refln::TAGS {
            assert!(ALL.contains(t), "{t} missing from ALL");
        }
        for t in reflns::TAGS {
            assert!(ALL.contains(t), "{t} missing from ALL");
        }
        for t in reflns_class::TAGS {
            assert!(ALL.contains(t), "{t} missing from ALL");
        }
        for t in reflns_scale::TAGS {
            assert!(ALL.contains(t), "{t} missing from ALL");
        }
        for t in reflns_shell::TAGS {
            assert!(ALL.contains(t), "{t} missing from ALL");
        }
        for t in symmetry::TAGS {
            assert!(ALL.contains(t), "{t} missing from ALL");
        }
        for t in symmetry_equiv::TAGS {
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
        assert_eq!(exptl::TAGS.len(), 9);
        assert_eq!(exptl_crystal::TAGS.len(), 24);
        assert_eq!(exptl_crystal_face::TAGS.len(), 8);
        assert_eq!(refine::TAGS.len(), 46);
        assert_eq!(refine_ls_class::TAGS.len(), 8);
        assert_eq!(refln::TAGS.len(), 31);
        assert_eq!(reflns::TAGS.len(), 17);
        assert_eq!(reflns_class::TAGS.len(), 11);
        assert_eq!(reflns_scale::TAGS.len(), 4);
        assert_eq!(reflns_shell::TAGS.len(), 23);
        assert_eq!(symmetry::TAGS.len(), 4);
        assert_eq!(symmetry_equiv::TAGS.len(), 2);
        assert_eq!(ALL.len(), 280);
        assert!(ALL.iter().all(|t| !t.starts_with('_')));
        assert_eq!(
            ALL.len(),
            ALL.iter().collect::<std::collections::HashSet<_>>().len()
        );
    }
}
