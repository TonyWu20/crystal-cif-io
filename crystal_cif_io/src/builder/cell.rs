//! The `cell` category builders.
//!
//! Two builders over the two minimum entry sets (decision D11).
//! `CellBuilder` takes the direct six: the three lengths and the
//! three angles. `CellVectorsBuilder` takes the three lattice
//! vectors. The derived items (`volume`, the reciprocal items,
//! `orthogonal_matrix`, `metric_tensor`) compute at build. A
//! supplied derived value cross-checks within the relative
//! tolerance. A mismatch rejects with a `Result`, never a panic.
//!
//! The optional recorded items of the spec table emit as given.
//! `formula_units_Z` is not cross-checked here. The structure-level
//! computation (`formula_units`) derives and checks `Z`.

// The dictionary item `formula_units_Z` keeps the capital Z. The
// builder parameters and the `bon`-generated setters keep the item
// spelling, so the non-`snake_case` lint is allowed for the module.
#![allow(non_snake_case)]

use bon::bon;

use crystal_cif_fmt::DataItems;
use crate::data_dict::core_cif::cell::CellAngle;
use crate::data_dict::CifTerm;
use crate::data_dict::SingleValueTerm;
use crate::data_dict::core_cif::cell::CellTerms;
use crate::data_dict::core_cif::range_check::number_as_f64;
use crystal_cif_fmt::grammar::{
    CharString, Float, Matrix, Number, Numeric, UnsignedInteger,
};

/// The default relative tolerance of the derived-value cross-check.
pub const DEFAULT_TOLERANCE: f64 = 1e-3;

/// A build failure of the `cell` builders.
#[derive(Debug, Clone, PartialEq)]
pub enum CellError {
    /// A supplied derived value misses the computed value beyond
    /// the relative tolerance.
    Mismatch {
        item: &'static str,
        supplied: String,
        computed: String,
        tolerance: f64,
    },
    /// A supplied matrix or vector carries the wrong shape.
    BadShape {
        item: &'static str,
        expected: &'static str,
        actual: (usize, usize),
    },
    /// The input describes no lattice: a zero length, a zero
    /// volume, or a zero sine of an angle.
    Degenerate(String),
}

impl std::fmt::Display for CellError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Mismatch {
                item,
                supplied,
                computed,
                tolerance,
            } => write!(
                f,
                "{item}: the supplied {supplied} misses the computed {computed} beyond the {tolerance} tolerance"
            ),
            Self::BadShape {
                item,
                expected,
                actual,
            } => write!(
                f,
                "{item}: the shape is {} x {}, the item expects {expected}",
                actual.0, actual.1
            ),
            Self::Degenerate(message) => write!(f, "cell: {message}"),
        }
    }
}

impl std::error::Error for CellError {}

/// The materialized `cell` category.
#[derive(Debug, Clone)]
pub struct Cell {
    length_a: Numeric,
    length_b: Numeric,
    length_c: Numeric,
    angle_alpha: CellAngle,
    angle_beta: CellAngle,
    angle_gamma: CellAngle,
    volume: Numeric,
    vector_a: Matrix,
    vector_b: Matrix,
    vector_c: Matrix,
    orthogonal_matrix: Matrix,
    metric_tensor: Matrix,
    reciprocal_vector_a: Matrix,
    reciprocal_vector_b: Matrix,
    reciprocal_vector_c: Matrix,
    reciprocal_length_a: Numeric,
    reciprocal_length_b: Numeric,
    reciprocal_length_c: Numeric,
    reciprocal_angle_alpha: CellAngle,
    reciprocal_angle_beta: CellAngle,
    reciprocal_angle_gamma: CellAngle,
    reciprocal_metric_tensor: Matrix,
    reciprocal_orthogonal_matrix: Matrix,
    terms: Vec<CellTerms>,
}

impl Cell {
    /// The optional recorded items, materialized as single-value
    /// terms. Use `terms` for the full write set.
    pub fn recorded_items(&self) -> Vec<CellTerms> {
        const KEPT: &[&str] = &[
            "cell_formula_units_z",
            "cell_formula_units_Z_prime",
            "cell_formula_units_Z_details",
            "cell_formula_units_Z_prime_details",
            "cell_special_details",
            "cell_measurement_reflns_used",
            "cell_measurement_temperature",
            "cell_measurement_wavelength",
            "cell_measurement_pressure",
            "cell_measurement_radiation",
            "cell_measurement_theta_min",
            "cell_measurement_theta_max",
        ];
        self.terms
            .iter()
            .filter(|t| KEPT.contains(&t.tag().as_ref()))
            .cloned()
            .collect()
    }
}

impl Cell {
    pub fn length_a(&self) -> &Numeric {
        &self.length_a
    }

    pub fn length_b(&self) -> &Numeric {
        &self.length_b
    }

    pub fn length_c(&self) -> &Numeric {
        &self.length_c
    }

    pub fn angle_alpha(&self) -> &CellAngle {
        &self.angle_alpha
    }

    pub fn angle_beta(&self) -> &CellAngle {
        &self.angle_beta
    }

    pub fn angle_gamma(&self) -> &CellAngle {
        &self.angle_gamma
    }

    pub fn volume(&self) -> &Numeric {
        &self.volume
    }

    pub fn vector_a(&self) -> &Matrix {
        &self.vector_a
    }

    pub fn vector_b(&self) -> &Matrix {
        &self.vector_b
    }

    pub fn vector_c(&self) -> &Matrix {
        &self.vector_c
    }

    pub fn orthogonal_matrix(&self) -> &Matrix {
        &self.orthogonal_matrix
    }

    pub fn metric_tensor(&self) -> &Matrix {
        &self.metric_tensor
    }

    pub fn reciprocal_vector_a(&self) -> &Matrix {
        &self.reciprocal_vector_a
    }

    pub fn reciprocal_vector_b(&self) -> &Matrix {
        &self.reciprocal_vector_b
    }

    pub fn reciprocal_vector_c(&self) -> &Matrix {
        &self.reciprocal_vector_c
    }

    pub fn reciprocal_length_a(&self) -> &Numeric {
        &self.reciprocal_length_a
    }

    pub fn reciprocal_length_b(&self) -> &Numeric {
        &self.reciprocal_length_b
    }

    pub fn reciprocal_length_c(&self) -> &Numeric {
        &self.reciprocal_length_c
    }

    pub fn reciprocal_angle_alpha(&self) -> &CellAngle {
        &self.reciprocal_angle_alpha
    }

