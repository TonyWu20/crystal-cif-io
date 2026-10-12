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

## IUCr published CIF examples

The files in `tests/real_data/iucr/` are the IUCr published CIF
example files, fetched 2026-10-12. They are used here as parser
regression inputs. The provenance record is
`tests/real_data/SOURCE.md`. The files are (c) IUCr.
