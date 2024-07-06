use std::fmt::Display;

use winnow::Parser;

use crate::grammar::SyntacticUnit;

use self::{body::LoopBody, header::LoopHeader};

mod body;
mod header;

#[derive(Debug, Clone)]
pub struct LoopUnit {
    header: LoopHeader,
    body: LoopBody,
}

impl LoopUnit {
    pub fn new(header: LoopHeader, body: LoopBody) -> Self {
        Self { header, body }
    }
}

impl SyntacticUnit for LoopUnit {
    type ParseResult = Self;

    type FormatOutput = String;

    fn parser(input: &mut &str) -> winnow::prelude::PResult<Self::ParseResult> {
        (LoopHeader::parser, LoopBody::parser)
            .map(|(header, body)| LoopUnit::new(header, body))
            .parse_next(input)
    }

    fn formatted_output(&self) -> Self::FormatOutput {
        let body_output = self
            .body
            .values()
            .chunks(self.header.num_of_tags())
            .map(|chunk| {
                chunk
                    .iter()
                    .map(|v| v.to_string())
                    .collect::<Vec<String>>()
                    .join(" ")
            })
            .collect::<Vec<String>>()
            .join("\n");
        format!("{}{}", self.header, body_output)
    }
}

impl Display for LoopUnit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.formatted_output())
    }
}

#[cfg(test)]
mod test {
    use crate::grammar::SyntacticUnit;

    use super::LoopUnit;

    #[test]
    fn loop_data_parsing() {
        let mut input = r#"loop_
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
"#;
        let parse_result = LoopUnit::parser(&mut input);
        match parse_result {
            Ok(l) => println!("{l}"),
            Err(e) => {
                dbg!(e);
            }
        }
    }
}