    pub fn reciprocal_angle_beta(&self) -> &CellAngle {
        &self.reciprocal_angle_beta
    }

    pub fn reciprocal_angle_gamma(&self) -> &CellAngle {
        &self.reciprocal_angle_gamma
    }

    pub fn reciprocal_metric_tensor(&self) -> &Matrix {
        &self.reciprocal_metric_tensor
    }

    pub fn reciprocal_orthogonal_matrix(&self) -> &Matrix {
        &self.reciprocal_orthogonal_matrix
    }

    /// The single-value items of the built cell, in write order.
    pub fn terms(&self) -> &[CellTerms] {
        &self.terms
    }

    /// Materialize the items as `DataItems` through the
    /// `SingleValueTerm` trait.
    pub fn to_data_items(&self) -> Vec<DataItems> {
        self.terms
            .iter()
            .map(|term| term.to_single_value_data())
            .collect()
    }
}

#[bon]
impl Cell {
    /// The direct-six entry set.
    ///
    /// The three lengths and the three angles are needed at the
    /// `build()` call. The derived values compute at build. A
    /// supplied derived value cross-checks within the relative
    /// tolerance (the `tolerance` field, default
    /// [`DEFAULT_TOLERANCE`]). A mismatch rejects.
    #[builder(start_fn = builder, builder_type = CellBuilder, finish_fn = build)]
    pub fn direct(
        length_a: Numeric,
        length_b: Numeric,
        length_c: Numeric,
        angle_alpha: CellAngle,
        angle_beta: CellAngle,
        angle_gamma: CellAngle,
        formula_units_Z: Option<UnsignedInteger>,
        formula_units_Z_prime: Option<Numeric>,
        formula_units_Z_details: Option<CharString>,
        formula_units_Z_prime_details: Option<CharString>,
        special_details: Option<CharString>,
        measurement_reflns_used: Option<Numeric>,
        measurement_temperature: Option<Numeric>,
        measurement_wavelength: Option<Numeric>,
        measurement_pressure: Option<Numeric>,
        measurement_radiation: Option<CharString>,
        measurement_theta_min: Option<Numeric>,
        measurement_theta_max: Option<Numeric>,
        volume: Option<Numeric>,
        orthogonal_matrix: Option<Matrix>,
        metric_tensor: Option<Matrix>,
        reciprocal_length_a: Option<Numeric>,
        reciprocal_length_b: Option<Numeric>,
        reciprocal_length_c: Option<Numeric>,
        reciprocal_angle_alpha: Option<CellAngle>,
        reciprocal_angle_beta: Option<CellAngle>,
        reciprocal_angle_gamma: Option<CellAngle>,
        reciprocal_vector_a: Option<Matrix>,
        reciprocal_vector_b: Option<Matrix>,
        reciprocal_vector_c: Option<Matrix>,
        reciprocal_metric_tensor: Option<Matrix>,
        reciprocal_orthogonal_matrix: Option<Matrix>,
        tolerance: Option<f64>,
    ) -> Result<Cell, CellError> {
        let tol = tolerance.unwrap_or(DEFAULT_TOLERANCE);
        let lengths = [
            number_as_f64(&length_a.number()),
            number_as_f64(&length_b.number()),
            number_as_f64(&length_c.number()),
        ];
        let angles = [
            angle_alpha.angle().0,
            angle_beta.angle().0,
            angle_gamma.angle().0,
        ];
        let geom = CellGeometry::from_lengths_and_angles(lengths, angles)?;
        let checks = CellChecks::new()
            .with_volume(volume)
            .with_orthogonal_matrix(orthogonal_matrix)
            .with_metric_tensor(metric_tensor)
            .with_reciprocal_lengths(reciprocal_length_a, reciprocal_length_b, reciprocal_length_c)
            .with_reciprocal_angles(
                reciprocal_angle_alpha,
                reciprocal_angle_beta,
                reciprocal_angle_gamma,
            )
            .with_reciprocal_vectors(reciprocal_vector_a, reciprocal_vector_b, reciprocal_vector_c)
            .with_reciprocal_metric_tensor(reciprocal_metric_tensor)
            .with_reciprocal_orthogonal_matrix(reciprocal_orthogonal_matrix);
        cross_check_cell(&geom, tol, &checks)?;
        Ok(assemble_cell(
            CellInputs {
                lengths,
                angles,
                formula_units_Z,
                formula_units_Z_prime,
                formula_units_Z_details,
                formula_units_Z_prime_details,
                special_details,
                measurement_reflns_used,
                measurement_temperature,
                measurement_wavelength,
                measurement_pressure,
                measurement_radiation,
                measurement_theta_min,
                measurement_theta_max,
            },
            geom,
        ))
    }

