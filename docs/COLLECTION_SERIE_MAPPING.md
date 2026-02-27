# Mapping MARC Series → Collection / Serie structs

This document explains how to build `Collection` and `Serie` structs (from your other project) from `Record::series()` and related accessors in marc-rs.

---

## Your target structs (summary)

```rust
// Collection: éditorial series with optional hierarchy (primary / secondary / tertiary title)
pub struct Collection {
    pub id: Option<i32>,
    pub key: Option<String>,
    pub primary_title: Option<String>,
    pub secondary_title: Option<String>,
    pub tertiary_title: Option<String>,
    pub issn: Option<String>,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

// Serie: author-based or “suite” series (e.g. “James Bond”, numéros de tomes)
pub struct Serie {
    pub id: Option<i32>,
    pub key: Option<String>,
    pub name: Option<String>,
    pub issn: Option<String>,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}
```

- `id`, `created_at`, `updated_at`: **never from MARC** — set when persisting in your DB.
- `key`: **derived** — stable key for deduplication (e.g. normalized title, or ISSN when present).

---

## Source data in marc-rs

- `record.series()` → `Vec<Series>` (enum with variants below).
- `record.collection_infos()` → `Vec<CollectionInfo>` (title, volume, issn, kind).
- `record.collection_links()` → links 410/411 (760/762): title, volume, issn, link_identifier.

### Series variants relevant for Collection / Serie

| Variant | MARC | Use for |
|--------|------|--------|
| `SeriesTitle(d)` | 440 / 225 | Mention (title, volume, issn). **Collection** or **Serie** depending on context. |
| `SeriesStatement(d)` | 490 / 225 | Same: **Collection** (if ISSN) or **Serie** (if $v crucial, no ISSN). |
| `SeriesUniformTitle(d)` | 830 | MARC21 uniform form → good **Collection** candidate (primary_title). |
| `SeriesAddedEntryPersonalName(d)` | 800 | Série auteur unique → **Serie.name**. |
| `SeriesAddedEntryCorporateName(d)` | 810 | → **Serie.name** (corporate). |
| `SeriesAddedEntryMeetingName(d)` | 811 | → **Serie.name** (meeting). |
| `SeriesPersonalName(d)` | 400 | Statement form → can feed **Serie.name**. |
| `SeriesCorporateName(d)` | 410 MARC21 | Idem. |
| `SeriesMeetingName(d)` | 411 | Idem. |

`d` for statement-like variants is `SeriesStatementData`: `statement` ($a), `volume` ($v), `issn` ($x), `subseries` ($i).

---

## How to build **Collection**

**When to create a Collection:**  
Entries that represent an editorial *collection* (often with ISSN): 225/490/830 with $x, or 830 (uniform) in MARC21.

1. **primary_title**
   - Prefer **830** (uniform): `SeriesUniformTitle(d)` → `d.statement`.
   - Else **225/490**: `SeriesTitle(d)` or `SeriesStatement(d)` → `d.statement`.
   - Optional: use first `record.collection_infos()` entry with `kind == Uniform`, then `Mention`.

2. **secondary_title / tertiary_title**
   - **225 $i** (sous-série): `SeriesStatementData.subseries` → e.g. secondary_title (or tertiary if you have two levels).
   - **Links 410/411**: `record.collection_links()` — link’s `title` can be the “parent” collection/série; you can map 410 → primary/secondary and 411 → secondary/tertiary depending on your hierarchy rules.

3. **issn**
   - From the same statement: `SeriesStatementData.issn` (225 $x / 490 $x).
   - Or from `CollectionInfo.issn` in `record.collection_infos()` for the chosen entry.

4. **key**
   - Derive from `primary_title` (normalize: lowercase, no accents, replace spaces with `_`) or from `issn` when present (e.g. `issn:{issn}`) so the same collection always yields the same key.

5. **id, created_at, updated_at**
   - Leave `None` when creating from MARC; set when inserting/updating in DB.

**Example (pseudo-code):**

```rust
// One Collection per 830, or per 225/490 that has ISSN (or first 225/490 if no 830)
for s in record.series() {
    match s {
        Series::SeriesUniformTitle(d) => {
            collections.push(Collection {
                id: None,
                key: make_key(&d.statement, d.issn.as_deref()),
                primary_title: Some(d.statement.clone()),
                secondary_title: d.subseries.clone(),
                tertiary_title: None,
                issn: d.issn.clone(),
                created_at: None,
                updated_at: None,
            });
        }
        Series::SeriesStatement(d) | Series::SeriesTitle(d) if d.issn.is_some() => {
            // or use as primary when no 830
            collections.push(Collection {
                primary_title: Some(d.statement.clone()),
                secondary_title: d.subseries.clone(),
                issn: d.issn.clone(),
                key: make_key(&d.statement, d.issn.as_deref()),
                ..Default::default()
            });
        }
        _ => {}
    }
}
```

---

## How to build **Serie**

**When to create a Serie:**  
Série/suite (often with volume numbers, “numéro de tome”): 225 with $v important, 411 (sous-série), or MARC21 800/810/811 (authorized series name).

1. **name**
   - **800/810/811**: `SeriesAddedEntryPersonalName(d)` → build display name from `PersonalNameData` (e.g. `d.name` + optional `d.dates`); same for Corporate/Meeting.
   - **400/410/411** (statement): idem from `PersonalNameData` / `CorporateNameData` / `MeetingNameData`.
   - **225/490** when treated as série: `SeriesStatementData.statement` ($a). Prefer when there is **no** ISSN and there is **$v** (volume).

2. **issn**
   - Usually **None** for série (“Généralement pas d'ISSN propre”). If you still have 225 $x on that entry, you can set `d.issn` for that statement.

3. **key**
   - Normalize `name` (e.g. slug) so the same série yields the same key.

4. **id, created_at, updated_at**
   - From DB, not MARC.

**Example (pseudo-code):**

```rust
for s in record.series() {
    match s {
        Series::SeriesAddedEntryPersonalName(d) | Series::SeriesPersonalName(d) => {
            let name = format!("{}", d.name); // or full display with dates, etc.
            series.push(Serie {
                id: None,
                key: make_key(&name),
                name: Some(name),
                issn: None,
                created_at: None,
                updated_at: None,
            });
        }
        Series::SeriesAddedEntryCorporateName(d) | Series::SeriesCorporateName(d) => {
            let name = corporate_display_name(d);
            series.push(Serie { name: Some(name), key: make_key(&name), .. });
        }
        Series::SeriesStatement(d) | Series::SeriesTitle(d) if d.volume.is_some() && d.issn.is_none() => {
            // Série/suite with volume number, no ISSN
            series.push(Serie {
                name: Some(d.statement.clone()),
                key: make_key(&d.statement),
                issn: d.issn.clone(),
                ..Default::default()
            });
        }
        _ => {}
    }
}
```

---

## Deduplication and links

- Use **key** (and optionally ISSN for collections) to deduplicate before inserting into your DB.
- **410/411** (`collection_links()`): use link’s `title` and `record_control_number` / `link_identifier` to associate the notice to the right parent Collection or Serie in your DB (e.g. set a `parent_id` or link table from notice → collection/serie).

---

## Summary table

| Field | Collection | Serie |
|-------|------------|--------|
| primary_title | 830/490/225 $a (or from 410 link $t) | — |
| secondary_title / tertiary_title | 225 $i, or 410/411 link titles | — |
| name | — | 800/810/811 display name, or 225 $a when $v present and no ISSN |
| issn | 225 $x / 490 $x | Usually None |
| key | From primary_title or ISSN | From name |
| id, created_at, updated_at | From DB | From DB |
