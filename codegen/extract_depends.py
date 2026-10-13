#!/usr/bin/env python3
"""Extract the item dependency data from the two pinned dictionaries.

Reads:
  tests/dict/cif_core.dic        DDL1 core dictionary
  tests/dict/cif_core_shell.dic  DDL2 (CIF 2.0) core shell

Writes codegen/depends.json. Per category it records the DDL1 item
groups (the `loop_ _name` families) and, per item, the source class,
range, container, expression, and the cross-references pulled from the
definition and description text. The committed JSON is a build
artifact: regenerate it with this script. The rerun check lives in
tests/depends_gen.rs.

Usage:
  python3 codegen/extract_depends.py [--ddl1 PATH] [--shell PATH] [--out PATH]
"""

import argparse
import hashlib
import json
import re
import sys
from pathlib import Path

REPO_ROOT = Path(__file__).resolve().parent.parent

DDL1_DEFAULT = REPO_ROOT / "tests" / "dict" / "cif_core.dic"
SHELL_DEFAULT = REPO_ROOT / "tests" / "dict" / "cif_core_shell.dic"
OUT_DEFAULT = REPO_ROOT / "codegen" / "depends.json"

# ---------------------------------------------------------------------------
# shared helpers
# ---------------------------------------------------------------------------


def unquote(value):
    v = value.strip()
    if len(v) >= 2 and v[0] == v[-1] and v[0] in "'\"":
        return v[1:-1]
    return v