    /// The three-vector entry set.
    ///
    /// The three lattice vectors are needed at the `build()` call.
    /// Each is a 3 x 1 or 1 x 3 matrix. The direct six and the
    /// derived items compute from the vectors. A supplied value
    /// cross-checks within the relative tolerance.
    #[builder(start_fn = vectors, builder_type = CellVectorsBuilder, finish_fn = build)]
    pub fn from_vectors(
        vector_a: Matrix,
        vector_b: Matrix,
        vector_c: Matrix,
        formula_units_Z: Option<UnsignedInteger>,
        formula_units_Z_prime: Option<Numeric>,
        formula_units_Z_details: Option<CharString>,
        formula_units_Z_prime_details: Option<CharString>,
        special_details: Option<CharString>,
        measurement_reflns_used: Option<Numeric>,
        measurement_temperature: Option<Numeric>,
        measurement_wavelength: Option<Numeric>,
        measurement_pressure: Option<Numeric>,
        measurement_radiation: Option<CharString>,
        measurement_theta_min: Option<Numeric>,
        measurement_theta_max: Option<Numeric>,
        length_a: Option<Numeric>,
        length_b: Option<Numeric>,
        length_c: Option<Numeric>,
        angle_alpha: Option<CellAngle>,
        angle_beta: Option<CellAngle>,
        angle_gamma: Option<CellAngle>,
        volume: Option<Numeric>,
        orthogonal_matrix: Option<Matrix>,
        metric_tensor: Option<Matrix>,
        reciprocal_length_a: Option<Numeric>,
        reciprocal_length_b: Option<Numeric>,
        reciprocal_length_c: Option<Numeric>,
        reciprocal_angle_alpha: Option<CellAngle>,
        reciprocal_angle_beta: Option<CellAngle>,
        reciprocal_angle_gamma: Option<CellAngle>,
        reciprocal_vector_a: Option<Matrix>,
        reciprocal_vector_b: Option<Matrix>,
        reciprocal_vector_c: Option<Matrix>,
        reciprocal_metric_tensor: Option<Matrix>,
        reciprocal_orthogonal_matrix: Option<Matrix>,
        tolerance: Option<f64>,
    ) -> Result<Cell, CellError> {
        let tol = tolerance.unwrap_or(DEFAULT_TOLERANCE);
        let vectors = [
            read_vector(&vector_a, "vector_a")?,
            read_vector(&vector_b, "vector_b")?,
            read_vector(&vector_c, "vector_c")?,
        ];
        let geom = CellGeometry::from_vectors(vectors)?;
        let checks = CellChecks::new()
            .with_volume(volume)
            .with_orthogonal_matrix(orthogonal_matrix)
            .with_metric_tensor(metric_tensor)
            .with_lengths(length_a, length_b, length_c)
            .with_angles(angle_alpha, angle_beta, angle_gamma)
            .with_reciprocal_lengths(reciprocal_length_a, reciprocal_length_b, reciprocal_length_c)
            .with_reciprocal_angles(
                reciprocal_angle_alpha,
                reciprocal_angle_beta,
                reciprocal_angle_gamma,
            )
            .with_reciprocal_vectors(reciprocal_vector_a, reciprocal_vector_b, reciprocal_vector_c)
            .with_reciprocal_metric_tensor(reciprocal_metric_tensor)
            .with_reciprocal_orthogonal_matrix(reciprocal_orthogonal_matrix);
        cross_check_cell(&geom, tol, &checks)?;
        Ok(assemble_cell(
            CellInputs {
                lengths: geom.lengths,
                angles: geom.angles,
                formula_units_Z,
                formula_units_Z_prime,
                formula_units_Z_details,
                formula_units_Z_prime_details,
                special_details,
                measurement_reflns_used,
                measurement_temperature,
                measurement_wavelength,
                measurement_pressure,
                measurement_radiation,
                measurement_theta_min,
                measurement_theta_max,
            },
            geom,
        ))
    }
}

/// The optional recorded items of the build.
#[derive(Debug, Clone)]
struct CellInputs {
    lengths: [f64; 3],
    angles: [f64; 3],
    formula_units_Z: Option<UnsignedInteger>,
    formula_units_Z_prime: Option<Numeric>,
    formula_units_Z_details: Option<CharString>,
    formula_units_Z_prime_details: Option<CharString>,
    special_details: Option<CharString>,
    measurement_reflns_used: Option<Numeric>,
    measurement_temperature: Option<Numeric>,
    measurement_wavelength: Option<Numeric>,
    measurement_pressure: Option<Numeric>,
    measurement_radiation: Option<CharString>,
    measurement_theta_min: Option<Numeric>,
    measurement_theta_max: Option<Numeric>,
}

/// The supplied derived items, cross-checked at build.
#[derive(Debug, Clone)]
struct CellChecks {
    volume: Option<Numeric>,
    orthogonal_matrix: Option<Matrix>,
    metric_tensor: Option<Matrix>,
    length_a: Option<Numeric>,
    length_b: Option<Numeric>,
    length_c: Option<Numeric>,
    angle_alpha: Option<CellAngle>,
    angle_beta: Option<CellAngle>,
    angle_gamma: Option<CellAngle>,
    reciprocal_length_a: Option<Numeric>,
    reciprocal_length_b: Option<Numeric>,
    reciprocal_length_c: Option<Numeric>,
    reciprocal_angle_alpha: Option<CellAngle>,
    reciprocal_angle_beta: Option<CellAngle>,
    reciprocal_angle_gamma: Option<CellAngle>,
    reciprocal_vector_a: Option<Matrix>,
    reciprocal_vector_b: Option<Matrix>,
    reciprocal_vector_c: Option<Matrix>,
    reciprocal_metric_tensor: Option<Matrix>,
    reciprocal_orthogonal_matrix: Option<Matrix>,
}

impl CellChecks {
    fn new() -> Self {
        Self {
            volume: None,
            orthogonal_matrix: None,
            metric_tensor: None,
            length_a: None,
            length_b: None,
            length_c: None,
            angle_alpha: None,
            angle_beta: None,
            angle_gamma: None,
            reciprocal_length_a: None,
            reciprocal_length_b: None,
            reciprocal_length_c: None,
            reciprocal_angle_alpha: None,
            reciprocal_angle_beta: None,
            reciprocal_angle_gamma: None,
            reciprocal_vector_a: None,
            reciprocal_vector_b: None,
            reciprocal_vector_c: None,
            reciprocal_metric_tensor: None,
            reciprocal_orthogonal_matrix: None,
        }
    }

    fn with_volume(mut self, value: Option<Numeric>) -> Self {
        self.volume = value;
        self
    }

    fn with_orthogonal_matrix(mut self, value: Option<Matrix>) -> Self {
        self.orthogonal_matrix = value;
        self
    }

    fn with_metric_tensor(mut self, value: Option<Matrix>) -> Self {
        self.metric_tensor = value;
        self
    }

    fn with_lengths(
        mut self,
        length_a: Option<Numeric>,
        length_b: Option<Numeric>,
        length_c: Option<Numeric>,
    ) -> Self {
        self.length_a = length_a;
        self.length_b = length_b;
        self.length_c = length_c;
        self
    }

    fn with_angles(
        mut self,
        angle_alpha: Option<CellAngle>,
        angle_beta: Option<CellAngle>,
        angle_gamma: Option<CellAngle>,
    ) -> Self {
        self.angle_alpha = angle_alpha;
        self.angle_beta = angle_beta;
        self.angle_gamma = angle_gamma;
        self
    }

    fn with_reciprocal_lengths(
        mut self,
        reciprocal_length_a: Option<Numeric>,
        reciprocal_length_b: Option<Numeric>,
        reciprocal_length_c: Option<Numeric>,
    ) -> Self {
        self.reciprocal_length_a = reciprocal_length_a;
        self.reciprocal_length_b = reciprocal_length_b;
        self.reciprocal_length_c = reciprocal_length_c;
        self
    }

    fn with_reciprocal_angles(
        mut self,
        reciprocal_angle_alpha: Option<CellAngle>,
        reciprocal_angle_beta: Option<CellAngle>,
        reciprocal_angle_gamma: Option<CellAngle>,
    ) -> Self {
        self.reciprocal_angle_alpha = reciprocal_angle_alpha;
        self.reciprocal_angle_beta = reciprocal_angle_beta;
        self.reciprocal_angle_gamma = reciprocal_angle_gamma;
        self
    }

