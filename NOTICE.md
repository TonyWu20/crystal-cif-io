# Third-party data attribution

## Crystallography Open Database (COD)

The test corpora in `tests/real_data/cod/` (6 entries) and
`tests/real_data/cod_sg/` (230 entries, one per space group)
were fetched from the Crystallography Open Database (COD),
`https://www.crystallography.net/`. The API notes live in
`docs/cod-api.md` and the fetch script in
`tests/real_data/cod_sg/fetch.py`.

Each CIF file keeps its COD header notice. Two notice variants
exist in the corpus:

- 230 files carry the public-domain notice: "All data on this
  site have been placed in the public domain by the
  contributors."
- 6 files carry the IUCr Journals notice. The notice allows use
  within the scientific community so long as proper attribution
  is given to the journal article.

The six IUCr-notice files and their citations:

| COD entry | file | citation |
|---|---|---|
| 1000000 | `tests/real_data/cod/1000000.cif` | Phan Thanh S., Marrot J., Renaudin J., Maisonneuve V. Acta Cryst. (2000). C, 56, 1073-1074. DOI 10.1107/S0108270100008532 |
| 1000001 | `tests/real_data/cod/1000001.cif` | Bachet B., Soyer A., Billy I., Mornon J.-P. The entry has no journal data. Attribute to the COD entry 1000001 |
| 2013563 | `tests/real_data/cod_sg/sg024_2013563.cif` | Guedira F., Saidi Idrissi M., Krautscheid H., El Ammari L. Acta Cryst. (2003). C, 59, m530-m532. DOI 10.1107/S0108270103021516 |
| 2232341 | `tests/real_data/cod_sg/sg168_2232341.cif` | Golenia I. A., Boyko A. N., Kotova N. V., Haukka M., Kalibabchuk V. A. Acta Cryst. (2011). E, 67, m1596-m1597. DOI 10.1107/S1600536811043303 |
| 1513328 | `tests/real_data/cod_sg/sg178_1513328.cif` | Moggach S. A., Allan D. R., Parsons S., Sawyer L., Warren J. E. J. Synchrotron Radiat. (2005). 12, 598-607. DOI 10.1107/S0909049505019850 |

## Citing COD

The COD wiki asks that users cite the project papers when they
use the database, its search, its servers, or its data
collections. The citing page is
`https://wiki.crystallography.net/cod/citing/`.

The recommended papers, in chronological order:

1. Gražulis S., Chateigner D., Downs R. T., Yokochi A. T.,
   Quiros M., Lutterotti L., Manakova E., Butkus J., Moeck P.
   & Le Bail A. (2009). Crystallography Open Database – an
   open-access collection of crystal structures. J. Appl. Cryst.
   42, 726-729. DOI 10.1107/S0021889809016690
2. Gražulis S., Daškevič A., Merkys A.,
   Chateigner D., Lutterotti L., Quirós M., Serebryanaya N. R.,
   Moeck P., Downs R. T. & LeBail A. (2012). Crystallography Open
   Database (COD): an open-access collection of crystal structures
   and platform for world-wide collaboration. Nucleic Acids
   Research 40, D420-D427. DOI 10.1093/nar/gkr900
3. Gražulis S., Merkys A., Vaitkus A. &
   Okulič-Kazarinas M. (2015). Computing stoichiometric
   molecular composition from crystal structures. J. Appl. Cryst.
   48(1), 85-91. DOI 10.1107/S1600576714025904
4. Merkys A., Vaitkus A., Butkus J., Okulič-Kazarinas M.,
   Kairys V. & Gražulis S. (2016). COD::CIF::Parser: an
   error-correcting CIF parser for the Perl language. J. Appl.
   Cryst. 49(1), 292-301. DOI 10.1107/S1600576715022396
5. Quirós M., Gražulis S., Girdzijauskaitė S.,
   Merkys A. & Vaitkus A. (2018). Using SMILES strings for the
   description of chemical connectivity in the Crystallography
   Open Database. J. Cheminformatics 10.
   DOI 10.1186/s13321-018-0279-6
6. Vaitkus A., Merkys A. & Gražulis S. (2021). Validation of
   the Crystallography Open Database using the Crystallographic
   Information Framework. J. Appl. Cryst. 54(2), 661-672.
   DOI 10.1107/S1600576720016532
7. Vaitkus A., Merkys A., Sander T., Quirós M., Thiessen
   P. A., Bolton E. E. & Gražulis S. (2023). A workflow for
   deriving chemical entities from crystallographic data and its
   application to the Crystallography Open Database. J.
   Cheminformatics 15. DOI 10.1186/s13321-023-00780-2
8. Merkys A., Vaitkus A., Grybauskas A., Konovalovas A.,
   Quirós M. & Gražulis S. (2023). Graph
   isomorphism-based algorithm for cross-checking chemical and
   crystallographic descriptions. J. Cheminformatics 15.
   DOI 10.1186/s13321-023-00692-1

## IUCr published CIF examples

The files in `tests/real_data/iucr/` are the IUCr published CIF
example files, fetched 2026-10-12. They are used here as parser
regression inputs. The provenance record is
`tests/real_data/SOURCE.md`. The files are (c) IUCr.

## COMCIFS cif_api test data

The 21 files in `tests/real_data/cif_api/` are the test-data CIFs
of the COMCIFS `cif_api` project, a C reference implementation of
the CIF 1.1 and 2.0 formats. The files are fetched at test time
and are not committed: `tests/real_data/cif_api/fetch.py` (run
by the CI workflow `.github/workflows/ci.yml` and available
locally) downloads them from
`https://github.com/COMCIFS/cif_api` at commit
`7882c1244bb4f69607561448ac82346f6637015f` and verifies each
file against the sha256 and the git blob SHA recorded in
`tests/real_data/cif_api/manifest.json`.

Copyright 2014, 2015 John C. Bollinger, per the upstream
`test-data/README` notice. The files are part of the CIF API
distribution, licensed under the GNU Lesser General Public
License v3 or later.
