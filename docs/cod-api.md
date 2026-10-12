# COD REST API and CIF file notes

The Crystallography Open Database (COD) hosts CIF structure files.
The API doc is `https://wiki.crystallography.net/RESTful_API/`.
The corpus in `tests/real_data/cod/` was fetched from this API.
See `tests/real_data/cod/SOURCE.md` for the file table.

## Endpoints

| endpoint | use |
|---|---|
| `https://www.crystallography.net/cod/result?<params>` | search |
| `https://www.crystallography.net/cod/<id>.cif` | fetch CIF of entry `<id>` |
| `https://www.crystallography.net/cod/<id>.cif@<rev>` | fetch a revision |
| `https://www.crystallography.net/cod/<id>.hkl` | diffraction data, if present |
| `https://www.crystallography.net/cod/<id>.html` / `.rdf` | metadata table |
| `https://www.crystallography.net/cod/cgi-bin/cif-deposit.pl` | deposition (POST) |

Entry access returns the newest version by default. The `@<rev>`
suffix selects a revision. This works for `cif` and `hkl`.

## Search parameters

- `id` – comma or newline separated COD IDs. The `%` wildcard
  selects a range (observed: `id=20000%` lists `2000000`, ...).
- `smarts` – substructure query (SMARTS syntax).
- `text` – keyword in entry metadata (publication data, compound
  name, mineral name).
- `formula` – empirical formula in Hill order, space separated
  (example: `C8 H10 N4 O2`).
- `el1`..`el8` – elements that must appear.
- `nel1`..`nel4` – elements that must not appear.
- `strictmin`, `strictmax` – min/max count of distinct elements.
- `spacegroup` – Hermann–Mauguin or superspace group symbol.
- `space_group_number` – ITA number.
- `amin`/`amax`, `bmin`/`bmax`, `cmin`/`cmax` – lattice a/b/c range.
- `alpmin`/`alpmax`, `betmin`/`betmax`, `gamin`/`gamax` –
  angle ranges (spelled `gamin` in the API doc).
- `vmin`/`vmax` – cell volume range.
- `minZ`/`maxZ`, `minZprime`/`maxZprime` – Z and Z' ranges.
- `journal`, `year`, `volume`, `issue`, `doi` – publication fields.
- `has_fobs` – entry has X-ray reflection files.
- `include_duplicates`, `include_errors`, `include_theoretical` –
  keep entries that are otherwise excluded.
- `format` – `html` (default), `csv`, `json`, `lst`, `urls`, `zip`,
  `count`. The `zip` format has an entry limit; past the limit the
  server answers with an error page.

## Observed behavior (2026-10-12)

- `json` answers are arrays of row objects. Fields match the COD
  MySQL schema: `file`, `a`/`b`/`c` and sigmas, `alpha`/`beta`/
  `gamma`, `vol`, `celltemp`, `diffrtemp`, `cellpressure`,
  `nel`, `sg`, `sgHall`, `sgNumber`, `commonname`, `chemname`,
  `mineral`, `formula`, `calcformula`, `cellformula`, `Z`,
  `Zprime`, `authors`, `title`, `journal`, `year`, `doi`,
  `wavelength`, `radType`, `Rall`/`Robs`, `wRall`/`wRobs`,
  `gofref`, `duplicateof`, `optimal`, `status`, `flags`,
  `svnrevision`, `date`, `time`, `onhold`.
- Duplicate entries are excluded from search results by default.
  Entry 1000000 is a duplicate of 2011331 (the file carries
  `_cod_duplicate_entry 2011331`). With `include_duplicates=1`
  the ID appears.
- Empty result: the `count` and `lst` bodies are empty, not `0`
  or a placeholder.
- A range with no entries (observed: `id=45500%`) answers empty.
- Plain HTTP clients work; no auth or challenge page is seen on
  the search and entry endpoints.

## COD CIF file shape (parser relevant)

- Files start with an SVN comment header: `#` lines carrying
  `#$Date:`, `#$Revision:`, `#$URL: file:///...` plus a license
  or public-domain notice.
- Single `data_<id>` block. CIF 1.1. No `##CIF_2.0` magic.
- `?` marks unknown values (common).
- A bare `.` marks missing values (observed in `_reflns_` and
  `_geom_bond_` loops).
- Sub- and superscripts use `~2~`, `~+2~` in unquoted values and
  in `;`-terminated text fields.
- SHELX output uses backslash letter escapes for Greek letters:
  `\w`, `\a`, `\s`, `\f` in unquoted values (example: `MoK\a`).
- Symmetry codes keep underscores: `2_767`.
- COD provenance tags: `_cod_duplicate_entry`,
  `_cod_data_source_file`, `_cod_data_source_block`,
  `_cod_original_cell_volume`, `_cod_original_formula_sum`,
  `_cod_database_code`, `_cod_depositor_comments`.
- Author names are normalized to ASCII. The fetched files contain
  no non-ASCII bytes.

## Terms

Data are released to the scientific community. Two notice
variants appear in the corpus headers:

- a public-domain notice ("All data on this site have been
  placed in the public domain by the contributors").
- an IUCr Journals notice that requires attribution to the
  source article.

The repo-level credit and the per-entry citations live in
`NOTICE.md`.