    fn with_reciprocal_vectors(
        mut self,
        reciprocal_vector_a: Option<Matrix>,
        reciprocal_vector_b: Option<Matrix>,
        reciprocal_vector_c: Option<Matrix>,
    ) -> Self {
        self.reciprocal_vector_a = reciprocal_vector_a;
        self.reciprocal_vector_b = reciprocal_vector_b;
        self.reciprocal_vector_c = reciprocal_vector_c;
        self
    }

    fn with_reciprocal_metric_tensor(mut self, value: Option<Matrix>) -> Self {
        self.reciprocal_metric_tensor = value;
        self
    }

    fn with_reciprocal_orthogonal_matrix(mut self, value: Option<Matrix>) -> Self {
        self.reciprocal_orthogonal_matrix = value;
        self
    }
}

fn assemble_cell(inputs: CellInputs, geom: CellGeometry) -> Cell {
    let CellInputs {
        lengths,
        angles,
        formula_units_Z,
        formula_units_Z_prime,
        formula_units_Z_details,
        formula_units_Z_prime_details,
        special_details,
        measurement_reflns_used,
        measurement_temperature,
        measurement_wavelength,
        measurement_pressure,
        measurement_radiation,
        measurement_theta_min,
        measurement_theta_max,
    } = inputs;

    let angle_terms = [
        CellTerms::Angle_alpha(CellAngle::new(Float(angles[0]))),
        CellTerms::Angle_beta(CellAngle::new(Float(angles[1]))),
        CellTerms::Angle_gamma(CellAngle::new(Float(angles[2]))),
    ];
    let mut terms: Vec<CellTerms> = vec![
        CellTerms::Length_a(Numeric::from(lengths[0])),
        CellTerms::Length_b(Numeric::from(lengths[1])),
        CellTerms::Length_c(Numeric::from(lengths[2])),
        angle_terms[0].clone(),
        angle_terms[1].clone(),
        angle_terms[2].clone(),
        CellTerms::Volume(Numeric::from(geom.volume)),
        CellTerms::Vector_a(vector_matrix(geom.vectors[0])),
        CellTerms::Vector_b(vector_matrix(geom.vectors[1])),
        CellTerms::Vector_c(vector_matrix(geom.vectors[2])),
        CellTerms::Orthogonal_matrix(matrix3(&geom.orthogonal)),
        CellTerms::Metric_tensor(matrix3(&geom.metric)),
        CellTerms::Reciprocal_vector_a(vector_matrix(
            geom.reciprocal_vectors[0],
        )),
        CellTerms::Reciprocal_vector_b(vector_matrix(
            geom.reciprocal_vectors[1],
        )),
        CellTerms::Reciprocal_vector_c(vector_matrix(
            geom.reciprocal_vectors[2],
        )),
        CellTerms::Reciprocal_length_a(Numeric::from(geom.reciprocal_lengths[0])),
        CellTerms::Reciprocal_length_b(Numeric::from(geom.reciprocal_lengths[1])),
        CellTerms::Reciprocal_length_c(Numeric::from(geom.reciprocal_lengths[2])),
        CellTerms::Reciprocal_angle_alpha(CellAngle::new(Float(
            geom.reciprocal_angles[0],
        ))),
        CellTerms::Reciprocal_angle_beta(CellAngle::new(Float(
            geom.reciprocal_angles[1],
        ))),
        CellTerms::Reciprocal_angle_gamma(CellAngle::new(Float(
            geom.reciprocal_angles[2],
        ))),
        CellTerms::Reciprocal_metric_tensor(matrix3(&geom.reciprocal_metric)),
        CellTerms::Reciprocal_orthogonal_matrix(matrix3(&geom.reciprocal_orthogonal)),
    ];
    if let Some(z) = formula_units_Z {
        terms.push(CellTerms::Formula_units_Z(z));
    }
    if let Some(zp) = formula_units_Z_prime {
        terms.push(CellTerms::Formula_units_z_prime(zp));
    }
    if let Some(zd) = &formula_units_Z_details {
        terms.push(CellTerms::Formula_units_z_details(zd.clone()));
    }
    if let Some(zpd) = &formula_units_Z_prime_details {
        terms.push(CellTerms::Formula_units_z_prime_details(zpd.clone()));
    }
    if let Some(sd) = &special_details {
        terms.push(CellTerms::Special_details(sd.clone()));
    }
    if let Some(m) = measurement_reflns_used {
        terms.push(CellTerms::Measurement_reflns_used(m));
    }
    if let Some(t) = measurement_temperature {
        terms.push(CellTerms::Measurement_temperature(t));
    }
    if let Some(w) = measurement_wavelength {
        terms.push(CellTerms::Measurement_wavelength(w));
    }
    if let Some(p) = measurement_pressure {
        terms.push(CellTerms::Measurement_pressure(p));
    }
    if let Some(r) = &measurement_radiation {
        terms.push(CellTerms::Measurement_radiation(r.clone()));
    }
    if let Some(tmin) = measurement_theta_min {
        terms.push(CellTerms::Measurement_theta_min(tmin));
    }
    if let Some(tmax) = measurement_theta_max {
        terms.push(CellTerms::Measurement_theta_max(tmax));
    }
    Cell {
        length_a: Numeric::from(lengths[0]),
        length_b: Numeric::from(lengths[1]),
        length_c: Numeric::from(lengths[2]),
        angle_alpha: CellAngle::new(Float(angles[0])),
        angle_beta: CellAngle::new(Float(angles[1])),
        angle_gamma: CellAngle::new(Float(angles[2])),
        volume: Numeric::from(geom.volume),
        vector_a: vector_matrix(geom.vectors[0]),
        vector_b: vector_matrix(geom.vectors[1]),
        vector_c: vector_matrix(geom.vectors[2]),
        orthogonal_matrix: matrix3(&geom.orthogonal),
        metric_tensor: matrix3(&geom.metric),
        reciprocal_vector_a: vector_matrix(geom.reciprocal_vectors[0]),
        reciprocal_vector_b: vector_matrix(geom.reciprocal_vectors[1]),
        reciprocal_vector_c: vector_matrix(geom.reciprocal_vectors[2]),
        reciprocal_length_a: Numeric::from(geom.reciprocal_lengths[0]),
        reciprocal_length_b: Numeric::from(geom.reciprocal_lengths[1]),
        reciprocal_length_c: Numeric::from(geom.reciprocal_lengths[2]),
        reciprocal_angle_alpha: CellAngle::new(Float(geom.reciprocal_angles[0])),
        reciprocal_angle_beta: CellAngle::new(Float(geom.reciprocal_angles[1])),
        reciprocal_angle_gamma: CellAngle::new(Float(geom.reciprocal_angles[2])),
        reciprocal_metric_tensor: matrix3(&geom.reciprocal_metric),
        reciprocal_orthogonal_matrix: matrix3(&geom.reciprocal_orthogonal),
        terms,
    }
}

