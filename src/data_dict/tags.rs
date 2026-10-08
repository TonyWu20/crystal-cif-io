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

/// `audit_author` category tags (2).
pub mod audit_author {
    pub const TAGS: &[&str] = &["audit_author_address", "audit_author_name"];
}

/// `audit_conform` category tags (3).
pub mod audit_conform {
    pub const TAGS: &[&str] = &[
        "audit_conform_dict_location",
        "audit_conform_dict_name",
        "audit_conform_dict_version",
    ];
}

/// `audit_contact_author` category tags (5).
pub mod audit_contact_author {
    pub const TAGS: &[&str] = &[
        "audit_contact_author_address",
        "audit_contact_author_email",
        "audit_contact_author_fax",
        "audit_contact_author_name",
        "audit_contact_author_phone",
    ];
}

/// `audit_link` category tags (2).
pub mod audit_link {
    pub const TAGS: &[&str] = &["audit_link_block_code", "audit_link_block_description"];
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

/// `citation` category tags (26).
pub mod citation {
    pub const TAGS: &[&str] = &[
        "citation_abstract",
        "citation_abstract_id_cas",
        "citation_book_id_isbn",
        "citation_book_publisher",
        "citation_book_publisher_city",
        "citation_book_title",
        "citation_coordinate_linkage",
        "citation_country",
        "citation_database_id_csd",
        "citation_database_id_medline",
        "citation_doi",
        "citation_id",
        "citation_journal_abbrev",
        "citation_journal_full",
        "citation_journal_id_astm",
        "citation_journal_id_csd",
        "citation_journal_id_issn",
        "citation_journal_issue",
        "citation_journal_volume",
        "citation_language",
        "citation_page_first",
        "citation_page_last",
        "citation_publisher",
        "citation_special_details",
        "citation_title",
        "citation_year",
    ];
}

/// `citation_author` category tags (3).
pub mod citation_author {
    pub const TAGS: &[&str] = &[
        "citation_author_citation_id",
        "citation_author_name",
        "citation_author_ordinal",
    ];
}

/// `citation_editor` category tags (3).
pub mod citation_editor {
    pub const TAGS: &[&str] = &[
        "citation_editor_citation_id",
        "citation_editor_name",
        "citation_editor_ordinal",
    ];
}

/// `computing` category tags (7).
pub mod computing {
    pub const TAGS: &[&str] = &[
        "computing_cell_refinement",
        "computing_data_collection",
        "computing_data_reduction",
        "computing_molecular_graphics",
        "computing_publication_material",
        "computing_structure_refinement",
        "computing_structure_solution",
    ];
}

/// `database` category tags (15).
pub mod database {
    pub const TAGS: &[&str] = &[
        "database_code_cas",
        "database_code_cod",
        "database_code_csd",
        "database_code_depnum_ccdc_archive",
        "database_code_depnum_ccdc_fiz",
        "database_code_depnum_ccdc_journal",
        "database_code_icsd",
        "database_code_mdf",
        "database_code_nbs",
        "database_code_pdb",
        "database_code_pdf",
        "database_csd_history",
        "database_dataset_doi",
        "database_journal_astm",
        "database_journal_csd",
    ];
}

/// `journal` category tags (38).
pub mod journal {
    pub const TAGS: &[&str] = &[
        "journal_coden_astm",
        "journal_coden_cambridge",
        "journal_coeditor_address",
        "journal_coeditor_code",
        "journal_coeditor_email",
        "journal_coeditor_fax",
        "journal_coeditor_name",
        "journal_coeditor_notes",
        "journal_coeditor_phone",
        "journal_data_validation_number",
        "journal_date_accepted",
        "journal_date_from_coeditor",
        "journal_date_printers_final",
        "journal_date_printers_first",
        "journal_date_proofs_in",
        "journal_date_proofs_out",
        "journal_date_recd_copyright",
        "journal_date_recd_electronic",
        "journal_date_recd_hard_copy",
        "journal_date_to_coeditor",
        "journal_issue",
        "journal_language",
        "journal_name_full",
        "journal_page_first",
        "journal_page_last",
        "journal_paper_category",
        "journal_paper_doi",
        "journal_suppl_publ_number",
        "journal_suppl_publ_pages",
        "journal_techeditor_address",
        "journal_techeditor_code",
        "journal_techeditor_email",
        "journal_techeditor_fax",
        "journal_techeditor_name",
        "journal_techeditor_notes",
        "journal_techeditor_phone",
        "journal_volume",
        "journal_year",
    ];
}

/// `journal_index` category tags (3).
pub mod journal_index {
    pub const TAGS: &[&str] = &[
        "journal_index_subterm",
        "journal_index_term",
        "journal_index_type",
    ];
}

/// `publ` category tags (32).
pub mod publ {
    pub const TAGS: &[&str] = &[
        "publ_contact_author",
        "publ_contact_author_address",
        "publ_contact_author_email",
        "publ_contact_author_fax",
        "publ_contact_author_id_iucr",
        "publ_contact_author_id_orcid",
        "publ_contact_author_name",
        "publ_contact_author_phone",
        "publ_contact_letter",
        "publ_manuscript_creation",
        "publ_manuscript_processed",
        "publ_manuscript_text",
        "publ_requested_category",
        "publ_requested_coeditor_name",
        "publ_requested_journal",
        "publ_section_abstract",
        "publ_section_acknowledgements",
        "publ_section_comment",
        "publ_section_discussion",
        "publ_section_experimental",
        "publ_section_exptl_prep",
        "publ_section_exptl_refinement",
        "publ_section_exptl_solution",
        "publ_section_figure_captions",
        "publ_section_introduction",
        "publ_section_keywords",
        "publ_section_references",
        "publ_section_related_literature",
        "publ_section_synopsis",
        "publ_section_table_legends",
        "publ_section_title",
        "publ_section_title_footnote",
    ];
}

/// `publ_author` category tags (6).
pub mod publ_author {
    pub const TAGS: &[&str] = &[
        "publ_author_address",
        "publ_author_email",
        "publ_author_footnote",
        "publ_author_id_iucr",
        "publ_author_id_orcid",
        "publ_author_name",
    ];
}

/// `publ_body` category tags (5).
pub mod publ_body {
    pub const TAGS: &[&str] = &[
        "publ_body_contents",
        "publ_body_element",
        "publ_body_format",
        "publ_body_label",
        "publ_body_title",
    ];
}

/// `publ_manuscript_incl` category tags (3).
pub mod publ_manuscript_incl {
    pub const TAGS: &[&str] = &[
        "publ_manuscript_incl_extra_defn",
        "publ_manuscript_incl_extra_info",
        "publ_manuscript_incl_extra_item",
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
    // audit_author
    "audit_author_address",
    "audit_author_name",
    // audit_conform
    "audit_conform_dict_location",
    "audit_conform_dict_name",
    "audit_conform_dict_version",
    // audit_contact_author
    "audit_contact_author_address",
    "audit_contact_author_email",
    "audit_contact_author_fax",
    "audit_contact_author_name",
    "audit_contact_author_phone",
    // audit_link
    "audit_link_block_code",
    "audit_link_block_description",
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
    // citation
    "citation_abstract",
    "citation_abstract_id_cas",
    "citation_book_id_isbn",
    "citation_book_publisher",
    "citation_book_publisher_city",
    "citation_book_title",
    "citation_coordinate_linkage",
    "citation_country",
    "citation_database_id_csd",
    "citation_database_id_medline",
    "citation_doi",
    "citation_id",
    "citation_journal_abbrev",
    "citation_journal_full",
    "citation_journal_id_astm",
    "citation_journal_id_csd",
    "citation_journal_id_issn",
    "citation_journal_issue",
    "citation_journal_volume",
    "citation_language",
    "citation_page_first",
    "citation_page_last",
    "citation_publisher",
    "citation_special_details",
    "citation_title",
    "citation_year",
    // citation_author
    "citation_author_citation_id",
    "citation_author_name",
    "citation_author_ordinal",
    // citation_editor
    "citation_editor_citation_id",
    "citation_editor_name",
    "citation_editor_ordinal",
    // computing
    "computing_cell_refinement",
    "computing_data_collection",
    "computing_data_reduction",
    "computing_molecular_graphics",
    "computing_publication_material",
    "computing_structure_refinement",
    "computing_structure_solution",
    // database
    "database_code_cas",
    "database_code_cod",
    "database_code_csd",
    "database_code_depnum_ccdc_archive",
    "database_code_depnum_ccdc_fiz",
    "database_code_depnum_ccdc_journal",
    "database_code_icsd",
    "database_code_mdf",
    "database_code_nbs",
    "database_code_pdb",
    "database_code_pdf",
    "database_csd_history",
    "database_dataset_doi",
    "database_journal_astm",
    "database_journal_csd",
    // journal
    "journal_coden_astm",
    "journal_coden_cambridge",
    "journal_coeditor_address",
    "journal_coeditor_code",
    "journal_coeditor_email",
    "journal_coeditor_fax",
    "journal_coeditor_name",
    "journal_coeditor_notes",
    "journal_coeditor_phone",
    "journal_data_validation_number",
    "journal_date_accepted",
    "journal_date_from_coeditor",
    "journal_date_printers_final",
    "journal_date_printers_first",
    "journal_date_proofs_in",
    "journal_date_proofs_out",
    "journal_date_recd_copyright",
    "journal_date_recd_electronic",
    "journal_date_recd_hard_copy",
    "journal_date_to_coeditor",
    "journal_issue",
    "journal_language",
    "journal_name_full",
    "journal_page_first",
    "journal_page_last",
    "journal_paper_category",
    "journal_paper_doi",
    "journal_suppl_publ_number",
    "journal_suppl_publ_pages",
    "journal_techeditor_address",
    "journal_techeditor_code",
    "journal_techeditor_email",
    "journal_techeditor_fax",
    "journal_techeditor_name",
    "journal_techeditor_notes",
    "journal_techeditor_phone",
    "journal_volume",
    "journal_year",
    // journal_index
    "journal_index_subterm",
    "journal_index_term",
    "journal_index_type",
    // publ
    "publ_contact_author",
    "publ_contact_author_address",
    "publ_contact_author_email",
    "publ_contact_author_fax",
    "publ_contact_author_id_iucr",
    "publ_contact_author_id_orcid",
    "publ_contact_author_name",
    "publ_contact_author_phone",
    "publ_contact_letter",
    "publ_manuscript_creation",
    "publ_manuscript_processed",
    "publ_manuscript_text",
    "publ_requested_category",
    "publ_requested_coeditor_name",
    "publ_requested_journal",
    "publ_section_abstract",
    "publ_section_acknowledgements",
    "publ_section_comment",
    "publ_section_discussion",
    "publ_section_experimental",
    "publ_section_exptl_prep",
    "publ_section_exptl_refinement",
    "publ_section_exptl_solution",
    "publ_section_figure_captions",
    "publ_section_introduction",
    "publ_section_keywords",
    "publ_section_references",
    "publ_section_related_literature",
    "publ_section_synopsis",
    "publ_section_table_legends",
    "publ_section_title",
    "publ_section_title_footnote",
    // publ_author
    "publ_author_address",
    "publ_author_email",
    "publ_author_footnote",
    "publ_author_id_iucr",
    "publ_author_id_orcid",
    "publ_author_name",
    // publ_body
    "publ_body_contents",
    "publ_body_element",
    "publ_body_format",
    "publ_body_label",
    "publ_body_title",
    // publ_manuscript_incl
    "publ_manuscript_incl_extra_defn",
    "publ_manuscript_incl_extra_info",
    "publ_manuscript_incl_extra_item",
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
        for t in audit_author::TAGS {
            assert!(ALL.contains(t), "{t} missing from ALL");
        }
        for t in audit_conform::TAGS {
            assert!(ALL.contains(t), "{t} missing from ALL");
        }
        for t in audit_contact_author::TAGS {
            assert!(ALL.contains(t), "{t} missing from ALL");
        }
        for t in audit_link::TAGS {
            assert!(ALL.contains(t), "{t} missing from ALL");
        }
        for t in cell::TAGS {
            assert!(ALL.contains(t), "{t} missing from ALL");
        }
        for t in cell_measurement_refln::TAGS {
            assert!(ALL.contains(t), "{t} missing from ALL");
        }
        for t in citation::TAGS {
            assert!(ALL.contains(t), "{t} missing from ALL");
        }
        for t in citation_author::TAGS {
            assert!(ALL.contains(t), "{t} missing from ALL");
        }
        for t in citation_editor::TAGS {
            assert!(ALL.contains(t), "{t} missing from ALL");
        }
        for t in computing::TAGS {
            assert!(ALL.contains(t), "{t} missing from ALL");
        }
        for t in database::TAGS {
            assert!(ALL.contains(t), "{t} missing from ALL");
        }
        for t in journal::TAGS {
            assert!(ALL.contains(t), "{t} missing from ALL");
        }
        for t in journal_index::TAGS {
            assert!(ALL.contains(t), "{t} missing from ALL");
        }
        for t in publ::TAGS {
            assert!(ALL.contains(t), "{t} missing from ALL");
        }
        for t in publ_author::TAGS {
            assert!(ALL.contains(t), "{t} missing from ALL");
        }
        for t in publ_body::TAGS {
            assert!(ALL.contains(t), "{t} missing from ALL");
        }
        for t in publ_manuscript_incl::TAGS {
            assert!(ALL.contains(t), "{t} missing from ALL");
        }
        for t in space_group::TAGS {
            assert!(ALL.contains(t), "{t} missing from ALL");
        }
        for t in space_group_symop::TAGS {
            assert!(ALL.contains(t), "{t} missing from ALL");
        }
    }

