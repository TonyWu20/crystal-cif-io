# CIF 1.1 Groundtruth

This document defines the reference data for the CIF file syntax. It is the
acceptance target for the parser, now (winnow) and later (chumsky). The
groundtruth is parser-agnostic: it describes the CIF structure, not any
parser's internal types.

## Layout

```
tests/fixtures/*.cif            reference inputs
tests/fixtures/expected/*.json  expected canonical IR (or expected error)
src/canonical.rs                IR -> canonical JSON encoder + spec validation
tests/ground_truth.rs           verify harness
examples/gt_generate.rs         generator for the expected files
```

Each `.cif` fixture has a sibling JSON in `tests/fixtures/expected/`. The JSON
is either a positive expected document or a negative expected error.

## Canonical IR schema

A parsed CIF is encoded as JSON. This shape is the stable contract. A new
parser must serialize into the same shape.

```json
{
  "schema_version": 1,
  "fixture": "01_minimal.cif",
  "blocks": [
    {
      "name": "I",
      "members": [
        { "kind": "item", "tag": "cell_length_a", "value": { "...": "..." } },
        {
          "kind": "loop",
          "tags": ["atom_site_label", "atom_site_type_symbol"],
          "rows": [ [ {"...": "value"}, {"...": "value"} ] ]
        },
        {
          "kind": "save_frame",
          "name": "atom_site_label",
          "members": [ { "kind": "item", "tag": "atom_site_label_type", "value": { "...": "..." } } ]
        }
      ]
    }
  ]
}
```

Rules for the encoding:

- A `block.name` is the block code without the `data_` prefix.
- A `tag` is the data name without the leading underscore.
- A `loop` stores `tags` (the column headers) and `rows`. Each row is a list
  of values, one per column, in column order.
- A `save_frame` mirrors a block: `name` plus `members`. Save frames do not
  nest (spec para. 6).
- Comments and other white space are elided. The spec says comments carry no
  machine-readable content (para. 45).

### Value encoding

Each value object has a `kind`. The fields per kind are:

| kind          | fields                                                        |
|---------------|---------------------------------------------------------------|
| `inapplicable`| none (the `.` value)                                          |
| `unknown`     | none (the `?` value)                                          |
| `numeric`     | `type` (`integer` or `float`), `value` (string), `std_uncertainty` (string or null) |
| `char_string` | `style` (`unquoted`, `single_quoted`, `double_quoted`), `text` |
| `text_field`  | `text` (the joined lines)                                     |

Numeric values are stored as the shortest string form of the parsed number.
For example `90.0` becomes `"90"` and `.5` becomes `"0.5"`. This normalizes
the spelling of the number while keeping its value. A `std_uncertainty` comes
from the `(n)` suffix, stored as the digit string or null.

A `char_string` `text` is the content between the delimiters, exactly as
written. Quoted delimiters are stripped, embedded quotes are kept.

A `text_field` `text` is the lines joined with a newline. The first text line
is the remainder of the line holding the opening `;`. It may be empty.

## Fixture catalog

Each fixture exercises a slice of the CIF 1.1 file syntax. The spec
paragraphs are from the IUCr CIF 1.1 "File syntax" document.

| fixture                  | exercises                                                                  |
|--------------------------|----------------------------------------------------------------------------|
| `01_minimal.cif`         | one block, all single-value kinds: integer, float, quoted, `?`, `.`        |
| `02_numeric_forms.cif`   | every `<Numeric>` form: integer, signed, all `<Float>` forms, exponents, uncertainty |
| `03_strings.cif`         | unquoted, single-quoted, double-quoted; embedded delimiters; backslash; quoted numeric |
| `04_text_fields.cif`     | single-line, multi-line, empty, and reserved-bracket text fields           |
| `05_loops.cif`           | multi-column and single-column loops, mixed value kinds                    |
| `06_save_frames.cif`     | a save frame inside a data block, closed by a bare `save_`                |
| `07_multi_block.cif`     | a sequence of data blocks in one file                                       |
| `08_comments.cif`        | full-line and trailing comments, elided from the IR                        |
| `09_realistic_structure.cif` | a synthetic small-molecule structure: cell, symmetry, atom_site, aniso, geom_bond loops |
| `10_crlf_line_endings.cif`  | the same constructs with CRLF line endings                                  |
| `11_whitespace_edge.cif` | tab separators, a comment after white space, and `#` glued to a value       |
| `12_mixed_case.cif`      | mixed-case reserved words, a mixed-case save frame terminator, an empty block |

