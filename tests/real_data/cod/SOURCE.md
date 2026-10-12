# Source of the COD corpus

The files in `tests/real_data/cod/` are CIF entries from the
Crystallography Open Database (COD), fetched over the COD REST API.

- API doc: `https://wiki.crystallography.net/RESTful_API/`
- entry URL: `https://www.crystallography.net/cod/<ID>.cif`
  (revisions: `.../<ID>.cif@<rev>`)
- search URL: `https://www.crystallography.net/cod/result?<params>`
  (formats: `html`, `csv`, `json`, `lst`, `urls`, `zip`, `count`)
- fetched: 2026-10-12, with plain HTTP (`curl`)
- API summary: `docs/cod-api.md`

COD terms: data are released for the scientific community with
attribution (some entries carry a public-domain notice in the file
header).

| file | entry | why this file |
|---|---|---|
| `1000000.cif` | the API doc example entry | `?` unknown values, `_cod_duplicate_entry` tag, subscripts `~2~`, backslash values (`MoK\a`) |
| `1000001.cif` | Pristinamycin complex (SHELX) | long `_reflns_` loop, subscripts, Greek-letter escapes in unquoted values |
| `2011331.cif` | the duplicate-of original of 1000000 | SHELX monoclinic report, `_cod_data_source_file` provenance tag |
| `1529437.cif` | high-pressure ScF3 phase | `R -3 c :H` hexagonal-axis space group, compact 2-element file |
| `1531255.cif` | modulated perovskite (superspace) | `_symmetry_equiv_pos_as_xyz` loop, `_cod_original_*` tags, text field with leading spaces |
| `1542340.cif` | large measured structure | 120 KB stress file, 8 loops, `.` missing values, `2_767` symmetry codes |

Notes:

- All files are CIF 1.1 (no `##CIF_2.0` magic) and single `data_`
  block. Multi-block files and save frames are covered by the IUCr
  corpus instead.
- COD normalizes author names to ASCII; no non-ASCII bytes occur in
  these files.
- Entry access returns the newest revision; a specific revision is
  requested with `@<rev>`.