/// The cell geometry in one place: the vectors, the orthogonal and
/// metric tensors, and the reciprocal quantities.
#[derive(Debug, Clone)]
struct CellGeometry {
    lengths: [f64; 3],
    angles: [f64; 3],
    volume: f64,
    vectors: [[f64; 3]; 3],
    orthogonal: [[f64; 3]; 3],
    metric: [[f64; 3]; 3],
    reciprocal_vectors: [[f64; 3]; 3],
    reciprocal_lengths: [f64; 3],
    reciprocal_angles: [f64; 3],
    reciprocal_metric: [[f64; 3]; 3],
    reciprocal_orthogonal: [[f64; 3]; 3],
}

impl CellGeometry {
    fn from_lengths_and_angles(
        lengths: [f64; 3],
        angles_deg: [f64; 3],
    ) -> Result<Self, CellError> {
        for (item, &length) in
            ["length_a", "length_b", "length_c"].into_iter().zip(&lengths)
        {
            if length <= 0.0 {
                return Err(CellError::Degenerate(format!(
                    "{item} must be positive, the value is {length}"
                )));
            }
        }
        let angles = angles_deg.map(deg_to_rad);
        let ca = angles[0].cos();
        let cb = angles[1].cos();
        let cg = angles[2].cos();
        let sa = angles[0].sin();
        let sb = angles[1].sin();
        let sg = angles[2].sin();
        let (a, b, c) = (lengths[0], lengths[1], lengths[2]);
        if sa.abs() < 1e-12 || sb.abs() < 1e-12 || sg.abs() < 1e-12 {
            return Err(CellError::Degenerate(
                "a zero sine of an angle describes no lattice".into(),
            ));
        }
        let v2 = 1.0 - ca * ca - cb * cb - cg * cg + 2.0 * ca * cb * cg;
        if v2 <= 0.0 {
            return Err(CellError::Degenerate(format!(
                "the lengths and angles give no positive volume (the argument is {v2})"
            )));
        }
        let volume = a * b * c * v2.sqrt();
        let vectors: [[f64; 3]; 3] = [
            [a, 0.0, 0.0],
            [b * cg, b * sg, 0.0],
            [
                c * cb,
                c * (ca - cb * cg) / sg,
                c * (1.0 - cb * cb + (cb * cg - ca) * (cb * cg - ca) / (sg * sg))
                    .max(0.0)
                    .sqrt(),
            ],
        ];
        Ok(Self::complete(lengths, angles_deg, volume, vectors))
    }

    fn from_vectors(raw: [Vec<f64>; 3]) -> Result<Self, CellError> {
        let mut vectors = [[0.0; 3]; 3];
        for (index, (item, v)) in
            ["vector_a", "vector_b", "vector_c"].into_iter().zip(&raw).enumerate()
        {
            let norm = v.iter().map(|x| x * x).sum::<f64>().sqrt();
            if norm <= 0.0 {
                return Err(CellError::Degenerate(format!("{item} is a zero vector")));
            }
            vectors[index] = [v[0], v[1], v[2]];
        }
        let lengths = vectors.map(|v| v.iter().map(|x| x * x).sum::<f64>().sqrt());
        let angles = [
            angle_between(&vectors[0], &vectors[1]),
            angle_between(&vectors[1], &vectors[2]),
            angle_between(&vectors[2], &vectors[0]),
        ];
        let det = determinant(&[
            [vectors[0][0], vectors[1][0], vectors[2][0]],
            [vectors[0][1], vectors[1][1], vectors[2][1]],
            [vectors[0][2], vectors[1][2], vectors[2][2]],
        ]);
        let volume = det.abs();
        if volume <= 0.0 {
            return Err(CellError::Degenerate(
                "the three vectors are coplanar, the volume is zero".into(),
            ));
        }
        Ok(Self::complete(lengths, angles, volume, vectors))
    }

    fn complete(
        lengths: [f64; 3],
        angles_deg: [f64; 3],
        volume: f64,
        vectors: [[f64; 3]; 3],
    ) -> Self {
        // The orthogonal matrix carries the vectors in its columns.
        // The shell expression: `vector_a = orthogonal_matrix * [1,0,0]`.
        let orthogonal = [[
            vectors[0][0],
            vectors[1][0],
            vectors[2][0],
        ], [
            vectors[0][1],
            vectors[1][1],
            vectors[2][1],
        ], [
            vectors[0][2],
            vectors[1][2],
            vectors[2][2],
        ]];
        let metric = [[
            dot(&vectors[0], &vectors[0]),
            dot(&vectors[0], &vectors[1]),
            dot(&vectors[0], &vectors[2]),
        ], [
            dot(&vectors[1], &vectors[0]),
            dot(&vectors[1], &vectors[1]),
            dot(&vectors[1], &vectors[2]),
        ], [
            dot(&vectors[2], &vectors[0]),
            dot(&vectors[2], &vectors[1]),
            dot(&vectors[2], &vectors[2]),
        ]];
        let reciprocal_vectors = [
            cross(&vectors[1], &vectors[2]),
            cross(&vectors[2], &vectors[0]),
            cross(&vectors[0], &vectors[1]),
        ]
        .map(|v| [v[0] / volume, v[1] / volume, v[2] / volume]);
        let reciprocal_lengths = [
            norm_of(&reciprocal_vectors[0]),
            norm_of(&reciprocal_vectors[1]),
            norm_of(&reciprocal_vectors[2]),
        ];
        let reciprocal_angles = [
            angle_between(&reciprocal_vectors[1], &reciprocal_vectors[2]),
            angle_between(&reciprocal_vectors[0], &reciprocal_vectors[2]),
            angle_between(&reciprocal_vectors[0], &reciprocal_vectors[1]),
        ];
        let reciprocal_metric = inverse_3x3(&metric)
            .expect("a non-degenerate cell gives an invertible metric tensor");
        // The shell expression: the reciprocal orthogonal matrix is the
        // inverse of the transpose of the orthogonal matrix.
        let reciprocal_orthogonal =
            inverse_3x3(&transpose(&orthogonal)).expect("a non-degenerate cell gives an invertible orthogonal matrix");
        Self {
            lengths,
            angles: angles_deg,
            volume,
            vectors,
            orthogonal,
            metric,
            reciprocal_vectors,
            reciprocal_lengths,
            reciprocal_angles,
            reciprocal_metric,
            reciprocal_orthogonal,
        }
    }
}