def sha256_of(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


def dedent_lines(lines):
    stripped = [ln.rstrip() for ln in lines]
    indents = [len(ln) - len(ln.lstrip()) for ln in stripped if ln]
    pad = min(indents) if indents else 0
    out = []
    for ln in stripped:
        if ln:
            out.append(ln[pad:])
        else:
            out.append("")
    while out and not out[0]:
        out.pop(0)
    while out and not out[-1]:
        out.pop()
    return "\n".join(out)


def read_text_block(lines, i):
    """Collect a `;`-fenced text block starting at line i.

    The fence opens on the first line whose first non-blank character
    is `;` and closes on the next such line. Returns (text, next_i).
    """
    block = []
    opened = False
    while i < len(lines):
        ln = lines[i]
        if re.match(r"^\s*;", ln):
            if opened:
                block.append(ln)
                i += 1
                break
            opened = True
            block.append(ln)
            i += 1
            continue
        if opened:
            block.append(ln)
        i += 1
    while block and re.match(r"^\s*;", block[0]):
        first = block.pop(0)
        block[0:0] = [first.lstrip(";")]
    while block and re.match(r"^\s*;", block[-1]):
        block.pop()
    return dedent_lines(block), i


# ---------------------------------------------------------------------------
# DDL1 (CIF 1.1 dictionary) parsing
# ---------------------------------------------------------------------------

DDL1_FIELD_RE = re.compile(r"^( ? ? ? ? ?)(?:(loop_ *)?(_[A-Za-z_][A-Za-z0-9_]*))?\s*(.*)$")
DDL1_ENUM_HEADER = "_enumeration_detail"


def split_blocks(lines):
    blocks = []
    current = None
    for ln in lines:
        m = re.match(r"^data_(\S+)\s*$", ln)
        if m:
            if current:
                blocks.append(current)
            current = [m.group(1), []]
        elif current is not None:
            current[1].append(ln)
    if current:
        blocks.append(current)
    return blocks


def parse_ddl1_block(name, lines):
    """Parse one `data_` block. Returns a dict, or None for blocks
    that hold no item data (the dictionary metadata block)."""
    if name == "on_this_dictionary":
        return {"_meta": parse_ddl1_meta(lines)}

    fields = {}
    loops = {"_name": [], "_enumeration": [], "_related_item": []}
    definition = []
    state = "normal"
    cur_loop = None

    for ln in lines:
        if state == "text":
            if re.match(r"^\s*;\s*$", ln):
                state = "normal"
                definition.append(ln)
            else:
                definition.append(ln)
            continue
        if state == "skip_example":
            m = DDL1_FIELD_RE.match(ln)
            if m and m.group(3) is not None:
                state = "normal"
            else:
                continue
        m = DDL1_FIELD_RE.match(ln)
        tag = m.group(3) if m else None
        if state == "loop":
            if tag is not None:
                state = "normal"
            elif not ln.strip() or re.match(r"^\s*#", ln):
                continue
            else:
                collect_loop_value(cur_loop, ln, loops)
                continue
        if tag is None:
            continue
        loop_word, tag, value = m.group(2), m.group(3), m.group(4).strip()
        if loop_word:
            sub_name = tag
            if sub_name == "_name":
                loops["_name"] = [unquote(value)] if value else []
                state, cur_loop = "loop", "_name"
            elif sub_name == "_enumeration":
                loops["_enumeration"] = [unquote(value)] if value else []
                state, cur_loop = "loop", "_enumeration"
            elif sub_name == "_related_item":
                loops["_related_item"] = []
                state, cur_loop = "loop", "_related_item"
            elif sub_name in ("_list_link_child", "_list_link_parent"):
                loops[sub_name] = [unquote(value)] if value else []
                state, cur_loop = "loop", sub_name
            elif sub_name == "_example":
                state = "skip_example"
            else:
                state = "skip_example"
        else:
            if tag == "_definition" and not value:
                state = "text"
                continue
            if tag in ("_list_link_child", "_list_link_parent") and value:
                loops[tag] = [unquote(value)]
                continue
            fields[tag] = unquote(value) if value else None

    text, _ = read_text_block(definition, 0) if definition else (None, 0)
    rec = {
        "name": fields.get("_name"),
        "category": fields.get("_category"),
        "type": fields.get("_type"),
        "type_conditions": fields.get("_type_conditions"),
        "list": fields.get("_list"),
        "list_mandatory": fields.get("_list_mandatory"),
        "list_reference": fields.get("_list_reference"),
        "list_link_child": loops.get("_list_link_child") or None,
        "list_link_parent": loops.get("_list_link_parent") or None,
        "list_uniqueness": fields.get("_list_uniqueness"),
        "units": fields.get("_units"),
        "units_detail": fields.get("_units_detail"),
        "enumeration_range": fields.get("_enumeration_range"),
        "enumeration_values": normalize_enum(loops["_enumeration"]),
        "related_items": parse_related(loops["_related_item"]),
        "definition": text,
    }
    return {"_names": loops["_name"], **rec}


def parse_ddl1_meta(lines):
    fields = {}
    m = None
    in_text = False
    text_lines = []
    for ln in lines:
        fm = re.match(r"^    (_[A-Za-z_][A-Za-z0-9_]*)\s*(.*)$", ln)
        if in_text:
            if re.match(r"^\s*;", ln):
                in_text = False
            else:
                text_lines.append(ln)
            continue
        if fm:
            tag, value = fm.group(1), fm.group(2).strip()
            if tag == "_dictionary_history" and not value:
                in_text = True
                continue
            fields[tag] = unquote(value) if value else None
    return fields


def collect_loop_value(kind, ln, loops):
    tokens = ln.split()
    if not tokens:
        return
    if kind == "_name":
        loops["_name"].append(unquote(tokens[0]))
    elif kind == "_enumeration":
        if tokens[0] == DDL1_ENUM_HEADER:
            tokens = tokens[1:]
            if not tokens:
                return
        loops["_enumeration"].append(unquote(tokens[0]))
    elif kind == "_related_item":
        if tokens[0] == "_related_function":
            tokens = tokens[1:]
        if len(tokens) >= 2:
            loops["_related_item"].append((unquote(tokens[0]), tokens[1]))
        elif tokens:
            loops["_related_item"].append((unquote(tokens[0]), None))
    elif kind in ("_list_link_child", "_list_link_parent"):
        for tok in re.findall(r"'([^']*)'", ln):
            loops[kind].append(tok)
        if not re.findall(r"'([^']*)'", ln):
            loops[kind].append(unquote(tokens[0]))


def normalize_enum(values):
    out = []
    for v in values:
        if v and v != ".":
            out.append(v)
    return out or None


def parse_related(pairs):
    out = []
    for name, kw in pairs:
        out.append({"item": name.lstrip("_"), "function": kw})
    return out or None


def parse_ddl1(text):
    lines = text.splitlines()
    blocks = split_blocks(lines)
    items = {}
    groups = {}
    categories = []
    meta = None
    for name, blines in blocks:
        rec = parse_ddl1_block(name, blines)
        if rec is None:
            continue
        if "_meta" in rec:
            meta = rec["_meta"]
            continue
        if name.endswith("[]"):
            cat = name[:-2]
            if cat not in categories:
                categories.append(cat)
            continue
        category = rec["category"]
        if category not in categories:
            categories.append(category)
        if rec["_names"]:
            group = []
            for nm in rec["_names"]:
                nm = nm.lstrip("_")
                group.append(nm)
                items[nm] = {k: v for k, v in rec.items() if k != "_names"}
                items[nm]["name"] = nm
            if category not in groups:
                groups[category] = []
            groups[category].append(group)
        else:
            nm = rec["name"].lstrip("_")
            rec_out = {k: v for k, v in rec.items() if k not in ("_names",)}
            rec_out["name"] = nm
            items[nm] = rec_out
    return {"items": items, "groups": groups, "categories": categories, "meta": meta}


# ---------------------------------------------------------------------------
# shell (CIF 2.0 dictionary) parsing
# ---------------------------------------------------------------------------

SHELL_FIELD_RE = re.compile(r"^    (?:(loop_ *)?(_[A-Za-z_.][A-Za-z0-9_.-]*))?\s*(.*)$")
SHELL_HEADER_RE = re.compile(r"^      (_[A-Za-z_.][A-Za-z0-9_.-]*)\s*$")
SHELL_TEXT_TAGS = {"_description.text", "_method.expression"}
SHELL_SKIP_TAGS = ("_description_example",)


def split_frames(lines):
    frames = []
    current = None
    data_block = []
    in_data = False
    for ln in lines:
        if in_data:
            m = re.match(r"^save_(\S*)\s*$", ln)
            if m:
                in_data = False
                frames.append((m.group(1), []))
                current = frames[-1]
            else:
                data_block.append(ln)
            continue
        m = re.match(r"^data_(\S+)\s*$", ln)
        if m:
            in_data = True
            data_block = [ln]
            continue
        sm = re.match(r"^save_(\S*)\s*$", ln)
        if sm:
            frames.append((sm.group(1), []))
            current = frames[-1]
        elif current is not None:
            current[1].append(ln)
    return data_block, frames


def parse_shell_frame(lines):
    fields = {}
    loop_cols = {}
    state = "normal"
    cur_loop = None
    text_target = None
    pending = None
    pending_lines = []
    text_buf = []
    anon_text_col = None
    anon_text_buf = []
    anon_counter = 0
    cur_anon_key = None

    def flush_anon_text():
        nonlocal anon_text_col, anon_text_buf, anon_counter
        if anon_text_col is None:
            return
        text, _ = read_text_block(anon_text_buf, 0)
        loop_cols[cur_anon_key]["cols"].setdefault(anon_text_col, []).append(text)
        anon_text_col = None
        anon_text_buf = []

    for ln in lines:
        if state == "text":
            if re.match(r"^\s*;", ln):
                if any(re.match(r"^\s*;", t) for t in text_buf):
                    text, _ = read_text_block(text_buf + [ln], 0)
                    fields[text_target] = text
                    text_buf = []
                    state = "normal"
                else:
                    text_buf.append(ln)
            else:
                text_buf.append(ln)
            continue
        if state == "skip":
            if re.match(r"^    _[A-Za-z_.]", ln):
                state = "normal"
            else:
                continue
        if state == "pending":
            if re.match(r"^    _[A-Za-z_.]", ln):
                state = "normal"
                if pending_lines:
                    fields[pending] = unquote(" ".join(pending_lines))
                    pending_lines = []
                    pending = None
            elif not ln.strip():
                continue
            elif re.match(r"^\s*#", ln):
                continue
            elif re.match(r"^\s*;", ln):
                text_target = pending
                text_buf = [ln]
                pending = None
                pending_lines = []
                state = "text"
                continue
            else:
                pending_lines.append(ln.strip())
                continue
        m = SHELL_FIELD_RE.match(ln)
        tag = m.group(2) if m else None
        if state == "loop":
            if tag is not None:
                flush_anon_text()
                state = "normal"
            elif not ln.strip() or re.match(r"^\s*#", ln):
                continue
            elif re.match(r"^\s*;", ln):
                if anon_text_col is not None:
                    anon_text_buf.append(ln)
                    if re.match(r"^\s*;\s*$", ln):
                        flush_anon_text()
                elif cur_loop and cur_loop[0].startswith("anon_") and loop_cols[cur_loop[0]]["headers"]:
                    anon_text_col = loop_cols[cur_anon_key]["headers"][-1]
                    anon_text_buf = [ln]
                continue
            else:
                if anon_text_col is not None:
                    anon_text_buf.append(ln)
                else:
                    collect_shell_loop_value(cur_loop, ln, loop_cols)
                continue
        if tag is None:
            if m and m.group(3).strip().startswith("loop_"):
                anon_counter += 1
                cur_loop = ("anon_%d" % anon_counter, {})
                cur_anon_key = cur_loop[0]
                loop_cols[cur_loop[0]] = {"headers": [], "cols": {}}
                state = "loop"
            continue
        loop_word, tag, value = m.group(1), m.group(2), m.group(3).strip()
        if loop_word:
            if tag.startswith(SHELL_SKIP_TAGS):
                state = "skip"
                continue
            cur_loop = ("named", tag)
            loop_cols[tag] = []
            if value:
                loop_cols[tag].append(unquote(value))
            state = "loop"
            continue
        if tag in SHELL_TEXT_TAGS and not value:
            text_target = tag
            text_buf = []
            state = "text"
            continue
        if tag.startswith(SHELL_SKIP_TAGS) and not value:
            state = "skip"
            continue
        if not value:
            pending = tag
            pending_lines = []
            state = "pending"
            continue
        fields[tag] = unquote(value) if value else None

    flush_anon_text()
    if state == "text" and text_buf:
        text, _ = read_text_block(text_buf, 0)
        fields[text_target] = text
    if pending and pending_lines:
        fields[pending] = unquote(" ".join(pending_lines))

    for key, col in loop_cols.items():
        if key.startswith("anon_"):
            for header, values in col.get("cols", {}).items():
                fields.setdefault(header, [])
                if isinstance(fields[header], list):
                    fields[header].extend(values)
                else:
                    fields[header] = values
        else:
            fields[key] = loop_cols[key]
    return fields


def collect_shell_loop_value(kind, ln, loop_cols):
    line = ln.strip()
    if kind[0].startswith("anon_"):
        entry = loop_cols[kind[0]]
        hm = SHELL_HEADER_RE.match(ln)
        if hm:
            entry["headers"].append(hm.group(1))
            return
        tokens = line.split()
        for idx, tok in enumerate(tokens):
            if idx >= len(entry["headers"]):
                break
            header = entry["headers"][idx]
            entry.setdefault("cols", {}).setdefault(header, [])
            entry["cols"][header].append(unquote(tok))
    else:
        _, header = kind
        tokens = line.split()
        for tok in tokens:
            loop_cols[header].append(unquote(tok))


def parse_shell(text):
    lines = text.splitlines()
    data_block, frames = split_frames(lines)
    meta = {}
    for ln in data_block:
        m = re.match(r"^    (_[A-Za-z_.][A-Za-z0-9_.]*)\s*(.*)$", ln)
        if m:
            val = m.group(2).strip()
            if m.group(1) == "_dictionary.uri":
                continue
            meta[m.group(1)] = unquote(val) if val else None
    items = {}
    for code, flines in frames:
        if not code or "." not in code:
            continue
        f = parse_shell_frame(flines)
        definition_id = f.get("_definition.id")
        if not definition_id:
            continue
        aliases = normalize_shell_aliases(f)
        if aliases:
            canonical = aliases[0]
        else:
            cat = f.get("_name.category_id")
            obj = f.get("_name.object_id")
            if cat and obj:
                canonical = cat + "_" + obj
            else:
                canonical = definition_id.replace(".", "_")
        key = canonical.lstrip("_").lower()
        items[key] = {
            "definition_id": definition_id,
            "aliases": [a.lstrip("_") for a in aliases],
            "name": canonical.lstrip("_"),
            "category_id": f.get("_name.category_id"),
            "object_id": f.get("_name.object_id"),
            "linked_item_id": f.get("_name.linked_item_id"),
            "type_purpose": f.get("_type.purpose"),
            "type_source": f.get("_type.source"),
            "type_container": f.get("_type.container"),
            "type_dimension": f.get("_type.dimension"),
            "type_contents": f.get("_type.contents"),
            "enumeration_range": f.get("_enumeration.range"),
            "enumeration_default": f.get("_enumeration.default"),
            "enumeration_states": f.get("_enumeration_set.state"),
            "units_code": f.get("_units.code"),
            "method_purpose": f.get("_method.purpose"),
            "expression": f.get("_method.expression"),
            "description": f.get("_description.text"),
            "imported": "_import.get" in f,
            "import_spec": f.get("_import.get"),
            "replaced_by": None if f.get("_definition_replaced.by") == "." else f.get("_definition_replaced.by"),
            "update": f.get("_definition.update"),
            "object_name": cat + "_" + obj if (cat := f.get("_name.category_id")) and (obj := f.get("_name.object_id")) else None,
        }
    return {"items": items, "meta": meta}


def normalize_shell_aliases(f):
    aliases = []
    for key in ("_alias.definition_id", "loop_ _alias.definition_id"):
        value = f.get(key)
        if isinstance(value, list):
            aliases.extend(value)
        elif value:
            aliases.append(value)
    return [a.lstrip("_") for a in aliases if a and a != "."]


# ---------------------------------------------------------------------------
# merge + cross-reference extraction
# ---------------------------------------------------------------------------

REF_RE = re.compile(r"(_[A-Za-z][A-Za-z0-9_]*(?:\.[A-Za-z0-9_+/-]+)?)|(\*[._]?([A-Za-z0-9+/-]+))")


def extract_refs(text, flat_map, dotted_map, group_members, self_name=None):
    if not text:
        return []
    last_full = None
    refs = set()
    for m in REF_RE.finditer(text):
        full, elided, seg = m.group(1), m.group(2), m.group(3)
        if full:
            last_full = full
            refs.add(full)
            continue
        if elided and last_full:
            expanded = expand_elision(last_full, seg)
            refs.add(expanded)
    resolved = set()
    for tok in refs:
        if "." in tok:
            target = dotted_map.get(tok)
            if target:
                resolved.add(target)
            continue
        target = flat_map.get(tok)
        if target:
            resolved.add(target)
            continue
        head = tok.lstrip("_")
        if head in group_members:
            for member in group_members[head]:
                resolved.add(member)
    if self_name:
        resolved.discard(self_name)
    return sorted(resolved)


def expand_elision(full, seg):
    if "." in full:
        prefix, _ = full.rsplit(".", 1)
        return prefix + "." + seg
    parts = full.rsplit("_", 1)
    if len(parts) == 2:
        return parts[0] + "_" + seg
    return full


def build_result(ddl1, shell):
    ddl1_items = ddl1["items"]
    shell_items = shell["items"]

    flat_map = {}
    dotted_map = {}
    for key, rec in shell_items.items():
        dotted_map[rec["definition_id"]] = rec["name"]
        for alias in rec["aliases"]:
            flat_map.setdefault("_" + alias, rec["name"])
    for name in ddl1_items:
        flat_map.setdefault("_" + name, name)

    group_members = {}
    for cat_groups in ddl1["groups"].values():
        for group in cat_groups:
            head = group[0].rsplit("_", 1)[0] + "_"
            group_members.setdefault(head, set()).update(group)

    categories = {}

    def cat_bucket(cat):
        return categories.setdefault(
            cat,
            {"name": cat, "groups": [], "items": {}},
        )

    for cat, groups in sorted(ddl1["groups"].items()):
        cat_bucket(cat)["groups"] = [list(g) for g in groups]

    ddl1_keys_lower = {name.lower(): name for name in ddl1_items}
    for name, rec in sorted(ddl1_items.items()):
        bucket = cat_bucket(rec["category"])
        bucket["items"][name] = name

    shell_only = {}
    for key, rec in sorted(shell_items.items()):
        if key in ddl1_keys_lower:
            continue
        cat = rec["category_id"] or rec["definition_id"].split(".", 1)[0]
        shell_only[cat] = shell_only.get(cat, []) + [rec["name"]]
    shell_cats = set()
    for rec in shell_items.values():
        if rec["category_id"]:
            shell_cats.add(rec["category_id"])
    for cat in sorted(shell_cats):
        cat_bucket(cat)
    for cat, names in sorted(shell_only.items()):
        cat_bucket(cat)["items"].update({n: n for n in names})

    item_records = []
    for cat, bucket in sorted(categories.items()):
        for name in sorted(bucket["items"]):
            srec = shell_items.get(name.lower())
            drec = ddl1_items.get(name)
            record = build_item_record(name, drec, srec, flat_map, dotted_map, group_members)
            item_records.append((cat, record))

    cats_out = []
    for cat, bucket in sorted(categories.items()):
        recs = [r for c, r in item_records if c == cat]
        cats_out.append({"name": cat, "groups": bucket["groups"], "items": recs})
    return cats_out, shell_items, ddl1_items


def build_item_record(name, drec, srec, flat_map, dotted_map, group_members):
    ddef = drec.get("definition") if drec else None
    sdesc = srec.get("description") if srec else None
    sexpr = srec.get("expression") if srec else None
    if isinstance(sexpr, list):
        sexpr = "\n".join(b for b in sexpr if b)
    refs = extract_refs(" ".join(t for t in [ddef, sdesc, sexpr] if t), flat_map, dotted_map, group_members, name)

    source_class = None
    source_basis = "none"
    if srec and srec.get("type_source"):
        source_class = srec["type_source"]
        source_basis = "shell"
    elif ddef and "=" in ddef:
        source_class = "Derived"
        source_basis = "ddl1_definition"

    container = None
    container_basis = "none"
    if srec and srec.get("type_container"):
        container = srec["type_container"]
        container_basis = "shell"
    elif drec and drec.get("list") in ("yes", "both"):
        container = "Loop"
        container_basis = "ddl1"
    elif drec and drec.get("type") in ("numb", "char"):
        container = "Single"
        container_basis = "ddl1"

    rng = None
    range_basis = "none"
    if srec and srec.get("enumeration_range"):
        rng = srec["enumeration_range"]
        range_basis = "shell"
    elif drec and drec.get("enumeration_range"):
        rng = drec["enumeration_range"]
        range_basis = "ddl1"

    closed = None
    if srec and srec.get("enumeration_states"):
        closed = srec["enumeration_states"]
    elif drec and drec.get("enumeration_values"):
        closed = drec["enumeration_values"]

    ddl1_part = None
    if drec:
        ddl1_part = {
            "present": True,
            "type": drec.get("type"),
            "type_conditions": drec.get("type_conditions"),
            "list": drec.get("list"),
            "list_mandatory": drec.get("list_mandatory"),
            "list_reference": drec.get("list_reference"),
            "list_link_child": drec.get("list_link_child"),
            "list_link_parent": drec.get("list_link_parent"),
            "list_uniqueness": drec.get("list_uniqueness"),
            "units": drec.get("units"),
            "enumeration_range": drec.get("enumeration_range"),
            "related_items": drec.get("related_items"),
            "definition": drec.get("definition"),
        }
    shell_part = None
    if srec:
        shell_part = {
            "present": True,
            "definition_id": srec.get("definition_id"),
            "aliases": srec.get("aliases"),
            "category_id": srec.get("category_id"),
            "object_id": srec.get("object_id"),
            "purpose": srec.get("type_purpose"),
            "source": srec.get("type_source"),
            "container": srec.get("type_container"),
            "dimension": srec.get("type_dimension"),
            "contents": srec.get("type_contents"),
            "enumeration_range": srec.get("enumeration_range"),
            "enumeration_states": srec.get("enumeration_states"),
            "units": srec.get("units_code"),
            "linked_item_id": srec.get("linked_item_id"),
            "expression": srec.get("expression"),
            "description": srec.get("description"),
            "imported": srec.get("imported"),
            "import_spec": srec.get("import_spec"),
            "replaced_by": srec.get("replaced_by"),
        }

    return {
        "name": name,
        "source_class": source_class,
        "source_class_basis": source_basis,
        "range": rng,
        "range_basis": range_basis,
        "closed_values": closed,
        "container": container,
        "container_basis": container_basis,
        "expression": sexpr,
        "expression_source": "shell_method" if sexpr else ("ddl1_definition" if (ddef and "=" in ddef) else None),
        "refs": refs,
        "ddl1": ddl1_part,
        "shell": shell_part,
    }


# ---------------------------------------------------------------------------
# main
# ---------------------------------------------------------------------------


def compute_naming_exceptions(categories, shell_items, ddl1_items):
    json_names = {it["name"].lower() for cat in categories for it in cat["items"]}
    ddl1_keys = {name.lower() for name in ddl1_items}
    exceptions = []
    for key, rec in sorted(shell_items.items()):
        obj_name = rec.get("object_name")
        if not obj_name or key not in ddl1_keys:
            continue
        if obj_name.lower() in json_names:
            continue
        exceptions.append(
            {
                "definition_id": rec["definition_id"],
                "inventory_name": obj_name.lower(),
                "item": rec["name"].lower(),
            }
        )
    return exceptions


def main():
    ap = argparse.ArgumentParser(description=__doc__)
    ap.add_argument("--ddl1", default=str(DDL1_DEFAULT))
    ap.add_argument("--shell", default=str(SHELL_DEFAULT))
    ap.add_argument("--out", default=str(OUT_DEFAULT))
    args = ap.parse_args()

    ddl1_path = Path(args.ddl1)
    shell_path = Path(args.shell)
    out_path = Path(args.out)

    ddl1 = parse_ddl1(ddl1_path.read_text())
    shell = parse_shell(shell_path.read_text())

    categories, shell_items, ddl1_items = build_result(ddl1, shell)

    source_classes = {}
    containers = {}
    for cat in categories:
        for it in cat["items"]:
            if it["source_class"]:
                source_classes[it["source_class"]] = source_classes.get(it["source_class"], 0) + 1
            if it["container"]:
                containers[it["container"]] = containers.get(it["container"], 0) + 1

    doc = {
        "schema_version": 1,
        "generated_by": "codegen/extract_depends.py",
        "sources": {
            "ddl1": {
                "file": "tests/dict/cif_core.dic",
                "sha256": sha256_of(ddl1_path),
                "dictionary_name": ddl1["meta"].get("_dictionary_name") if ddl1["meta"] else None,
                "dictionary_version": ddl1["meta"].get("_dictionary_version") if ddl1["meta"] else None,
                "dictionary_update": ddl1["meta"].get("_dictionary_update") if ddl1["meta"] else None,
            },
            "shell": {
                "file": "tests/dict/cif_core_shell.dic",
                "sha256": sha256_of(shell_path),
                "dictionary_version": shell["meta"].get("_dictionary.version"),
                "dictionary_date": shell["meta"].get("_dictionary.date"),
                "ddl_conformance": shell["meta"].get("_dictionary.ddl_conformance"),
            },
        },
        "counts": {
            "categories": len(categories),
            "ddl1_items": len(ddl1_items),
            "shell_items": len(shell_items),
            "items": sum(len(c["items"]) for c in categories),
            "source_class": dict(sorted(source_classes.items())),
            "container": dict(sorted(containers.items())),
        },
        "naming_exceptions": compute_naming_exceptions(categories, shell_items, ddl1_items),
        "categories": categories,
    }
    out_path.write_text(json.dumps(doc, indent=2, sort_keys=True) + "\n")
    print(
        "wrote {out}: {n} categories, {i} items (ddl1 {d}, shell {s})".format(
            out=out_path,
            n=len(categories),
            i=sum(len(c["items"]) for c in categories),
            d=len(ddl1_items),
            s=len(shell_items),
        )
    )
    return 0


if __name__ == "__main__":
    sys.exit(main())
