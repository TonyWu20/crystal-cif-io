//! Scratch: the read / edit / write snippets of README sections 2-5.
//! Every import here names `crystal_cif_io` only. The re-exported IR
//! types of `crystal-cif-fmt` and `group::SpaceGroup` carry the
//! workflow. The error paths show what the typed parse reports.

fn main() {
    // 2. Reading a data block: the typed document
    use crystal_cif_io::{parse_core_cif, CoreCifParseError};

    let source =
        std::fs::read_to_string("crystal_cif_fmt/tests/real_data/cod/1000000.cif").unwrap();
    let mut doc = parse_core_cif(&source).expect("the file parses and types");
    println!("{} block(s), {:?} dialect", doc.len(), doc.dialect);
    assert!(doc.comments.is_some(), "the leading comment lines are carried");
    let cif = doc.block("1000000").expect("the block is present");
    println!("block {}, {}", cif.heading.name(), cif.cell.length_a.as_ref().expect("a length a"));
    match cif.validate() {
        Ok(()) => println!("the required sets are complete"),
        Err(gaps) => {
            println!("{} gaps", gaps.len());
        }
    }
    use crystal_cif_io::data_dict::Ineq;
    println!("{}", cif.exptl.absorpt_correction_type.as_ref().expect("a correction type"));
    println!("{}", cif.refine.ls_hydrogen_treatment.as_ref().expect("a treatment"));
    match cif.refine.ls_shift_su_max {
        Some(Ineq::LessThan(bound)) => println!("su_max < {bound}"),
        Some(Ineq::Value(value)) => println!("su_max = {value}"),
        None => {}
    }

    // 3. Editing the typed block: a scoped mutable borrow of the document
    {
        use crystal_cif_io::{CharString, Numeric, UnquotedString};
        use crystal_cif_io::data_dict::core_cif::geom_angle::GeomAngleRow;
        use crystal_cif_io::data_dict::core_cif::refine::{MatrixType, NonNeg};

        fn unquoted(s: &str) -> CharString {
            CharString::Unquoted(UnquotedString::new(s.to_string()))
        }

        let cif = doc.block_mut("1000000").expect("the block is present");
        cif.chemical_formula.structural = cif.chemical_formula.sum.clone();
        cif.refine.ls_matrix_type = Some(MatrixType::Diagonal);
        cif.refine.ls_shift_su_max =
            Some(Ineq::LessThan(NonNeg::from_numeric(Numeric::from(0.001))));
        let mut row = GeomAngleRow::default();
        row.atom_site_label_1 = Some(unquoted("P1"));
        row.atom_site_label_2 = Some(unquoted("Al1"));
        row.atom_site_label_3 = Some(unquoted("P2"));
        row.angle = Some(Numeric::from(51.2));
        cif.geom_angle.rows.push(row);
    }

    // 4. Writing the block back: the document write step
    let cif = doc.block("1000000").expect("the block is present");
    match cif.validate() {
        Ok(()) => println!("the required sets are complete"),
        Err(gaps) => {
            println!("{} gaps stay", gaps.len());
        }
    }
    let out_text = doc.to_cif();
    assert!(out_text.contains("diagonal"), "the edited matrix type writes");
    assert!(out_text.contains("<0.001"), "the edited assertion writes");
    assert!(out_text.contains("geom_angle"), "the pushed row writes");
    assert!(out_text.contains("data_1000000"), "the recorded heading writes");
    println!("the document re-writes with the edits");

    // 5. The builder path: the re-exported group type
    use crystal_cif_io::group::SpaceGroup as Resolved;
    use crystal_cif_io::data_dict::core_cif::space_group_symop::Symop;

    let symops = Symop::builder()
        .group(Resolved::try_from_str("-P 2ybc").expect("IT 14 resolves"))
        .build()
        .expect("the rows build");
    println!(
        "{} symops, first {}",
        symops.len(),
        symops[0].operation_xyz.as_ref().expect("an xyz").content()
    );

    // The error paths of the typed parse
    match parse_core_cif("data_1\n_cell_length_a [ 7.0 ]\n") {
        Ok(_) => panic!("the 1.1 value rule must reject the list"),
        Err(CoreCifParseError::Syntax { reports }) => {
            println!("{} syntax report(s)", reports.len());
            println!("{}", reports[0].lines().next().expect("a report line"));
        }
        Err(CoreCifParseError::Typed { .. }) => {
            panic!("a syntax failure is not a typing failure")
        }
    }
    let broken = source.replacen("90.00", "'90'", 1);
    match parse_core_cif(&broken) {
        Ok(_) => panic!("the value-class mismatch must report"),
        Err(CoreCifParseError::Syntax { .. }) => {
            panic!("the char string parses in the IR")
        }
        Err(CoreCifParseError::Typed { failures }) => {
            println!(
                "block {}: {} error(s)",
                failures[0].block,
                failures[0].errors.len()
            );
            println!("{}", failures[0].errors[0]);
        }
    }
}