fn read_vector(m: &Matrix, item: &'static str) -> Result<Vec<f64>, CellError> {
    match m.as_vector() {
        Some(vector) => Ok(vector
            .iter()
            .map(|n| match n {
                Number::Integer(i) => i.0 as f64,
                Number::Float(f) => f.0,
            })
            .collect()),
        None => Err(CellError::BadShape {
            item,
            expected: "3 x 1 or 1 x 3",
            actual: m.shape(),
        }),
    }
}

fn cross_check_cell(geom: &CellGeometry, tol: f64, checks: &CellChecks) -> Result<(), CellError> {
    if let Some(v) = &checks.volume {
        cross_check_value(
            "volume",
            number_as_f64(&v.number()),
            geom.volume,
            tol,
        )?;
    }
    if let Some(m) = &checks.orthogonal_matrix {
        cross_check_matrix("orthogonal_matrix", &m, &geom.orthogonal, tol)?;
    }
    if let Some(m) = &checks.metric_tensor {
        cross_check_matrix("metric_tensor", &m, &geom.metric, tol)?;
    }
    if let Some(l) = &checks.length_a {
        cross_check_value(
            "length_a",
            number_as_f64(&l.number()),
            geom.lengths[0],
            tol,
        )?;
    }
    if let Some(l) = &checks.length_b {
        cross_check_value(
            "length_b",
            number_as_f64(&l.number()),
            geom.lengths[1],
            tol,
        )?;
    }
    if let Some(l) = &checks.length_c {
        cross_check_value(
            "length_c",
            number_as_f64(&l.number()),
            geom.lengths[2],
            tol,
        )?;
    }
    if let Some(a) = &checks.angle_alpha {
        cross_check_value("angle_alpha", a.angle().0, geom.angles[0], tol)?;
    }
    if let Some(a) = &checks.angle_beta {
        cross_check_value("angle_beta", a.angle().0, geom.angles[1], tol)?;
    }
    if let Some(a) = &checks.angle_gamma {
        cross_check_value("angle_gamma", a.angle().0, geom.angles[2], tol)?;
    }
    if let Some(l) = &checks.reciprocal_length_a {
        cross_check_value(
            "reciprocal_length_a",
            number_as_f64(&l.number()),
            geom.reciprocal_lengths[0],
            tol,
        )?;
    }
    if let Some(l) = &checks.reciprocal_length_b {
        cross_check_value(
            "reciprocal_length_b",
            number_as_f64(&l.number()),
            geom.reciprocal_lengths[1],
            tol,
        )?;
    }
    if let Some(l) = &checks.reciprocal_length_c {
        cross_check_value(
            "reciprocal_length_c",
            number_as_f64(&l.number()),
            geom.reciprocal_lengths[2],
            tol,
        )?;
    }
    if let Some(a) = &checks.reciprocal_angle_alpha {
        cross_check_value(
            "reciprocal_angle_alpha",
            a.angle().0,
            geom.reciprocal_angles[0],
            tol,
        )?;
    }
    if let Some(a) = &checks.reciprocal_angle_beta {
        cross_check_value(
            "reciprocal_angle_beta",
            a.angle().0,
            geom.reciprocal_angles[1],
            tol,
        )?;
    }
    if let Some(a) = &checks.reciprocal_angle_gamma {
        cross_check_value(
            "reciprocal_angle_gamma",
            a.angle().0,
            geom.reciprocal_angles[2],
            tol,
        )?;
    }
    if let Some(v) = &checks.reciprocal_vector_a {
        cross_check_vector("reciprocal_vector_a", &v, &geom.reciprocal_vectors[0], tol)?;
    }
    if let Some(v) = &checks.reciprocal_vector_b {
        cross_check_vector("reciprocal_vector_b", &v, &geom.reciprocal_vectors[1], tol)?;
    }
    if let Some(v) = &checks.reciprocal_vector_c {
        cross_check_vector("reciprocal_vector_c", &v, &geom.reciprocal_vectors[2], tol)?;
    }
    if let Some(m) = &checks.reciprocal_metric_tensor {
        cross_check_matrix(
            "reciprocal_metric_tensor",
            m,
            &geom.reciprocal_metric,
            tol,
        )?;
    }
    if let Some(m) = &checks.reciprocal_orthogonal_matrix {
        cross_check_matrix(
            "reciprocal_orthogonal_matrix",
            m,
            &geom.reciprocal_orthogonal,
            tol,
        )?;
    }
    Ok(())
}

fn cross_check_value(
    item: &'static str,
    supplied: f64,
    computed: f64,
    tol: f64,
) -> Result<(), CellError> {
    if within_tolerance(supplied, computed, tol) {
        Ok(())
    } else {
        Err(CellError::Mismatch {
            item,
            supplied: supplied.to_string(),
            computed: computed.to_string(),
            tolerance: tol,
        })
    }
}

fn cross_check_vector(
    item: &'static str,
    supplied: &Matrix,
    computed: &[f64; 3],
    tol: f64,
) -> Result<(), CellError> {
    let supplied = read_vector(supplied, item)?;
    for (i, (s, c)) in supplied.iter().zip(computed).enumerate() {
        cross_check_value(
            item,
            *s,
            *c,
            tol,
        )?;
        let _ = i;
    }
    Ok(())
}

fn cross_check_matrix(
    item: &'static str,
    supplied: &Matrix,
    computed: &[[f64; 3]; 3],
    tol: f64,
) -> Result<(), CellError> {
    if supplied.shape() != (3, 3) {
        return Err(CellError::BadShape {
            item,
            expected: "3 x 3",
            actual: supplied.shape(),
        });
    }
    for (i, row) in supplied.rows().iter().enumerate() {
        for (j, value) in row.iter().enumerate() {
            let s = match value {
                Number::Integer(v) => v.0 as f64,
                Number::Float(v) => v.0,
            };
            cross_check_value(item, s, computed[i][j], tol)?;
        }
    }
    Ok(())
}

