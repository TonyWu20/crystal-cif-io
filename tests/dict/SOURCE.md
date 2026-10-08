# Dictionary source

- file: `cif_core.dic` (DDL1 core dictionary, "coreCIF")
- fetched: 2026-10-08
- source: `https://raw.githubusercontent.com/COMCIFS/DDL1-legacy-dictionaries/main/dictionaries/cif_core.dic`
- `_dictionary_version` in the file: 2.5.0-dev.1
- sha256: 56fd2d078706c97a4013a45c316339dd720010e6e7247a1a20d084c8c00e73d2

## Version note

The roadmap pins DDL1 v2.4.5 (`ftp.iucr.org/cifdics/cif_core.dic`).
That host is unreachable from the build machine (2026-10-08: the FTP
site times out, the IUCr web site blocks the fetch, the archive
mirror is rate limited). This file is the IUCr-managed DDL1 archive
at `COMCIFS/DDL1-legacy-dictionaries`.

T0 (the foundation tier) records the actual item counts in
`tests/dict_inventory.json`. If the total differs from the 734 of
v2.4.5, the coverage gate uses the recorded counts. The roadmap's
per-category counts are then guidance, not a hard pin.