### Negative fixtures

A negative fixture must not parse (or must fail spec validation). The JSON
records the failure stage and the violation code.

| fixture                          | stage    | violation                 |
|----------------------------------|----------|---------------------------|
| `neg_global_block.cif`           | `parse`  | `unparseable` (forbidden `global_` block, para. 33) |
| `neg_reserved_word_value.cif`    | `parse`  | `unparseable` (reserved word as unquoted value) |
| `neg_tag_without_value.cif`      | `parse`  | `unparseable` (a tag with no value) |
| `neg_unterminated_text_field.cif`| `parse`  | `unparseable` (no closing `;`) |
| `neg_duplicate_block_name.cif`   | `validate` | `duplicate_block_name` (case-insensitive, para. 6) |
| `neg_duplicate_tag.cif`          | `validate` | `duplicate_tag_in_block` (para. 7) |

The `parse` stage means the parser itself must reject the input. The
`validate` stage means the parse succeeds but the spec semantic check must
reject the document.

## Running the harness

```
cargo test ground_truth_verify                  # verify (read-only)
cargo run --example gt_generate                 # write missing expected files
cargo run --example gt_generate -- --force      # overwrite existing expected files
```

- `ground_truth_verify` parses every fixture with the current parser,
  encodes it to the canonical JSON, and compares against the expected file.
  Negative fixtures are checked against their recorded stage and violation.
- The generator writes only missing expected files by default. With
  `--force` it overwrites existing files and re-stages the negative
  fixtures, with two protections:
  - a file with `"known_gap": true` is never overwritten. Fix the parser,
    then remove the flag by hand.
  - a human-written `expect_error.description` is preserved when the
    regenerated body keeps the same shape.
- After any run, review `git diff -- tests/fixtures/expected` before
  commit. The generator is an explicit command, not a test. `cargo test`
  never writes files.

### Known-gap flag

A positive expected file may carry `"known_gap": true`. This means the
expected JSON is spec-correct but the current parser disagrees with it. The
verify test reports such fixtures as gaps and does not fail on them. Remove
the flag once the parser produces the spec-correct output. The generator
never overwrites a flagged file, so the spec-correct expectations survive a
`--force` regeneration.

## Known gaps in the current (winnow) parser

The groundtruth records where the current parser diverges from the spec.

1. **Mid-line `.` and `?`.** The `inapplicable` and `unknown` parsers only
   accept `.` and `?` at end of line. Inside a loop row, a `.` or `?` that is
   not the last token is returned as a `char_string` instead of
   `inapplicable` / `unknown`. This is the gap recorded by
   `09_realistic_structure.json` (`"known_gap": true`). The spec Appendix A
   `<Value>` production allows `.` and `?` anywhere a value occurs.
2. **Lossy top level.** `CifDocument::parse_from_str` uses `opt` for the
   block sequence. Garbage that is not a data block yields `Ok` with no
   blocks and an unconsumed tail, instead of an error. The harness detects
   this with the full-consumption check.
3. **No semantic validation.** The parser does not enforce unique block
   names, unique tags, or save-frame code rules (spec para. 6, 7). The
   harness `validate` function supplies these checks for now.
4. **Numeric token spelling is not preserved.** The IR stores parsed numbers
   (`i32` / `f32`), not the source token. The canonical JSON normalizes to
   the shortest number form. The chumsky refactor should keep the raw token
   to support round-trip writing, matching the `castep_cell_fmt` style.
5. **Floats are stored as `f32`.** Values with more than about six
   significant digits lose precision. The groundtruth numbers are short
   enough to be exact, but the refactor should store `f64`.

## Notes for the chumsky refactor

- The acceptance criterion is unchanged: the new parser must serialize to the
  same canonical JSON for every fixture.
- Replace `src/canonical.rs` with an encoder over the new IR, or
  keep this one if the IR shape stays the same.
- Use `ariadne` for parse errors, matching the `castep_cell_fmt` error
  style.
- Keep the raw source token for every value so a writer can round-trip.
- Close the gaps above, then remove each `"known_gap"` flag and re-run the
  verify test.

## Spec references

- IUCr CIF 1.1 "File syntax":
  https://www.iucr.org/what-we-do/digital-standards/cif/cif1/file-syntax
- Mirror of the same working specification:
  https://www.cristal.org/DU-SDPD/nexus/iucr2/iucr-top/cif/spec/version1/cifsynta.htm