fn within_tolerance(supplied: f64, computed: f64, tol: f64) -> bool {
    if supplied == computed {
        return true;
    }
    let scale = supplied.abs().max(computed.abs());
    (supplied - computed).abs() <= tol * scale
}

fn vector_matrix(v: [f64; 3]) -> Matrix {
    Matrix::try_new(vec![
        vec![Number::Float(Float(v[0]))],
        vec![Number::Float(Float(v[1]))],
        vec![Number::Float(Float(v[2]))],
    ])
    .expect("a 3 x 1 vector")
}

fn matrix3(m: &[[f64; 3]; 3]) -> Matrix {
    Matrix::try_new((0..3)
        .map(|i| (0..3).map(|j| Number::Float(Float(m[i][j]))).collect())
        .collect())
    .expect("a 3 x 3 matrix")
}

fn deg_to_rad(deg: f64) -> f64 {
    deg * std::f64::consts::PI / 180.0
}

fn dot(u: &[f64; 3], v: &[f64; 3]) -> f64 {
    u[0] * v[0] + u[1] * v[1] + u[2] * v[2]
}

fn cross(u: &[f64; 3], v: &[f64; 3]) -> [f64; 3] {
    [
        u[1] * v[2] - u[2] * v[1],
        u[2] * v[0] - u[0] * v[2],
        u[0] * v[1] - u[1] * v[0],
    ]
}

fn norm_of(v: &[f64; 3]) -> f64 {
    dot(v, v).sqrt()
}

fn angle_between(u: &[f64; 3], v: &[f64; 3]) -> f64 {
    let cos = (dot(u, v) / (norm_of(u) * norm_of(v))).clamp(-1.0, 1.0);
    cos.acos() * 180.0 / std::f64::consts::PI
}

fn determinant(m: &[[f64; 3]; 3]) -> f64 {
    m[0][0] * (m[1][1] * m[2][2] - m[1][2] * m[2][1])
        - m[0][1] * (m[1][0] * m[2][2] - m[1][2] * m[2][0])
        + m[0][2] * (m[1][0] * m[2][1] - m[1][1] * m[2][0])
}

fn transpose(m: &[[f64; 3]; 3]) -> [[f64; 3]; 3] {
    [[
        m[0][0], m[1][0], m[2][0],
    ], [
        m[0][1], m[1][1], m[2][1],
    ], [
        m[0][2], m[1][2], m[2][2],
    ]]
}