    #[test]
    fn registry_counts() {
        assert_eq!(atom_site::TAGS.len(), 54);
        assert_eq!(audit::TAGS.len(), 5);
        assert_eq!(audit_author::TAGS.len(), 2);
        assert_eq!(audit_conform::TAGS.len(), 3);
        assert_eq!(audit_contact_author::TAGS.len(), 5);
        assert_eq!(audit_link::TAGS.len(), 2);
        assert_eq!(cell::TAGS.len(), 22);
        assert_eq!(cell_measurement_refln::TAGS.len(), 4);
        assert_eq!(citation::TAGS.len(), 26);
        assert_eq!(citation_author::TAGS.len(), 3);
        assert_eq!(citation_editor::TAGS.len(), 3);
        assert_eq!(computing::TAGS.len(), 7);
        assert_eq!(database::TAGS.len(), 15);
        assert_eq!(journal::TAGS.len(), 38);
        assert_eq!(journal_index::TAGS.len(), 3);
        assert_eq!(publ::TAGS.len(), 32);
        assert_eq!(publ_author::TAGS.len(), 6);
        assert_eq!(publ_body::TAGS.len(), 5);
        assert_eq!(publ_manuscript_incl::TAGS.len(), 3);
        assert_eq!(space_group::TAGS.len(), 5);
        assert_eq!(space_group_symop::TAGS.len(), 3);
        assert_eq!(ALL.len(), 246);
        assert!(ALL.iter().all(|t| !t.starts_with('_')));
        assert_eq!(
            ALL.len(),
            ALL.iter().collect::<std::collections::HashSet<_>>().len()
        );
    }
}