fn inverse_3x3(m: &[[f64; 3]; 3]) -> Option<[[f64; 3]; 3]> {
    let det = determinant(m);
    if det.abs() < 1e-300 {
        return None;
    }
    // The cofactor of row i, column j, transposed into the
    // adjugate position.
    let cofactor = |i: usize, j: usize| -> f64 {
        let mut minor = [[0.0; 3]; 3];
        let mut r = 0;
        for (ri, row) in m.iter().enumerate() {
            if ri == i {
                continue;
            }
            let mut c = 0;
            for (cj, &value) in row.iter().enumerate() {
                if cj != j {
                    minor[r][c] = value;
                    c += 1;
                }
            }
            r += 1;
        }
        let sign = if (i + j) % 2 == 0 { 1.0 } else { -1.0 };
        sign * determinant(&minor)
    };
    let adj = [[
        cofactor(0, 0),
        cofactor(1, 0),
        cofactor(2, 0),
    ], [
        cofactor(0, 1),
        cofactor(1, 1),
        cofactor(2, 1),
    ], [
        cofactor(0, 2),
        cofactor(1, 2),
        cofactor(2, 2),
    ]];
    Some(adj.map(|row| row.map(|v| v / det)))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crystal_cif_fmt::grammar::UnquotedString;

    fn direct_six() -> (Numeric, Numeric, Numeric, CellAngle, CellAngle, CellAngle) {
        (
            Numeric::from(5.0),
            Numeric::from(10.0),
            Numeric::from(15.0),
            CellAngle::from(90.0f64),
            CellAngle::from(90.0f64),
            CellAngle::from(90.0f64),
        )
    }

    fn direct() -> Result<Cell, CellError> {
        let (la, lb, lc, aa, ab, ag) = direct_six();
        Cell::builder()
            .length_a(la)
            .length_b(lb)
            .length_c(lc)
            .angle_alpha(aa)
            .angle_beta(ab)
            .angle_gamma(ag)
            .build()
    }

    fn vectors() -> Result<Cell, CellError> {
        Cell::vectors()
            .vector_a(vector_matrix([5.0, 0.0, 0.0]))
            .vector_b(vector_matrix([0.0, 10.0, 0.0]))
            .vector_c(vector_matrix([0.0, 0.0, 15.0]))
            .build()
    }

    fn cs(text: &str) -> CharString {
        CharString::Unquoted(UnquotedString::new(text.to_string()))
    }

    #[test]
    fn the_happy_path_builds_the_derived_values() {
        let cell = direct().expect("the orthorhombic cell builds");
        assert_eq!(
            cell.volume().number().as_float().copied().map(|f| f.0),
            Some(750.0),
            "the volume is the product of the six"
        );
        assert_eq!(cell.orthogonal_matrix().shape(), (3, 3));
        assert_eq!(cell.metric_tensor().shape(), (3, 3));
        let g = cell.metric_tensor();
        let g11 = g.rows()[0][0].as_float().copied().map(|f| f.0);
        assert_eq!(g11, Some(25.0), "the metric diagonal is the squared length");
        assert!(cell.terms().iter().any(|t| t.tag().as_ref() == "cell_volume"));
        assert_eq!(cell.to_data_items().len(), 23);
    }

    #[test]
    fn the_reciprocal_items_compute_from_the_direct_six() {
        let cell = direct().expect("builds");
        assert_eq!(
            cell.reciprocal_length_a().number().as_float().copied().map(|f| f.0),
            Some(1.0 / 5.0),
            "the reciprocal length is the inverse of a right cell"
        );
        assert!((cell.reciprocal_angle_alpha().angle().0 - 90.0).abs() < 1e-9);
    }

    #[test]
    fn a_matching_supplied_volume_passes_the_cross_check() {
        let cell = direct_with_volume(Numeric::from(750.0)).expect("the volume matches");
        assert_eq!(
            cell.volume().number().as_float().copied().map(|f| f.0),
            Some(750.0)
        );
    }

    fn direct_with_volume(volume: Numeric) -> Result<Cell, CellError> {
        let (la, lb, lc, aa, ab, ag) = direct_six();
        Cell::builder()
            .length_a(la)
            .length_b(lb)
            .length_c(lc)
            .angle_alpha(aa)
            .angle_beta(ab)
            .angle_gamma(ag)
            .volume(volume)
            .build()
    }

    #[test]
    fn a_mismatching_supplied_volume_rejects() {
        let err = direct_with_volume(Numeric::from(800.0)).unwrap_err();
        assert!(matches!(err, CellError::Mismatch { item: "volume", .. }));
    }

    #[test]
    fn a_wrong_metric_tensor_shape_rejects() {
        let (la, lb, lc, aa, ab, ag) = direct_six();
        let wrong = Matrix::try_new(vec![
            vec![Number::Float(Float(1.0)), Number::Float(Float(1.0))],
            vec![Number::Float(Float(1.0)), Number::Float(Float(1.0))],
        ])
        .expect("a 2 x 2 matrix");
        let err = Cell::builder()
            .length_a(la)
            .length_b(lb)
            .length_c(lc)
            .angle_alpha(aa)
            .angle_beta(ab)
            .angle_gamma(ag)
            .metric_tensor(wrong)
            .build()
            .unwrap_err();
        assert!(matches!(
            err,
            CellError::BadShape {
                item: "metric_tensor",
                ..
            }
        ));
    }

    #[test]
    fn a_zero_sine_rejects_as_degenerate() {
        let (la, lb, lc, aa, _, ag) = direct_six();
        let err = Cell::builder()
            .length_a(la)
            .length_b(lb)
            .length_c(lc)
            .angle_alpha(aa)
            .angle_beta(CellAngle::from(0.0f64))
            .angle_gamma(ag)
            .build()
            .unwrap_err();
        assert!(matches!(err, CellError::Degenerate(_)));
    }

    #[test]
    fn the_vectors_builder_computes_the_direct_six() {
        let cell = vectors().expect("the right vectors build");
        assert_eq!(
            cell.length_a().number().as_float().copied().map(|f| f.0),
            Some(5.0)
        );
        assert_eq!(
            cell.length_b().number().as_float().copied().map(|f| f.0),
            Some(10.0)
        );
        assert_eq!(
            cell.length_c().number().as_float().copied().map(|f| f.0),
            Some(15.0)
        );
        assert_eq!(
            cell.volume().number().as_float().copied().map(|f| f.0),
            Some(750.0)
        );
        assert!((cell.angle_alpha().angle().0 - 90.0).abs() < 1e-9);
    }

    #[test]
    fn a_supplied_length_cross_checks_in_the_vectors_builder() {
        let err = Cell::vectors()
            .vector_a(vector_matrix([5.0, 0.0, 0.0]))
            .vector_b(vector_matrix([0.0, 10.0, 0.0]))
            .vector_c(vector_matrix([0.0, 0.0, 15.0]))
            .length_a(Numeric::from(6.0))
            .build()
            .unwrap_err();
        assert!(matches!(
            err,
            CellError::Mismatch { item: "length_a", .. }
        ));
    }

    #[test]
    fn the_vectors_builder_rejects_a_wrong_vector_shape() {
        let square = Matrix::try_new(vec![
            vec![Number::Float(Float(1.0)), Number::Float(Float(1.0))],
            vec![Number::Float(Float(1.0)), Number::Float(Float(1.0))],
        ])
        .expect("a 2 x 2 matrix");
        let err = Cell::vectors()
            .vector_a(square)
            .vector_b(vector_matrix([0.0, 1.0, 0.0]))
            .vector_c(vector_matrix([0.0, 0.0, 1.0]))
            .build()
            .unwrap_err();
        assert!(matches!(
            err,
            CellError::BadShape {
                item: "vector_a",
                ..
            }
        ));
    }

    #[test]
    fn the_supplied_reciprocal_values_cross_check() {
        let (la, lb, lc, aa, ab, ag) = direct_six();
        let ok = Cell::builder()
            .length_a(la)
            .length_b(lb)
            .length_c(lc)
            .angle_alpha(aa)
            .angle_beta(ab)
            .angle_gamma(ag)
            .reciprocal_length_a(Numeric::from(0.2))
            .build()
            .expect("0.2 is the reciprocal length of a = 5");
        assert_eq!(
            ok.reciprocal_length_a().number().as_float().copied().map(|f| f.0),
            Some(0.2)
        );
        let err = Cell::builder()
            .length_a(la)
            .length_b(lb)
            .length_c(lc)
            .angle_alpha(aa)
            .angle_beta(ab)
            .angle_gamma(ag)
            .reciprocal_length_a(Numeric::from(0.3))
            .build()
            .unwrap_err();
        assert!(matches!(
            err,
            CellError::Mismatch {
                item: "reciprocal_length_a",
                ..
            }
        ));
    }

    #[test]
    fn the_tolerance_relaxes_the_cross_check() {
        let (la, lb, lc, aa, ab, ag) = direct_six();
        let base = || {
            Cell::builder()
                .length_a(la)
                .length_b(lb)
                .length_c(lc)
                .angle_alpha(aa)
                .angle_beta(ab)
                .angle_gamma(ag)
                .volume(Numeric::from(755.0))
        };
        assert!(
            base().build().is_err(),
            "5.0 misses 750 beyond the default relative 1e-3"
        );
        let relaxed = base()
            .tolerance(1e-2)
            .build()
            .expect("the relaxed tolerance passes");
        assert_eq!(
            relaxed.volume().number().as_float().copied().map(|f| f.0),
            Some(750.0),
            "the computed value stays the output"
        );
    }

    #[test]
    fn the_optional_recorded_items_emit() {
        let (la, lb, lc, aa, ab, ag) = direct_six();
        let cell = Cell::builder()
            .length_a(la)
            .length_b(lb)
            .length_c(lc)
            .angle_alpha(aa)
            .angle_beta(ab)
            .angle_gamma(ag)
            .formula_units_Z(UnsignedInteger(4))
            .special_details(cs("orthorhombic primitive"))
            .measurement_reflns_used(Numeric::from(5000))
            .build()
            .expect("builds");
        let recorded: Vec<String> = cell
            .recorded_items()
            .iter()
            .map(|t| t.tag().as_ref().to_string())
            .collect();
        assert_eq!(recorded.len(), 3, "{recorded:?}");
        assert!(recorded.iter().any(|t| t == "cell_formula_units_z"));
        assert!(recorded.iter().any(|t| t == "cell_special_details"));
        assert!(recorded.iter().any(|t| t == "cell_measurement_reflns_used"));
        assert_eq!(cell.to_data_items().len(), 26);
    }
}
