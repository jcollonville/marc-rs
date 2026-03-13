# Structure UNIMARC Bibliographique — Référence complète des zones et sous-zones

> **Source :** *UNIMARC Bibliographic Format Manual*, édition en ligne v1.1.0, 2024 (IFLA/PUC).
>
> **Répétabilité :** R = Répétable · NR = Non répétable
>
> **Occurrence :** M = Obligatoire · MA = Obligatoire si applicable · O = Optionnel
>
> Les zones du **Bloc 1 (1xx)** contenant des données codées à longueur fixe sont enrichies
> d'un tableau d'offsets (positions de caractères) et des valeurs de code autorisées.

## Table des matières

- [BLOC 0 — IDENTIFICATION](#0)
- [BLOC 1 — INFORMATIONS CODÉES](#1)
- [BLOC 2 — INFORMATIONS DESCRIPTIVES](#2)
- [BLOC 3 — NOTES](#3)
- [BLOC 4 — LIENS (LINKING ENTRY)](#4)
- [BLOC 5 — TITRES ASSOCIÉS](#5)
- [BLOC 6 — ANALYSE SUJET & HISTORIQUE BIBL.](#6)
- [BLOC 7 — RESPONSABILITÉ](#7)
- [BLOC 8 — USAGE INTERNATIONAL](#8)
- [BLOC 9 — USAGE NATIONAL](#9)

---

## BLOC 0 — IDENTIFICATION

### Zone 001 — RECORD IDENTIFIER

**Répétabilité :** Non rép. &nbsp;|&nbsp; **Occurrence :** Oblig.

*Champ de contrôle — pas de sous-zones ni d'indicateurs.*

### Zone 003 — PERSISTENT RECORD IDENTIFIER

**Répétabilité :** Non rép. &nbsp;|&nbsp; **Occurrence :** Optionnel

*Champ de contrôle — pas de sous-zones ni d'indicateurs.*

### Zone 005 — VERSION IDENTIFIER

**Répétabilité :** Non rép. &nbsp;|&nbsp; **Occurrence :** Optionnel

*Champ de contrôle — pas de sous-zones ni d'indicateurs.*

### Zone 010 — INTERNATIONAL STANDARD BOOK NUMBER (ISBN)

**Répétabilité :** Rép. &nbsp;|&nbsp; **Occurrence :** Optionnel

| Sous-zone | Nom | Rép. | Occurrence | Description |
|-----------|-----|------|------------|-------------|
| `$a` | Number (ISBN) | Non rép. | Optionnel | A correctly applied ISBN including hyphens. The ISBN are assigned by the designated agency in each country. |
| `$b` | Qualification | Rép. | Optionnel | An indication of the scope of the ISBN in subfield $a (if present), usually the name of a publisher, an indication of the binding of the resource, or an indication of the relationship of an ISBN to... |
| `$d` | Terms of Availability and/or Price | Non rép. | Optionnel | The price of the resource and any comment on its availability (EX 1, 2, 3, 4, 5, 6, 8). |
| `$z` | Erroneous ISBN | Rép. | Optionnel | An ISBN that has been identified as being erroneously applied to the resource or is otherwise invalid. It may have been allocated to two different publications and in this instance cancelled (EX 8)... |
| `$6` | Interfield Linking Data | Rép. | Optionnel | This subfield contains information allowing the field to be linked for processing purposes to other fields in the record. (See section 3.13 Interfield Linking Data, 3 Format structure). |

### Zone 011 — ISSN

**Répétabilité :** Rép. &nbsp;|&nbsp; **Occurrence :** Optionnel

| Sous-zone | Nom | Rép. | Occurrence | Description |
|-----------|-----|------|------------|-------------|
| `$a` | Number (ISSN) | Non rép. | Optionnel | A correctly applied ISSN including the hyphen between the fourth and fifth digits. |
| `$b` | Qualification | Non rép. | Optionnel | Used to distinguish between ISSN if a record contains more than one. Not used by the ISSN International Centre. |
| `$d` | Terms of Availability and/or Price | Rép. | Optionnel | The price of the resource and any comment on its availability (EX 3, 6). |
| `$f` | Cluster ISSN Identifier | Non rép. | Optionnel | ===PAGE 37=== |
| `$g` | Cancelled Cluster ISSN | Rép. | Optionnel |  |
| `$y` | Cancelled ISSN | Rép. | Optionnel |  |
| `$z` | Erroneous ISSN or Cluster ISSN | Rép. | Optionnel |  |
| `$2` | Source | Non rép. | Oblig. si appl. |  |
| `$3` | Authority Record Identifier or Standard Number | Non rép. | Optionnel |  |

### Zone 012 — FINGERPRINT IDENTIFIER

**Répétabilité :** Rép. &nbsp;|&nbsp; **Occurrence :** Optionnel

| Sous-zone | Nom | Rép. | Occurrence | Description |
|-----------|-----|------|------------|-------------|
| `$a` | Fingerprint | Non rép. | Optionnel | Calculated Fingerprint Identifier. The Fingerprint is assigned by the agency creating a record. |
| `$2` | Source | Non rép. | Optionnel | An identification in coded form of the system from which fingerprint identifier is derived. |
| `$5` | Institution to which the Field Applies | Non rép. | Optionnel | Name of the institution to which field applies in the coded form if it is considered necessary by the agency which creates the record to identify specific attributes of an item. If the subfield is ... |

### Zone 013 — INTERNATIONAL STANDARD MUSIC NUMBER (ISMN)

**Répétabilité :** — &nbsp;|&nbsp; **Occurrence :** —

| Sous-zone | Nom | Rép. | Occurrence | Description |
|-----------|-----|------|------------|-------------|
| `$a` | Number (ISMN) | Non rép. | Optionnel | A correctly applied ISMN including hyphens. The ISMN are assigned by the designated agency in each country. |
| `$b` | Qualification | Non rép. | Optionnel | An indication of the scope of the ISMN in subfield $a (if present), usually the name of a publisher, an indication of the binding of the resource, or an indication of the relationship of an ISMN to... |
| `$d` | Terms of Availability and/or Price | Non rép. | Optionnel | The price of the resource, and any comment on its availability. Where there is an 010 International Standard Book Number (ISBN) field and the contents of this subfield would simply repeat what is i... |
| `$z` | Erroneous ISMN | Rép. | Optionnel | An ISMN that has been identified as being erroneously applied to an the resource or is otherwise invalid. It may have been allocated to two different publications and in this instance cancelled, or... |
| `$6` | Interfield Linking Data | Rép. | Optionnel | This subfield contains information allowing the field to be linked for processing purposes to other fields in the record. (See section 3.13 Interfield Linking Data, 3 Format structure). (EX 6). |

### Zone 014 — ARTICLE IDENTIFIER

**Répétabilité :** Rép. &nbsp;|&nbsp; **Occurrence :** Optionnel

| Sous-zone | Nom | Rép. | Occurrence | Description |
|-----------|-----|------|------------|-------------|
| `$a` | Article Identifier | Non rép. | Optionnel |  |
| `$z` | Erroneous Article Identifier | Rép. | Optionnel |  |
| `$2` | Source | Non rép. | Optionnel | The subfield contains an identification of the system under which the identifier was constructed. Not repeatable. Codes: biblid Bibliographic Identification of Contributions in Serials and Books (I... |

### Zone 015 — INTERNATIONAL STANDARD TECHNICAL REPORT NUMBER (ISRN)

**Répétabilité :** — &nbsp;|&nbsp; **Occurrence :** —

| Sous-zone | Nom | Rép. | Occurrence | Description |
|-----------|-----|------|------------|-------------|
| `$a` | Number (ISRN) | Non rép. | Optionnel | A correctly applied ISRN including hyphens. The ISRN are assigned by the designated agency in each country. |
| `$b` | Qualification | Non rép. | Optionnel | An indication of the scope of the ISRN in subfield $a (if present), usually the name of a publisher, an indication of the binding of the resource, or an indication of the relationship of an ISRN to... |
| `$d` | Terms of Availability and/or Price | Non rép. | Optionnel | The price of the resource and any comment on its availability. |
| `$z` | Cancelled/Invalid/Erroneous ISRN | Rép. | Optionnel | An ISRN that has been identified as being erroneously applied to resource or is otherwise invalid. It may have been allocated to two different publications and in this instance cancelled, or it may... |

### Zone 016 — INTERNATIONAL STANDARD RECORDING CODE (ISRC)

**Répétabilité :** — &nbsp;|&nbsp; **Occurrence :** —

| Sous-zone | Nom | Rép. | Occurrence | Description |
|-----------|-----|------|------------|-------------|
| `$a` | Number (ISRC) | Non rép. | Oblig. si appl. | A correctly applied ISRC including hyphens. The ISRCs are assigned by the designated agency in each country. Mandatory unless $z is present. |
| `$b` | Qualification | Non rép. | Optionnel | An indication of the scope of the ISRC in subfield $a (if present), usually an indication of the relationship of an ISRC to a particular expression. |
| `$z` | Erroneous ISRC | Rép. | Optionnel | An ISRC that has been identified as being erroneously applied to a recording or is otherwise invalid. It may have been allocated to two different recordings and in this instance cancelled, or it ma... |

### Zone 017 — OTHER IDENTIFIER

**Répétabilité :** Rép. &nbsp;|&nbsp; **Occurrence :** Optionnel

| Sous-zone | Nom | Rép. | Occurrence | Description |
|-----------|-----|------|------------|-------------|
| `$a` | Identifier | Non rép. | Optionnel | A correctly formatted identifier. Numbers or codes are formulated according to type. |
| `$b` | Qualification | Non rép. | Optionnel | An indication of the scope of the identifier in subfields $a (if present), usually the name of publisher, an identification of the binding of the resource, or an indication of the relationship of a... |
| `$d` | Terms of Availability and/or Price | Non rép. | Optionnel | The price of the resource and any comment on its availability. |
| `$z` | Erroneous Identifier | Rép. | Optionnel | ===PAGE 55=== |
| `$2` | Source | Non rép. | Optionnel |  |

### Zone 020 — NATIONAL BIBLIOGRAPHY NUMBER

**Répétabilité :** Rép. &nbsp;|&nbsp; **Occurrence :** Optionnel

| Sous-zone | Nom | Rép. | Occurrence | Description |
|-----------|-----|------|------------|-------------|
| `$a` | Country Code | Non rép. | Optionnel | An identification of the country of the national bibliography (EX 1-4). Two characters. For country codes use ISO 3166-1. |
| `$b` | Number | Non rép. | Optionnel | The number as assigned by the agency (EX 1-4). |
| `$z` | Erroneous Number | Rép. | Optionnel | A number erroneously assigned to a record (EX 2). |

### Zone 021 — LEGAL DEPOSIT NUMBER

**Répétabilité :** Rép. &nbsp;|&nbsp; **Occurrence :** Optionnel

| Sous-zone | Nom | Rép. | Occurrence | Description |
|-----------|-----|------|------------|-------------|
| `$a` | Country Code | Non rép. | Optionnel | An identification of the country of the legal deposit agency assigning the number. For country codes use ISO 3166-1. Two characters. |
| `$b` | Number | Non rép. | Optionnel | As assigned by the agency. |
| `$z` | Erroneous Number | Rép. | Optionnel | A number erroneously assigned to the resource. |

### Zone 022 — GOVERNMENT PUBLICATION NUMBER

**Répétabilité :** Rép. &nbsp;|&nbsp; **Occurrence :** Optionnel

| Sous-zone | Nom | Rép. | Occurrence | Description |
|-----------|-----|------|------------|-------------|
| `$a` | Country Code | Non rép. | Optionnel | An identification of the country in which the resource is published (EX l-3). Two characters. For country codes use ISO 3166-1. |
| `$b` | Number | Non rép. | Optionnel | The number as assigned by the government body. |
| `$z` | Erroneous Number | Rép. | Optionnel | A number erroneously assigned to a government publication. |

### Zone 033 — OTHER SYSTEM PERSISTENT RECORD IDENTIFIER

**Répétabilité :** — &nbsp;|&nbsp; **Occurrence :** —

| Sous-zone | Nom | Rép. | Occurrence | Description |
|-----------|-----|------|------------|-------------|
| `$a` | Persistent Record Identifier | Non rép. | Optionnel | Non repeatable. |
| `$z` | Cancelled or Invalid Persistent Record Identifier | Rép. | Optionnel |  |

### Zone 035 — OTHER SYSTEM IDENTIFIERS

**Répétabilité :** Rép. &nbsp;|&nbsp; **Occurrence :** Optionnel

| Sous-zone | Nom | Rép. | Occurrence | Description |
|-----------|-----|------|------------|-------------|
| `$a` | System Identifier | Non rép. | Optionnel | A code for the organisation in parentheses followed by the system identifier for the record in that organisation’s database. Since there are no internationally accepted codes, the codes from MARC C... |
| `$z` | Cancelled or Invalid Identifier | Rép. | Optionnel |  |

### Zone 036 — MUSIC INCIPIT

**Répétabilité :** Rép. &nbsp;|&nbsp; **Occurrence :** Optionnel

| Sous-zone | Nom | Rép. | Occurrence | Description |
|-----------|-----|------|------------|-------------|
| `$a` | Number of Work | Non rép. | Oblig. si appl. | A two-digit code indicates the work to which the incipit refers, if a set of compositions (e.g. six sonatas) is entirely described in a single record, without the use of pieceanalytic level records... |
| `$b` | Number of Movement | Non rép. | Oblig. si appl. | A two-digit code indicates the movement within a work to which the incipit refers. If the work has only one movement use “01”. Mandatory. Not repeatable. E.g. An incipit describing the third moveme... |
| `$c` | Number of Incipit | Non rép. | Oblig. si appl. | A two-digit code distinguishes different incipits referring to the same movement. If there is only one incipit for a movement use “01”. Mandatory. Not repeatable. ===PAGE 65=== |
| `$d` | Voice/Instrument | Non rép. | Oblig. si appl. |  |
| `$e` | Role | Non rép. | Optionnel |  |
| `$f` | Movement Caption/Heading | Rép. | Optionnel |  |
| `$g` | Key or Mode | Non rép. | Optionnel |  |
| `$m` | Clef | Non rép. | Oblig. si appl. |  |
| `$n` | Key Signature | Non rép. | Optionnel |  |
| `$o` | Time Signature | Non rép. | Optionnel |  |
| `$p` | Musical Notation | Non rép. | Optionnel |  |
| `$q` | Comments (free text) | Rép. | Optionnel |  |
| `$r` | Codified Note | Non rép. | Optionnel |  |
| `$t` | Text Incipit | Rép. | Optionnel |  |
| `$u` | Uniform Resource Identifier (URI) | Rép. | Optionnel |  |
| `$z` | Language of Text | Rép. | Optionnel |  |
| `$2` | Source | Non rép. | Oblig. si appl. |  |

### Zone 040 — CODEN

**Répétabilité :** Rép. &nbsp;|&nbsp; **Occurrence :** Optionnel

| Sous-zone | Nom | Rép. | Occurrence | Description |
|-----------|-----|------|------------|-------------|
| `$a` | CODEN | Non rép. | Optionnel | The six character code of which the last character is an alphanumeric digit. |
| `$z` | Erroneous CODEN | Rép. | Optionnel | A CODEN that has been identified as being erroneously applied to the resource or is otherwise invalid. It may have been allocated to two different publications and in this instance cancelled, or it... |

### Zone 071 — PUBLISHER’S NUMBER

**Répétabilité :** Rép. &nbsp;|&nbsp; **Occurrence :** Optionnel

| Sous-zone | Nom | Rép. | Occurrence | Description |
|-----------|-----|------|------------|-------------|
| `$a` | Publisher’s Number | Non rép. | Optionnel | The number as assigned by the publisher. |
| `$b` | Source | Non rép. | Optionnel | The publisher which assigned the number. |
| `$c` | Qualification | Non rép. | Optionnel | ===PAGE 71=== |
| `$d` | Terms of Availability and/or Price | Non rép. | Optionnel |  |
| `$z` | Erroneous Publisher’s Number | Non rép. | Optionnel |  |

### Zone 072 — UNIVERSAL PRODUCT CODE (UPC)

**Répétabilité :** Rép. &nbsp;|&nbsp; **Occurrence :** Optionnel

| Sous-zone | Nom | Rép. | Occurrence | Description |
|-----------|-----|------|------------|-------------|
| `$a` | Standard Number | Non rép. | Optionnel | A correctly formatted standard number or code. Number or code is formatted according to type. |
| `$b` | Qualification | Non rép. | Optionnel | An indication of the scope of the number or code in subfield $a, usually the name of a publisher, an indication of the binding of the resource, or an indication of the relationship of a number or c... |
| `$d` | Terms of Availability and/or Price | Non rép. | Optionnel | The price of the resource and any comment on its availability. |
| `$z` | Erroneous Number or Code | Rép. | Optionnel | A number or code that has been identified as being erroneously applied to the resource or is otherwise invalid. It may have been allocated to two different publications or products and in this inst... |

### Zone 073 — INTERNATIONAL ARTICLE NUMBER (EAN)

**Répétabilité :** Rép. &nbsp;|&nbsp; **Occurrence :** Optionnel

| Sous-zone | Nom | Rép. | Occurrence | Description |
|-----------|-----|------|------------|-------------|
| `$a` | Standard Number | Non rép. | Optionnel | A correctly formatted standard number or code. |
| `$b` | Qualification | Non rép. | Optionnel | An indication of the scope of the number or code in subfield $a, usually the name of a publisher, an indication of the binding of the resource, or an indication of the relationship of a number or c... |
| `$d` | Terms of Availability and/or Price | Non rép. | Optionnel | The price of the resource, and any comment on its availability. |
| `$z` | Erroneous Number or Code | Rép. | Optionnel | A number or code that has been identified as being erroneously applied to the resource, or is otherwise invalid. It may have been allocated to two different publications or products and in this ins... |

---

## BLOC 1 — INFORMATIONS CODÉES

### Zone 100 — GENERAL PROCESSING DATA

**Répétabilité :** Non rép. &nbsp;|&nbsp; **Occurrence :** Oblig.

> 🔢 **Champ à données codées** — sous-champ(s) à longueur fixe décodables par position de caractère.

| Sous-zone | Nom | Rép. | Occurrence | Description |
|-----------|-----|------|------------|-------------|
| `$a` | General Processing Data | Non rép. | Oblig. | Codes indicate the aspects of general processing. The subfield is 36-characters in length. Mandatory. Not repeatable. |

#### 🔢 Structure interne par position de caractère

| Offset | Lg. | Élément | Obl. |
|--------|-----|---------|------|
| `0-7` | 8 | Date Entered on File | ✓ |
| `8` | 1 | Type of Date | |
| `9-12` | 4 | Date 1 |  |
| `13-16` | 4 | Date 2 |  |
| `17-19` | 3 | Target Audience Code |  |
| `20` | 1 | Government Publication Code | |
| `21` | 1 | Modified Record Code | |
| `22-24` | 3 | Language of Cataloguing | ✓ |
| `25` | 1 | Transliteration Code | |
| `26-29` | 4 | Character Set | ✓ |
| `30-33` | 4 | Additional Character Set |  |
| `34-35` | 2 | Script of Title |  |

##### Pos. `8` — Type of Date

| Code | Signification |
|------|---------------|
| `a` | ressource continue en cours de publication |
| `b` | continuing |
| `c` | continuing |
| `d` | monograph |
| `e` | reproduction d'un document |
| `f` | monograph, date |
| `g` | monographie dont la publication s'étend sur plus d'un an |
| `h` | monographie avec date |
| `i` | monographie avec date |
| `j` | document avec date détaillée |
| `k` | monograph |
| `l` | dates incluses d'une collection |
| `u` | date(s) de publication inconnue(s) |

##### Pos. `17-19` — Target Audience Code

| Code | Signification |
|------|---------------|
| `a` | juvenile, general |
| `b` | pre-primary, ages 0-5 |
| `c` | primary, ages 5-10 |
| `d` | children, ages 9-14 |
| `e` | young adult, ages 14-20 |
| `k` | adult, serious |
| `m` | adult, general |
| `u` | unknown |
| `x` | not applicable |

##### Pos. `20` — Government Publication Code

| Code | Signification |
|------|---------------|
| `a` | federal/national Sovereign states and territories with some measure of self-government, for |
| `b` | state/province An administrative subdivision at one level below the federal or national level, |
| `c` | county/département |
| `d` | local (municipal, etc.) |
| `e` | multi-local (interstate, etc., au-dessous du niveau national) |
| `f` | intergovernmental International organisations and their agencies, e.g. those entities which are |
| `g` | gouvernement en exil ou clandestin |
| `h` | niveau non déterminé |
| `u` | unknown Used when it is impossible to determine whether the work is a government |
| `y` | pas une publication gouvernementale |
| `z` | autre niveau gouvernemental |

##### Pos. `21` — Modified Record Code

| Code | Signification |
|------|---------------|
| `0` | unmodified record |
| `1` | modified record |

##### Pos. `25` — Transliteration Code

| Code | Signification |
|------|---------------|
| `a` | ISO transliteration scheme |
| `b` | other Used for identified transliteration |
| `c` | multiple transliterations ISO or other schemes. |
| `d` | transliteration table established by the national |
| `e` | transliteration without any identified transliteration |
| `f` | other identified transliteration scheme |
| `g` | ALA-LC romanization table |
| `h` | DIN transliteration scheme |
| `y` | not applicable  No transliteration scheme used. |

##### Pos. `26-29` — Character Set

| Code | Signification |
|------|---------------|
| `01` | ISO 646, IRV version (basic Latin set) |
| `02` | ISO Registration # 37 (basic Cyrillic set) |
| `03` | ISO 5426 (extended Latin set) |
| `04` | ISO 5427 (extended Cyrillic set) |
| `05` | ISO 5428 (Greek set) |
| `06` | ISO 6438 (African coded character set) |
| `07` | ISO 10586 (Georgian set) |
| `08` | ISO 8957 (Hebrew set) Table 1 |
| `09` | ISO 8957 (Hebrew set) Table 2 |
| `10` | [Reserved] |
| `11` | ISO 5426-2 (Latin characters used in minor European languages and obsolete typography) |
| `50` | ISO 10646 Level 3 (Unicode, UTF-8) |

##### Pos. `34-35` — Script of Title

| Code | Signification |
|------|---------------|
| `ba` | Latin |
| `ca` | Cyrillic |
| `da` | Japanese -- script unspecified (mixed scripts) |
| `db` | Japanese – kanji |
| `dc` | Japanese – kana |
| `ea` | Chinese |
| `eb` | Chinese – simplified variant |
| `ec` | Chinese – traditional variant |
| `ed` | Mongolian |
| `ee` | Manchu |
| `eh` | Naxi Geba |
| `fa` | Arabic |
| `ga` | Greek |
| `ha` | Hebrew |
| `ia` | Thai |
| `ib` | Burmese |
| `ic` | Khmer (Cambodian) |
| `ja` | Devanagari |
| `jb` | Bengalese |
| `jc` | Gujarati |
| `jd` | Gurmukhi |
| `je` | Odia (Oriya) |
| `jf` | Tibetan |
| `jg` | Newa (Newar) |
| `ka` | Korean |
| `la` | Tamil |
| `lb` | Kannada |
| `lc` | Malayalam |
| `ld` | Sinhala (Singhalese) |
| `le` | Telugu |
| `lf` | Grantha |
| `ma` | Georgian |
| `mb` | Armenian |
| `na` | Ethiopic (Ge’ez) |
| `nb` | Tifinagh (Berber) |
| `nc` | N’ko |
| `oa` | Syriac |
| `pa` | Egyptian hieroglyphs |
| `zz` | Other |
| `8` | a Currently published continuing resource. |
| `20` | c A subdivision of a sovereign state, without any legislature. |
| `21` | 0 Unmodified record. |
| `25` | y No transliteration scheme used. |

### Zone 101 — LANGUAGE OF THE RESOURCE

**Répétabilité :** Rép. &nbsp;|&nbsp; **Occurrence :** Oblig. si appl.

| Sous-zone | Nom | Rép. | Occurrence | Description |
|-----------|-----|------|------------|-------------|
| `$a` | Language of Text, Soundtrack etc. [LRM: Expression] | Rép. | Optionnel |  |
| `$c` | Language of Original Work [LRM: Expression] | Rép. | Optionnel |  |
| `$d` | Language of Summary [LRM: Expression] | Rép. | Optionnel |  |
| `$e` | Language of Contents Page | Rép. | Optionnel |  |
| `$h` | Language of Libretto, etc. | Rép. | Optionnel |  |
| `$j` | Language of Subtitles [LRM: Expression] | Rép. | Optionnel |  |
| `$2` | Source | Non rép. | Optionnel |  |

### Zone 102 — COUNTRY OF PUBLICATION OR PRODUCTION

**Répétabilité :** — &nbsp;|&nbsp; **Occurrence :** —

| Sous-zone | Nom | Rép. | Occurrence | Description |
|-----------|-----|------|------------|-------------|
| `$a` | Country of Publication | Rép. | Optionnel | Contains a code representing the country in which the resource was published or produced. For country codes use ISO 3166-1. Repeatable if the resource is published in more than one country or in mo... |
| `$b` | Locality (non-ISO) | Rép. | Optionnel | A code representing the locality, where a more specific code is required. Repeatable if there is more than one country code. The codes are taken from code lists other than ISO 3166-2. Details of th... |
| `$c` | Locality (ISO) | Rép. | Optionnel | A code representing the locality, where a more specific code is required. Repeatable if there is more than one country code. The codes are to be taken from ISO 3166-2. |
| `$2` | Source of non-ISO Code | Rép. | Optionnel | The source of the code used in subfield $b. For a list of code sources see Appendix A. Repeatable if there is more than one country code. |

### Zone 105 — CODED DATA FIELD: TEXTUAL LANGUAGE MATERIAL, MONOGRAPHIC

**Répétabilité :** — &nbsp;|&nbsp; **Occurrence :** —

> 🔢 **Champ à données codées** — sous-champ(s) à longueur fixe décodables par position de caractère.

| Sous-zone | Nom | Rép. | Occurrence | Description |
|-----------|-----|------|------------|-------------|
| `$a` | Monograph Coded Data | Non rép. | Oblig. si appl. | Codes indicate aspects of monographic language material. The subfield is 13-characters in length. Not repeatable. Table of subfield $a fixed-length data elements: |

#### 🔢 Structure interne par position de caractère

| Offset | Lg. | Élément | Obl. |
|--------|-----|---------|------|
| `0-3` | 4 | Illustration Codes |  |
| `4-7` | 4 | Form of Contents Codes |  |
| `8` | 1 | Conference or Meeting Code | |
| `9` | 1 | Festschrift Indicator | |
| `10` | 1 | Index Indicator | |
| `11` | 1 | Literature Code | |
| `12` | 1 | Biography Code | |

##### Pos. `0-3` — Illustration Codes

| Code | Signification |
|------|---------------|
| `a` | illustrations Used for types of illustrations which are not covered below, e.g., diagrams, graphs, |
| `b` | maps |
| `c` | portraits Individual or group portraits. |
| `d` | charts Special purpose maps designed for the use of navigators. |
| `e` | plans E.g. ground plans of buildings. |
| `f` | plates A leaf containing illustrative matter with or without explanatory text, which does |
| `g` | music In monographic textual-language material. For sound recordings accompanying |
| `h` | facsimiles Reproductions of a part of or the whole of a document not only reproducing the |
| `i` | coats of arms |
| `j` | genealogical |
| `k` | forms |
| `1` | samples |
| `m` | sound |
| `n` | transparencies E.g. a set of transparencies in a pocket attached to a book. |
| `o` | illuminations Manuscript embellishments of a letter or writing with colours etc. |
| `#` | value position |

##### Pos. `4-7` — Form of Contents Codes

| Code | Signification |
|------|---------------|
| `7` | academic thesis below the |
| `a` | bibliography A list of the bibliographic resources sharing one or more |
| `b` | catalogue A list of bibliographic resources in a collection or exhibition or |
| `c` | index An alphabetical list of terms, i.e. names or subject giving the |
| `d` | abstract or summary  Includes descriptive, indicative, and informative abstracts. |
| `e` | dictionary Alphabetical listings of words with a short description of their |
| `f` | encyclopaedia Listing of names or terms on a given subject with an extensive |
| `g` | directory Ordered list of persons, organizations or places, with |
| `h` | project description |
| `i` | statistics Numerical facts on a subject systematically collected and usually |
| `j` | programmed text book A text book in which material is presented to be learnt |
| `k` | patent A document including a specification of a new invention giving |
| `l` | standard A document produced by an official body specifying a |
| `m` | dissertation or thesis (original) A statement of investigation or research presenting the author’s |
| `n` | laws and legislation Resources including texts of laws. For treaties use code “s”. |
| `o` | numeric table Numerical information presented in a tabular form. Use code |
| `p` | technical report A document containing the findings of an investigation or |
| `q` | examination paper A set of questions printed for use in an examination. |
| `r` | literature surveys/reviews Narrative surveys, often critical, of activity in a specific field as |
| `s` | treaties Formally concluded and ratified agreements between states. For |
| `t` | cartoons or comic strips Books published as cartoons for children or adults. |
| `v` | dissertation or thesis (revised) A thesis or dissertation reproduced or revised for commercial |
| `w` | religious text Language material containing religious, liturgical or sacred text |
| `z` | other |
| `#` | value position not needed |

##### Pos. `8` — Conference or Meeting Code

| Code | Signification |
|------|---------------|
| `0` | not a conference publication |
| `1` | conference publication |

##### Pos. `9` — Festschrift Indicator

| Code | Signification |
|------|---------------|
| `0` | not a festschrift |
| `1` | festschrift |

##### Pos. `10` — Index Indicator

| Code | Signification |
|------|---------------|
| `0` | no index |
| `1` | index present |

##### Pos. `11` — Literature Code

| Code | Signification |
|------|---------------|
| `a` | fiction |
| `b` | drama Includes television plays, scenarios, etc. |
| `c` | essays |
| `d` | humour, satire For cartoons, etc. see character positions 4-7 Form of contents codes, |
| `e` | letters As a literary form. For correspondence see character position 12 |
| `f` | short stories |
| `g` | poetry Includes non-literary works in verse. |
| `h` | speeches, oratory |
| `i` | libretto A book giving the literary text, both sung and spoken, of an opera or |
| `y` | not a literary text |
| `z` | multiple or other |

##### Pos. `12` — Biography Code

| Code | Signification |
|------|---------------|
| `a` | autobiography Includes letters, correspondence. |
| `b` | individual biography |
| `c` | collective biography E.g. works containing biographies of more than one person or of a |
| `d` | contains biographical |
| `y` | not biographical |
| `8` | 0 Not a conference publication. |
| `9` | 0 Not a festschrift. |
| `10` | 1 Resource has an index. |
| `11` | y Not a literary text. |
| `12` | b Individual biography. |

### Zone 106 — CODED DATA FIELD: TEXTUAL RESOURCE – FORM

**Répétabilité :** — &nbsp;|&nbsp; **Occurrence :** —

> 🔢 **Champ à données codées** — sous-champ(s) à longueur fixe décodables par position de caractère.

| Sous-zone | Nom | Rép. | Occurrence | Description |
|-----------|-----|------|------------|-------------|
| `$a` | Form of Resource: Coded Data: Medium Designator | Non rép. | Oblig. si appl. | 1-character code indicates the physical medium of the resource. The following codes are used: d large print e newspaper format f Braille or Moon script g microprint h hand-written i multimedia Obso... |

#### 🔢 Structure interne par position de caractère

| Offset | Lg. | Élément | Obl. |
|--------|-----|---------|------|
| `0` | 1 | code “m”= “multiple media”. | |

##### Pos. `0` — code “m”= “multiple media”.

| Code | Signification |
|------|---------------|
| `d` | large print |
| `e` | newspaper |
| `f` | Braille ou Moon script |
| `g` | microprint |
| `h` | hand-written |
| `i` | multimedia [obsolète] |
| `j` | mini-print |
| `r` | regular print |
| `s` | electronic [obsolète] |
| `t` | Microform [obsolète] |
| `z` | autre forme de matériau |

### Zone 110 — CODED DATA FIELD: CONTINUING RESOURCES

**Répétabilité :** — &nbsp;|&nbsp; **Occurrence :** —

> 🔢 **Champ à données codées** — sous-champ(s) à longueur fixe décodables par position de caractère.

| Sous-zone | Nom | Rép. | Occurrence | Description |
|-----------|-----|------|------------|-------------|
| `$a` | Continuing Resource Coded Data | Non rép. | Oblig. si appl. | Codes indicate aspects of continuing resource. The subfield is 11-characters in length. Not repeatable. Subfield $a fixed-length data elements: |

#### 🔢 Structure interne par position de caractère

| Offset | Lg. | Élément | Obl. |
|--------|-----|---------|------|
| `0` | 1 | Type of Continuing Resource Designator | |
| `1` | 1 | Frequency of Issue | |
| `2` | 1 | Regularity | |
| `3` | 1 | Type of Material Code | |
| `4-6` | 3 | Nature of Contents Code |  |
| `7` | 1 | Conference Publication Indicator | |
| `8` | 1 | Title Page Availability Codes | |
| `9` | 1 | Index Availability Code | |
| `10` | 1 | Cumulative Index Availability Code | |

##### Pos. `0` — Type of Continuing Resource Designator

| Code | Signification |
|------|---------------|
| `a` | periodical Broad category of publications that includes resources with separate articles, stories, |
| `b` | monographi |
| `c` | series |
| `d` | directory An itemized listing of information for the identification or location of persons, |
| `e` | updating |
| `f` | updating |
| `g` | updating |
| `h` | blog Online periodical appearing on a web page that may contain web links and/or |
| `i` | repository An archive for collecting, preserving, and disseminating digital copies of the |
| `j` | journal A periodical addressing readers interested in a specific subject or profession. Often |
| `m` | magazine A specialized periodical addressing non-scientific, non-professional general interest |
| `n` | newsletter A short periodical that can be issued by an organization, generally to its members, |
| `z` | other |

##### Pos. `1` — Frequency of Issue

| Code | Signification |
|------|---------------|
| `a` | daily |
| `b` | semiweekly (twice a week) |
| `c` | weekly |
| `d` | biweekly (every two weeks) |
| `e` | semimonthly (twice a month) |
| `f` | monthly |
| `g` | bimonthly (every two months) |
| `h` | quarterly |
| `i` | three times a year |
| `k` | annual |
| `l` | biennial (every two years) |
| `m` | triennial (every three years) |
| `n` | three times a week |
| `o` | three times a month |
| `p` | continuously updated |
| `u` | unknown |
| `y` | no frequency (i.e. irregular) See also character position 2 below. |
| `z` | other |

##### Pos. `2` — Regularity

| Code | Signification |
|------|---------------|
| `a` | regular Continuing resource is issued at regular intervals. |
| `b` | normalised irregular Continuing resource is not completely regular, but is irregular |
| `u` | not known |
| `y` | irregular |

##### Pos. `3` — Type of Material Code

| Code | Signification |
|------|---------------|
| `a` | bibliography The continuing resource consists of a bibliography, e.g., a national |
| `b` | catalogue A list of bibliographic resources in a collection or exhibition or available |
| `c` | index E.g. a continuing resource index to a continuing resource. For the |
| `d` | abstract or summary Includes descriptive, indicative and informative abstracts. |
| `e` | dictionary Continuing resource consisting of an alphabetical listing of words with a |
| `f` | encyclopaedia Continuing resource listing names or terms in a given subject with an |
| `g` | directory Ordered list of persons, corporate bodies or places with information |
| `h` | yearbook Continuing resource designed to present a body of reference matter |
| `i` | statistics Numerical facts on a subject systematically collected and usually arranged |
| `j` | programmed texts Continuing resource in which material is presented to be learnt |
| `k` | reviews i.e. a continuing resource consisting of book reviews and/or reviews of |
| `l` | laws and legislation Documents including texts of laws. |
| `m` | law reports and |
| `n` | legal articles Articles in a journal dealing with general legal topics. |
| `o` | legal cases and case |
| `p` | biography Used when a continuing resource consists of an individual bibliography, |
| `r` | literature |
| `t` | cartoons or comic |
| `z` | other kinds of |
| `#` | value position not |

##### Pos. `7` — Conference Publication Indicator

| Code | Signification |
|------|---------------|
| `0` | not a conference publication |
| `l` | conference publication |

##### Pos. `8` — Title Page Availability Codes

| Code | Signification |
|------|---------------|
| `a` | in last issue of volume – loose |
| `b` | in last issue of volume – attached |
| `c` | in first issue of next volume – loose |
| `d` | in first issue of next volume – attached |
| `e` | published separately – free upon request |
| `f` | published separately – free – sent automatically |
| `g` | published separately – purchase – request |
| `u` | unknown at time of record creation |
| `x` | not applicable |
| `y` | no title-page issued |
| `z` | other |

##### Pos. `9` — Index Availability Code

| Code | Signification |
|------|---------------|
| `a` | continuing resource and the method of acquisition. A continuing resource which is an index is indicated |
| `b` | in last issue of volume – loose – separately paged |
| `c` | in last issue of volume – unpaged |
| `d` | in last issue of volume – attached |
| `e` | in first issue of next volume – loose – separately paged |
| `f` | in first issue of next volume – loose – unpaged |
| `g` | in first issue of next volume – attached |
| `h` | published separately – free – sent automatically |
| `i` | published separately – free upon request |
| `j` | published separately – bound from publisher – free – sent automatically |
| `k` | published separately – bound from publisher – free upon request |
| `l` | published separately – bound from publisher – purchase upon request |
| `m` | this continuing resource is a supplement or subseries indexed |
| `u` | unknown at time of record creation |
| `x` | not applicable |
| `y` | index is not available |
| `z` | other |

##### Pos. `10` — Cumulative Index Availability Code

| Code | Signification |
|------|---------------|
| `0` | no cumulative index or table of contents |
| `1` | cumulative index or table of contents available |
| `2` | a Regular |
| `3` | h Yearbook |
| `7` | l Contains annual conference proceedings |
| `8` | z Other (title page is part of annual volume) |
| `9` | z Other (index is part of annual volume) |
| `10` | 1 Cumulative index available |

### Zone 111 — CODED DATA FIELD: SERIALS – PHYSICAL ATTRIBUTES [OBSOLETE]

**Répétabilité :** — &nbsp;|&nbsp; **Occurrence :** —

*Champ de contrôle — pas de sous-zones ni d'indicateurs.*

### Zone 115 — CODED DATA FIELD: VISUAL PROJECTIONS, VIDEO RECORDINGS AND MOTION PICTURES

**Répétabilité :** — &nbsp;|&nbsp; **Occurrence :** —

> 🔢 **Champ à données codées** — sous-champ(s) à longueur fixe décodables par position de caractère.

| Sous-zone | Nom | Rép. | Occurrence | Description |
|-----------|-----|------|------------|-------------|
| `$a` | Coded Data – General | Non rép. | Optionnel | Codes indicate aspects of visual projection, video recording or motion picture. The subfield is 20-characters in length. Not repeatable. Subfield $a fixed-length data elements: |
| `$b` | Motion Picture Coded Data Archival | Non rép. | Optionnel |  |

#### 🔢 Structure interne par position de caractère

| Offset | Lg. | Élément | Obl. |
|--------|-----|---------|------|
| `0` | 1 | Generation | |
| `1` | 1 | Production Elements | |
| `2` | 1 | Refined Categories of Colour for Moving Pictures | |
| `3` | 1 | Film Emulsion (Polarity) | |
| `4` | 1 | Film Base | |
| `5` | 1 | Kind of Sound for Moving Images | |
| `6` | 1 | Kind of Film Stock or Print | |
| `7` | 1 | Deterioration Stage | |
| `8` | 1 | Completeness | |
| `9` | 1 | Technique – Video recording, Motion Picture | |
| `10` | 1 | Presentation Format – Motion Picture | |
| `11-14` | 4 | Accompanying Material |  |
| `15` | 1 | Form of Release – Video recording | |
| `16` | 1 | Presentation Format – Video recording | |
| `17` | 1 | Base of Emulsion Material – Visual Projection | |
| `18` | 1 | Secondary Support Material – Visual Projection | |
| `19` | 1 | Broadcast Standard – Video recording | |

##### Pos. `0` — Generation

| Code | Signification |
|------|---------------|
| `a` | original |
| `b` | master |
| `c` | duplicate |
| `d` | reference print/viewing copy |
| `u` | unknown |
| `x` | not applicable |
| `z` | other |

##### Pos. `1` — Production Elements

| Code | Signification |
|------|---------------|
| `a` | workprint |
| `b` | trims |
| `c` | outtakes |
| `d` | rushes |
| `e` | mixing tracks |
| `g` | title bands/intertitle rolls |
| `h` | production rolls |
| `u` | unknown |
| `x` | not applicable |
| `z` | other |

##### Pos. `2` — Refined Categories of Colour for Moving Pictures

| Code | Signification |
|------|---------------|
| `a` | 3 layer colour |
| `b` | 2 colour, single strip |
| `c` | undetermined 2 colour |
| `d` | undetermined 3 colour |
| `e` | 3 strip colour |
| `f` | 2 strip colour |
| `g` | red strip |
| `h` | blue or green strip |
| `i` | cyan strip |
| `j` | magenta strip |
| `k` | yellow strip |
| `l` | S E N 2 |
| `m` | S E N 3 |
| `n` | sepia tone |
| `o` | other tone |
| `p` | tint |
| `q` | tinted and toned |
| `r` | stencil colour |
| `s` | handcoloured |
| `u` | unknown |
| `x` | not applicable Resource not a colour film. |
| `z` | other |

##### Pos. `3` — Film Emulsion (Polarity)

| Code | Signification |
|------|---------------|
| `a` | positive |
| `b` | negative |
| `u` | unknown |
| `x` | not applicable |
| `z` | other |

##### Pos. `4` — Film Base

| Code | Signification |
|------|---------------|
| `a` | safety (triacetate) |
| `b` | nitrate |
| `c` | safety (diacetate) |
| `d` | polyester base (e.g. ester) |
| `u` | unknown |
| `v` | mixed base (nitrate and safety) |
| `z` | other |

##### Pos. `5` — Kind of Sound for Moving Images

| Code | Signification |
|------|---------------|
| `a` | monaural |
| `b` | stereophonic |
| `c` | multichannel, surround or quadraphonic |
| `u` | unknown |
| `v` | mixed |
| `x` | not applicable  Resource is silent. |
| `z` | other |
| `#` | value position not needed According to cataloguing rules based on IFLA LRM, |

##### Pos. `6` — Kind of Film Stock or Print

| Code | Signification |
|------|---------------|
| `a` | imbibition dye transfer |
| `b` | three layer stock |
| `c` | three layer stock (low fade) |
| `d` | duplitised stock |
| `u` | unknown |
| `x` | not applicable Resource not a colour film. |
| `z` | other |

##### Pos. `7` — Deterioration Stage

| Code | Signification |
|------|---------------|
| `b` | nitrate – suspicious odour |
| `c` | nitrate – pungent odour |
| `d` | nitrate – brownish, discoloration, fading, dusty |
| `e` | nitrate – sticky |
| `f` | nitrate – frothy, bubbles, blisters |
| `g` | nitrate – congealed |
| `h` | nitrate – powder |
| `k` | non-nitrate – detectable deterioration  E.g. diacetate odour. |
| `l` | non-nitrate – advanced deterioration |
| `m` | non-nitrate – disaster |
| `y` | no deterioration |

##### Pos. `8` — Completeness

| Code | Signification |
|------|---------------|
| `a` | incomplete |
| `b` | complete |
| `u` | unknown |
| `x` | not applicable  When it is impossible or inapplicable to determine completeness, e.g. home |

##### Pos. `9` — Technique – Video recording, Motion Picture

| Code | Signification |
|------|---------------|
| `a` | animation |
| `b` | live action |
| `c` | animation and live action |
| `u` | unspecified/unknown |
| `x` | not a motion picture or video recording |
| `z` | other |

##### Pos. `10` — Presentation Format – Motion Picture

| Code | Signification |
|------|---------------|
| `a` | standard sound aperture (reduced format) |
| `b` | non-anamorphic (widescreen) |
| `d` | anamorphic (widescreen) |
| `e` | standard silent aperture |
| `f` | other widescreen format |
| `x` | not a motion picture |
| `u` | unknown |
| `z` | other |

##### Pos. `11-14` — Accompanying Material

| Code | Signification |
|------|---------------|
| `a` | stills |
| `b` | script material |
| `c` | posters |
| `d` | programmes and pressbooks |
| `e` | lobby cards |
| `f` | instructional materials |
| `g` | score or other music format |
| `h` | set or costume designs |
| `z` | other accompanying material |

##### Pos. `15` — Form of Release – Video recording

| Code | Signification |
|------|---------------|
| `a` | videocartridge |
| `b` | videodisc |
| `c` | videocassette |
| `d` | videoreel |
| `e` | electronic video recording (EVR) |
| `x` | not a video recording |
| `z` | other videotype |

##### Pos. `16` — Presentation Format – Video recording

| Code | Signification |
|------|---------------|
| `a` | Beta (videocassette) Home video format introduced in 1975 by Sony Corporation. |
| `b` | VHS (videocassette) Home video format developed by Japan Victor Corporation (JVC) and |
| `c` | U-matic (videocassette) Video format using Sony’s trademark name, referring to its tape |
| `d` | EIAJ (reel) The standard ½-inch reel-to-reel helical scan videotape format. Named |
| `e` | Type C (reel) Video format using one recording head, with tape speed at 9.61 inches |
| `f` | Quadruplex (reel) Video system using four recording heads, developed by Ampex in the |
| `g` | Laser optical |
| `h` | CED (capacitance |
| `i` | V2000 (videocassette) Home video format developed in Europe by Philips since 1980. |
| `j` | Video8 (videocassette) Home video format. |
| `k` | DVD-Video Digital Versatile Disc. |
| `l` | Blu-ray Disc |
| `u` | unknown |
| `x` | not a video recording |
| `z` | other |

##### Pos. `17` — Base of Emulsion Material – Visual Projection

| Code | Signification |
|------|---------------|
| `a` | safety film |
| `b` | film base other than safety film |
| `c` | synthetics (plastic, vinyl, etc.) |
| `u` | unknown |
| `v` | mixed collection Containing more than one type of base. |
| `x` | not a visual projection |
| `z` | other |

##### Pos. `18` — Secondary Support Material – Visual Projection

| Code | Signification |
|------|---------------|
| `a` | cardboard |
| `b` | glass |
| `c` | synthetics (plastic, vinyl, etc.) |
| `d` | metal |
| `e` | metal and glass |
| `f` | synthetics (plastic, vinyl, etc.) and glass |
| `u` | unknown |
| `x` | not a visual projection |
| `y` | not present No secondary support. |
| `z` | other |

##### Pos. `19` — Broadcast Standard – Video recording

| Code | Signification |
|------|---------------|
| `a` | 405 |
| `b` | 525 (e.g. NTSC) |
| `c` | 625 PAL |
| `d` | 625 SECAM |
| `g` | 1125 |
| `#` | not a video recording EX 3 |

### Zone 116 — CODED DATA FIELD: GRAPHICS

**Répétabilité :** Rép. &nbsp;|&nbsp; **Occurrence :** Optionnel

> 🔢 **Champ à données codées** — sous-champ(s) à longueur fixe décodables par position de caractère.

| Sous-zone | Nom | Rép. | Occurrence | Description |
|-----------|-----|------|------------|-------------|
| `$a` | Coded Data for Graphics | Non rép. | Optionnel | Codes indicate aspects of graphic. The subfield is 17-characters in length. Not repeatable. Subfield $a fixed-length data elements: |

#### 🔢 Structure interne par position de caractère

| Offset | Lg. | Élément | Obl. |
|--------|-----|---------|------|
| `0` | 1 | Specific Material Designation | |
| `1` | 1 | Primary Support Material | |
| `2` | 1 | Secondary Support Material | |
| `3` | 1 | Colour | |
| `4-9` | 6 | Technique (Drawings, Paintings) |  |
| `10-15` | 6 | Technique (Prints) |  |
| `16-17` | 2 | Functional Designation |  |

##### Pos. `0` — Specific Material Designation

| Code | Signification |
|------|---------------|
| `a` | collage An original work created by affixing various materials (paper, wood, |
| `b` | drawing An original visual representation (other than a print or painting) made with |
| `c` | painting An original visual representation produced by applying paint to a surface. |
| `d` | photomechanical |
| `e` | photonegative A piece of film, a glass plate, or paper on which appears a “negative” image, |
| `f` | photoprint A positive image made either directly or indirectly on a sensitised surface by |
| `h` | picture A two-dimensional visual representation accessible to the naked eye and |
| `i` | print A design or picture transferred from an engraved plate, wood block, |
| `k` | technical |
| `m` | master Any plate, mould, matrix, die etc. which allows the reproduction of the same |
| `z` | other non- |

##### Pos. `1` — Primary Support Material

| Code | Signification |
|------|---------------|
| `a` | canvas |
| `b` | bristol board |
| `c` | cardboard/illustration board |
| `d` | glass |
| `e` | synthetics (plastic, vinyl, etc.) |
| `f` | skins (leather, parchment, vellum, etc.) |
| `g` | textiles (including man-made fibres) |
| `h` | metal |
| `i` | paper |
| `j` | plaster |
| `k` | hardboard |
| `l` | porcelain |
| `m` | stone |
| `n` | wood |
| `u` | unknown |
| `v` | mixed collection |
| `z` | other |

##### Pos. `2` — Secondary Support Material

| Code | Signification |
|------|---------------|
| `y` | no secondary support |

##### Pos. `3` — Colour

| Code | Signification |
|------|---------------|
| `a` | one colour The image is in a single colour (i. e. monochrome). Used for monochrome |
| `b` | black-and- |
| `c` | multicoloured The image is in more than one colour. |
| `d` | hand coloured The image, produced by a printing or photographic process, is coloured by |
| `u` | unknown |
| `v` | mixed The image is in a combination of black-and-white, one colour, and/or colours. |
| `x` | not applicable The colour characteristics do not apply. |
| `z` | other The image has colour characteristics not covered by the other defined codes. |
| `#` | value position |

##### Pos. `4-9` — Technique (Drawings, Paintings)

| Code | Signification |
|------|---------------|
| `aa` | pencil |
| `ab` | graphite |
| `ac` | colour pencil |
| `ad` | India ink |
| `ae` | lavierung India ink |
| `af` | coal |
| `ag` | chalk |
| `ah` | black chalk |
| `ai` | sanguine |
| `aj` | water colour |
| `ak` | tempera |
| `al` | gouache |
| `am` | pastel |
| `ba` | felt-tip pen |
| `bb` | stain |
| `bc` | crayon |
| `bd` | sepia |
| `bf` | casein |
| `bg` | golding |
| `bh` | encaustic |
| `bi` | acrylics |
| `bj` | collage |
| `bk` | silver point |
| `bl` | air brush |
| `uu` | unknown |
| `vv` | mixed |
| `xx` | not applicable |
| `zz` | other |

##### Pos. `10-15` — Technique (Prints)

| Code | Signification |
|------|---------------|
| `ba` | woodcut |
| `bb` | chiaroscuro woodcut |
| `bc` | white-line woodcut |
| `bd` | camaiu |
| `bf` | chromolithography |
| `bg` | lino-cut |
| `bh` | etching |
| `bi` | lithography |
| `bj` | photolithography |
| `bk` | zincography |
| `bl` | algraphy |
| `bm` | aquatint |
| `bn` | reservage (sugar aquatint) |
| `ca` | vernis-mou (soft ground etching or tracing) |
| `cb` | engraving |
| `cc` | engraving in the crayon manner |
| `cd` | burin engraving |
| `ce` | drypoint |
| `cf` | mezzotinta (mezzotint) |
| `cg` | monotype |
| `ch` | silkscreen |
| `ci` | steel engraving |
| `cj` | computer graphics |
| `ck` | photocopying |
| `uu` | unknown |
| `vv` | mixed |
| `xx` | not applicable |
| `zz` | other |

##### Pos. `16-17` — Functional Designation

| Code | Signification |
|------|---------------|
| `aa` | architectural drawing Graphic delineation made for the design and construction (or |
| `ab` | item cover Cover forming the binding or outer enclosure of an resource, i.e. |
| `ac` | sticker Message or design on slips of paper that are gummed or otherwise |
| `ad` | poster Single or multi-sheet chiefly pictorial notice made for posting, usually |
| `ae` | postcard Card with a graphic scene on one side, on which a message may be |
| `af` | greetings card Card sent or given on special occasions; usually bearing messages of |
| `ag` | chart An opaque sheet that exhibits information in a graphic or tabular |
| `ah` | playing cards Cards made in sets of a designated number of cards and marked for |
| `ai` | flash card A card or other opaque material printed with words, numerals, or |
| `aj` | ephemera Transient everyday resources, usually printed and on paper, that are |
| `au` | santino Printed card which has, on one side, the picture of a holy or sacred |
| `uu` | unknown |
| `vv` | mixed |
| `xx` | not applicable |
| `zz` | other |

### Zone 117 — CODED DATA FIELD: THREE-DIMENSIONAL ARTEFACTS AND REALIA

**Répétabilité :** — &nbsp;|&nbsp; **Occurrence :** —

> 🔢 **Champ à données codées** — sous-champ(s) à longueur fixe décodables par position de caractère.

| Sous-zone | Nom | Rép. | Occurrence | Description |
|-----------|-----|------|------------|-------------|
| `$b` | Coded Data for Numismatic Resources | Non rép. | Optionnel | Codes indicate specific physical and production information and the class of material to which the numismatic resource belongs. The subfield is 11-characters in length. Not repeatable. Subfield $b ... |

#### 🔢 Structure interne par position de caractère

| Offset | Lg. | Élément | Obl. |
|--------|-----|---------|------|
| `0` | 1 | Specific Material Designation | |
| `4-6` | 3 | Peculiarity of Production |  |
| `8` | 1 | Colour | |
| `10` | 1 | Monetary Status | |

##### Pos. `0` — Specific Material Designation

| Code | Signification |
|------|---------------|
| `a` | coin Small, flat, usually round piece of metal or plastic used primarily as a medium |
| `b` | medal Small portable artistic object, a thin disc, normally of metal, carrying a design, |
| `c` | token A coin-like item used in commerce but not issued by a government as |
| `d` | tesserae An ancient form of token. |
| `e` | medallion A coin-like Roman period object, often commemorative in nature, that seem |
| `f` | coin ornament Imitation of a coin used as jewellery or as decoration on clothing or similar; |
| `g` | pattern coin Test strike with a coin die. |
| `h` | coin weight Weight for checking whether coins are of the correct weight. |
| `i` | paper money A banknote (often known as a bill, paper money, or simply a note) is a type |
| `j` | die Stamps, usually engraved, used for impressing a design upon a softer |
| `k` | punch Small stamp for making test or similar marks on coins or similar objects, or |
| `l` | flan Coin flan, generally in the form of a small metal plate. |
| `m` | ingot A mass of metal, usually purified, sometimes with indication of its value in a |
| `n` | commodity |
| `o` | piedfort A coin often exactly twice its normal weight and thickness. The name derives |
| `z` | other Other numismatic objects. |

##### Pos. `4-6` — Peculiarity of Production

| Code | Signification |
|------|---------------|
| `a` | flawed die A cracked, worn, corroded, or similarly defective die used to strike a |
| `b` | recut die Secondary re-engraving of a coin die after it has already been used to strike |
| `c` | off centre A strike whereby the die was not placed centrally on the flan, but displaced |
| `d` | brockage A misstrike whereby a numismatic resource remains in the upper die after |
| `e` | double strike Double or multiple strike during production whereby the die is displaced for |
| `f` | overstrike Striking of a new type using an existing numismatic resource as a flan. |
| `g` | gouging The practice of scraping material from the surface of a flan to reduce its |
| `h` | cracked flan Flan that has cracked and been partially split as a result of the striking |
| `i` | folded flan A flan or coin which is folded in two or more prior to being struck on the |
| `j` | surface |
| `k` | central point Central point marked into a flan before striking. |
| `l` | hammered rim Creation of a raised rim by hammering parallel to the two faces of the |
| `m` | serratus A numismatic resource with a notched edge. |
| `n` | hybrid A combination of obverse and reverse dies that do not correspond to an |

##### Pos. `8` — Colour

| Code | Signification |
|------|---------------|
| `a` | one colour The resource is in a single colour. Does not include black-and-white. |
| `b` | black-and-white The resource is in black-and-white tones only. |
| `c` | multicoloured The resource is in more than one colour. |
| `d` | hand coloured The resource is coloured by hand. |
| `u` | unknown |
| `v` | mixed The resource or collection is in a combination of black-and-white and/or |
| `x` | not applicable The colour characteristics do not apply. |
| `z` | other The resource has colour characteristics not covered by the other defined |
| `#` | value position |

##### Pos. `10` — Monetary Status

| Code | Signification |
|------|---------------|
| `a` | official issue A numismatic resource issued by an official authority. |
| `b` | unofficial issue A numismatic resource which is not produced by an official authority. |
| `c` | imitation A numismatic resource whose type imitates the characteristics another issue |
| `d` | contemporary |
| `e` | ulterior forgery A counterfeit numismatic resource made after the time of the original issue |
| `#` | not applicable |

### Zone 120 — CODED DATA FIELD: CARTOGRAPHIC RESOURCES – GENERAL

**Répétabilité :** — &nbsp;|&nbsp; **Occurrence :** —

> 🔢 **Champ à données codées** — sous-champ(s) à longueur fixe décodables par position de caractère.

| Sous-zone | Nom | Rép. | Occurrence | Description |
|-----------|-----|------|------------|-------------|
| `$a` | Cartographic Resource Coded Data (General) | Non rép. | Optionnel | Codes indicate aspects of cartographic resource. The subfield is 13-characters in length. Not repeatable. Subfield $a fixed-length data elements: |

#### 🔢 Structure interne par position de caractère

| Offset | Lg. | Élément | Obl. |
|--------|-----|---------|------|
| `0` | 1 | Colour Indicator | |
| `1` | 1 | Index Indicator | |
| `2` | 1 | Narrative Text Indicator | |
| `3-6` | 4 | Relief Codes |  |
| `7-8` | 2 | Map Projection |  |
| `9-12` | 4 | Prime Meridian |  |

##### Pos. `0` — Colour Indicator

| Code | Signification |
|------|---------------|
| `a` | one colour The resource is in a single colour, including black-and-white. |
| `b` | multicoloured The resource is in more than one colour. |
| `#` | value position not |

##### Pos. `1` — Index Indicator

| Code | Signification |
|------|---------------|
| `a` | index or name list on cartographic resource itself |
| `b` | index or name list accompanying cartographic resource in booklet, pamphlet, unattached cover, etc. |
| `c` | index or name list present but location not specified |
| `y` | no index or name list |

##### Pos. `2` — Narrative Text Indicator

| Code | Signification |
|------|---------------|
| `a` | text on cartographic resource itself |
| `b` | text accompanying cartographic resource in booklet, pamphlet, unattached cover, etc. |
| `y` | no narrative text |

##### Pos. `3-6` — Relief Codes

| Code | Signification |
|------|---------------|
| `a` | contours |
| `b` | continuous tone shaded relief |
| `c` | hypsometric tints – layer method |
| `d` | hachures |
| `e` | bathymetry – soundings |
| `f` | form lines |
| `g` | spot heights |
| `h` | other methods in colour (e.g. in the style of Imhof) |
| `i` | pictorially |
| `j` | landforms (e.g. in the style of Lobeck, Raisz, Fenneman) |
| `k` | bathymetry – isolines |
| `x` | not applicable |
| `z` | other methods of relief representation |

##### Pos. `7-8` — Map Projection

| Code | Signification |
|------|---------------|
| `aa` | Aitoff Azimuthal projections: Conic projections |
| `ca` | Albers equal area Azimuthal projections: Conic projections |
| `ab` | gnomonic Azimuthal projections: Conic projections |
| `cb` | Bonne Azimuthal projections: Conic projections |
| `ac` | Lambert’s azimuthal equal area Azimuthal projections: Conic projections |
| `cc` | Lambert’s conformal conic Azimuthal projections: Conic projections |
| `ad` | orthographic Azimuthal projections: Conic projections |
| `cd` | conic (simple) Azimuthal projections: Conic projections |
| `ae` | azimuthal equidistant Azimuthal projections: Conic projections |
| `ce` | Miller’s bipolar oblique conformal conic Azimuthal projections: Conic projections |
| `af` | stereographic Azimuthal projections: Conic projections |
| `cf` | De Lisle Azimuthal projections: Conic projections |
| `ag` | azimuthal equal area Azimuthal projections: Conic projections |
| `cg` | projection of the International Map Azimuthal projections: Conic projections |
| `au` | azimuthal, specific type unknown Azimuthal projections: Conic projections |
| `az` | azimuthal, other known specific type Azimuthal projections: Conic projections |
| `ch` | Tissot’s conformal conic Azimuthal projections: Conic projections |
| `cp` | polyconic Azimuthal projections: Conic projections |
| `cu` | conic, specific type unknown Azimuthal projections: Conic projections |
| `cz` | conic, other known specific type Azimuthal projections: Conic projections |
| `ba` | Gall Cylindrical projections |
| `bb` | Goode’s homolographic Cylindrical projections |
| `bc` | Lambert’s cylindrical equal area Cylindrical projections |
| `bd` | Mercator Cylindrical projections |
| `bf` | Mollweide Cylindrical projections |
| `bg` | sinusoidal Cylindrical projections |
| `bh` | transverse Mercator Cylindrical projections |
| `bi` | Gauss Cylindrical projections |
| `bj` | Plate Carree Cylindrical projections |
| `bk` | Cassini’s Cylindrical projections |
| `bl` | Laborde Cylindrical projections |
| `bm` | Oblique Mercator Cylindrical projections |
| `bu` | cylindrical, specific type unknown Cylindrical projections |
| `bz` | cylindrical, other known specific type Cylindrical projections |
| `da` | armadillo Other projections |
| `db` | butterfly Other projections |
| `dc` | Eckert Other projections |
| `dd` | Goode’s homolosine Other projections |
| `de` | Miller’s bipolar Other projections |
| `df` | Van der Griten Other projections |
| `dg` | dimaxion Other projections |
| `dh` | cordiform Other projections |
| `di` | polyhedric Other projections |
| `uu` | type of projection unknown |
| `xx` | not applicable |
| `zz` | other known type |

##### Pos. `9-12` — Prime Meridian

| Code | Signification |
|------|---------------|
| `aa` | Greenwich, United Kingdom International prime meridian |
| `ab` | Amsterdam, Netherlands |
| `ac` | Athens, Greece |
| `ad` | Batavia (Djakarta) Indonesia |
| `ae` | Berne, Switzerland |
| `af` | Bogota, Colombia |
| `ag` | Bombay, India |
| `ah` | Brussels, Belgium |
| `ai` | Cadiz, Spain |
| `aj` | Capetown, South Africa |
| `ak` | Caracas, Venezuela |
| `al` | Copenhagen, Denmark |
| `am` | Cordoba, Argentina |
| `ao` | Helsinki, Finland |
| `ap` | Istanbul, Turkey |
| `aq` | Julianehaab, Greenland |
| `ar` | Lisbon, Portugal |
| `ba` | Madrid, Spain |
| `bb` | Mexico City, Mexico |
| `bc` | Moscow, Russia |
| `bd` | Munich, Germany |
| `bf` | Oslo (Christiania) Norway |
| `bg` | Paris, France |
| `bh` | Peking, China |
| `bi` | Philadelphia, USA |
| `bj` | Pulkova, Russia |
| `bk` | Rio de Janeiro, Brazil |
| `bl` | Rome, Italy |
| `bm` | Santiago, Chile |
| `bn` | Stockholm, Sweden |
| `bo` | Sydney, Australia |
| `bp` | Tirana, Albania |
| `bq` | Tokyo, Japan |
| `br` | Washington, DC, USA |
| `uu` | unknown |
| `zz` | other |

### Zone 121 — CODED DATA FIELD: CARTOGRAPHIC RESOURCES – PHYSICAL ATTRIBUTES

**Répétabilité :** — &nbsp;|&nbsp; **Occurrence :** —

> 🔢 **Champ à données codées** — sous-champ(s) à longueur fixe décodables par position de caractère.

*Champ de contrôle — pas de sous-zones ni d'indicateurs.*

#### 🔢 Structure interne par position de caractère

| Offset | Lg. | Élément | Obl. |
|--------|-----|---------|------|
| `0` | 1 | Altitude of Sensor | |
| `1` | 1 | Attitude of Sensor | |
| `4` | 1 | Quality of Image | |
| `5` | 1 | Cloud Cover | |
| `6` | 1 | Form of Reproduction | |
| `7` | 1 | Geodetic Adjustment | |
| `8` | 1 | Physical Form of Publication | |

##### Pos. `0` — Altitude of Sensor

| Code | Signification |
|------|---------------|
| `a` | terrestrial |
| `b` | aerial |
| `c` | space |

##### Pos. `1` — Attitude of Sensor

| Code | Signification |
|------|---------------|
| `a` | low oblique |
| `b` | high oblique |
| `c` | vertical |

##### Pos. `4` — Quality of Image

| Code | Signification |
|------|---------------|
| `a` | poor |
| `b` | fair |
| `c` | good |
| `d` | very good |

##### Pos. `5` — Cloud Cover

| Code | Signification |
|------|---------------|
| `1` | 1/8 cover |
| `2` | 2/8 cover |
| `3` | 3/8 cover |
| `4` | 4/8 cover |
| `5` | 5/8 cover |
| `6` | 6/8 cover |
| `7` | 7/8 cover |
| `8` | completely covered by clouds |

##### Pos. `6` — Form of Reproduction

| Code | Signification |
|------|---------------|
| `x` | not applicable |

##### Pos. `7` — Geodetic Adjustment

| Code | Signification |
|------|---------------|
| `c` | centimetres |
| `i` | decimetres |
| `m` | metres |
| `d` | decametres |
| `h` | hectametres |
| `k` | kilometres |
| `x` | not applicable |

##### Pos. `8` — Physical Form of Publication

| Code | Signification |
|------|---------------|
| `a` | single |
| `b` | in parts |
| `c` | atlas including loose-leaf published atlas |
| `d` | as a separate supplement to a journal, monograph, etc. |
| `e` | bound into a journal, monograph, etc. |
| `z` | other |

### Zone 122 — CODED DATA FIELD: TIME PERIOD OF RESOURCE CONTENT

**Répétabilité :** — &nbsp;|&nbsp; **Occurrence :** —

> 🔢 **Champ à données codées** — sous-champ(s) à longueur fixe décodables par position de caractère.

| Sous-zone | Nom | Rép. | Occurrence | Description |
|-----------|-----|------|------------|-------------|
| `$a` | Time Period, 9999 B.C. to Present | Rép. | Optionnel | Codes and structured data indicate the time period from 9999 BC to the present. The subfield is minimum of 5-character and a maximum of 11-character in length. Repeatable. (EX 1) See also EX 2,3,4.... |

#### 🔢 Structure interne par position de caractère

| Offset | Lg. | Élément | Obl. |
|--------|-----|---------|------|
| `0` | 1 | Era Identifier | |
| `1-4` | 4 | Year |  |
| `5-6` | 2 | Month |  |
| `7-8` | 2 | Day |  |
| `9-10` | 2 | Time |  |

##### Pos. `0` — Era Identifier

| Code | Signification |
|------|---------------|
| `c` | date falls before the year 1 in the Gregorian calendar, i.e. B.C. |
| `d` | date falls after the year 1 in the Gregorian calendar, i.e. A.D. |

### Zone 123 — CODED DATA FIELD: CARTOGRAPHIC RESOURCES – SCALE AND CO-ORDINATES

**Répétabilité :** — &nbsp;|&nbsp; **Occurrence :** —

| Sous-zone | Nom | Rép. | Occurrence | Description |
|-----------|-----|------|------------|-------------|
| `$a` | Type of Scale | Non rép. | Oblig. si appl. | Mandatory. Not repeatable. A one-character code indicating the type of scale with the following values: ===PAGE 145=== |
| `$b` | Constant Ratio Linear Horizontal Scale | Rép. | Optionnel |  |
| `$c` | Constant Ratio Linear Vertical Scale | Rép. | Optionnel |  |
| `$d` | Co-ordinates – Westernmost Longitude | Non rép. | Optionnel |  |
| `$e` | Co-ordinates – Easternmost Longitude | Non rép. | Optionnel |  |
| `$f` | Co-ordinates – Northernmost Latitude | Non rép. | Optionnel |  |
| `$g` | Co-ordinates – Southernmost Latitude | Non rép. | Optionnel |  |
| `$h` | Angular Scale | Rép. | Optionnel |  |
| `$i` | Declination – Northern Limit | Non rép. | Optionnel |  |
| `$j` | Declination – Southern Limit | Non rép. | Optionnel |  |
| `$k` | Right ascension – Eastern Limits | Non rép. | Optionnel |  |
| `$m` | Right ascension – Western Limits | Non rép. | Optionnel |  |
| `$n` | Equinox | Non rép. | Optionnel |  |
| `$o` | Epoch | Non rép. | Optionnel |  |
| `$p` | Planet to which the Field Applies | Non rép. | Oblig. si appl. |  |

### Zone 125 — CODED DATA FIELD: SOUND RECORDINGS AND MUSIC

**Répétabilité :** — &nbsp;|&nbsp; **Occurrence :** —

> 🔢 **Champ à données codées** — sous-champ(s) à longueur fixe décodables par position de caractère.

| Sous-zone | Nom | Rép. | Occurrence | Description |
|-----------|-----|------|------------|-------------|
| `$a` | Format of Notated Music | Non rép. | Optionnel | Codes indicate the aspects of the format of notated music. The subfield is 2-characters in length. Not repeatable. Subfield $a fixed-length data elements: |
| `$b` | Literary Text Indicator (Non-Music Performance) | Non rép. | Optionnel | 2-characters long. |
| `$c` | Multiple Musical Formats | Non rép. | Optionnel | Variable length. |

#### 🔢 Structure interne par position de caractère

| Offset | Lg. | Élément | Obl. |
|--------|-----|---------|------|
| `0` | 1 | Type of Score | |
| `1` | 1 | Parts Indicator | |

##### Pos. `0` — Type of Score

| Code | Signification |
|------|---------------|
| `a` | full score The resource is a full score format, i.e. a series of staves on which |
| `b` | score (miniature or study size) A musical score not primarily intended for performance use, with |
| `c` | vocal score, accompaniment |
| `d` | voice score, chorus score, |
| `e` | condensed score, |
| `f` | graphic score Notation including non traditional graphic elements. |
| `g` | close score Format giving all the parts on a minimum number of staves, |
| `h` | tablature A music notation where letters, numbers or graphic symbols |
| `i` | choir-book Format giving the different vocal parts on two facing pages of |
| `j` | compressed score Score of only voices and continuo, omitting the other |
| `k` | pseudo-score Score without vertical coincidence of voices. |
| `l` | solo part A part containing only one solo for a voice of a choir. |
| `m` | multiple formats Indicate individual formats in subfield $c. |
| `n` | score with only homogeneous |
| `o` | condensed score with text and |
| `p` | table book Parts written with different orientation on facing pages, so as to |
| `u` | unknown |
| `x` | not applicable The resource is not printed or manuscript music. All sound |
| `z` | other Format of music for which none of the other defined codes are |

##### Pos. `1` — Parts Indicator

| Code | Signification |
|------|---------------|
| `a` | parts exist (instrumental and vocal) |
| `b` | instrumental parts |
| `c` | vocal parts |
| `u` | unknown |
| `x` | not applicable  Including compositions for single instrument or |
| `y` | parts not present |
| `d` | history |
| `e` | lectures, speeches |
| `f` | instructions (How to ...) |
| `g` | sounds |
| `h` | autobiography |
| `i` | biography |
| `j` | essays |
| `k` | reporting |
| `l` | memoirs |
| `m` | rehearsals |
| `n` | interviews |
| `o` | advertising texts |
| `p` | instruction (language) |
| `q` | conference proceedings |
| `r` | comedy |
| `s` | folktales |
| `t` | sacred texts |
| `z` | other types of literary text |
| `#` | not used |

### Zone 126 — CODED DATA FIELD: SOUND RECORDINGS – PHYSICAL ATTRIBUTES

**Répétabilité :** — &nbsp;|&nbsp; **Occurrence :** —

> 🔢 **Champ à données codées** — sous-champ(s) à longueur fixe décodables par position de caractère.

| Sous-zone | Nom | Rép. | Occurrence | Description |
|-----------|-----|------|------------|-------------|
| `$a` | Sound Recording Coded Data (General) | Non rép. | Optionnel | Codes indicate the general aspects of the sound recording. The subfield is 15-characters in length. Not repeatable. Subfield $a fixed-length data elements: |
| `$b` | Sound Recording Coded Data (Detail) | Non rép. | Optionnel | Codes indicate the detail aspects of the sound recording. The subfield is 3-characters in length. Not repeatable. This subfield is used only for pre-FRBR/IFLA LRM context. Do not use this subfield,... |

#### 🔢 Structure interne par position de caractère

| Offset | Lg. | Élément | Obl. |
|--------|-----|---------|------|
| `0` | 1 | Form of Release | |
| `1` | 1 | Kind of Material | |
| `2` | 1 | Kind of Cutting | |
| `3` | 1 | Groove Width | |
| `4` | 1 | Dimensions (Sound Recordings) | |
| `5` | 1 | Tape Width | |
| `6` | 1 | Tape Configuration | |
| `7-12` | 6 | Accompanying Textual Material |  |
| `13` | 1 | Recording Technique | |
| `14` | 1 | Special Reproduction Characteristics | |

##### Pos. `0` — Form of Release

| Code | Signification |
|------|---------------|
| `a` | instantaneous Unique, cut on the spot. |
| `b` | mass produced Most commercial discs or tapes. |
| `c` | master tape |
| `d` | tape duplication master |
| `e` | disc master (negative) |
| `f` | mother (positive) |
| `g` | stamper (negative) |
| `h` | test pressing |
| `u` | unknown |
| `x` | not applicable |
| `z` | other |

##### Pos. `1` — Kind of Material

| Code | Signification |
|------|---------------|
| `u` | unknown |
| `x` | not applicable |
| `z` | other |
| `a` | lacquered (e.g. acetate) Discs |
| `b` | metal (e.g. aluminium) Discs |
| `c` | shellac pressing (mass produced) Discs |
| `d` | plastic pressing (mass produced) Discs |
| `e` | metal and plastic (compact discs) Discs |
| `g` | wax (instantaneous) Cylinders |
| `h` | plastic (mass produced) Cylinders |
| `i` | paper backed Tapes |
| `j` | acetate Tapes |
| `k` | pvc Tapes |
| `l` | polyester Tapes |

##### Pos. `2` — Kind of Cutting

| Code | Signification |
|------|---------------|
| `a` | lateral or combined cutting |
| `b` | vertical (hill and dale) cutting |
| `u` | unknown |
| `x` | not applicable |

##### Pos. `3` — Groove Width

| Code | Signification |
|------|---------------|
| `a` | coarse/standard Discs issued at 78 rpm are usually coarse. Cylinders issued at 120 rpm |
| `b` | microgroove/fine Discs issued at 16 2/3, 33 1/3, and 45 rpm are usually issued as |
| `u` | unknown |
| `x` | not applicable This includes compact discs, which do not have grooves. |
| `z` | other |

##### Pos. `4` — Dimensions (Sound Recordings)

| Code | Signification |
|------|---------------|
| `a` | 3 in. |
| `b` | 5 in. |
| `c` | 7 in. |
| `d` | 10 in. |
| `e` | 12 in. |
| `f` | 16 in. |
| `g` | 14 in. |
| `h` | 4¾ in. (compact disc) |
| `j` | 3 7/8 x 2½ in. cassette |
| `o` | 5¼ x 3 7/8 in. cartridge |
| `s` | 2¾ x 4 in. cylinder |
| `u` | unknown |
| `x` | not applicable |
| `z` | other |

##### Pos. `5` — Tape Width

| Code | Signification |
|------|---------------|
| `a` | ¼ in. |
| `b` | ½ in. |
| `c` | 1 in. |
| `d` | 1/8 in. |
| `e` | 2 in. |
| `f` | 1/3 in. (8 mm.) |
| `u` | unknown |
| `x` | recording not a tape |
| `z` | other |

##### Pos. `6` — Tape Configuration

| Code | Signification |
|------|---------------|
| `a` | full (1) track |
| `b` | half (2) track |
| `c` | quarter (4) track |
| `d` | eight track |
| `e` | twelve track |
| `f` | sixteen track |
| `g` | twenty-four track |
| `h` | six track |
| `u` | unknown |
| `x` | recording not a tape |
| `z` | other |

##### Pos. `7-12` — Accompanying Textual Material

| Code | Signification |
|------|---------------|
| `a` | discography |
| `b` | bibliography |
| `c` | thematic index |
| `d` | libretto or text |
| `e` | biography of composer |
| `f` | biography of performer or history of ensemble |
| `g` | technical or historical information on instruments |
| `h` | technical information on music |
| `i` | historical information about music |
| `j` | other historical information |
| `k` | ethnological information |
| `l` | biography of arranger or transcriber |
| `r` | instructional material |
| `s` | score |
| `z` | other accompanying textual material |

##### Pos. `13` — Recording Technique

| Code | Signification |
|------|---------------|
| `a` | acoustic |
| `b` | electric |
| `c` | digital |
| `d` | analog |
| `u` | unknown |
| `z` | other |

##### Pos. `14` — Special Reproduction Characteristics

| Code | Signification |
|------|---------------|
| `a` | NAB standard |
| `b` | CCIR/IEC standard |
| `c` | DBX processed |
| `d` | digital (compact disc) |
| `e` | Dolby-A encoded |
| `f` | Dolby-B encoded |
| `g` | Dolby-C encoded |
| `h` | CX encoded |
| `x` | not applicable |
| `u` | unknown |
| `z` | other |

### Zone 127 — CODED DATA FIELD: DURATION OF SOUND RECORDINGS AND NOTATED MUSIC

**Répétabilité :** — &nbsp;|&nbsp; **Occurrence :** —

| Sous-zone | Nom | Rép. | Occurrence | Description |
|-----------|-----|------|------------|-------------|
| `$a` | Duration | Rép. | Optionnel | This subfield contains a six-character numeric string representing the duration of a manifestation consisting in or containing sound recordings or a section of a sound recording, or the estimated d... |

### Zone 128 — CODED DATA FIELD: FORM OF MUSICAL WORK AND KEY OR MODE

**Répétabilité :** — &nbsp;|&nbsp; **Occurrence :** —

| Sous-zone | Nom | Rép. | Occurrence | Description |
|-----------|-----|------|------------|-------------|
| `$a` | Form of Musical Work | Rép. | Optionnel | Contains a code indicating the form of a musical work. If more than one form is present the subfield is repeated. Use the codes maintained and updated by IAML (International Association of Music Li... |
| `$d` | Key or Mode of Musical Work | Non rép. | Optionnel | Contains a code indicating the key or church mode of the musical work, if significant. Major keys are indicated with a letter “a”-“g”, minor keys with a letter “a”-“g” followed by “m”; sharps with ... |

### Zone 130 — CODED DATA FIELD: MICROFORMS – PHYSICAL ATTRIBUTES

**Répétabilité :** — &nbsp;|&nbsp; **Occurrence :** —

> 🔢 **Champ à données codées** — sous-champ(s) à longueur fixe décodables par position de caractère.

| Sous-zone | Nom | Rép. | Occurrence | Description |
|-----------|-----|------|------------|-------------|
| `$a` | Microform Coded Data – Physical Attributes | Non rép. | Optionnel | Codes indicate the physical attributes of the microform. The subfield is 11-characters in length. Not repeatable. Subfield $a fixed-length data elements: |

#### 🔢 Structure interne par position de caractère

| Offset | Lg. | Élément | Obl. |
|--------|-----|---------|------|
| `0` | 1 | Specific Material Designation | |
| `1` | 1 | Polarity | |
| `2` | 1 | Dimensions | |
| `3` | 1 | Reduction Ratio | |
| `4-6` | 3 | Specific Reduction Ratio |  |
| `7` | 1 | Colour | |
| `8` | 1 | Emulsion on Film | |
| `9` | 1 | Generation | |
| `10` | 1 | Base of Film | |

##### Pos. `0` — Specific Material Designation

| Code | Signification |
|------|---------------|
| `a` | aperture card |
| `b` | microform cartridge |
| `c` | microfilm cassette |
| `d` | microfilm reel |
| `e` | microfiche |
| `f` | microfiche cassette |
| `g` | micro opaque |
| `h` | microfilm slip |
| `u` | unspecified |
| `z` | other |

##### Pos. `1` — Polarity

| Code | Signification |
|------|---------------|
| `a` | positive |
| `b` | negative |
| `d` | mixed polarity |
| `u` | unknown |

##### Pos. `2` — Dimensions

| Code | Signification |
|------|---------------|
| `a` | 8 mm (microfilm) |
| `d` | 16 mm (microfilm) |
| `f` | 35 mm (microfilm) |
| `g` | 70 mm (microfilm) |
| `h` | 105 mm (microfilm) |
| `l` | 3x5 in. (8x13 cm) (microfiche or micro opaque) |
| `m` | 4x6 in. (llx15 cm) (microfiche or micro opaque) |
| `o` | 6x9 in. (16x23 cm) (microfiche or micro opaque) |
| `p` | 3 1/4 x 7 3/8 in. (9x19 cm) (aperture card) |
| `u` | unknown |
| `z` | other |

##### Pos. `3` — Reduction Ratio

| Code | Signification |
|------|---------------|
| `a` | low reduction |
| `b` | normal (16x – 30x) |
| `c` | high (31x – 60x) |
| `d` | very high (61x – 90x) |
| `e` | ultra-high (9lx – ) |
| `u` | unknown |
| `v` | varies |

##### Pos. `7` — Colour

| Code | Signification |
|------|---------------|
| `a` | one colour The resource is in a single colour, including black-and-white. |
| `b` | multicoloured The resource is in more than one colour. |
| `u` | unknown |
| `v` | mixed The resource is in a combination of black-and-white and/or colours. |
| `z` | other The resource has colour characteristics not covered by the other defined codes. |
| `#` | value position |

##### Pos. `8` — Emulsion on Film

| Code | Signification |
|------|---------------|
| `a` | silver halide |
| `b` | diazo |
| `c` | vesicular |
| `u` | unknown |
| `v` | mixed emulsion |
| `x` | not applicable |
| `z` | other |

##### Pos. `9` — Generation

| Code | Signification |
|------|---------------|
| `a` | first generation (Master) |
| `b` | printing master |
| `c` | service copy |
| `u` | unknown |
| `v` | mixed generations |

##### Pos. `10` — Base of Film

| Code | Signification |
|------|---------------|
| `a` | safety base, undetermined |
| `b` | not a safety base (e.g. nitrate) |
| `c` | safety base, acetate undetermined |
| `d` | safety base, diacetate |
| `e` | safety base, polyester |
| `f` | safety base, mixed, mixed safety base films spliced together, no nitrate film |
| `g` | safety base, triacetate |
| `u` | unknown |
| `x` | not applicable |
| `m` | - 4x6 inches |
| `h` | Contour Interval R O |
| `i` | Supplementary Contour Interval R O |
| `j` | Unit of Measurement of Bathymetry R O |
| `k` | Bathymetric Interval R O |
| `l` | Supplementary Bathymetric Interval R O |
| `1` | # blank (not defined) |
| `2` | # blank (not defined) |

### Zone 135 — CODED DATA FIELD: ELECTRONIC RESOURCES

**Répétabilité :** — &nbsp;|&nbsp; **Occurrence :** —

> 🔢 **Champ à données codées** — sous-champ(s) à longueur fixe décodables par position de caractère.

| Sous-zone | Nom | Rép. | Occurrence | Description |
|-----------|-----|------|------------|-------------|
| `$a` | Coded Data for Electronic Resources | Non rép. | Optionnel | Codes indicate the aspects of the electronic resource. The subfield is 13-characters in length. Not repeatable. Subfield $a fixed-length data elements: |

#### 🔢 Structure interne par position de caractère

| Offset | Lg. | Élément | Obl. |
|--------|-----|---------|------|
| `0` | 1 | Type of Electronic Resource | |
| `1` | 1 | Special Material Designation | |
| `2` | 1 | Colour | |
| `3` | 1 | Dimensions | |
| `4` | 1 | Sound | |
| `5-7` | 3 | Image Bit Depth |  |
| `8` | 1 | Number of File Formats | |
| `9` | 1 | Quality Assurance Target(s) | |
| `10` | 1 | Antecedent/Source | |
| `11` | 1 | Level of Compression | |
| `12` | 1 | Reformatting Quality | |

##### Pos. `0` — Type of Electronic Resource

| Code | Signification |
|------|---------------|
| `a` | numeric A data file that contains mostly numbers or representation by numbers, such |
| `b` | computer |
| `c` | representational A data file that contains pictorial or graphic information that can be |
| `d` | text A data file that contains mostly alphabetic information (words or sentences) |
| `e` | bibliographic |
| `f` | font Code f indicates a resource contains information for a computer to produce |
| `g` | game Code g indicates that the resource is a game, intended for recreational or |
| `h` | sound Code h indicates that the resource consists of data encoding sounds produced |
| `i` | interactive |
| `j` | online system or |
| `u` | unknown The type of data file is not known. |
| `v` | combination A data file with different types of content. |
| `z` | other A type of data file for which no specific code has been assigned. |

##### Pos. `1` — Special Material Designation

| Code | Signification |
|------|---------------|
| `a` | cartridge magnetic tape Removable module containing a secondary storage |
| `b` | computer chip cartridge Removable module containing a miniaturized |
| `c` | computer optical disc cartridge Removable module containing one or more |
| `d` | computer disc, type unspecified A disc whose type is not specified. |
| `e` | computer disc cartridge, type unspecified A disc cartridge whose type is not specified. |
| `f` | computer magnetic cassette tape Removable module, somewhat like an audio cassette, |
| `h` | magnetic tape for main-frame computers Removable spool containing magnetic tape that can be |
| `j` | magnetic disk Digital information storage medium usually consisting |
| `k` | computer card A card containing digitally encoded data designed for |
| `m` | computer magneto-optical disk Erasable or semi-erasable storage medium, similar to a |
| `o` | computer optical disk Medium that uses a series of laser-burned micron-sized |
| `r` | online  A special material designation for digital resources that |
| `s` | standalone device Standalone devices consist of storage and reader |
| `u` | unknown Special material designation for the digital resource is |
| `z` | other None of the other codes is appropriate. |

##### Pos. `2` — Colour

| Code | Signification |
|------|---------------|
| `a` | one colour The resource is in a single colour. Does not include black-and-white and |
| `b` | black-and-white The resource is in black-and-white only (i.e., only on two levels). |
| `c` | multicoloured The resource is in more than one colour. |
| `g` | greyscale The resource is in a large number of shades of grey. |
| `m` | mixed The resource is in a combination of black-and-white, greyscale, and/or |
| `n` | not applicable The colour characteristics do not apply. |
| `u` | unknown |
| `z` | other The resource has colour characteristics not covered by the other defined |
| `#` | value position |

##### Pos. `3` — Dimensions

| Code | Signification |
|------|---------------|
| `a` | 3½ in. |
| `e` | 12 in. |
| `g` | 4¾ in. or 12 cm. |
| `i` | 1 1/8 x 2 3/8 in. |
| `j` | 3 7/8 x 2 1/2 in. |
| `n` | Not applicable |
| `o` | 5¼ in. |
| `u` | Unknown |
| `v` | 8 in. |
| `z` | Other |

##### Pos. `4` — Sound

| Code | Signification |
|------|---------------|
| `#` | no sound (silent) |
| `a` | sound on medium |
| `u` | unknown |
| `x` | value position not needed According to cataloguing rules based on IFLA LRM, this data is not |

##### Pos. `8` — Number of File Formats

| Code | Signification |
|------|---------------|
| `a` | one file format |
| `m` | multiple file formats |
| `u` | unknown |

##### Pos. `9` — Quality Assurance Target(s)

| Code | Signification |
|------|---------------|
| `a` | absent |
| `n` | not applicable |
| `p` | present |
| `u` | unknown |

##### Pos. `10` — Antecedent/Source

| Code | Signification |
|------|---------------|
| `a` | file reproduced from original |
| `b` | file reproduced from microform |
| `c` | file reproduced from electronic resource |
| `d` | file reproduced from an intermediate source other than microform |
| `m` | mixed |
| `n` | not applicable |
| `u` | unknown |

##### Pos. `11` — Level of Compression

| Code | Signification |
|------|---------------|
| `a` | uncompressed |
| `b` | lossless |
| `d` | lossy |
| `m` | mixed |
| `u` | unknown |

##### Pos. `12` — Reformatting Quality

| Code | Signification |
|------|---------------|
| `a` | access Indicates that the electronic resource is of a quality that will support current, |
| `n` | not applicable |
| `p` | preservation Indicates that the electronic resource was created via reformatting to help |
| `r` | replacement Indicates the electronic resource is of very high quality and, when printed |
| `u` | unknown |

### Zone 140 — CODED DATA FIELD: ANTIQUARIAN – GENERAL

**Répétabilité :** — &nbsp;|&nbsp; **Occurrence :** —

> 🔢 **Champ à données codées** — sous-champ(s) à longueur fixe décodables par position de caractère.

| Sous-zone | Nom | Rép. | Occurrence | Description |
|-----------|-----|------|------------|-------------|
| `$a` | Antiquarian Coded Data – General | Non rép. | Optionnel | Codes indicate the aspects of the antiquarian resource. The subfield is 28-characters in length. Not repeatable. Subfield $a fixed-length data elements: |

#### 🔢 Structure interne par position de caractère

| Offset | Lg. | Élément | Obl. |
|--------|-----|---------|------|
| `0-3` | 4 | Illustration Codes – Book |  |
| `4-7` | 4 | Illustration Codes – Full Page Plates |  |
| `8` | 1 | Illustration Code – Technique | |
| `9-16` | 8 | Form of Contents Code |  |
| `17-18` | 2 | Literature Code |  |
| `19` | 1 | Biography Code | |
| `20` | 1 | Support Material – Book | |
| `21` | 1 | Support Material – Plates | |
| `22` | 1 | Watermark Code | |
| `23` | 1 | Printer’s Device Code | |
| `24` | 1 | Publisher’s Device Code | |
| `25` | 1 | Ornamental Device Code | |
| `26-27` | 2 | Unassigned |  |

##### Pos. `0-3` — Illustration Codes – Book

| Code | Signification |
|------|---------------|
| `a` | illustrations (use for types of illustrations which are not covered below, or when types of |
| `b` | illuminations |
| `c` | ornamental letter |
| `d` | miniature |
| `e` | rubric (i.e. heading etc. in special lettering) |
| `f` | vignette (i.e. ornament round capital letter etc. or in blank space) |
| `g` | frontispiece |
| `h` | portrait |
| `i` | vedute (i.e. panoramic view of a city etc. with important features identified) |
| `j` | maps |
| `k` | charts (i.e. maps for navigation) |
| `l` | plans |
| `m` | music |
| `n` | coats of arms |
| `o` | genealogical tables |
| `y` | no illustrations |
| `z` | other |
| `#` | value position not needed |

##### Pos. `4-7` — Illustration Codes – Full Page Plates

| Code | Signification |
|------|---------------|
| `a` | illustrations |
| `g` | frontispiece |
| `h` | portraits |
| `i` | vedute |
| `j` | maps |
| `k` | charts (i.e. maps for navigation) |
| `l` | plans |
| `m` | music |
| `n` | coats of arms |
| `o` | genealogical tables |
| `y` | no illustrations |
| `z` | other |
| `#` | value position not needed |

##### Pos. `8` — Illustration Code – Technique

| Code | Signification |
|------|---------------|
| `a` | woodcut |
| `b` | lithography |
| `c` | etching |
| `d` | aquatint |
| `e` | engraving |
| `u` | unknown |
| `v` | mixed |
| `z` | other |
| `#` | value position not needed |

##### Pos. `9-16` — Form of Contents Code

| Code | Signification |
|------|---------------|
| `aa` | religious work (use for Bible stories, catechism, devotional literature, hymn, indulgences, litany, |
| `ab` | catechism |
| `ac` | devotional literature (i.e. books of hours, devotional calendars, novenas, prayer books, psalters, |
| `ad` | sermon, i.e. apocalyptic sermons, children’s sermons, occasional sermons, etc. |
| `ae` | service books, i.e. liturgical books: antiphonary, breviary, evangelistary, gradual, hymnal, missal, |
| `ba` | scientific work, i.e. scientific atlas, herbal, laboratory note, pharmacopoeia, etc. |
| `bb` | discussion, dissertation, thesis |
| `ca` | social customs, i.e. courtesy books, festival books, fraternity and lodge rituals, etc. |
| `da` | legal work, i.e. laws, acts, bills, charters, treaties, regulations, etc. |
| `db` | political work |
| `ea` | ephemera, i.e. advertisements (booksellers’, printers’, publishers’ advertisements), salescatalogues |
| `fa` | reference work (use for academic catalogue, library catalogue, museum catalogue, almanac, |
| `fb` | library catalogue |
| `fc` | bibliography |
| `fd` | calendar |
| `fe` | index |
| `ff` | dictionary |
| `fg` | encyclopedia |
| `ga` | historical work, i.e. chronicles, annals, biography, genealogy, family, local, oral and military |
| `ha` | polemical treatise |
| `ia` | discursive work, i.e. addresses, dialogue, interview, letters, questions and answers, etc. |
| `ja` | commemorative work, i.e. Festschrift, album, epitaph, eulogy, memorial, etc. |
| `ka` | instructional work (use for instructional work in general. If specific designation is needed for |
| `kb` | manual, i.e. handbook |
| `kc` | textbook, i.e. alphabet, anthology, chrestomathy, primer, reader, grammar, etc. |
| `la` | record-keeping work, i.e. certificate, price list, subscribers’ list, voting register, etc. |
| `ma` | recreations, i.e. colouring book, games, puzzles, etc. |
| `na` | version of a work, i.e. adaptation, abridgement, parody, expurgated edition, scenario, etc. |
| `zz` | other |
| `##` | value position not needed |

##### Pos. `17-18` — Literature Code

| Code | Signification |
|------|---------------|
| `aa` | poetry |
| `ab` | romance, e.g. gesta, pastoral romance |
| `ca` | drama |
| `da` | libretto |
| `ea` | fiction (use for fiction in general. If specific designation is needed for novel, novella, fable, fairy |
| `eb` | novel |
| `ec` | novella |
| `ed` | fable |
| `ef` | fairy tale |
| `eh` | legend |
| `ei` | parable |
| `ej` | short story |
| `fa` | essay, feuilleton |
| `ga` | humour, satire |
| `ha` | letters |
| `ia` | miscellanea |
| `ja` | maxim, aphorism, proverb, anecdote |
| `ka` | juvenile literature |
| `la` | other (use for chronicle, memoir, diary, biography, hagiography, travelogue, erotica, mystic |
| `lb` | chronicle |
| `lc` | memoir |
| `ld` | diary |
| `le` | biography |
| `lf` | hagiography |
| `lg` | travelogue |
| `lh` | erotica |
| `li` | mystic literature |
| `ma` | oratory, speeches |
| `yy` | not a literary text |
| `zz` | multiple or other |

##### Pos. `19` — Biography Code

| Code | Signification |
|------|---------------|
| `a` | autobiography (use for memoir and confession) |
| `b` | individual biography |
| `c` | collective biography |
| `d` | contains biographical information |
| `y` | not biographical |
| `z` | multiple or other form |

##### Pos. `20` — Support Material – Book

| Code | Signification |
|------|---------------|
| `a` | paper, general |
| `b` | hand-made paper |
| `c` | rice paper |
| `d` | wood-pulp paper |
| `e` | parchment, vellum |
| `z` | other |

##### Pos. `21` — Support Material – Plates

| Code | Signification |
|------|---------------|
| `a` | paper, general |
| `b` | hand-made paper |
| `c` | rice paper |
| `d` | wood-pulp paper |
| `e` | parchment, vellum |
| `z` | other |
| `#` | value position not needed |

##### Pos. `22` — Watermark Code

| Code | Signification |
|------|---------------|
| `0` | paper does not contain watermark |
| `1` | paper contains watermark |

##### Pos. `23` — Printer’s Device Code

| Code | Signification |
|------|---------------|
| `0` | printer’s device not present |
| `1` | printer’s device present |

##### Pos. `24` — Publisher’s Device Code

| Code | Signification |
|------|---------------|
| `0` | publisher’s device not present |
| `1` | publisher’s device present |

##### Pos. `25` — Ornamental Device Code

| Code | Signification |
|------|---------------|
| `0` | ornamental device not present |
| `1` | ornamental device present |

### Zone 141 — CODED DATA FIELD: ITEM SPECIFIC ATTRIBUTES

**Répétabilité :** — &nbsp;|&nbsp; **Occurrence :** —

> 🔢 **Champ à données codées** — sous-champ(s) à longueur fixe décodables par position de caractère.

| Sous-zone | Nom | Rép. | Occurrence | Description |
|-----------|-----|------|------------|-------------|
| `$a` | Coded Data – Item Specific Attributes | Non rép. | Optionnel | Codes indicate the attributes of the item. The subfield is 8-characters in length. Not repeatable. Subfield $a fixed-length data elements: |
| `$b` | Binding Specific Characteristics | Non rép. | Optionnel | Codes indicate binding specific characteristics of item of Binding Material Code – General ($a/0-2). The subfield is 8-characters in length. Not repeatable. Subfield $b fixed-length data elements: |
| `$c` | Age | Non rép. | Optionnel |  |
| `$d` | Binding State of Preservation Code –Specific | Non rép. | Optionnel |  |
| `$e` | Body of the Book Specific Characteristics | Non rép. | Optionnel |  |
| `$5` | Institution to which the Field Applies | Non rép. | Optionnel |  |

#### 🔢 Structure interne par position de caractère

| Offset | Lg. | Élément | Obl. |
|--------|-----|---------|------|
| `0-1` | 2 | Primary Binding Material |  |
| `3` | 1 | Types of Binding Code | |
| `4` | 1 | Binding Decoration | |
| `5` | 1 | Decoration Motifs | |
| `6` | 1 | Binding Pieces | |
| `7` | 1 | Boards | |

##### Pos. `0-1` — Primary Binding Material

| Code | Signification |
|------|---------------|
| `aa` | parchment Skins/Leathers. |
| `ab` | vellum Skins/Leathers. |
| `ac` | calfskin Skins/Leathers. |
| `ad` | sheepskin Skins/Leathers. |
| `ae` | basil, basan, bazan Skins/Leathers. |
| `af` | roan Skins/Leathers. |
| `ag` | goatskin Skins/Leathers. |
| `ah` | morocco Skins/Leathers. |
| `ai` | niger Skins/Leathers. |
| `aj` | chamois Skins/Leathers. |
| `al` | pigskin Skins/Leathers. |
| `am` | alum tawed, white leather, hungarian leather Skins/Leathers. |
| `ao` | horse- or ass-skin, etc. Skins/Leathers. |
| `ap` | fish and marine mammals: ray, shark, sealskin, etc. Skins/Leathers. |
| `aq` | snakeskin Skins/Leathers. |
| `ar` | other animal skins Skins/Leathers. |
| `bi` | ivory Other animal materials. |
| `bm` | mother-of-pearl, nacre Other animal materials. |
| `bt` | tortoiseshell Other animal materials. |
| `bz` | other materials from animal shells, tusks, etc. Other animal materials. |
| `ca` | cardboard Paper and by-products. |
| `cb` | paper Paper and by-products. |
| `cc` | marbled paper Paper and by-products. |
| `cd` | papier mâché Paper and by-products. |
| `da` | cloth, book cloth Other natural materials. |
| `db` | buckram Other natural materials. |
| `dc` | calico Other natural materials. |
| `dd` | canvas Other natural materials. |
| `de` | linen Other natural materials. |
| `df` | moiré Other natural materials. |
| `dg` | silk Other natural materials. |
| `dh` | watered silk (moiré effect) Other natural materials. |
| `dj` | satin Other natural materials. |
| `dl` | velvet Other natural materials. |
| `dm` | other fabrics made of natural materials Other natural materials. |
| `dw` | wood Other natural materials. |
| `ep` | plastic coverings Artificial materials. |
| `es` | synthetic fibres Artificial materials. |
| `fb` | brass, bronze Metals. |
| `fg` | gold Metals. |
| `fs` | silver Metals. |
| `tt` | mixed Used in positions 2-3 if there are more |
| `uu` | unknown |
| `xx` | not applicable The item is unbound. Use ## in |
| `zz` | other binding materials |

##### Pos. `3` — Types of Binding Code

| Code | Signification |
|------|---------------|
| `a` | original binding, i.e. primary |
| `b` | non-original, i.e. rebound |
| `c` | modern |
| `d` | restored, facsimile |
| `e` | restored, imitation |
| `f` | work bound with another (use for publisher’s, distributor’s and/or owner’s binding) |
| `h` | in sheets, unbound |
| `j` | facsimile |
| `k` | restored original |
| `l` | restored non-original |
| `u` | unknown |
| `z` | other |
| `#` | information not available |

##### Pos. `4` — Binding Decoration

| Code | Signification |
|------|---------------|
| `#` | information not available |
| `a` | gold tooling |
| `b` | silver tooling |
| `c` | blind tooling |
| `u` | unknown |
| `x` | not applicable |
| `z` | other |

##### Pos. `5` — Decoration Motifs

| Code | Signification |
|------|---------------|
| `#` | information not available |
| `a` | geometric |
| `b` | anthropomorphic |
| `c` | floral |
| `d` | animal |
| `e` | heraldic |
| `f` | monograms |
| `g` | mixed |
| `u` | unknown |
| `x` | not applicable |
| `z` | other |

##### Pos. `6` — Binding Pieces

| Code | Signification |
|------|---------------|
| `#` | information not available |
| `a` | toggles or ties |
| `b` | buckles |
| `c` | fastenings |
| `d` | bosses |
| `e` | metal decorative pieces |
| `f` | stiffeners |
| `u` | unknown |
| `x` | not applicable |
| `z` | other |

##### Pos. `7` — Boards

| Code | Signification |
|------|---------------|
| `#` | information not available |
| `a` | wood |
| `b` | paper |
| `c` | pasteboard |
| `u` | unknown |
| `x` | not applicable |
| `z` | other |

### Zone 145 — CODED DATA FIELD: MEDIUM OF PERFORMANCE [OBSOLETE]

**Répétabilité :** — &nbsp;|&nbsp; **Occurrence :** —

*Champ de contrôle — pas de sous-zones ni d'indicateurs.*

### Zone 146 — CODED DATA FIELD: MEDIUM OF PERFORMANCE

**Répétabilité :** — &nbsp;|&nbsp; **Occurrence :** —

> 🔢 **Champ à données codées** — sous-champ(s) à longueur fixe décodables par position de caractère.

| Sous-zone | Nom | Rép. | Occurrence | Description |
|-----------|-----|------|------------|-------------|
| `$a` | Type of Performance Medium | Non rép. | Optionnel |  |
| `$b` | Instrument or Voice Soloist | Rép. | Optionnel |  |
| `$d` | Vocal or Instrumental Ensemble | Rép. | Oblig. si appl. |  |
| `$h` | Number of Parts | Rép. | Optionnel |  |
| `$i` | Number of Players | Rép. | Optionnel |  |

#### 🔢 Structure interne par position de caractère

| Offset | Lg. | Élément | Obl. |
|--------|-----|---------|------|
| `2-4` | 3 | Category of Instrument |  |
| `5-8` | 4 | Other Details |  |
| `7-8` | 2 | Other Details |  |

### Zone 181 — CODED DATA FIELD: CONTENT FORM

**Répétabilité :** Rép. &nbsp;|&nbsp; **Occurrence :** Optionnel

> 🔢 **Champ à données codées** — sous-champ(s) à longueur fixe décodables par position de caractère.

| Sous-zone | Nom | Rép. | Occurrence | Description |
|-----------|-----|------|------------|-------------|
| `$a` | ISBD Content Form Code | Non rép. | Optionnel | Codes indicate the content form that applies to the resource, according to the provisions of ISBD for this element, and the extent of applicability of a content form to the resource described. The ... |
| `$b` | ISBD Content Qualification Code | Rép. | Optionnel | Codes indicate the content qualification applying to the resource, according to the provisions of ISBD for this element. The subfield is 6-characters in length. Optional. Repeatable. Subfield $b fi... |
| `$c` | Other Coding for Content Form | Rép. | Optionnel |  |
| `$2` | Source | Non rép. | Oblig. si appl. |  |
| `$6` | Interfield Linking Data | Rép. | Optionnel |  |

#### 🔢 Structure interne par position de caractère

| Offset | Lg. | Élément | Obl. |
|--------|-----|---------|------|
| `0` | 1 | Specification of Type | |
| `1` | 1 | Specification of Motion | |
| `2` | 1 | Specification of Dimensionality | |
| `3-5` | 3 | Sensory Specification |  |

##### Pos. `0` — Specification of Type

| Code | Signification |
|------|---------------|
| `a` | notated |
| `b` | performed |
| `c` | cartographic |
| `x` | not applicable |
| `#` | position not used |

##### Pos. `1` — Specification of Motion

| Code | Signification |
|------|---------------|
| `a` | moving |
| `b` | still |
| `x` | not applicable Resource is not an image. |
| `#` | position not used |

##### Pos. `2` — Specification of Dimensionality

| Code | Signification |
|------|---------------|
| `2` | two-dimensional |
| `3` | three-dimensional |
| `x` | not applicable Resource is not an image. |
| `#` | position not used |

##### Pos. `3-5` — Sensory Specification

| Code | Signification |
|------|---------------|
| `a` | aural |
| `b` | gustatory |
| `c` | olfactory |
| `d` | tactile |
| `e` | visual |
| `#` | position not used |

### Zone 182 — CODED DATA FIELD: MEDIA TYPE

**Répétabilité :** Rép. &nbsp;|&nbsp; **Occurrence :** Optionnel

> 🔢 **Champ à données codées** — sous-champ(s) à longueur fixe décodables par position de caractère.

| Sous-zone | Nom | Rép. | Occurrence | Description |
|-----------|-----|------|------------|-------------|
| `$a` | ISBD Media Type Code | Non rép. | Optionnel | Codes indicate the content form that applies to the resource, according to the provisions of ISBD for this element, and the extent of applicability of a content form to the resource described. The ... |
| `$c` | Other Coding for Media Type | Rép. | Optionnel | A code for a source of codes other than ISBD. For example, codes from Term and Code List for RDA Media Type (http://www.loc.gov/standards/valuelist/rdamedia.html), defined for use in MARC21. |
| `$2` | Source | Non rép. | Oblig. si appl. | Source of the code used in $c. Mandatory if $c is used. |
| `$6` | Interfield Linking Data | Rép. | Optionnel | ===PAGE 269=== |

#### 🔢 Structure interne par position de caractère

| Offset | Lg. | Élément | Obl. |
|--------|-----|---------|------|
| `0` | 1 | ISBD Media Type Code | |

##### Pos. `0` — ISBD Media Type Code

| Code | Signification |
|------|---------------|
| `a` | audio |
| `b` | electronic |
| `c` | microform |
| `d` | microscopic |
| `e` | projected |
| `f` | stereographic |
| `g` | video |
| `m` | multiple media |
| `n` | unmediated |
| `z` | other media |

### Zone 183 — CODED DATA FIELD: TYPE OF CARRIER

**Répétabilité :** Rép. &nbsp;|&nbsp; **Occurrence :** Optionnel

| Sous-zone | Nom | Rép. | Occurrence | Description |
|-----------|-----|------|------------|-------------|
| `$a` | Type of Carrier Code | Rép. | Optionnel | A coded value specifying the type of carrier applying to the resource, according to the provisions of the system specified in subfield $2. Repeatable when several types of carrier, associated with ... |
| `$2` | Source | Non rép. | Optionnel | Source of the code used in $a. |
| `$6` | Interfield Linking Data | Rép. | Optionnel | This subfield contains information allowing the field to be linked for processing purposes to other fields in the record. (See section 3.13 Interfield Linking Data, 3 Format structure). |
| `$8` | Materials Specified | Rép. | Optionnel | Part of the described materials to which the field applies. |

---

## BLOC 2 — INFORMATIONS DESCRIPTIVES

### Zone 200 — TITLE AND STATEMENT OF RESPONSIBILITY

**Répétabilité :** — &nbsp;|&nbsp; **Occurrence :** —

| Sous-zone | Nom | Rép. | Occurrence | Description |
|-----------|-----|------|------------|-------------|
| `$a` | Title Proper | Rép. | Oblig. |  |
| `$b` | General Material Designation | Rép. | Optionnel |  |
| `$c` | Title Proper by Another Author | Rép. | Optionnel |  |
| `$d` | Parallel Title Proper | Rép. | Optionnel |  |
| `$e` | Other Title Information | Rép. | Optionnel |  |
| `$f` | First Statement of Responsibility | Rép. | Optionnel |  |
| `$g` | Subsequent Statement of Responsibility | Rép. | Optionnel |  |
| `$h` | Number of a Part | Rép. | Optionnel |  |
| `$i` | Name of a Part | Rép. | Optionnel |  |
| `$j` | Inclusive Dates | Non rép. | Optionnel |  |
| `$k` | Bulk Dates | Non rép. | Optionnel |  |
| `$v` | Volume Designation | Non rép. | Optionnel |  |
| `$z` | Language of Parallel Title Proper | Rép. | Optionnel |  |
| `$2` | Source | Non rép. | Optionnel |  |
| `$5` | Institution to which the Field Applies | Non rép. | Optionnel |  |

### Zone 203 — CONTENT FORM AND MEDIA TYPE

**Répétabilité :** Rép. &nbsp;|&nbsp; **Occurrence :** Optionnel

| Sous-zone | Nom | Rép. | Occurrence | Description |
|-----------|-----|------|------------|-------------|
| `$a` | Content Form | Rép. | Oblig. | $b Content qualification 0.1.1 ( ) |
| `$b` | Content Qualification | Rép. | Oblig. si appl. | $a Subsequent content form (contained in same Media type) 0.1 . |
| `$c` | Media Type | Non rép. | Oblig. | Field repeated Subsequent content form: media type statement + ===PAGE 285=== |

### Zone 204 — GENERAL MATERIAL DESIGNATION (GMD) [OBSOLETE]

**Répétabilité :** — &nbsp;|&nbsp; **Occurrence :** —

*Champ de contrôle — pas de sous-zones ni d'indicateurs.*

### Zone 205 — EDITION STATEMENT

**Répétabilité :** Rép. &nbsp;|&nbsp; **Occurrence :** Optionnel

| Sous-zone | Nom | Rép. | Occurrence | Description |
|-----------|-----|------|------------|-------------|
| `$a` | Edition Statement | Non rép. | Optionnel | A word, phrase or group of characters in a formal statement, identifying the resource as a member of an edition. Not repeatable. Additional edition statements are entered in $b. |
| `$b` | Issue Statement | Rép. | Optionnel | This subfield contains an edition statement relating to an issue or an additional edition statement which: − is formally identified as constituting an edition within an edition, or − has significan... |
| `$d` | Parallel Edition Statement | Rép. | Optionnel | The statement of an edition in a language and/or script other than the edition statement in subfield $a (EX 9). Repeatable for each edition statement in other languages. |
| `$f` | Statement of Responsibility Relating to Edition | Rép. | Optionnel | The first of any statements of responsibility relating to the edition. May appear following subfields $a (EX 8, 9), $b (EX 7), or $d (EX 9). Repeatable for parallel first statements and first state... |
| `$g` | Subsequent Statement of Responsibility | Rép. | Optionnel | Any subsequent statement of responsibility relating to the edition (EX 7). Follows subfield $f. Repeatable for additional subsequent statements and parallel subsequent statements. |

### Zone 206 — MATERIAL SPECIFIC AREA: CARTOGRAPHIC RESOURCES – MATHEMATICAL DATA

**Répétabilité :** — &nbsp;|&nbsp; **Occurrence :** —

| Sous-zone | Nom | Rép. | Occurrence | Description |
|-----------|-----|------|------------|-------------|
| `$a` | Mathematical Data Statement | Non rép. | Optionnel | The text of the statements of scale, projection, coordinates, equinoxes and epochs including ISBD punctuation. |
| `$b` | Statement of Scale | Rép. | Optionnel | Includes any equivalency statements, vertical scales or vertical exaggeration statements for relief models and other three-dimensional resources. |
| `$c` | Statement of Projection | Non rép. | Optionnel |  |
| `$d` | Statement of Coordinates | Non rép. | Optionnel |  |
| `$e` | Right Ascension and Declination | Non rép. | Optionnel | Used for celestial charts. |
| `$f` | Statement of Equinox | Non rép. | Optionnel |  |

### Zone 207 — MATERIAL SPECIFIC AREA: NUMBERING OF CONTINUING RESOURCES

**Répétabilité :** — &nbsp;|&nbsp; **Occurrence :** —

| Sous-zone | Nom | Rép. | Occurrence | Description |
|-----------|-----|------|------------|-------------|
| `$a` | Numbering: Dates and Volume Designations | Rép. | Optionnel | The numbering and/or dates of coverage as defined in the Field Definition above. Repeatable when a new sequence of numbering is started. |
| `$z` | Source of Numbering Information | Rép. | Optionnel | A note on the source of numbering information when it is not taken from the resource itself (EX 6). |

### Zone 208 — MATERIAL SPECIFIC AREA: MUSIC FORMAT STATEMENT

**Répétabilité :** — &nbsp;|&nbsp; **Occurrence :** —

| Sous-zone | Nom | Rép. | Occurrence | Description |
|-----------|-----|------|------------|-------------|
| `$a` | Music Format Statement | Non rép. | Optionnel | The statement related to the particular format of the resource, taken from the title page (EX 1, 2). |
| `$d` | Parallel Music Format Statement | Rép. | Optionnel | Any additional statements in a different language and/or script, taken from the title page (EX 2). Repeatable for each additional statement. |

### Zone 210 — PUBLICATION, DISTRIBUTION, ETC.

**Répétabilité :** Rép. &nbsp;|&nbsp; **Occurrence :** Optionnel

| Sous-zone | Nom | Rép. | Occurrence | Description |
|-----------|-----|------|------------|-------------|
| `$a` | Place of Publication, Distribution, etc. | Rép. | Optionnel |  |
| `$b` | Address of Publisher, Distributor, etc | Rép. | Optionnel |  |
| `$c` | Name of Publisher, Distributor, etc. | Rép. | Optionnel |  |
| `$d` | Date of Publication, Distribution, etc. | Rép. | Optionnel |  |
| `$e` | Place of Manufacture | Rép. | Optionnel |  |
| `$f` | Address of Manufacturer | Rép. | Optionnel |  |
| `$g` | Name of Manufacturer | Rép. | Optionnel |  |
| `$h` | Date of Manufacture | Rép. | Optionnel |  |

### Zone 211 — PROJECTED PUBLICATION DATE

**Répétabilité :** Non rép. &nbsp;|&nbsp; **Occurrence :** Optionnel

| Sous-zone | Nom | Rép. | Occurrence | Description |
|-----------|-----|------|------------|-------------|
| `$a` | Date | Non rép. | Optionnel | The data should be recorded according to ISO 8601-1 in all-numeric form without separators between year, month, and day, i.e. YYYYMMDD. Where day or month and day are unknown, those positions conta... |

### Zone 215 — PHYSICAL DESCRIPTION

**Répétabilité :** Rép. &nbsp;|&nbsp; **Occurrence :** Optionnel

| Sous-zone | Nom | Rép. | Occurrence | Description |
|-----------|-----|------|------------|-------------|
| `$a` | Specific Material Designation and Extent | Rép. | Optionnel | The name of the specific type of material to which the resource belongs (EX 3, 6, 7-14) and/or an indication of the number of pieces or constituent parts (EX 1, 2, 5). Repeatable (EX 8). |
| `$b` | Materials and Technique Display | Non rép. | Optionnel | Free-text note about the substances or materials used in the creation of the resource, as well as description of the technique, media, and support used in the creation of the resource. It clarifies... |
| `$c` | Other Physical Details | Non rép. | Optionnel | Physical data about the resource other than that contained elsewhere in this field or in the notes fields, e.g. details of illustrative matter, whether the resource has colour or sound, etc. (EX 9-... |
| `$d` | Dimensions | Rép. | Optionnel | The measurements relevant of the resource being described. Repeatable (EX 8). |
| `$e` | Accompanying Material | Rép. | Optionnel | A brief description of any material accompanying the resource being described, which is intended to be used with the resource (EX 4, 8). |
| `$f` | Weight | Non rép. | Optionnel | The weight of the resource. It is usually expressed in grams. If an object is fragmented or consists of several separate parts, the weight of each part is entered in the same subfield $f, separated... |

### Zone 225 — SERIES

**Répétabilité :** Rép. &nbsp;|&nbsp; **Occurrence :** Optionnel

| Sous-zone | Nom | Rép. | Occurrence | Description |
|-----------|-----|------|------------|-------------|
| `$a` | Title | Non rép. | Optionnel |  |
| `$d` | Parallel Title | Rép. | Optionnel |  |
| `$e` | Other Title Information | Rép. | Optionnel |  |
| `$f` | Statement of Responsibility | Rép. | Optionnel |  |
| `$g` | Subsequent Statement of Responsibility | Rép. | Optionnel |  |
| `$h` | Number of a Part | Rép. | Optionnel |  |
| `$i` | Name of a Part | Rép. | Optionnel |  |
| `$v` | Volume Designation | Rép. | Optionnel |  |
| `$x` | ISSN of Series | Rép. | Optionnel |  |
| `$z` | Language of Parallel Title | Rép. | Optionnel |  |
| `$2` | Source | Non rép. | Optionnel |  |

### Zone 230 — MATERIAL SPECIFIC AREA: ELECTRONIC RESOURCE CHARACTERISTICS

**Répétabilité :** — &nbsp;|&nbsp; **Occurrence :** —

| Sous-zone | Nom | Rép. | Occurrence | Description |
|-----------|-----|------|------------|-------------|
| `$a` | Designation and Extent of File | Non rép. | Optionnel | The designation of the file identifies the particular type of file(s) which constitutes the resource and is given in the language of the bibliographic agency. Designation of type of file is mandato... |

### Zone 231 — DIGITAL FILE CHARACTERISTICS [PROVISIONAL]

**Répétabilité :** — &nbsp;|&nbsp; **Occurrence :** —

| Sous-zone | Nom | Rép. | Occurrence | Description |
|-----------|-----|------|------------|-------------|
| `$a` | File Type | Rép. | Optionnel | A general type of data content encoded in a computer file (EX 1, 2, 3). |
| `$b` | Encoding Standard | Rép. | Optionnel | A schema, standard, etc., used to encode the digital content of a resource (EX 1, 2, 3). |
| `$c` | Version of Encoding Standard | Rép. | Optionnel | The version of the standard recorded in $b (EX 2). |
| `$d` | Details of Encoding Standard | Rép. | Optionnel | Details of the standard recorded in $b (EX 2). |
| `$e` | File Size | Rép. | Optionnel | Number of bytes in a digital file (EX 2). |
| `$f` | Resolution | Rép. | Optionnel | The smallest measuring unit used to register data for a computer image, expressed as dots per inch, pixels per line, or lines per millimetre. It indicates the amount of detail found in one pixel of... |
| `$g` | Regional Encoding | Rép. | Optionnel |  |
| `$h` | Encoded Bitrate | Rép. | Optionnel |  |
| `$i` | Accessibility Information | Rép. | Optionnel |  |
| `$2` | Source | Non rép. | Optionnel |  |
| `$6` | Interfield Linking Data | Rép. | Optionnel |  |

### Zone 251 — ORGANIZATION AND ARRANGEMENT OF MATERIALS

**Répétabilité :** — &nbsp;|&nbsp; **Occurrence :** —

| Sous-zone | Nom | Rép. | Occurrence | Description |
|-----------|-----|------|------------|-------------|
| `$a` | Organization | Rép. | Optionnel | Manner in which the materials being described have been subdivided into smaller units, such as how the record groups are divided into series, and series into subseries. |
| `$b` | Arrangement | Rép. | Optionnel | Terms used to describe the pattern of arrangement of materials within a unit, such as alphabetical, chronological, etc. |
| `$c` | Level | Non rép. | Optionnel | Term used to identify hierarchical position of the described materials in the whole hierarchy of description. Normally, standard archival units are used – fonds, series, file, resource. |

### Zone 260 — MATERIAL SPECIFIC AREA: NUMISMATIC RESOURCES

**Répétabilité :** — &nbsp;|&nbsp; **Occurrence :** —

| Sous-zone | Nom | Rép. | Occurrence | Description |
|-----------|-----|------|------------|-------------|
| `$a` | Denomination | Non rép. | Optionnel |  |
| `$b` | Weight Standard or Theoretical Weight | Non rép. | Optionnel |  |
| `$c` | Issue | Non rép. | Optionnel |  |
| `$d` | Other Appellation | Rép. | Optionnel |  |
| `$e` | Type Name or Reference | Rép. | Optionnel |  |
| `$f` | Legend | Non rép. | Optionnel |  |
| `$g` | Other Inscription | Rép. | Optionnel |  |
| `$h` | Type Description | Non rép. | Optionnel |  |
| `$i` | Shape | Non rép. | Optionnel |  |
| `$j` | Axis | Non rép. | Optionnel |  |
| `$u` | Uniform Resource Identifier (URI) | Non rép. | Optionnel |  |

### Zone 283 — CARRIER TYPE

**Répétabilité :** Rép. &nbsp;|&nbsp; **Occurrence :** Optionnel

| Sous-zone | Nom | Rép. | Occurrence | Description |
|-----------|-----|------|------------|-------------|
| `$a` | Carrier Type Designation | Rép. | Oblig. | The carrier type applying to the resource. Mandatory. Repeatable when several carrier types, associated with the same media type, are present in the resource. |
| `$2` | Source | Non rép. | Oblig. si appl. | Source of the carrier type designation used in $a. Mandatory. |
| `$6` | Interfield Linking Data | Rép. | Optionnel | This subfield contains information allowing the field to be linked for processing purposes to other fields in the record. (See section 3.13 Interfield Linking Data, 3 Format structure). |
| `$8` | Materials Specified | Rép. | Optionnel | Part of the described materials to which the field applies. |

---

## BLOC 3 — NOTES

### Zone 300 — GENERAL NOTES

**Répétabilité :** Rép. &nbsp;|&nbsp; **Occurrence :** Optionnel

| Sous-zone | Nom | Rép. | Occurrence | Description |
|-----------|-----|------|------------|-------------|
| `$a` | Text of Note | Non rép. | Optionnel |  |

### Zone 301 — NOTES PERTAINING TO IDENTIFICATION NUMBERS

**Répétabilité :** — &nbsp;|&nbsp; **Occurrence :** —

| Sous-zone | Nom | Rép. | Occurrence | Description |
|-----------|-----|------|------------|-------------|
| `$a` | Text of Note | Non rép. | Optionnel |  |

### Zone 302 — NOTES PERTAINING TO CODED INFORMATION

**Répétabilité :** — &nbsp;|&nbsp; **Occurrence :** —

| Sous-zone | Nom | Rép. | Occurrence | Description |
|-----------|-----|------|------------|-------------|
| `$a` | Text of Note | Non rép. | Optionnel |  |

### Zone 303 — GENERAL NOTES PERTAINING TO DESCRIPTIVE INFORMATION

**Répétabilité :** — &nbsp;|&nbsp; **Occurrence :** —

| Sous-zone | Nom | Rép. | Occurrence | Description |
|-----------|-----|------|------------|-------------|
| `$a` | Text of Note | Non rép. | Optionnel |  |

### Zone 304 — NOTES PERTAINING TO TITLE AND STATEMENT OF RESPONSIBILITY

**Répétabilité :** — &nbsp;|&nbsp; **Occurrence :** —

| Sous-zone | Nom | Rép. | Occurrence | Description |
|-----------|-----|------|------------|-------------|
| `$a` | Text of Note | Non rép. | Optionnel |  |

### Zone 305 — NOTES PERTAINING TO EDITION AND BIBLIOGRAPHIC HISTORY

**Répétabilité :** — &nbsp;|&nbsp; **Occurrence :** —

| Sous-zone | Nom | Rép. | Occurrence | Description |
|-----------|-----|------|------------|-------------|
| `$a` | Text of Note | Non rép. | Optionnel |  |

### Zone 306 — NOTES PERTAINING TO PUBLICATION, DISTRIBUTION, ETC.

**Répétabilité :** — &nbsp;|&nbsp; **Occurrence :** —

| Sous-zone | Nom | Rép. | Occurrence | Description |
|-----------|-----|------|------------|-------------|
| `$a` | Text of Note | Non rép. | Optionnel |  |

### Zone 307 — NOTES PERTAINING TO PHYSICAL DESCRIPTION

**Répétabilité :** — &nbsp;|&nbsp; **Occurrence :** —

| Sous-zone | Nom | Rép. | Occurrence | Description |
|-----------|-----|------|------------|-------------|
| `$a` | Text of Note | Non rép. | Optionnel |  |

### Zone 308 — NOTES PERTAINING TO SERIES

**Répétabilité :** Rép. &nbsp;|&nbsp; **Occurrence :** Optionnel

| Sous-zone | Nom | Rép. | Occurrence | Description |
|-----------|-----|------|------------|-------------|
| `$a` | Text of Note | Non rép. | Optionnel |  |

### Zone 310 — NOTES PERTAINING TO BINDING AND AVAILABILITY

**Répétabilité :** — &nbsp;|&nbsp; **Occurrence :** —

| Sous-zone | Nom | Rép. | Occurrence | Description |
|-----------|-----|------|------------|-------------|
| `$a` | Text of Note | Non rép. | Optionnel |  |

### Zone 311 — NOTES PERTAINING TO LINKING FIELDS

**Répétabilité :** Rép. &nbsp;|&nbsp; **Occurrence :** Optionnel

| Sous-zone | Nom | Rép. | Occurrence | Description |
|-----------|-----|------|------------|-------------|
| `$a` | Text of Note | Non rép. | Optionnel |  |

### Zone 312 — NOTES PERTAINING TO RELATED TITLES

**Répétabilité :** Rép. &nbsp;|&nbsp; **Occurrence :** Optionnel

| Sous-zone | Nom | Rép. | Occurrence | Description |
|-----------|-----|------|------------|-------------|
| `$a` | Text of Note | Non rép. | Optionnel |  |

### Zone 313 — NOTES PERTAINING TO SUBJECT ACCESS

**Répétabilité :** Rép. &nbsp;|&nbsp; **Occurrence :** Optionnel

| Sous-zone | Nom | Rép. | Occurrence | Description |
|-----------|-----|------|------------|-------------|
| `$a` | Text of Note | Non rép. | Optionnel |  |

### Zone 314 — NOTES PERTAINING TO RESPONSIBILITY

**Répétabilité :** Rép. &nbsp;|&nbsp; **Occurrence :** Optionnel

| Sous-zone | Nom | Rép. | Occurrence | Description |
|-----------|-----|------|------------|-------------|
| `$a` | Text of Note | Non rép. | Optionnel |  |

### Zone 315 — NOTES PERTAINING TO MATERIAL (OR TYPE OF PUBLICATION) SPECIFIC INFORMATION

**Répétabilité :** — &nbsp;|&nbsp; **Occurrence :** —

| Sous-zone | Nom | Rép. | Occurrence | Description |
|-----------|-----|------|------------|-------------|
| `$a` | Text of Note | Non rép. | Optionnel |  |

### Zone 316 — NOTE RELATING TO THE ITEM

**Répétabilité :** Rép. &nbsp;|&nbsp; **Occurrence :** Optionnel

| Sous-zone | Nom | Rép. | Occurrence | Description |
|-----------|-----|------|------------|-------------|
| `$a` | Text of Note | Rép. | Optionnel |  |
| `$u` | Uniform Resource Identifier (URI) | Rép. | Optionnel | A Uniform Resource Identifier (URI), such as a URL (Uniform Resource Locator) or URN (Uniform Resource Name), serves as a standardized string that identifies a resource and provides electronic acce... |
| `$5` | Institution to which the Field Applies | Non rép. | Optionnel | Name of the institution to which field applies in the coded form if it is considered necessary by the agency which creates the record to identify specific attributes of an item. If the subfield is ... |
| `$6` | Interfield Linking Data | Rép. | Optionnel | This subfield contains information allowing the field to be linked for processing purposes to other fields in the record. (See section 3.13 Interfield Linking Data, 3 Format structure). The instruc... |

### Zone 317 — PROVENANCE NOTE

**Répétabilité :** Rép. &nbsp;|&nbsp; **Occurrence :** Optionnel

| Sous-zone | Nom | Rép. | Occurrence | Description |
|-----------|-----|------|------------|-------------|
| `$a` | Text of Note | Non rép. | Optionnel |  |
| `$u` | Uniform Resource Identifier (URI) | Rép. | Optionnel | A Uniform Resource Identifier (URI), such as a URL (Uniform Resource Locator) or URN (Uniform Resource Name), serves as a standardized string that identifies a resource and provides electronic acce... |
| `$5` | Institution to which the Field Applies | Non rép. | Optionnel | Name of the institution to which field applies in the coded form if it is considered necessary by the agency which creates the record to identify specific attributes of an item. If the subfield is ... |
| `$6` | Interfield Linking Data | Rép. | Optionnel |  |
| `$8` | Materials Specified | Non rép. | Optionnel |  |

### Zone 318 — ACTION NOTE

**Répétabilité :** Rép. &nbsp;|&nbsp; **Occurrence :** Optionnel

| Sous-zone | Nom | Rép. | Occurrence | Description |
|-----------|-----|------|------------|-------------|
| `$a` | Action | Non rép. | Optionnel |  |
| `$b` | Action Identification | Rép. | Optionnel | A code or designation designed to identify a specific action or identify it in conjunction with time of action, e.g. a project code. |
| `$c` | Time of Action | Rép. | Optionnel | A code for time of action in ISO format (year, month [if applicable], day [if applicable]). |
| `$d` | Action Interval | Rép. | Optionnel | Subfield is used when the time cannot be expressed as a specific date, e.g. at the end of academic term or every five years (EX 7). |
| `$e` | Contingency for Action | Rép. | Optionnel | Subfield is used when the time is linked to an unpredictable event, e.g. upon receipt (EX 3,6). |
| `$f` | Authorisation | Rép. | Optionnel | Subfield contains the text of or a citation to a rule etc. governing the action (EX 4). Repeatable. ===PAGE 364=== |
| `$h` | Jurisdiction | Rép. | Optionnel |  |
| `$i` | Method of Action | Rép. | Optionnel |  |
| `$j` | Site of Action | Rép. | Optionnel |  |
| `$k` | Action Agent | Rép. | Optionnel |  |
| `$l` | Status | Rép. | Optionnel |  |
| `$n` | Extent | Rép. | Optionnel |  |
| `$o` | Type of Unit | Rép. | Optionnel |  |
| `$p` | Non-public Note | Rép. | Optionnel |  |
| `$r` | Public Note | Rép. | Optionnel |  |
| `$u` | Uniform Resource Identifier (URI) | Rép. | Optionnel |  |
| `$5` | Institution to which the Field Applies | Non rép. | Optionnel |  |

### Zone 320 — INTERNAL BIBLIOGRAPHIES/INDEXES NOTE

**Répétabilité :** — &nbsp;|&nbsp; **Occurrence :** —

| Sous-zone | Nom | Rép. | Occurrence | Description |
|-----------|-----|------|------------|-------------|
| `$a` | Text of Note | Non rép. | Optionnel |  |
| `$u` | Uniform Resource Identifier (URI) | Rép. | Optionnel | A Uniform Resource Identifier (URI), such as a URL (Uniform Resource Locator) or URN (Uniform Resource Name), serves as a standardized string that identifies a resource and provides electronic acce... |

### Zone 321 — EXTERNAL INDEXES/ABSTRACTS/REFERENCES NOTE

**Répétabilité :** — &nbsp;|&nbsp; **Occurrence :** —

| Sous-zone | Nom | Rép. | Occurrence | Description |
|-----------|-----|------|------------|-------------|
| `$a` | Name of Source | Non rép. | Optionnel | Contains the name of the external source where the resource is indexed or cited. This may be the title of the source or the name of an organization or company. The Key Title is used when available.... |
| `$b` | Dates of Coverage | Non rép. | Optionnel |  |
| `$c` | Location within Source | Non rép. | Optionnel |  |
| `$u` | Uniform Resource Identifier (URI) | Non rép. | Optionnel |  |
| `$x` | International Standard Number | Non rép. | Optionnel |  |
| `$5` | Institution to which Field Applies | Non rép. | Optionnel |  |
| `$6` | Interfield Linking Data | Rép. | Optionnel |  |

### Zone 322 — CREDITS NOTE (PROJECTED AND VIDEO MATERIAL AND SOUND RECORDINGS)

**Répétabilité :** — &nbsp;|&nbsp; **Occurrence :** —

| Sous-zone | Nom | Rép. | Occurrence | Description |
|-----------|-----|------|------------|-------------|
| `$a` | Text of Note | Non rép. | Optionnel |  |

### Zone 323 — CAST NOTE (PROJECTED AND VIDEO MATERIAL AND SOUND RECORDINGS)

**Répétabilité :** — &nbsp;|&nbsp; **Occurrence :** —

| Sous-zone | Nom | Rép. | Occurrence | Description |
|-----------|-----|------|------------|-------------|
| `$a` | Text of Note | Non rép. | Optionnel |  |

### Zone 324 — ORIGINAL VERSION NOTE

**Répétabilité :** Non rép. &nbsp;|&nbsp; **Occurrence :** Optionnel

| Sous-zone | Nom | Rép. | Occurrence | Description |
|-----------|-----|------|------------|-------------|
| `$a` | Text of Note | Non rép. | Optionnel |  |

### Zone 325 — REPRODUCTION NOTE

**Répétabilité :** Rép. &nbsp;|&nbsp; **Occurrence :** Optionnel

| Sous-zone | Nom | Rép. | Occurrence | Description |
|-----------|-----|------|------------|-------------|
| `$a` | Text of Unstructured Note | Non rép. | Optionnel | ===PAGE 377=== |
| `$b` | Type of Reproduction | Non rép. | Optionnel |  |
| `$c` | Place of Reproduction | Rép. | Optionnel |  |
| `$d` | Agency Responsible for the Reproduction | Rép. | Optionnel |  |
| `$e` | Date of Publication for the Reproduction | Non rép. | Optionnel |  |
| `$f` | Physical Description of the Reproduction | Non rép. | Optionnel |  |
| `$g` | Series Statement for the Reproduction | Non rép. | Optionnel |  |
| `$h` | Completeness of the Reproduction Code | Non rép. | Optionnel |  |
| `$i` | Coverage of the Reproduction | Non rép. | Optionnel |  |
| `$j` | Terms of Access to the Reproduction | Rép. | Optionnel |  |
| `$n` | Note About Reproduction | Rép. | Optionnel |  |
| `$u` | Uniform Resource Identifier (URI) | Non rép. | Optionnel |  |
| `$v` | Date of Consultation | Non rép. | Optionnel |  |
| `$x` | ISSN of the Reproduction | Non rép. | Optionnel |  |
| `$y` | ISBN of the Reproduction | Rép. | Optionnel |  |
| `$z` | Date when the URL in $u was Found to be Invalid | Non rép. | Optionnel |  |
| `$5` | Institution to which the Field Applies | Non rép. | Optionnel |  |

### Zone 326 — FREQUENCY STATEMENT NOTE (CONTINUING RESOURCES)

**Répétabilité :** — &nbsp;|&nbsp; **Occurrence :** —

| Sous-zone | Nom | Rép. | Occurrence | Description |
|-----------|-----|------|------------|-------------|
| `$a` | Frequency | Non rép. | Optionnel | A statement indicating the frequency with which a continuing resource is issued. |
| `$b` | Dates of Frequency | Non rép. | Optionnel | The date span over which the frequency mentioned in subfield $a applies. |

### Zone 327 — CONTENTS NOTE

**Répétabilité :** Rép. &nbsp;|&nbsp; **Occurrence :** Optionnel

| Sous-zone | Nom | Rép. | Occurrence | Description |
|-----------|-----|------|------------|-------------|
| `$a` | Text of Note | Rép. | Optionnel |  |
| `$b` | Title of Level 1 Subdivision | Rép. | Optionnel |  |
| `$c` | Title of Level 2 Subdivision | Rép. | Optionnel |  |
| `$d` | Title of Level 3 Subdivision | Rép. | Optionnel |  |
| `$e` | Title of Level 4 Subdivision | Rép. | Optionnel |  |
| `$f` | Title of Level 5 Subdivision | Rép. | Optionnel |  |
| `$g` | Title of Level 6 Subdivision | Rép. | Optionnel |  |
| `$h` | Title of Level 7 Subdivision | Rép. | Optionnel |  |
| `$i` | Title of Level 8 Subdivision | Rép. | Optionnel |  |
| `$p` | Sequence of Pages or First Pages of a Subdivision | Rép. | Optionnel |  |
| `$u` | Uniform Resource Identifier (URI) | Rép. | Optionnel |  |
| `$z` | Other Information Concerning a Subdivision | Rép. | Optionnel |  |

### Zone 328 — DISSERTATION (THESIS) NOTE

**Répétabilité :** Rép. &nbsp;|&nbsp; **Occurrence :** Optionnel

| Sous-zone | Nom | Rép. | Occurrence | Description |
|-----------|-----|------|------------|-------------|
| `$a` | Text of Note | Non rép. | Optionnel |  |
| `$b` | Dissertation or Thesis Details and Type of Degree | Non rép. | Optionnel |  |
| `$c` | Discipline of Degree | Non rép. | Optionnel |  |
| `$d` | Date of Degree | Non rép. | Optionnel |  |
| `$e` | Body Granting the Degree | Non rép. | Optionnel |  |
| `$t` | Title of Other Edition of Dissertation or Thesis | Non rép. | Optionnel |  |
| `$z` | Text Preceding or Following the Note | Rép. | Optionnel |  |

### Zone 330 — SUMMARY OR ABSTRACT

**Répétabilité :** Rép. &nbsp;|&nbsp; **Occurrence :** Optionnel

| Sous-zone | Nom | Rép. | Occurrence | Description |
|-----------|-----|------|------------|-------------|
| `$a` | Text of Note | Non rép. | Optionnel |  |
| `$u` | Uniform Resource Identifier (URI) | Rép. | Optionnel | A Uniform Resource Identifier (URI), such as a URL (Uniform Resource Locator) or URN (Uniform Resource Name), serves as a standardized string that identifies a resource and provides electronic acce... |

### Zone 332 — PREFERRED CITATION OF DESCRIBED MATERIALS

**Répétabilité :** — &nbsp;|&nbsp; **Occurrence :** —

| Sous-zone | Nom | Rép. | Occurrence | Description |
|-----------|-----|------|------------|-------------|
| `$a` | Preferred Citation | Non rép. | Optionnel |  |

### Zone 333 — USERS/INTENDED AUDIENCE NOTE

**Répétabilité :** Rép. &nbsp;|&nbsp; **Occurrence :** Optionnel

| Sous-zone | Nom | Rép. | Occurrence | Description |
|-----------|-----|------|------------|-------------|
| `$a` | Text of Note | Non rép. | Optionnel |  |
| `$5` | Institution to which the Field Applies | Non rép. | Optionnel | Name of the institution to which field applies in the coded form if it is considered necessary by the agency which creates the record to identify specific attributes of an item. If the subfield is ... |

### Zone 334 — AWARDS NOTE

**Répétabilité :** Rép. &nbsp;|&nbsp; **Occurrence :** Optionnel

| Sous-zone | Nom | Rép. | Occurrence | Description |
|-----------|-----|------|------------|-------------|
| `$a` | Text of Awards Note | Non rép. | Optionnel | This subfield contains a free text note, describing the prize or award. This subfield field may be used instead of subfields $b-$d when the information cannot be separated into specific subfields. |
| `$b` | Name of Award | Non rép. | Optionnel | This subfield contains the name of the award. |
| `$c` | Year of Award | Non rép. | Optionnel | This subfield contains the year in which the award was made. The year is entered in the form YYYY. |
| `$d` | Country of Award | Non rép. | Optionnel | This subfield contain the code for the country which makes the award. For codes use ISO 3166-1 (see Appendix A). |
| `$u` | Uniform Resource Identifier (URI) | Rép. | Optionnel | A Uniform Resource Identifier (URI), such as a URL (Uniform Resource Locator) or URN (Uniform Resource Name), serves as a standardized string that identifies a resource and provides electronic acce... |
| `$5` | Institution to which the Field Applies | Non rép. | Optionnel | Name of the institution to which field applies in the coded form if it is considered necessary by the agency which creates the record to identify specific attributes of an item. If the subfield is ... |

### Zone 335 — LOCATION OF ORIGINALS/REPRODUCTIONS

**Répétabilité :** — &nbsp;|&nbsp; **Occurrence :** —

| Sous-zone | Nom | Rép. | Occurrence | Description |
|-----------|-----|------|------------|-------------|
| `$a` | Identification of the Holder | Non rép. | Optionnel | Name of the repository or individual holding the item. |
| `$b` | Address | Rép. | Optionnel | Postal address of the holder. |
| `$c` | Country | Rép. | Optionnel | Country code for the location indentified in subfield $a. The codes are to be taken from the two-character codes of ISO 3166-1. |
| `$g` | Control Number | Rép. | Optionnel | Any significant control number for the materials described; e.g. call number, inventory number, accession numbers, etc. |
| `$u` | Uniform Resource Identifier (URI) | Rép. | Optionnel | A Uniform Resource Identifier (URI), such as a URL (Uniform Resource Locator) or URN (Uniform Resource Name), serves as a standardized string that identifies a resource and provides electronic acce... |
| `$8` | Materials Specified | Rép. | Optionnel | Part of the described materials to which the field applies. |

### Zone 336 — TYPE OF ELECTRONIC RESOURCE NOTE

**Répétabilité :** Rép. &nbsp;|&nbsp; **Occurrence :** Optionnel

| Sous-zone | Nom | Rép. | Occurrence | Description |
|-----------|-----|------|------------|-------------|
| `$a` | Text of Note | Non rép. | Optionnel |  |

### Zone 337 — SYSTEM REQUIREMENTS NOTE (ELECTRONIC RESOURCES)

**Répétabilité :** — &nbsp;|&nbsp; **Occurrence :** —

| Sous-zone | Nom | Rép. | Occurrence | Description |
|-----------|-----|------|------------|-------------|
| `$a` | Text of Note | Non rép. | Optionnel |  |
| `$u` | Uniform Resource Identifier (URI) | Rép. | Optionnel | A Uniform Resource Identifier (URI), such as a URL (Uniform Resource Locator) or URN (Uniform Resource Name), serves as a standardized string that identifies a resource and provides electronic acce... |

### Zone 338 — FUNDING INFORMATION NOTE

**Répétabilité :** Rép. &nbsp;|&nbsp; **Occurrence :** Optionnel

| Sous-zone | Nom | Rép. | Occurrence | Description |
|-----------|-----|------|------------|-------------|
| `$a` | Text of Unstructured Note | Non rép. | Optionnel | Use only for the complete text of an unstructured note. Subfield $a should be present if Indicator 2 is blank (#). |
| `$b` | Funding Organization | Rép. | Optionnel | Information concerning the sponsors or funding agencies. |
| `$c` | Program | Rép. | Optionnel | The name refers to a specific program. |
| `$d` | Project Number | Non rép. | Optionnel | A unique identifier in the scope of funding organization (e.g., a grant agreement number). |
| `$e` | Jurisdiction | Rép. | Optionnel | ===PAGE 407=== |
| `$f` | Project Name | Non rép. | Optionnel |  |
| `$g` | Project Acronym | Non rép. | Optionnel |  |

### Zone 345 — ACQUISITION INFORMATION NOTE

**Répétabilité :** Non rép. &nbsp;|&nbsp; **Occurrence :** Optionnel

| Sous-zone | Nom | Rép. | Occurrence | Description |
|-----------|-----|------|------------|-------------|
| `$a` | Source for Acquisition/Subscription Address | Rép. | Optionnel | The name and/or address of a source for acquisition, including subscription address, of the resource. |
| `$b` | Stock Number | Rép. | Optionnel | The number associated with the item by the supplier to facilitate distribution of the item. If the stock number is the ISBN or ISSN, it need not be repeated in this field. |
| `$c` | Medium | Rép. | Optionnel | The medium(s) in which the item is available. Repeatable (EX 3). |
| `$d` | Terms of Availability | Rép. | Optionnel | The price of the item in monetary or other units. |
| `$u` | Uniform Resource Identifier (URI) | Rép. | Optionnel | A Uniform Resource Identifier (URI), such as a URL (Uniform Resource Locator) or URN (Uniform Resource Name), serves as a standardized string that identifies a resource and provides electronic acce... |
| `$5` | Institution to which the Field Applies | Non rép. | Optionnel | Name of the institution to which field applies in the coded form if it is considered necessary by the agency which creates the record to identify specific attributes of an item. If the subfield is ... |

### Zone 346 — NOTE ON ACCRUALS AND FREQUENCY OF USE

**Répétabilité :** — &nbsp;|&nbsp; **Occurrence :** —

| Sous-zone | Nom | Rép. | Occurrence | Description |
|-----------|-----|------|------------|-------------|
| `$a` | Accruals | Rép. | Optionnel | A rate at which the described materials are accumulating is expressed as a ratio of volume to a time period. It may indicate the quantity and frequency of the accruals. Optional. Repeatable. (EX 1,... |
| `$b` | Frequency of Use | Rép. | Optionnel | A measure of reference activity usually expressed as a ratio of a number of retrievals to time period, or by general terms such as active or inactive this subfiield indicates the measure of referen... |
| `$5` | Institution to which the Field Applies | Non rép. | Optionnel | Name of the institution to which field applies in the coded form if it is considered necessary by the agency which creates the record to identify specific attributes of an item. If the subfield is ... |
| `$8` | Materials Specified | Non rép. | Optionnel | Part of the described materials to which the field applies. Not repeatable. (EX 3) |

### Zone 360 — NOTE ON SPECIFIC ELEMENTS OF NUMISMATIC DESCRIPTION

**Répétabilité :** — &nbsp;|&nbsp; **Occurrence :** —

| Sous-zone | Nom | Rép. | Occurrence | Description |
|-----------|-----|------|------------|-------------|
| `$a` | Text of Note | Non rép. | Oblig. si appl. |  |
| `$b` | Citation | Rép. | Optionnel |  |
| `$u` | Uniform Resource Identifier (URI) | Rép. | Optionnel |  |

### Zone 371 — NOTES ON INFORMATION SERVICE POLICY

**Répétabilité :** Rép. &nbsp;|&nbsp; **Occurrence :** Optionnel

| Sous-zone | Nom | Rép. | Occurrence | Description |
|-----------|-----|------|------------|-------------|
| `$a` | Terms Governing Access, Use and Reproduction | Non rép. | Oblig. si appl. | The text of a legal or official statement of restrictions. Mandatory. |
| `$b` | Jurisdiction | Non rép. | Optionnel | The name of a person, an organization, or a position or function within the organization, by whom or which the terms governing access, use and reproduction are imposed and enforced and to whom the ... |
| `$c` | Authorization | Non rép. | Optionnel | A citation to the specific source that is the authority for the restriction. |
| `$d` | Authorized Users | Non rép. | Optionnel | The class of users or specific individuals to whom the restrictions in subfield $a do not apply. |
| `$5` | Institution to which the Field Applies | Non rép. | Optionnel | Name of the institution to which field applies in the coded form if it is considered necessary by the agency which creates the record to identify specific attributes of an item. If the subfield is ... |
| `$8` | Materials Specified | Non rép. | Optionnel | Part of the described materials to which the field applies. Not repeatable. ===PAGE 416=== |

---

## BLOC 4 — LIENS (LINKING ENTRY)

### Zone 410 — SERIES

**Répétabilité :** — &nbsp;|&nbsp; **Occurrence :** —

*Champ de contrôle — pas de sous-zones ni d'indicateurs.*

### Zone 411 — SUBSERIES

**Répétabilité :** — &nbsp;|&nbsp; **Occurrence :** —

*Champ de contrôle — pas de sous-zones ni d'indicateurs.*

### Zone 412 — SOURCE OF EXCERPT OR OFFPRINT

**Répétabilité :** — &nbsp;|&nbsp; **Occurrence :** —

*Champ de contrôle — pas de sous-zones ni d'indicateurs.*

### Zone 413 — EXCERPT OR OFFPRINT

**Répétabilité :** — &nbsp;|&nbsp; **Occurrence :** —

*Champ de contrôle — pas de sous-zones ni d'indicateurs.*

### Zone 421 — SUPPLEMENT

**Répétabilité :** — &nbsp;|&nbsp; **Occurrence :** —

*Champ de contrôle — pas de sous-zones ni d'indicateurs.*

### Zone 422 — PARENT OF SUPPLEMENT

**Répétabilité :** — &nbsp;|&nbsp; **Occurrence :** —

*Champ de contrôle — pas de sous-zones ni d'indicateurs.*

### Zone 423 — ISSUED WITH

**Répétabilité :** — &nbsp;|&nbsp; **Occurrence :** —

*Champ de contrôle — pas de sous-zones ni d'indicateurs.*

### Zone 424 — IS UPDATED BY

**Répétabilité :** — &nbsp;|&nbsp; **Occurrence :** —

*Champ de contrôle — pas de sous-zones ni d'indicateurs.*

### Zone 425 — UPDATES

**Répétabilité :** — &nbsp;|&nbsp; **Occurrence :** —

*Champ de contrôle — pas de sous-zones ni d'indicateurs.*

### Zone 430 — CONTINUES

**Répétabilité :** — &nbsp;|&nbsp; **Occurrence :** —

*Champ de contrôle — pas de sous-zones ni d'indicateurs.*

### Zone 431 — CONTINUES IN PART

**Répétabilité :** — &nbsp;|&nbsp; **Occurrence :** —

*Champ de contrôle — pas de sous-zones ni d'indicateurs.*

### Zone 432 — SUPERSEDES

**Répétabilité :** — &nbsp;|&nbsp; **Occurrence :** —

*Champ de contrôle — pas de sous-zones ni d'indicateurs.*

### Zone 433 — SUPERSEDES IN PART

**Répétabilité :** — &nbsp;|&nbsp; **Occurrence :** —

*Champ de contrôle — pas de sous-zones ni d'indicateurs.*

### Zone 434 — ABSORBED

**Répétabilité :** — &nbsp;|&nbsp; **Occurrence :** —

*Champ de contrôle — pas de sous-zones ni d'indicateurs.*

### Zone 435 — ABSORBED IN PART

**Répétabilité :** — &nbsp;|&nbsp; **Occurrence :** —

*Champ de contrôle — pas de sous-zones ni d'indicateurs.*

### Zone 436 — FORMED BY MERGER OF ..., ..., AND ...

**Répétabilité :** — &nbsp;|&nbsp; **Occurrence :** —

*Champ de contrôle — pas de sous-zones ni d'indicateurs.*

### Zone 437 — SEPARATED FROM

**Répétabilité :** — &nbsp;|&nbsp; **Occurrence :** —

*Champ de contrôle — pas de sous-zones ni d'indicateurs.*

### Zone 440 — CONTINUED BY

**Répétabilité :** — &nbsp;|&nbsp; **Occurrence :** —

*Champ de contrôle — pas de sous-zones ni d'indicateurs.*

### Zone 441 — CONTINUED IN PART BY

**Répétabilité :** — &nbsp;|&nbsp; **Occurrence :** —

*Champ de contrôle — pas de sous-zones ni d'indicateurs.*

### Zone 442 — SUPERSEDED BY

**Répétabilité :** — &nbsp;|&nbsp; **Occurrence :** —

*Champ de contrôle — pas de sous-zones ni d'indicateurs.*

### Zone 443 — SUPERSEDED IN PART BY

**Répétabilité :** — &nbsp;|&nbsp; **Occurrence :** —

*Champ de contrôle — pas de sous-zones ni d'indicateurs.*

### Zone 444 — ABSORBED BY

**Répétabilité :** — &nbsp;|&nbsp; **Occurrence :** —

*Champ de contrôle — pas de sous-zones ni d'indicateurs.*

### Zone 445 — ABSORBED IN PART BY

**Répétabilité :** — &nbsp;|&nbsp; **Occurrence :** —

*Champ de contrôle — pas de sous-zones ni d'indicateurs.*

### Zone 446 — SPLIT INTO .., ..., AND ...

**Répétabilité :** — &nbsp;|&nbsp; **Occurrence :** —

*Champ de contrôle — pas de sous-zones ni d'indicateurs.*

### Zone 447 — MERGED WITH ... AND ... TO FORM ...

**Répétabilité :** — &nbsp;|&nbsp; **Occurrence :** —

*Champ de contrôle — pas de sous-zones ni d'indicateurs.*

### Zone 448 — CHANGED BACK TO

**Répétabilité :** — &nbsp;|&nbsp; **Occurrence :** —

*Champ de contrôle — pas de sous-zones ni d'indicateurs.*

### Zone 451 — OTHER EDITION IN THE SAME MEDIUM

**Répétabilité :** — &nbsp;|&nbsp; **Occurrence :** —

*Champ de contrôle — pas de sous-zones ni d'indicateurs.*

### Zone 452 — OTHER EDITION IN ANOTHER MEDIUM

**Répétabilité :** — &nbsp;|&nbsp; **Occurrence :** —

*Champ de contrôle — pas de sous-zones ni d'indicateurs.*

### Zone 453 — TRANSLATED AS

**Répétabilité :** — &nbsp;|&nbsp; **Occurrence :** —

*Champ de contrôle — pas de sous-zones ni d'indicateurs.*

### Zone 454 — TRANSLATION OF

**Répétabilité :** — &nbsp;|&nbsp; **Occurrence :** —

*Champ de contrôle — pas de sous-zones ni d'indicateurs.*

### Zone 455 — REPRODUCTION OF

**Répétabilité :** — &nbsp;|&nbsp; **Occurrence :** —

*Champ de contrôle — pas de sous-zones ni d'indicateurs.*

### Zone 456 — REPRODUCED AS

**Répétabilité :** — &nbsp;|&nbsp; **Occurrence :** —

*Champ de contrôle — pas de sous-zones ni d'indicateurs.*

### Zone 461 — SET

**Répétabilité :** — &nbsp;|&nbsp; **Occurrence :** —

*Champ de contrôle — pas de sous-zones ni d'indicateurs.*

### Zone 462 — SUBSET

**Répétabilité :** — &nbsp;|&nbsp; **Occurrence :** —

*Champ de contrôle — pas de sous-zones ni d'indicateurs.*

### Zone 463 — PIECE

**Répétabilité :** — &nbsp;|&nbsp; **Occurrence :** —

*Champ de contrôle — pas de sous-zones ni d'indicateurs.*

### Zone 464 — PIECE-ANALYTIC

**Répétabilité :** — &nbsp;|&nbsp; **Occurrence :** —

*Champ de contrôle — pas de sous-zones ni d'indicateurs.*

### Zone 470 — RESOURCE REVIEWED

**Répétabilité :** — &nbsp;|&nbsp; **Occurrence :** —

*Champ de contrôle — pas de sous-zones ni d'indicateurs.*

### Zone 481 — ALSO BOUND IN THIS VOLUME

**Répétabilité :** — &nbsp;|&nbsp; **Occurrence :** —

*Champ de contrôle — pas de sous-zones ni d'indicateurs.*

### Zone 482 — BOUND WITH

**Répétabilité :** — &nbsp;|&nbsp; **Occurrence :** —

*Champ de contrôle — pas de sous-zones ni d'indicateurs.*

### Zone 488 — OTHER RELATED WORK

**Répétabilité :** — &nbsp;|&nbsp; **Occurrence :** —

*Champ de contrôle — pas de sous-zones ni d'indicateurs.*

---

## BLOC 5 — TITRES ASSOCIÉS

### Zone 500 — PREFERRED TITLE ACCESS POINT

**Répétabilité :** Rép. &nbsp;|&nbsp; **Occurrence :** Optionnel

| Sous-zone | Nom | Rép. | Occurrence | Description |
|-----------|-----|------|------------|-------------|
| `$a` | Preferred Title | Non rép. | Optionnel |  |
| `$b` | General Material Designation | Rép. | Optionnel |  |
| `$h` | Number of Section or Part | Rép. | Optionnel |  |
| `$i` | Name of Section or Part | Rép. | Optionnel |  |
| `$g` | Form Subdivision for Title | Rép. | Optionnel |  |
| `$k` | Date of Publication | Non rép. | Optionnel |  |
| `$l` | Form Subheading | Rép. | Optionnel |  |
| `$m` | Language (when Part of a Access Point) | Non rép. | Optionnel |  |
| `$n` | Miscellaneous Information | Rép. | Optionnel |  |
| `$q` | Version (or Date of Version) | Non rép. | Optionnel |  |
| `$r` | Medium of Performance (for Music) | Rép. | Optionnel |  |
| `$s` | Numeric Designation (for Music) | Rép. | Optionnel |  |
| `$u` | Key (for Music) | Non rép. | Optionnel |  |
| `$v` | Volume Designation | Non rép. | Optionnel |  |
| `$w` | Additional Elements (for Music) | Non rép. | Optionnel |  |
| `$j` | Form Subdivision | Rép. | Optionnel |  |
| `$x` | Topical Subdivision | Rép. | Optionnel |  |
| `$y` | Geographical Subdivision | Rép. | Optionnel |  |
| `$z` | Chronological Subdivision | Rép. | Optionnel |  |
| `$2` | Source | Non rép. | Optionnel |  |
| `$3` | Authority Record Identifier or Standard Number | Non rép. | Optionnel |  |

### Zone 501 — COLLECTIVE PREFERRED TITLE

**Répétabilité :** Rép. &nbsp;|&nbsp; **Occurrence :** Optionnel

| Sous-zone | Nom | Rép. | Occurrence | Description |
|-----------|-----|------|------------|-------------|
| `$a` | Collective Preferred Title | Non rép. | Optionnel | The main term used to bring together works of one kind. |
| `$b` | General Material Designation | Rép. | Optionnel | Text of general material designation. Repeatable. It corresponds to General Material Designation in the ISBDs prior to ISBD Consolidated Edition (2011). According to ISBD Consolidated Edition, GMD ... |
| `$e` | Collective Preferred Subtitle | Non rép. | Optionnel | ===PAGE 524=== |
| `$g` | Form Subdivision for Title | Rép. | Optionnel |  |
| `$k` | Date of Publication, etc. | Non rép. | Optionnel |  |
| `$m` | Language (when Part of Access Point) | Non rép. | Optionnel |  |
| `$r` | Medium of Performance (for Music) | Rép. | Optionnel |  |
| `$s` | Numeric Designation (for Music) | Rép. | Optionnel |  |
| `$u` | Key (for Music) | Non rép. | Optionnel |  |
| `$w` | Arranged Statement (for Music) | Non rép. | Optionnel |  |
| `$j` | Form Subdivision | Rép. | Optionnel |  |
| `$x` | Topical Subdivision | Rép. | Optionnel |  |
| `$y` | Geographical Subdivision | Rép. | Optionnel |  |
| `$z` | Chronological Subdivision | Rép. | Optionnel |  |
| `$2` | Source | Non rép. | Optionnel |  |
| `$3` | Authority Record Identifier or Standard Number | Non rép. | Optionnel |  |

### Zone 503 — CONVENTIONAL PREFERRED TITLE

**Répétabilité :** Rép. &nbsp;|&nbsp; **Occurrence :** Optionnel

| Sous-zone | Nom | Rép. | Occurrence | Description |
|-----------|-----|------|------------|-------------|
| `$a` | Form Heading Proper | Non rép. | Optionnel | The form heading as prescribed by the cataloguing code. |
| `$b` | Form Subheading | Non rép. | Optionnel | A subdivision of the form heading proper as prescribed by the code used. Not repeatable. ===PAGE 527=== |
| `$d` | Month and Day | Rép. | Optionnel |  |
| `$e` | Personal Name – Entry Element of Person | Non rép. | Optionnel |  |
| `$g` | Personal Names – Dates | Non rép. | Optionnel |  |
| `$h` | Personal Name Qualifier | Non rép. | Optionnel |  |
| `$i` | Title of Part | Non rép. | Optionnel |  |
| `$j` | Year | Rép. | Optionnel |  |
| `$k` | Numeration (Arabic) | Non rép. | Optionnel |  |
| `$l` | Numeration (Roman) | Non rép. | Optionnel |  |
| `$m` | Locality | Non rép. | Optionnel |  |
| `$n` | Institution in Locality | Non rép. | Optionnel |  |
| `$o` | Place in Locality | Non rép. | Optionnel |  |

### Zone 506 — PREFERRED ACCESS POINT – IDENTIFICATION OF A WORK

**Répétabilité :** — &nbsp;|&nbsp; **Occurrence :** —

| Sous-zone | Nom | Rép. | Occurrence | Description |
|-----------|-----|------|------------|-------------|
| `$h` | Number of Section or Part | Rép. | Optionnel |  |
| `$i` | Name of Section or Part | Rép. | Optionnel |  |
| `$c` | Form of Work | Non rép. | Optionnel |  |
| `$d` | Date of Work | Non rép. | Optionnel |  |
| `$e` | Place of Origin of Work | Non rép. | Optionnel |  |
| `$f` | Original Language of the Work | Non rép. | Optionnel |  |
| `$k` | Other Distinguishing Characteristics of a Work | Rép. | Optionnel |  |
| `$r` | Medium of Performance (for Music) | Rép. | Optionnel |  |
| `$s` | Numeric Designation (for Music) | Rép. | Optionnel |  |
| `$u` | Key (for Music) | Non rép. | Optionnel |  |
| `$3` | Authority Record Number | Non rép. | Optionnel |  |

### Zone 507 — PREFERRED ACCESS POINT – IDENTIFICATION OF AN EXPRESSION

**Répétabilité :** — &nbsp;|&nbsp; **Occurrence :** —

| Sous-zone | Nom | Rép. | Occurrence | Description |
|-----------|-----|------|------------|-------------|
| `$h` | Number of Section or Part [Work] | Rép. | Optionnel |  |
| `$i` | Name of Section or Part [Work] | Rép. | Optionnel |  |
| `$c` | Form of Work [Work] | Non rép. | Optionnel |  |
| `$d` | Date of Work [Work] | Non rép. | Optionnel |  |
| `$e` | Place of Origin of Work [Work] | Non rép. | Optionnel |  |
| `$f` | Original Language of the Work [Work] | Non rép. | Optionnel |  |
| `$r` | Medium of Performance (for Music) [Work] | Rép. | Optionnel |  |
| `$s` | Numeric Designation (for Music) [Work] | Rép. | Optionnel |  |
| `$u` | Key (for Music) [Work] | Non rép. | Optionnel |  |
| `$l` | Form of the Expression [Expression] | Non rép. | Optionnel |  |
| `$m` | Language of the Expression [Expression] | Non rép. | Optionnel |  |
| `$n` | Content Type [Expression] | Non rép. | Optionnel |  |
| `$o` | Date of Expression [Expression] | Non rép. | Optionnel |  |
| `$v` | Medium of Performance (for Music) [Expression] | Rép. | Optionnel |  |
| `$w` | Other Characteristics of Expression [Expression] | Rép. | Optionnel |  |
| `$3` | Authority Record Identifier or Standard Number | Non rép. | Optionnel |  |

### Zone 510 — PARALLEL TITLE PROPER

**Répétabilité :** Rép. &nbsp;|&nbsp; **Occurrence :** Optionnel

| Sous-zone | Nom | Rép. | Occurrence | Description |
|-----------|-----|------|------------|-------------|
| `$a` | Parallel Title | Non rép. | Optionnel | The chief title of the item in a language and/or script relating to the title proper in 200 $a and excluding any other title information in that language. |
| `$e` | Other Title Information | Rép. | Optionnel | Subtitles and other title information that appear subordinate to the parallel title in $a and are required as part of the access point or note. |
| `$h` | Number of Part | Rép. | Optionnel | The section or part numbering for items which are identified by a main title and a subsection title, and relate to the parallel title in $a. |
| `$i` | Name of Part | Rép. | Optionnel | ===PAGE 538=== |
| `$j` | Volume or Dates Associated with Title | Non rép. | Optionnel |  |
| `$n` | Miscellaneous Information | Non rép. | Optionnel |  |
| `$z` | Language of Title | Non rép. | Optionnel |  |
| `$2` | Source | Non rép. | Optionnel |  |

### Zone 511 — HALF TITLE

**Répétabilité :** Rép. &nbsp;|&nbsp; **Occurrence :** Optionnel

| Sous-zone | Nom | Rép. | Occurrence | Description |
|-----------|-----|------|------------|-------------|
| `$a` | Half Title | Non rép. | Optionnel | The variant title appearing on the half-title page. |
| `$e` | Other Title Information | Rép. | Optionnel | Subtitles and other title information that appear subordinate to the half title in $a and are required as part of the access point or note. |
| `$h` | Number of Part | Rép. | Optionnel | The section or part numbering for items which are identified by a main title and a subsection title, and relate to the half title in $a. |
| `$i` | Name of Part | Rép. | Optionnel | The section or part name for items which are identified by a main title and a subsection title and are in the same language as the half title in $a. |
| `$j` | Volume or Dates Associated with Title | Non rép. | Optionnel | The part of a multivolume resource or continuing resource to which the half title pertains. |
| `$n` | Miscellaneous Information | Non rép. | Optionnel | Text intended primarily for display in a note, e.g. “varies slightly”, “paperback edition”. It will be placed in relation to the other subfields in the position where the cataloguing agency intends... |
| `$z` | Language of Title | Non rép. | Optionnel | ===PAGE 541=== |
| `$2` | Source | Non rép. | Optionnel |  |

### Zone 512 — COVER TITLE

**Répétabilité :** Rép. &nbsp;|&nbsp; **Occurrence :** Optionnel

| Sous-zone | Nom | Rép. | Occurrence | Description |
|-----------|-----|------|------------|-------------|
| `$a` | Cover Title | Non rép. | Optionnel | The title as on the cover of the item without any other title information or statement of responsibility. |
| `$e` | Other Title Information | Rép. | Optionnel | Subtitles and other title information that appear on the cover subordinate to the cover title in $a. |
| `$h` | Number of Part | Rép. | Optionnel | The section or part numbering for items which are identified by a main title and a subsection title, and relate to the cover title in $a. |
| `$i` | Name of Part | Rép. | Optionnel | ===PAGE 543=== |
| `$j` | Volume or Dates Associated with Title | Non rép. | Optionnel |  |
| `$n` | Miscellaneous Information | Non rép. | Optionnel |  |
| `$z` | Language of Title | Non rép. | Optionnel |  |
| `$2` | Source | Non rép. | Optionnel |  |

### Zone 513 — ADDED TITLE-PAGE TITLE

**Répétabilité :** Rép. &nbsp;|&nbsp; **Occurrence :** Optionnel

| Sous-zone | Nom | Rép. | Occurrence | Description |
|-----------|-----|------|------------|-------------|
| `$a` | Added Title-Page Title | Non rép. | Optionnel | The title as on the added title-page of the item without any other title information or statement of responsibility. |
| `$e` | Other Title Information | Rép. | Optionnel | Subtitles and other title information that appear on the added title-page subordinate to the title in $a. |
| `$h` | Number of Part | Rép. | Optionnel | The section or part numbering for items which are identified by a main title and a subsection title and relate to the added title-page title in $a. |
| `$i` | Name of Part | Rép. | Optionnel | ===PAGE 545=== |
| `$j` | Volume or Dates Associated with Title | Non rép. | Optionnel |  |
| `$n` | Miscellaneous Information | Non rép. | Optionnel |  |
| `$z` | Language of Title | Non rép. | Optionnel |  |
| `$2` | Source | Non rép. | Optionnel |  |

### Zone 514 — CAPTION TITLE

**Répétabilité :** Rép. &nbsp;|&nbsp; **Occurrence :** Optionnel

| Sous-zone | Nom | Rép. | Occurrence | Description |
|-----------|-----|------|------------|-------------|
| `$a` | Caption Title | Non rép. | Optionnel | The title as given at the beginning of the first page of the text excluding any other title information or statement of responsibility. |
| `$e` | Other Title Information | Rép. | Optionnel | Subtitles and other title information that are subordinate to the caption title. |
| `$h` | Number of Part | Rép. | Optionnel | The section or part numbering for items which are identified by a main title and a subsection title and relate to the caption title in $a. |
| `$i` | Name of Part | Rép. | Optionnel | The section or part name for items which are identified by a main title and a subsection title. Repeatable. ===PAGE 547=== |
| `$j` | Volume or Dates Associated with Title | Non rép. | Optionnel |  |
| `$n` | Miscellaneous Information | Non rép. | Optionnel |  |
| `$z` | Language of Title | Non rép. | Optionnel |  |
| `$2` | Source | Non rép. | Optionnel |  |

### Zone 515 — RUNNING TITLE

**Répétabilité :** Rép. &nbsp;|&nbsp; **Occurrence :** Optionnel

| Sous-zone | Nom | Rép. | Occurrence | Description |
|-----------|-----|------|------------|-------------|
| `$a` | Running Title | Non rép. | Optionnel | The title as taken from the head or foot of the page of the item. |
| `$e` | Other Title Information | Rép. | Optionnel | Subtitles and other title information that are subordinate to the running title. |
| `$h` | Number of Part | Rép. | Optionnel | The section or part numbering for items which are identified by a main title and a subsection title and relate to the running title in $a. |
| `$i` | Name of Part | Rép. | Optionnel | The section or part name for items which are identified by a main title and a subsection title. |
| `$j` | Volume or Dates Associated with Title | Non rép. | Optionnel | ===PAGE 549=== |
| `$n` | Miscellaneous Information | Non rép. | Optionnel |  |
| `$z` | Language of Title | Non rép. | Optionnel |  |
| `$2` | Source | Non rép. | Optionnel |  |

### Zone 516 — SPINE TITLE

**Répétabilité :** Rép. &nbsp;|&nbsp; **Occurrence :** Optionnel

| Sous-zone | Nom | Rép. | Occurrence | Description |
|-----------|-----|------|------------|-------------|
| `$a` | Spine Title | Non rép. | Optionnel | The title as on the spine of the item without any other title information or statement of responsibility. |
| `$e` | Other Title Information | Rép. | Optionnel | Subtitles and other title information that appear on the spine subordinate to the spine title in $a. |
| `$h` | Number of Part | Rép. | Optionnel | The section or part numbering for items which are identified by a main title and a subsection title and relate to the spine title in $a. |
| `$i` | Name of Part | Rép. | Optionnel | ===PAGE 551=== |
| `$j` | Volume or Dates Associated with Title | Non rép. | Optionnel |  |
| `$n` | Miscellaneous Information | Non rép. | Optionnel |  |
| `$z` | Language of Title | Non rép. | Optionnel |  |
| `$2` | Source | Non rép. | Optionnel |  |

### Zone 517 — OTHER VARIANT TITLES

**Répétabilité :** Rép. &nbsp;|&nbsp; **Occurrence :** Optionnel

| Sous-zone | Nom | Rép. | Occurrence | Description |
|-----------|-----|------|------------|-------------|
| `$a` | Variant Title | Non rép. | Optionnel | The variant title without any other title information or statement of responsibility. |
| `$e` | Other Title Information | Rép. | Optionnel | Subtitles and other title information that appear on the item subordinate to the variant title in $a. |
| `$h` | Number of Part | Rép. | Optionnel | The section or part numbering for items which are identified by a main title and a subsection title and relate to the variant title in $a. Repeatable. ===PAGE 553=== |
| `$i` | Name of Part | Rép. | Optionnel |  |
| `$j` | Volume or Dates Associated with Title | Non rép. | Optionnel |  |
| `$n` | Miscellaneous Information | Non rép. | Optionnel |  |
| `$z` | Language of Title | Non rép. | Optionnel |  |
| `$2` | Source | Non rép. | Optionnel |  |

### Zone 518 — TITLE IN STANDARD MODERN SPELLING

**Répétabilité :** Rép. &nbsp;|&nbsp; **Occurrence :** Optionnel

| Sous-zone | Nom | Rép. | Occurrence | Description |
|-----------|-----|------|------------|-------------|
| `$e` | Other Title Information | Rép. | Optionnel | Subtitles and other title information that appear on the item subordinate to the title in standard modern spelling in $a. Repeatable. ===PAGE 556=== |
| `$h` | Number of Part | Rép. | Optionnel |  |
| `$i` | Name of Part | Rép. | Optionnel |  |
| `$j` | Volume or Dates Associated with Title | Non rép. | Optionnel |  |
| `$n` | Miscellaneous Information | Non rép. | Optionnel |  |
| `$z` | Language of Title | Non rép. | Optionnel |  |
| `$2` | Source | Non rép. | Optionnel |  |

### Zone 520 — FORMER TITLE (CONTINUING RESOURCES)

**Répétabilité :** Rép. &nbsp;|&nbsp; **Occurrence :** Optionnel

| Sous-zone | Nom | Rép. | Occurrence | Description |
|-----------|-----|------|------------|-------------|
| `$a` | Former Title Proper | Non rép. | Optionnel | The title proper of the former title of the continuing resource. |
| `$e` | Other Title Information | Rép. | Optionnel | Subtitles and other title information that appear subordinate to the title proper of the former title in $a. |
| `$h` | Number of Part | Non rép. | Optionnel | The number of a part of the continuing resource when the continuing resource is subdivided. |
| `$i` | Name of Part | Non rép. | Optionnel | ===PAGE 559=== |
| `$j` | Volumes or Dates of Former Title | Non rép. | Optionnel |  |
| `$n` | Miscellaneous Information | Non rép. | Optionnel |  |
| `$x` | ISSN of Former Title | Non rép. | Optionnel |  |

### Zone 530 — KEY TITLE

**Répétabilité :** Rép. &nbsp;|&nbsp; **Occurrence :** Optionnel

| Sous-zone | Nom | Rép. | Occurrence | Description |
|-----------|-----|------|------------|-------------|
| `$a` | Key Title | Non rép. | Optionnel | The key title without the addition of any qualification. |
| `$b` | Qualifier | Non rép. | Optionnel | Any qualification included in the key title in order to distinguish it from other continuing resources with the same title (EX 2, 4). Following ISSN practice, the qualification is enclosed in paren... |
| `$j` | Volume or Dates Associated with Key Title | Non rép. | Optionnel | This subfield is used when the record contains more than one key title which has been assigned to different volumes or date-spans of the life of the periodical. The data distinguishes the periods o... |
| `$v` | Volume Designation | Non rép. | Optionnel | This subfield is used to indicate a particular part of the resource (volume, issue and pages if appropriate) that is related to another item. This subfield is used only when the field is embedded i... |

### Zone 531 — ABBREVIATED KEY TITLE (CONTINUING RESOURCES)

**Répétabilité :** — &nbsp;|&nbsp; **Occurrence :** —

| Sous-zone | Nom | Rép. | Occurrence | Description |
|-----------|-----|------|------------|-------------|
| `$a` | Abbreviated Title | Non rép. | Optionnel | The abbreviated key title without the addition of any qualifying information. |
| `$b` | Qualifier | Non rép. | Optionnel | Any qualifying information added to the key title to make it distinctive from an otherwise identical key title. This may or may not be abbreviated. Following ISSN practice, the qualification is enc... |
| `$v` | Volume Designation | Non rép. | Optionnel | This subfield is used to indicate a particular part of the resource (volume, issue and pages if appropriate) that is related to another item. This subfield is used only when the field is embedded i... |

### Zone 532 — EXPANDED TITLE

**Répétabilité :** Rép. &nbsp;|&nbsp; **Occurrence :** Optionnel

| Sous-zone | Nom | Rép. | Occurrence | Description |
|-----------|-----|------|------------|-------------|
| `$a` | Expanded Title | Non rép. | Optionnel |  |
| `$z` | Language of Title | Non rép. | Optionnel |  |
| `$2` | Source | Non rép. | Optionnel |  |

### Zone 540 — ADDITIONAL TITLE SUPPLIED BY CATALOGUER

**Répétabilité :** — &nbsp;|&nbsp; **Occurrence :** —

| Sous-zone | Nom | Rép. | Occurrence | Description |
|-----------|-----|------|------------|-------------|
| `$a` | Additional Title | Non rép. | Optionnel | The text of the additional title as supplied by the cataloguer. |
| `$e` | Other Title Information | Rép. | Optionnel | Subtitles and other title information that appear subordinate to the supplied title in $a. |
| `$h` | Number of Part | Non rép. | Optionnel | The section or part numbering for items which are identified by a main title and a subsection title, and relate to the additional title in $a. |
| `$i` | Name of Part | Non rép. | Optionnel | ===PAGE 568=== |
| `$j` | Volume or Dates Associated with Title | Non rép. | Optionnel |  |
| `$n` | Miscellaneous Information | Non rép. | Optionnel |  |
| `$z` | Language of Title | Non rép. | Optionnel |  |
| `$2` | Source | Non rép. | Optionnel |  |

### Zone 541 — TRANSLATED TITLE SUPPLIED BY CATALOGUER

**Répétabilité :** — &nbsp;|&nbsp; **Occurrence :** —

| Sous-zone | Nom | Rép. | Occurrence | Description |
|-----------|-----|------|------------|-------------|
| `$a` | Translated Title | Non rép. | Optionnel | The title proper without any other title information, in translation (EX 3). |
| `$e` | Other Title Information | Non rép. | Optionnel | Subtitles and other title information in translation appearing subordinate to the translated title (EX 2). |
| `$h` | Number of a Part | Non rép. | Optionnel | The section or part numbering for items identified by a common name and a subsection name, with or without accompanying caption translated. |
| `$i` | Name of Part | Non rép. | Optionnel | The section or part name for items which are identified by a common name and a subsection name. |
| `$z` | Language of Translated Title | Non rép. | Optionnel | ===PAGE 571=== |
| `$2` | Source | Non rép. | Optionnel |  |

### Zone 545 — SECTION TITLE

**Répétabilité :** Rép. &nbsp;|&nbsp; **Occurrence :** Optionnel

| Sous-zone | Nom | Rép. | Occurrence | Description |
|-----------|-----|------|------------|-------------|
| `$a` | Section Title | Non rép. | Optionnel |  |
| `$e` | Other Title Information | Rép. | Optionnel | Subtitles and other title information that appear subordinate to the supplied title in $a. |
| `$h` | Number of Part | Non rép. | Optionnel | The section or part numbering for items which are identified by a main title and a subsection title, and relate to the section title in $a. |
| `$i` | Name of Part | Non rép. | Optionnel | The section or part name for items which are identified by a main title and a subsection title. |
| `$j` | Volume or Dates Associated with Title | Non rép. | Optionnel | ===PAGE 574=== |
| `$n` | Miscellaneous Information | Non rép. | Optionnel |  |
| `$z` | Language of Title | Non rép. | Optionnel |  |
| `$2` | Source | Non rép. | Optionnel |  |

### Zone 560 — ARTIFICIAL TITLE

**Répétabilité :** Rép. &nbsp;|&nbsp; **Occurrence :** Optionnel

| Sous-zone | Nom | Rép. | Occurrence | Description |
|-----------|-----|------|------------|-------------|
| `$a` | Artificial title | Non rép. | Optionnel |  |
| `$e` | Other Title Information | Rép. | Optionnel | Subtitles and other title information that appear on the resource subordinate to the title in subfield $a. |
| `$5` | Institution to which the Field Applies | Non rép. | Optionnel | Name of the institution to which field applies in the coded form if it is considered necessary by the agency which creates the record to identify specific attributes of an item. If the subfield is ... |

### Zone 576 — NAME/ACCESS POINT– IDENTIFICATION OF A WORK

**Répétabilité :** — &nbsp;|&nbsp; **Occurrence :** —

| Sous-zone | Nom | Rép. | Occurrence | Description |
|-----------|-----|------|------------|-------------|
| `$1` | Linking Data | Rép. | Optionnel |  |
| `$a` | Name | Non rép. | Oblig. si appl. |  |
| `$t` | Title | Non rép. | Oblig. si appl. |  |
| `$h` | Number of Section or Part | Rép. | Optionnel |  |
| `$i` | Name of Section or Part | Rép. | Optionnel |  |
| `$c` | Form of Work | Non rép. | Optionnel |  |
| `$d` | Date of Work | Non rép. | Optionnel |  |
| `$e` | Place of Origin of Work | Non rép. | Optionnel |  |
| `$f` | Original Language of the Work | Non rép. | Optionnel |  |
| `$k` | Other Distinguishing Characteristics of a Work | Rép. | Optionnel |  |
| `$r` | Medium of Performance (for Music) | Rép. | Optionnel |  |
| `$s` | Numeric Designation (for Music) | Rép. | Optionnel |  |
| `$u` | Key (for Music) | Non rép. | Optionnel |  |
| `$j` | Form Subdivision | Rép. | Optionnel |  |
| `$x` | Topical Subdivision | Rép. | Optionnel |  |
| `$y` | Geographical Subdivision | Rép. | Optionnel |  |
| `$z` | Chronological Subdivision | Rép. | Optionnel |  |
| `$3` | Authority Record Identifier or Standard Number | Non rép. | Optionnel |  |

### Zone 577 — NAME/ACCESS POINT – IDENTIFICATION OF AN EXPRESSION

**Répétabilité :** — &nbsp;|&nbsp; **Occurrence :** —

| Sous-zone | Nom | Rép. | Occurrence | Description |
|-----------|-----|------|------------|-------------|
| `$1` | Linking Data | Rép. | Optionnel |  |
| `$a` | Name [Work] | Non rép. | Oblig. si appl. |  |
| `$h` | Number of Section or Part [Work] | Rép. | Optionnel |  |
| `$i` | Name of Section or Part [Work] | Rép. | Optionnel |  |
| `$c` | Form of Work [Work] | Non rép. | Optionnel |  |
| `$d` | Date of Work [Work] | Non rép. | Optionnel |  |
| `$e` | Place of Origin of Work [Work] | Non rép. | Optionnel |  |
| `$f` | Original Language of the Work [Work] | Non rép. | Optionnel |  |
| `$r` | Medium of Performance (for Music) [Work] | Rép. | Optionnel |  |
| `$s` | Numeric Designation (for Music) [Work] | Non rép. | Optionnel |  |
| `$u` | Key (for Music) [Work] | Non rép. | Optionnel |  |
| `$l` | Form of the Expression [Expression] | Non rép. | Optionnel |  |
| `$m` | Language of the Expression [Expression] | Non rép. | Optionnel |  |
| `$n` | Content Type [Expression] | Non rép. | Optionnel |  |
| `$o` | Date of Expression [Expression] | Non rép. | Optionnel |  |
| `$j` | Form Subdivision | Rép. | Optionnel |  |
| `$x` | Topical Subdivision | Rép. | Optionnel |  |
| `$y` | Geographical Subdivision | Rép. | Optionnel |  |
| `$z` | Chronological Subdivision | Rép. | Optionnel |  |
| `$3` | Authority Record Identifier or Standard Number | Non rép. | Optionnel |  |

---

## BLOC 6 — ANALYSE SUJET & HISTORIQUE BIBL.

### Zone 600 — PERSONAL NAME USED AS SUBJECT

**Répétabilité :** Rép. &nbsp;|&nbsp; **Occurrence :** Optionnel

| Sous-zone | Nom | Rép. | Occurrence | Description |
|-----------|-----|------|------------|-------------|
| `$a` | Entry Element | Non rép. | Optionnel | The portion of the name used as the entry element in the access point; that part of the name by which the name is entered in ordered lists. This subfield must be present if the field is present. |
| `$b` | Part of Name Other than Entry Element | Non rép. | Optionnel | ===PAGE 590=== |
| `$c` | Additions to Name Other than Dates | Rép. | Optionnel |  |
| `$d` | Roman Numerals | Non rép. | Optionnel |  |
| `$f` | Dates | Non rép. | Optionnel |  |
| `$g` | Expansion of Initials of Forename | Non rép. | Optionnel |  |
| `$p` | Affiliation/Address | Non rép. | Optionnel |  |
| `$j` | Form Subdivision | Rép. | Optionnel |  |
| `$x` | Topical Subdivision | Rép. | Optionnel |  |
| `$y` | Geographical Subdivision | Rép. | Optionnel |  |
| `$z` | Chronological Subdivision | Rép. | Optionnel |  |
| `$2` | Source | Non rép. | Optionnel |  |
| `$3` | Authority Record Identifier or Standard Number | Rép. | Optionnel |  |

### Zone 601 — CORPORATE BODY NAME USED AS SUBJECT

**Répétabilité :** Rép. &nbsp;|&nbsp; **Occurrence :** Optionnel

| Sous-zone | Nom | Rép. | Occurrence | Description |
|-----------|-----|------|------------|-------------|
| `$a` | Entry Element | Non rép. | Oblig. si appl. |  |
| `$b` | Subdivision (or Name if Entered Under Place) | Rép. | Optionnel |  |
| `$c` | Addition to Name or Qualifier | Rép. | Optionnel |  |
| `$l` | Location of Meeting | Non rép. | Optionnel |  |
| `$f` | Date of Meeting | Non rép. | Optionnel |  |
| `$g` | Inverted Element | Non rép. | Optionnel |  |
| `$j` | Form Subdivision | Rép. | Optionnel |  |
| `$x` | Topical Subdivision | Rép. | Optionnel |  |
| `$y` | Geographical Subdivision | Rép. | Optionnel |  |
| `$z` | Chronological Subdivision | Rép. | Optionnel |  |
| `$2` | Source | Non rép. | Optionnel |  |
| `$3` | Authority Record Identifier or Standard Number | Rép. | Optionnel |  |

### Zone 602 — FAMILY NAME USED AS SUBJECT

**Répétabilité :** Rép. &nbsp;|&nbsp; **Occurrence :** Optionnel

| Sous-zone | Nom | Rép. | Occurrence | Description |
|-----------|-----|------|------------|-------------|
| `$a` | Entry Element | Non rép. | Optionnel | The name of the family in access point form. |
| `$c` | Type of Family | Non rép. | Optionnel | A categorization or generic descriptor for the type of family. Includes categorizations such as clan, dynasty, family unit, patriarchy, matriarchy, etc. |
| `$d` | Places Associated with the Family | Rép. | Optionnel | Information pertaining to places where the family resides or resided or had some connection. |
| `$f` | Dates | Non rép. | Optionnel | The dates of a family when they are required as part of the access point. Not repeatable (EX 2). |
| `$j` | Form Subdivision | Rép. | Optionnel | A term added to the subject access point to further specify the kind(s) or genre(s) of material (EX 2). Agencies not using this subdivision should use $x instead. |
| `$x` | Topical Subdivision | Rép. | Optionnel | A term added to a subject access point to further specify the topic the subject access point represents. |
| `$y` | Geographical Subdivision | Rép. | Optionnel | ===PAGE 597=== |
| `$z` | Chronological Subdivision | Rép. | Optionnel |  |
| `$2` | Source | Non rép. | Optionnel |  |
| `$3` | Authority Record Identifier or Standard Number | Rép. | Optionnel |  |

### Zone 604 — NAME AND TITLE USED AS SUBJECT

**Répétabilité :** Rép. &nbsp;|&nbsp; **Occurrence :** Optionnel

| Sous-zone | Nom | Rép. | Occurrence | Description |
|-----------|-----|------|------------|-------------|
| `$1` | Linking Data | Rép. | Optionnel | Contains the tag and indicator values of the embedded fields, without spacing or punctuation. Repeatable for each embedded field. (B) Subfields: Standard Subfields Technique |
| `$a` | Name | Non rép. | Oblig. | The name of the person, corporate body or family with primary responsibility for the resource registered in subfield $t. When subfield $a is a composite of more than one standard subfield use punct... |
| `$t` | Title | Non rép. | Oblig. | The title by which the work is known. When subfield $t is a composite of more than one standard subfield use ISBD punctuation to separate the elements. Mandatory. |
| `$j` | Form Subdivision | Rép. | Optionnel | A term added to a subject access point to further specify the kind(s) or genre(s) of material. Agencies not using this subdivision should use $x instead. |
| `$x` | Topical Subdivision | Rép. | Optionnel | A term added to a subject access point to specify further the topic that the access point represents. |
| `$y` | Geographical Subdivision | Rép. | Optionnel | ===PAGE 599=== |
| `$z` | Chronological Subdivision | Rép. | Optionnel |  |
| `$2` | Source | Non rép. | Optionnel |  |
| `$3` | Authority Record Identifier or Standard Number | Rép. | Optionnel |  |

### Zone 605 — TITLE USED AS SUBJECT

**Répétabilité :** Rép. &nbsp;|&nbsp; **Occurrence :** Optionnel

| Sous-zone | Nom | Rép. | Occurrence | Description |
|-----------|-----|------|------------|-------------|
| `$a` | Entry Element | Non rép. | Optionnel | The short title or title proper. |
| `$h` | Number of Section or Part | Rép. | Optionnel | The number of a part when the resource to which the title or preferred title refers is only a part of the work named in subfield $a. Repeatable for a subdivided part. |
| `$i` | Name of Section or Part | Rép. | Optionnel | The name of a part when the resource to which the title or preferred title refers is only a part of the work named in subfield $a (EX 3). Repeatable for a subdivided part. (EX 3) |
| `$k` | Date of Publication | Non rép. | Optionnel | The date of publication of the resource as subject when it is necessary to add it to the preferred title to distinguish the resource. |
| `$l` | Form Subheading | Non rép. | Optionnel | A standard phrase added to a access point to further specify the preferred title. Not repeatable. ===PAGE 602=== |
| `$m` | Language (when Part of Access Point) | Non rép. | Optionnel |  |
| `$n` | Miscellaneous Information | Rép. | Optionnel |  |
| `$q` | Version (or Date of Version) | Non rép. | Optionnel |  |
| `$r` | Medium of Performance (for Music) | Rép. | Optionnel |  |
| `$s` | Numeric Designation (for Music) | Rép. | Optionnel |  |
| `$u` | Key (for Music) | Non rép. | Optionnel |  |
| `$w` | Arranged Statement (for Music) | Non rép. | Optionnel |  |
| `$j` | Form Subdivision | Rép. | Optionnel |  |
| `$x` | Topical Subdivision | Rép. | Optionnel |  |
| `$y` | Geographical Subdivision | Rép. | Optionnel |  |
| `$z` | Chronological Subdivision | Rép. | Optionnel |  |
| `$2` | Source | Non rép. | Optionnel |  |
| `$3` | Authority Record Identifier or Standard Number | Rép. | Optionnel |  |

### Zone 606 — TOPICAL NAME USED AS SUBJECT

**Répétabilité :** Rép. &nbsp;|&nbsp; **Occurrence :** Optionnel

| Sous-zone | Nom | Rép. | Occurrence | Description |
|-----------|-----|------|------------|-------------|
| `$a` | Entry Element | Non rép. | Optionnel | The term in the form prescribed by the system of subject access points used. |
| `$j` | Form Subdivision | Rép. | Optionnel | A term added to the subject access point to further specify the kind(s) or genre(s) of material (EX 7,8). Agencies not using this subdivision should use $x instead. Repeatable (EX 8). |
| `$x` | Topical Subdivision | Rép. | Optionnel | A term added to the subject access point to specify the aspect that the subject access point represents (EX 2, 3). |
| `$y` | Geographical Subdivision | Rép. | Optionnel | A term added to the subject access point to specify a place in relation to the topic that the subject access point represents (EX 2, 4). |
| `$z` | Chronological Subdivision | Rép. | Optionnel | ===PAGE 606=== |
| `$2` | Source | Non rép. | Optionnel |  |
| `$3` | Authority Record Identifier or Standard Number | Rép. | Optionnel |  |

### Zone 607 — GEOGRAPHICAL NAME USED AS SUBJECT

**Répétabilité :** Rép. &nbsp;|&nbsp; **Occurrence :** Optionnel

| Sous-zone | Nom | Rép. | Occurrence | Description |
|-----------|-----|------|------------|-------------|
| `$a` | Entry Element | Non rép. | Optionnel | The geographical name in the form prescribed by the system of authority access points used. |
| `$j` | Form Subdivision | Rép. | Optionnel | A term added to the subject access point to further specify the kind(s) or genre(s) of material (EX 5, 6). Agencies not using this subdivision should use $x instead. |
| `$x` | Topical Subdivision | Rép. | Optionnel | A term added to the geographical name to specify the aspect that the subject access point represents. |
| `$y` | Geographical Subdivision | Rép. | Optionnel | A term added to the geographical name to further specify a place that the subject access point represents (EX 5). |
| `$z` | Chronological Subdivision | Rép. | Optionnel | A term added to the geographical name to specify the period in time in relation to the name that the subject access point represents (EX 1, 2, 4). |
| `$2` | Source | Non rép. | Optionnel | An identification in coded form of the system from which the subject access point is derived. It is recommended that subfield $2 always be present in each occurrence of the field. For a list of sub... |
| `$3` | Authority Record Identifier or Standard Number | Rép. | Optionnel | The identifier for the authority record for the access point. This subfield is for use with UNIMARC/Authorities format. ===PAGE 609=== |

### Zone 608 — FORM, GENRE OR PHYSICAL CHARACTERISTICS ACCESS POINT

**Répétabilité :** — &nbsp;|&nbsp; **Occurrence :** —

| Sous-zone | Nom | Rép. | Occurrence | Description |
|-----------|-----|------|------------|-------------|
| `$a` | Entry Element | Non rép. | Optionnel | The term in the form prescribed by the system of form access points used. |
| `$j` | Form Subdivision | Rép. | Optionnel | A term added to the subject access point to further specify the kind(s) or genre(s) of material (EX 6). Agencies not using this subdivision should use $x instead. |
| `$x` | Topical Subdivision | Rép. | Optionnel | A term added to the form of the access point to specify the aspect that the access point represents. |
| `$y` | Geographical Subdivision | Rép. | Optionnel | A term added to the form of the access point to specify a place in relation to the topic that the access point represents. |
| `$z` | Chronological Subdivision | Rép. | Optionnel | A term added to the form of the access point to specify the period of time in relation to the topic that the access point represents. |
| `$2` | Source | Non rép. | Optionnel | An identification in coded form of the system from which the subject access point is derived. It is recommended that subfield $2 always be present in each occurrence of the field. For a list of sub... |
| `$3` | Authority Record Identifier or Standard Number | Rép. | Optionnel | ===PAGE 611=== |
| `$5` | Institution to which the Field Applies | Non rép. | Optionnel |  |

### Zone 610 — UNCONTROLLED SUBJECT TERMS

**Répétabilité :** Rép. &nbsp;|&nbsp; **Occurrence :** Optionnel

| Sous-zone | Nom | Rép. | Occurrence | Description |
|-----------|-----|------|------------|-------------|
| `$a` | Subject Term | Rép. | Optionnel | Repeatable when more than one term is assigned for the resource. |

### Zone 615 — SUBJECT CATEGORY [PROVISIONAL]

**Répétabilité :** Rép. &nbsp;|&nbsp; **Occurrence :** Optionnel

| Sous-zone | Nom | Rép. | Occurrence | Description |
|-----------|-----|------|------------|-------------|
| `$a` | Subject Category Entry Element Text | Non rép. | Optionnel | The term in the form prescribed by the system of subject categories used (EX 3, 4). |
| `$x` | Subject Category Subdivision Text | Rép. | Optionnel | The term added to the subject category to specify a particular aspect of the subject category in the $a subfield. |
| `$n` | Subject Category Code | Rép. | Optionnel | A coded representation of a subject category (EX 1, 2). |
| `$m` | Subject Category Subdivision Code | Rép. | Optionnel | A coded representation of a subject category subdivision (EX 2). |
| `$2` | Source | Non rép. | Optionnel | An identification in coded form of the system from which the subject access point is derived. It is recommended that subfield $2 always be present in each occurrence of the field. For a list of sub... |
| `$3` | Authority Record Identifier or Standard Number | Rép. | Optionnel | The identifier for the authority record for the category. |

### Zone 616 — TRADEMARK USED AS SUBJECT [PROVISIONAL]

**Répétabilité :** Rép. &nbsp;|&nbsp; **Occurrence :** Optionnel

| Sous-zone | Nom | Rép. | Occurrence | Description |
|-----------|-----|------|------------|-------------|
| `$a` | Entry Element | Non rép. | Optionnel | The trademark name in access point form. Not repeatable |
| `$f` | Dates | Non rép. | Optionnel | The dates between which a particular trademark was in use, when they are required as part of the access point, for example, as qualifiers. |
| `$c` | Qualification | Rép. | Optionnel | Any addition to the name of the trademark added by the cataloguer, other than dates (EX 3, 4). |
| `$j` | Form Subdivision | Rép. | Optionnel | A term added to a subject access point to further specify the kind(s) or genre(s) of material. Agencies not using this subdivision should use $x instead. |
| `$x` | Topical Subdivision | Rép. | Optionnel | A term added to a subject access point to further specify the topic the access point represents. |
| `$y` | Geographical Subdivision | Rép. | Optionnel | A term added to a subject access point to specify a place in relation to a trademark which the subject access point represents. |
| `$z` | Chronological Subdivision | Rép. | Optionnel | A term added to a subject access point to specify the period in time in relation to a trademark which the subject access point represents. Repeatable. ===PAGE 618=== |
| `$2` | Source | Non rép. | Optionnel |  |
| `$3` | Authority Record Identifier or Standard Number | Non rép. | Optionnel |  |

### Zone 617 — HIERARCHICAL GEOGRAPHICAL NAME USED AS SUBJECT

**Répétabilité :** — &nbsp;|&nbsp; **Occurrence :** —

| Sous-zone | Nom | Rép. | Occurrence | Description |
|-----------|-----|------|------------|-------------|
| `$b` | State or Province, etc. | Non rép. | Optionnel | First-order political jurisdiction below a country. Not repeatable |
| `$c` | Intermediate Political Jurisdiction | Rép. | Optionnel | Second-order or lower political jurisdiction, but not including cities, etc. Repeatable when multiple levels are given, in order from highest to lowest. |
| `$d` | City, etc. | Non rép. | Optionnel | The name of a city, town, commune, village or other distinct populated area not defined as a subsection of a larger one (see $k). |
| `$e` | Venue | Rép. | Optionnel | Named buildings, urban spaces, vehicles, etc. |
| `$f` | Date | Rép. | Optionnel | ===PAGE 620=== |
| `$g` | Season | Non rép. | Optionnel |  |
| `$h` | Occasion | Non rép. | Optionnel |  |
| `$i` | Final Date | Non rép. | Optionnel |  |
| `$k` | Subsection of City, etc. | Rép. | Optionnel |  |
| `$m` | Other Geographical Regions or Features | Rép. | Optionnel |  |
| `$n` | Extraterrestrial Areas | Rép. | Optionnel |  |
| `$2` | Source | Non rép. | Oblig. si appl. |  |
| `$3` | Authority Record Identifier or Standard Number | Non rép. | Optionnel |  |

### Zone 620 — PLACE AND DATE OF PUBLICATION, PERFORMANCE, ETC.

**Répétabilité :** — &nbsp;|&nbsp; **Occurrence :** —

| Sous-zone | Nom | Rép. | Occurrence | Description |
|-----------|-----|------|------------|-------------|
| `$a` | Country (Nation State) | Non rép. | Optionnel |  |
| `$b` | State or Province, etc. | Non rép. | Optionnel | First-order political jurisdiction below a country. Not repeatable. ===PAGE 623=== |
| `$c` | Intermediate Political Jurisdiction | Rép. | Optionnel |  |
| `$d` | City, etc. | Non rép. | Optionnel |  |
| `$e` | Venue | Rép. | Optionnel |  |
| `$f` | Date | Rép. | Optionnel |  |
| `$g` | Season | Non rép. | Optionnel |  |
| `$h` | Occasion | Non rép. | Optionnel |  |
| `$i` | Final Date | Non rép. | Optionnel |  |
| `$k` | Subsection of City, etc. | Rép. | Optionnel |  |
| `$m` | Other Geographical Regions or Features | Rép. | Optionnel |  |
| `$n` | Extraterrestrial Areas | Rép. | Optionnel |  |
| `$2` | Source | Non rép. | Oblig. si appl. |  |
| `$3` | Authority Record Identifier or Standard Number | Non rép. | Optionnel |  |

### Zone 621 — PLACE AND DATE OF PROVENANCE

**Répétabilité :** Rép. &nbsp;|&nbsp; **Occurrence :** Optionnel

| Sous-zone | Nom | Rép. | Occurrence | Description |
|-----------|-----|------|------------|-------------|
| `$a` | Country (Nation State) or Larger Entity Country | Non rép. | Optionnel | Optional. |
| `$b` | State or Province, etc. | Non rép. | Optionnel | First-order political jurisdiction below a country. Not repeatable. ===PAGE 627=== |
| `$c` | Intermediate Political Jurisdiction | Rép. | Optionnel |  |
| `$e` | Building, Vehicle, etc. | Rép. | Optionnel |  |
| `$f` | Date 1 | Non rép. | Optionnel |  |
| `$g` | Season | Non rép. | Optionnel |  |
| `$h` | Occasion | Non rép. | Optionnel |  |
| `$i` | Date 2 | Non rép. | Optionnel |  |
| `$k` | Subsection of City, etc. | Rép. | Optionnel |  |
| `$m` | Other Geographic Regions or Features | Rép. | Optionnel |  |
| `$n` | Extraterrestrial Area | Rép. | Optionnel |  |
| `$2` | Source | Non rép. | Oblig. si appl. |  |
| `$3` | Authority Record Identifier or Standard Number | Non rép. | Optionnel |  |
| `$5` | Institution to which the Field Applies | Non rép. | Optionnel |  |
| `$6` | Interfield Linking Data | Rép. | Optionnel |  |

### Zone 623 — CHARACTER

**Répétabilité :** Rép. &nbsp;|&nbsp; **Occurrence :** Optionnel

| Sous-zone | Nom | Rép. | Occurrence | Description |
|-----------|-----|------|------------|-------------|
| `$a` | Entry Element | Non rép. | Oblig. si appl. | The portion of the name used as the entry element. Mandatory. |
| `$b` | Part of Name Other than Entry Element | Non rép. | Optionnel | The remainder of the name. Not repeatable. (EX 8) |
| `$c` | Additions to Name of the Character | Rép. | Optionnel | Any additions to name of character which do not form an integral part of the name itself including titles, epithets, indications of office or relationship with another character. |
| `$3` | Authority Record Identifier or Standard Number | Non rép. | Optionnel | The identifier for the authority record for the access point. This subfield is for use with UNIMARC/Authorities format. |
| `$6` | Interfield Linking Data | Rép. | Optionnel | This subfield contains information allowing the field to be linked for processing purposes to other fields in the record. (See section 3.13 Interfield Linking Data, 3 Format structure). Data requir... |

### Zone 626 — TECHNICAL DETAILS ACCESS (ELECTRONIC RESOURCES) [OBSOLETE]

**Répétabilité :** — &nbsp;|&nbsp; **Occurrence :** —

*Champ de contrôle — pas de sous-zones ni d'indicateurs.*

### Zone 631 — OCCUPATION

**Répétabilité :** Rép. &nbsp;|&nbsp; **Occurrence :** Optionnel

| Sous-zone | Nom | Rép. | Occurrence | Description |
|-----------|-----|------|------------|-------------|
| `$a` | Occupation | Non rép. | Optionnel | Term specifying the profession, business, or avocation of an individual. |
| `$b` | Form | Non rép. | Optionnel | Class of material to which materials described in the record belong and which is essential to distinguish special kinds of materials according to their physical character, subject of intellectual c... |
| `$j` | Form Subdivision | Rép. | Optionnel | A term added to the subject access point to further specify the kind(s) or genre(s) of material. Agencies not using this subdivision should use $x instead. |
| `$x` | Topical Subdivision | Rép. | Optionnel | A term added to a subject access point to further specify the occupation the subject access point represents. |
| `$y` | Geographical Subdivision | Rép. | Optionnel | A term added to a subject access point to specify a place in relation to occupation which the subject access point represents. |
| `$z` | Chronological Subdivision | Rép. | Optionnel | A term added to a subject access point to specify the period in time in relation to occupation which the subject access point represents. |
| `$2` | Source | Non rép. | Optionnel | ===PAGE 636=== |
| `$3` | Authority Record Identifier or Standard Number | Rép. | Optionnel |  |
| `$8` | Materials Specified | Non rép. | Optionnel |  |

### Zone 632 — FUNCTION

**Répétabilité :** Rép. &nbsp;|&nbsp; **Occurrence :** Optionnel

| Sous-zone | Nom | Rép. | Occurrence | Description |
|-----------|-----|------|------------|-------------|
| `$a` | Function | Non rép. | Optionnel | Term specifying the function, activity, organizational processes that generated the materials described in the record. |
| `$j` | Form Subdivision | Rép. | Optionnel | A term added to the subject access point to further specify the kind(s) or genre(s) of material. Agencies not using this subdivision should use $x instead. |
| `$x` | Topical Subdivision | Rép. | Optionnel | A term added to a subject access point to further specify the function the subject access point represents. |
| `$y` | Geographical Subdivision | Rép. | Optionnel | A term added to a subject access point to specify a place in relation to function which the subject access point represents. |
| `$z` | Chronological Subdivision | Rép. | Optionnel | A term added to a subject access point to specify the period in time in relation to function which the subject access point represents. |
| `$2` | Source | Non rép. | Optionnel | An identification in coded form of the system or thesaurus from which the term in subfield $a is derived. It is recommended that subfield $2 always be present in each occurrence of the field. For a... |
| `$3` | Authority Record Identifier or Standard Number | Rép. | Optionnel | The identifier for the authority record for the access point. This subfield is for use with UNIMARC/Authorities format. ===PAGE 638=== |
| `$8` | Materials Specified | Non rép. | Optionnel |  |

### Zone 660 — GEOGRAPHIC AREA CODE

**Répétabilité :** Rép. &nbsp;|&nbsp; **Occurrence :** Optionnel

| Sous-zone | Nom | Rép. | Occurrence | Description |
|-----------|-----|------|------------|-------------|
| `$a` | Code | Non rép. | Optionnel | Geographic area code. 7 characters. Not repeatable. For codes use MARC Code List for Geographic Areas (http://www.loc.gov/marc/geoareas/gacshome.html). |

### Zone 661 — TIME PERIOD CODE

**Répétabilité :** Rép. &nbsp;|&nbsp; **Occurrence :** Optionnel

| Sous-zone | Nom | Rép. | Occurrence | Description |
|-----------|-----|------|------------|-------------|
| `$a` | Time Period Code | Non rép. | Optionnel | It consists of four alphanumeric characters. |

### Zone 670 — PRECIS

**Répétabilité :** Rép. &nbsp;|&nbsp; **Occurrence :** Optionnel

| Sous-zone | Nom | Rép. | Occurrence | Description |
|-----------|-----|------|------------|-------------|
| `$b` | Subject Indicator Number | Non rép. | Optionnel | The Subject Indicator Number (SIN) is a fixed-length number ending in a modulus 11 check digit. It identifies uniquely the address of the subject packet on a machine readable file. |
| `$c` | String | Non rép. | Optionnel | The string is a sequence of subject index terms in which each term is preceded by a code which determines how it should appear in entries generated by the computer in printed indexes, COM output, c... |
| `$e` | Reference Indicator Number | Rép. | Optionnel | A Reference Indication Number (RIN) is a fixed-length number ending in a modulus 11 check digit which identifies the address of a term in the machine-held thesaurus used as the source of See and Se... |
| `$z` | Language of Terms | Non rép. | Optionnel | A three character language code giving the language of the terms used in the string. For codes use ISO 639-2. |

### Zone 675 — UNIVERSAL DECIMAL CLASSIFICATION (UDC)

**Répétabilité :** — &nbsp;|&nbsp; **Occurrence :** —

| Sous-zone | Nom | Rép. | Occurrence | Description |
|-----------|-----|------|------------|-------------|
| `$a` | Number | Non rép. | Optionnel | The class number as taken from the UDC schedules. |
| `$v` | Edition | Non rép. | Optionnel | An identification of the edition from which the number in subfield $a is taken. |
| `$z` | Language of Edition | Non rép. | Optionnel | The language in coded form of the edition from which the number in subfield $a is taken. For codes use ISO 639-2. |
| `$3` | Classification Record Number | Non rép. | Optionnel | The identifier for the classification record for the access point. This subfield is for use with UNIMARC/ Classification Format. |

### Zone 676 — DEWEY DECIMAL CLASSIFICATION

**Répétabilité :** Rép. &nbsp;|&nbsp; **Occurrence :** Optionnel

| Sous-zone | Nom | Rép. | Occurrence | Description |
|-----------|-----|------|------------|-------------|
| `$a` | Number | Non rép. | Optionnel | The number as taken from the Dewey Decimal Classification schedules. Prime marks are indicated by /. |
| `$v` | Edition | Non rép. | Optionnel | The number of the edition used (EX 1-7). An “a” is added to the number to indicate abridged edition (EX 6). |
| `$z` | Language of Edition | Non rép. | Optionnel | The language in coded form of the edition from which the number in subfield $a is taken (EX 7) For codes use ISO 639-2. |
| `$3` | Classification Record Number | Non rép. | Optionnel | The identifier for the classification record for the access point. This subfield is for use with UNIMARC/ Classification Format. |

### Zone 680 — LIBRARY OF CONGRESS CLASSIFICATION

**Répétabilité :** Rép. &nbsp;|&nbsp; **Occurrence :** Optionnel

| Sous-zone | Nom | Rép. | Occurrence | Description |
|-----------|-----|------|------------|-------------|
| `$a` | Class Number | Non rép. | Optionnel | The class number taken from the Library of Congress classification schedules. |
| `$b` | Book Number | Non rép. | Optionnel | The individual book number applied by the cataloguing agency. |
| `$3` | Classification Record Number | Non rép. | Optionnel | The identifier for the classification record for the access point. This subfield is for use with UNIMARC/ Classification Format. |

### Zone 686 — OTHER CLASS NUMBERS

**Répétabilité :** — &nbsp;|&nbsp; **Occurrence :** —

| Sous-zone | Nom | Rép. | Occurrence | Description |
|-----------|-----|------|------------|-------------|
| `$a` | Class Number | Non rép. | Optionnel | The class number taken from the classification scheme. |
| `$b` | Book Number | Rép. | Optionnel | The book number applied by the assigning agency. |
| `$c` | Classification Subdivision | Rép. | Optionnel | A subdivision of the class number taken from the classification scheme. |
| `$2` | Source | Non rép. | Optionnel | A code for the classification scheme used in formulating the number. For a list of subject sources, see Appendix A. |
| `$3` | Classification Record Number | Non rép. | Optionnel | The identifier for the classification record for the acess point. This subfield is for use with UNIMARC/ Classification Format. Not repeatable. Examples EX 1 686 ##$aW1$bRE359$2usnlm A U.S. Nationa... |

---

## BLOC 7 — RESPONSABILITÉ

### Zone 700 — PERSONAL NAME – PRIMARY RESPONSIBILITY

**Répétabilité :** — &nbsp;|&nbsp; **Occurrence :** —

| Sous-zone | Nom | Rép. | Occurrence | Description |
|-----------|-----|------|------------|-------------|
| `$a` | Entry Element | Non rép. | Oblig. si appl. | The portion of the name used as the entry element in the heading; that part of the name by which the name is entered in ordered lists. Mandatory when the field is present. |
| `$b` | Part of Name Other than Entry Element | Non rép. | Optionnel | The remainder of the name, used when the entry element is a surname or family name (EX 1-11, 14). It contains forenames and other given names. The form of name indicator should be set to 1 when thi... |
| `$c` | Additions to Names Other than Dates | Non rép. | Optionnel | ===PAGE 654=== |
| `$d` | Roman Numerals | Non rép. | Optionnel |  |
| `$f` | Dates | Non rép. | Optionnel |  |
| `$g` | Expansion of Initials of Forename | Non rép. | Optionnel |  |
| `$k` | Attribution Qualifier | Rép. | Optionnel |  |
| `$o` | International Standard Identifier for the Name | Rép. | Optionnel |  |
| `$p` | Affiliation/Address | Non rép. | Optionnel |  |
| `$2` | Source | Non rép. | Optionnel |  |
| `$3` | Authority Record Identifier or Standard Number | Non rép. | Optionnel |  |
| `$4` | Relator Code | Rép. | Optionnel |  |
| `$8` | Materials Specified | Rép. | Optionnel |  |

### Zone 701 — PERSONAL NAME – ALTERNATIVE RESPONSIBILITY

**Répétabilité :** — &nbsp;|&nbsp; **Occurrence :** —

| Sous-zone | Nom | Rép. | Occurrence | Description |
|-----------|-----|------|------------|-------------|
| `$a` | Entry Element | Non rép. | Oblig. si appl. | The portion of the name used as the entry element in the heading; that part of the name by which the name is entered in ordered lists. Mandatory when the field is present. |
| `$b` | Part of Name Other than Entry Element | Non rép. | Optionnel | The remainder of the name, used when the entry element is a surname or family name (see EX 1-11, 14 in the field 700). It contains forenames and other given names. The form of name indicator should... |
| `$c` | Additions to Names Other than Dates | Non rép. | Optionnel | Any additions to names (other than dates) which do not form an integral part of the name itself including titles, epithets or indications of office (see EX 6, 7, 8, 12, 15, 16 in the field 700). Re... |
| `$d` | Roman Numerals | Non rép. | Optionnel | ===PAGE 660=== |
| `$f` | Dates | Non rép. | Optionnel |  |
| `$g` | Expansion of Initials of Forename | Non rép. | Optionnel |  |
| `$k` | Attribution Qualifier | Rép. | Optionnel |  |
| `$o` | International Standard Identifier for the Name | Rép. | Optionnel |  |
| `$p` | Affiliation/Address | Non rép. | Optionnel |  |
| `$2` | Source | Non rép. | Optionnel |  |
| `$3` | Authority Record Identifier or Standard Number | Non rép. | Optionnel |  |
| `$4` | Relator Code | Rép. | Optionnel |  |
| `$8` | Materials Specified | Rép. | Optionnel |  |

### Zone 702 — PERSONAL NAME – SECONDARY RESPONSIBILITY

**Répétabilité :** — &nbsp;|&nbsp; **Occurrence :** —

| Sous-zone | Nom | Rép. | Occurrence | Description |
|-----------|-----|------|------------|-------------|
| `$a` | Entry Element | Non rép. | Oblig. si appl. | The portion of the name used as the entry element in the heading; that part of the name by which the name is entered in ordered lists. Mandatory when the field is present. |
| `$b` | Part of Name Other than Entry Element | Non rép. | Optionnel | The remainder of the name, used when the entry element is a surname or family name (see EX 1-11, 14 in the field 700). It contains forenames and other given names. The form of name indicator should... |
| `$c` | Additions to Names Other than Dates | Non rép. | Optionnel | Any additions to names (other than dates) which do not form an integral part of the name itself including titles, epithets or indications of office (see EX 6, 7, 8, 12, 15, 16 in the field 700). Re... |
| `$d` | Roman Numerals | Non rép. | Optionnel |  |
| `$f` | Dates | Non rép. | Optionnel |  |
| `$g` | Expansion of Initials of Forename | Non rép. | Optionnel |  |
| `$k` | Attribution Qualifier | Rép. | Optionnel |  |
| `$o` | International Standard Identifier for the Name | Rép. | Optionnel |  |
| `$p` | Affiliation/Address | Non rép. | Optionnel |  |
| `$r` | Part or Role Played | Rép. | Optionnel |  |
| `$2` | Source | Non rép. | Optionnel |  |
| `$3` | Authority Record Identifier or Standard Number | Non rép. | Optionnel |  |
| `$4` | Relator Code | Rép. | Optionnel |  |
| `$8` | Materials Specified | Rép. | Optionnel |  |
| `$5` | Institution to which the Field Applies | Non rép. | Optionnel |  |
| `$6` | Interfield Linking Data | Rép. | Optionnel |  |

### Zone 703 — PERSONAL NAME – PROVENANCE OR OWNERSHIP

**Répétabilité :** — &nbsp;|&nbsp; **Occurrence :** —

| Sous-zone | Nom | Rép. | Occurrence | Description |
|-----------|-----|------|------------|-------------|
| `$a` | Entry Element | Non rép. | Oblig. si appl. | The portion of the name used as the entry element in the heading; that part of the name by which the name is entered in ordered lists. Mandatory when the field is present. |
| `$b` | Part of Name Other than Entry Element | Non rép. | Optionnel | The remainder of the name, used when the entry element is a surname or family name (see EX 1-11, 14 in the field 700). It contains forenames and other given names. The form of name indicator should... |
| `$c` | Additions to Names Other than Dates | Non rép. | Optionnel | Any additions to names (other than dates) which do not form an integral part of the name itself including titles, epithets or indications of office (see EX 6, 7, 8, 12, 15, 16 in the field 700). Re... |
| `$d` | Roman Numerals | Non rép. | Optionnel | Roman numerals associated with names of certain popes, royalty and ecclesiastics (see EX 12 in the field 700). If an epithet (or a further forename) is associated with the numeration, this too shou... |
| `$f` | Dates | Non rép. | Optionnel |  |
| `$g` | Expansion of Initials of Forename | Non rép. | Optionnel |  |
| `$k` | Attribution Qualifier | Rép. | Optionnel |  |
| `$o` | International Standard Identifier for the Name | Rép. | Optionnel |  |
| `$p` | Affiliation/Address | Non rép. | Optionnel |  |
| `$2` | Source | Non rép. | Optionnel |  |
| `$3` | Authority Record Identifier or Standard Number | Non rép. | Optionnel |  |
| `$4` | Relator Code | Rép. | Optionnel |  |
| `$8` | Materials Specified | Rép. | Optionnel |  |
| `$5` | Institution to which the Field Applies | Non rép. | Optionnel |  |

### Zone 710 — CORPORATE BODY NAME – PRIMARY RESPONSIBILITY

**Répétabilité :** — &nbsp;|&nbsp; **Occurrence :** —

| Sous-zone | Nom | Rép. | Occurrence | Description |
|-----------|-----|------|------------|-------------|
| `$a` | Entry Element | Non rép. | Oblig. si appl. |  |
| `$b` | Subdivision | Rép. | Optionnel |  |
| `$c` | Addition to Name or Qualifier | Rép. | Optionnel |  |
| `$e` | Location of Meeting | Non rép. | Optionnel |  |
| `$f` | Date of Meeting | Non rép. | Optionnel |  |
| `$g` | Inverted Element | Non rép. | Optionnel |  |
| `$o` | International Standard Identifier for the Name | Rép. | Optionnel |  |
| `$p` | Affiliation/Address | Non rép. | Optionnel |  |
| `$2` | Source | Non rép. | Optionnel |  |
| `$3` | Authority Record Identifier or Standard Number | Non rép. | Optionnel |  |
| `$4` | Relator Code | Rép. | Optionnel |  |
| `$8` | Materials Specified | Rép. | Optionnel |  |

### Zone 711 — CORPORATE BODY NAME – ALTERNATIVE RESPONSIBILITY

**Répétabilité :** — &nbsp;|&nbsp; **Occurrence :** —

| Sous-zone | Nom | Rép. | Occurrence | Description |
|-----------|-----|------|------------|-------------|
| `$a` | Entry Element | Non rép. | Oblig. si appl. |  |
| `$b` | Subdivision | Rép. | Optionnel |  |
| `$c` | Addition to Name or Qualifier | Rép. | Optionnel |  |
| `$e` | Location of Meeting | Non rép. | Optionnel |  |
| `$f` | Date of Meeting | Non rép. | Optionnel |  |
| `$g` | Inverted Element | Non rép. | Optionnel |  |
| `$o` | International Standard Identifier for the Name | Rép. | Optionnel |  |
| `$p` | Affiliation/Address | Non rép. | Optionnel |  |
| `$2` | Source | Non rép. | Optionnel |  |
| `$3` | Authority Record Identifier or Standard Number | Non rép. | Optionnel |  |
| `$4` | Relator Code | Rép. | Optionnel |  |
| `$8` | Materials Specified | Rép. | Optionnel |  |

### Zone 712 — CORPORATE BODY NAME – SECONDARY RESPONSIBILITY

**Répétabilité :** — &nbsp;|&nbsp; **Occurrence :** —

| Sous-zone | Nom | Rép. | Occurrence | Description |
|-----------|-----|------|------------|-------------|
| `$a` | Entry Element | Non rép. | Oblig. si appl. |  |
| `$b` | Subdivision | Rép. | Optionnel |  |
| `$c` | Addition to Name or Qualifier | Rép. | Optionnel |  |
| `$e` | Location of Meeting | Non rép. | Optionnel |  |
| `$f` | Date of Meeting | Non rép. | Optionnel |  |
| `$g` | Inverted Element | Non rép. | Optionnel |  |
| `$o` | International Standard Identifier for the Name | Rép. | Optionnel |  |
| `$p` | Affiliation/Address | Non rép. | Optionnel |  |
| `$r` | Part or Role Played | Rép. | Optionnel |  |
| `$2` | Source | Non rép. | Optionnel |  |
| `$3` | Authority Record Identifier or Standard Number | Non rép. | Optionnel |  |
| `$4` | Relator Code | Rép. | Optionnel |  |
| `$5` | Institution to which the Field Applies | Non rép. | Optionnel |  |
| `$8` | Materials Specified | Rép. | Optionnel |  |

### Zone 713 — CORPORATE BODY NAME – PROVENANCE OR OWNERSHIP

**Répétabilité :** — &nbsp;|&nbsp; **Occurrence :** —

| Sous-zone | Nom | Rép. | Occurrence | Description |
|-----------|-----|------|------------|-------------|
| `$a` | Entry Element | Non rép. | Oblig. si appl. |  |
| `$b` | Subdivision | Rép. | Optionnel |  |
| `$c` | Addition to Name or Qualifier | Rép. | Optionnel |  |
| `$e` | Location of Meeting | Non rép. | Optionnel |  |
| `$f` | Date of Meeting | Non rép. | Optionnel |  |
| `$g` | Inverted Element | Non rép. | Optionnel |  |
| `$o` | International Standard Identifier for the Name | Rép. | Optionnel |  |
| `$p` | Affiliation/Address | Non rép. | Optionnel |  |
| `$2` | Source | Non rép. | Optionnel |  |
| `$3` | Authority Record Identifier or Standard Number | Non rép. | Optionnel |  |
| `$4` | Relator Code | Rép. | Optionnel |  |
| `$5` | Institution to which the Field Applies | Non rép. | Optionnel |  |
| `$8` | Materials Specified | Rép. | Optionnel |  |

### Zone 716 — TRADEMARK

**Répétabilité :** Rép. &nbsp;|&nbsp; **Occurrence :** Optionnel

| Sous-zone | Nom | Rép. | Occurrence | Description |
|-----------|-----|------|------------|-------------|
| `$a` | Data Element | Non rép. | Optionnel | The trademark name access point form. |
| `$f` | Dates | Non rép. | Optionnel | The dates between which a particular trademark is in use, when they are required as part of the access point, for example, as qualifiers. |
| `$c` | Qualification | Rép. | Optionnel | Any addition to the name of the trademark added by the cataloguer, other than dates. |
| `$3` | Authority Record Identifier or Standard Number | Non rép. | Optionnel | The identifier for the authority record for the access point. This subfield is for use with UNIMARC/Authorities format. Not repeatable. Note on Field Contents Form of Name The form of name which ap... |

### Zone 720 — FAMILY NAME – PRIMARY RESPONSIBILITY

**Répétabilité :** Non rép. &nbsp;|&nbsp; **Occurrence :** Optionnel

| Sous-zone | Nom | Rép. | Occurrence | Description |
|-----------|-----|------|------------|-------------|
| `$a` | Entry Element | Non rép. | Oblig. si appl. | The family name in access point form. This subfield is must be present if the field is present. |
| `$c` | Type of Family | Non rép. | Optionnel | A categorization or generic descriptor for the type of family. Includes categorizations such as clan, dynasty, family unit, patriarchy, matriarchy, etc. |
| `$d` | Places Associated with the Family | Rép. | Optionnel | Information pertaining to places where the family resides or resided or had some connection. |
| `$f` | Dates | Non rép. | Optionnel | The dates of a family when they are required as part of the access point. Not repeatable. (EX 3). |
| `$o` | International Standard Identifier for the Name | Rép. | Optionnel | The ISNI or another international identifier assigned to the name recorded in the field. The first four character positions contain an alphabetic code specifying the nature of the identifier; in th... |
| `$2` | Source | Non rép. | Optionnel |  |
| `$3` | Authority Record Identifier or Standard Number | Non rép. | Optionnel |  |
| `$4` | Relator Code | Rép. | Optionnel |  |
| `$8` | Materials Specified | Rép. | Optionnel |  |

### Zone 721 — FAMILY NAME – ALTERNATIVE RESPONSIBILITY

**Répétabilité :** — &nbsp;|&nbsp; **Occurrence :** —

| Sous-zone | Nom | Rép. | Occurrence | Description |
|-----------|-----|------|------------|-------------|
| `$a` | Entry Element | Non rép. | Oblig. si appl. | The family name in access point form. This subfield is must be present if the field is present. |
| `$c` | Type of Family | Non rép. | Optionnel | A categorization or generic descriptor for the type of family. Includes categorizations such as clan, dynasty, family unit, patriarchy, matriarchy, etc. |
| `$d` | Places Associated with the Family | Rép. | Optionnel | Information pertaining to places where the family resides or resided or had some connection. |
| `$f` | Dates | Non rép. | Optionnel | The dates of a family when they are required as part of the access point. Not repeatable. (See EX 3 in the field 720). |
| `$o` | International Standard Identifier for the Name | Rép. | Optionnel | The ISNI or another international identifier assigned to the name recorded in the field. The first four character positions contain an alphabetic code specifying the nature of the identifier; in th... |
| `$2` | Source | Non rép. | Optionnel | ===PAGE 702=== |
| `$3` | Authority Record Identifier or Standard Number | Non rép. | Optionnel |  |
| `$4` | Relator Code | Rép. | Optionnel |  |
| `$8` | Materials Specified | Rép. | Optionnel |  |

### Zone 722 — FAMILY NAME – SECONDARY RESPONSIBILITY

**Répétabilité :** — &nbsp;|&nbsp; **Occurrence :** —

| Sous-zone | Nom | Rép. | Occurrence | Description |
|-----------|-----|------|------------|-------------|
| `$a` | Entry Element | Non rép. | Oblig. si appl. | The family name in access point form. This subfield is must be present if the field is present. |
| `$c` | Type of Family | Non rép. | Optionnel | A categorization or generic descriptor for the type of family. Includes categorizations such as clan, dynasty, family unit, patriarchy, matriarchy, etc. |
| `$d` | Places Associated with the Family | Rép. | Optionnel | Information pertaining to places where the family resides or resided or had some connection. |
| `$f` | Dates | Non rép. | Optionnel | The dates of a family when they are required as part of the access point. Not repeatable. (See EX 3 in the field 720). |
| `$o` | International Standard Identifier for the Name | Rép. | Optionnel | The ISNI or another international identifier assigned to the name recorded in the field. The first four character positions contain an alphabetic code specifying the nature of the identifier; in th... |
| `$r` | Part or Role Played | Rép. | Optionnel | The name of a role interpreted, played or sung by the corporate body designed by the access point. This subfield is especially used in records for audiovisual material (motion pictures, sound or vi... |
| `$3` | Authority Record Identifier or Standard Number | Non rép. | Optionnel |  |
| `$4` | Relator Code | Rép. | Optionnel |  |
| `$5` | Institution to which the Field Applies | Non rép. | Optionnel |  |
| `$8` | Materials Specified | Rép. | Optionnel |  |

### Zone 723 — FAMILY NAME – PROVENANCE OR OWNERSHIP

**Répétabilité :** — &nbsp;|&nbsp; **Occurrence :** —

| Sous-zone | Nom | Rép. | Occurrence | Description |
|-----------|-----|------|------------|-------------|
| `$a` | Entry Element | Non rép. | Oblig. si appl. | The family name in access point form. This subfield is must be present if the field is present. |
| `$c` | Type of Family | Non rép. | Optionnel | A categorization or generic descriptor for the type of family. Includes categorizations such as clan, dynasty, family unit, patriarchy, matriarchy, etc. |
| `$d` | Places Associated with the Family | Rép. | Optionnel | Information pertaining to places where the family resides or resided or had some connection. |
| `$f` | Dates | Non rép. | Optionnel | The dates of a family when they are required as part of the access point. Not repeatable. (See EX 3 in the field 720). |
| `$o` | International Standard Identifier for the Name | Rép. | Optionnel | The ISNI or another international identifier assigned to the name recorded in the field. The first four character positions contain an alphabetic code specifying the nature of the identifier; in th... |
| `$2` | Source | Non rép. | Optionnel | An identification in coded form for the relator code schema from which the code in $4 is derived, when the code is not from UNIMARC Relator Codes. |
| `$3` | Authority Record Identifier or Standard Number | Non rép. | Optionnel | ===PAGE 707=== |
| `$4` | Relator Code | Rép. | Optionnel |  |
| `$5` | Institution to which the Field Applies | Non rép. | Optionnel |  |
| `$8` | Materials Specified | Rép. | Optionnel |  |

### Zone 730 — NAME – ENTITY RESPONSIBLE

**Répétabilité :** Rép. &nbsp;|&nbsp; **Occurrence :** Optionnel

| Sous-zone | Nom | Rép. | Occurrence | Description |
|-----------|-----|------|------------|-------------|
| `$a` | Entry Element | Non rép. | Optionnel | The name used as the entry element in the access point. |
| `$4` | Relator Code | Rép. | Optionnel | The code used to designate the relationship between the entity named in the field and the bibliographic resource to which the record refers. The list of codes is to be found in Appendix B. Repeatab... |

---

## BLOC 8 — USAGE INTERNATIONAL

### Zone 801 — ORIGINATING SOURCE

**Répétabilité :** Rép. &nbsp;|&nbsp; **Occurrence :** Oblig. si appl.

| Sous-zone | Nom | Rép. | Occurrence | Description |
|-----------|-----|------|------------|-------------|
| `$a` | Country | Non rép. | Oblig. si appl. | The country of the issuing agency in two-character coded form. For country codes use ISO 3166-1. |
| `$b` | Agency | Non rép. | Oblig. si appl. | It is recommended that the agency be identified according to the International Standard Identifier for Libraries and Related Organizations (ISIL) ISO 15511. In this case, the whole ISIL, including ... |
| `$c` | Date of Transaction | Non rép. | Oblig. si appl. |  |
| `$g` | Cataloguing Rules (Descriptive Conventions) | Rép. | Oblig. si appl. |  |
| `$h` | Original Record identifier | Non rép. | Optionnel |  |
| `$2` | Source | Non rép. | Optionnel |  |

### Zone 802 — ISSN CENTRE

**Répétabilité :** Non rép. &nbsp;|&nbsp; **Occurrence :** Optionnel

| Sous-zone | Nom | Rép. | Occurrence | Description |
|-----------|-----|------|------------|-------------|
| `$a` | ISSN Centre Code | Non rép. | Optionnel | Each centre has been assigned a code by the ISSN International Centre. The ISSN Centre code consists of one or two alphanumeric characters. For codes, see List of Country and ISSN Centre Codes. |

### Zone 830 — GENERAL CATALOGUER’S NOTE

**Répétabilité :** Rép. &nbsp;|&nbsp; **Occurrence :** Optionnel

| Sous-zone | Nom | Rép. | Occurrence | Description |
|-----------|-----|------|------------|-------------|
| `$a` | Text of Note | Non rép. | Optionnel |  |

### Zone 850 — HOLDING INSTITUTION

**Répétabilité :** Rép. &nbsp;|&nbsp; **Occurrence :** Optionnel

| Sous-zone | Nom | Rép. | Occurrence | Description |
|-----------|-----|------|------------|-------------|
| `$a` | Code of Institution | Rép. | Optionnel |  |

### Zone 852 — LOCATION AND CALL NUMBER

**Répétabilité :** Rép. &nbsp;|&nbsp; **Occurrence :** Optionnel

| Sous-zone | Nom | Rép. | Occurrence | Description |
|-----------|-----|------|------------|-------------|
| `$a` | Institution Identifier | Non rép. | Oblig. si appl. |  |
| `$b` | Sub-Location Identifier | Rép. | Optionnel |  |
| `$c` | Address | Non rép. | Optionnel |  |
| `$d` | Coded Location Qualifier | Non rép. | Optionnel |  |
| `$g` | Call Number Prefix | Non rép. | Optionnel |  |
| `$j` | Call Number | Non rép. | Optionnel |  |
| `$k` | Shelving Form of Title, Author, Author/Title | Non rép. | Optionnel |  |
| `$l` | Call Number Suffix | Non rép. | Optionnel |  |
| `$m` | Item Identifier | Non rép. | Optionnel |  |
| `$n` | Copy Identifier | Non rép. | Optionnel |  |
| `$p` | Country | Non rép. | Optionnel |  |
| `$t` | Copy Number | Non rép. | Optionnel |  |
| `$x` | Non-public Note | Rép. | Optionnel |  |
| `$y` | Public Note | Rép. | Optionnel |  |
| `$2` | Source | Non rép. | Optionnel |  |

### Zone 856 — ELECTRONIC LOCATION AND ACCESS

**Répétabilité :** Rép. &nbsp;|&nbsp; **Occurrence :** Oblig. si appl.

| Sous-zone | Nom | Rép. | Occurrence | Description |
|-----------|-----|------|------------|-------------|
| `$a` | Host Name | Rép. | Optionnel |  |
| `$b` | Access Number | Rép. | Optionnel |  |
| `$c` | Compression Information | Rép. | Optionnel |  |
| `$d` | Path | Rép. | Optionnel |  |
| `$e` | Date and Hour of Consultation and Access | Non rép. | Optionnel |  |
| `$f` | Electronic Name | Rép. | Optionnel |  |
| `$h` | Processor of Request | Non rép. | Optionnel |  |
| `$i` | Instruction | Rép. | Optionnel |  |
| `$j` | Bits per Second | Non rép. | Optionnel |  |
| `$k` | Password | Non rép. | Optionnel |  |
| `$l` | Logon/Login | Non rép. | Optionnel |  |
| `$m` | Contact for Access Assistance | Rép. | Optionnel |  |
| `$n` | Name of Location of Host in Subfield $a | Non rép. | Optionnel |  |
| `$o` | Operating System | Non rép. | Optionnel |  |
| `$p` | Port | Non rép. | Optionnel |  |
| `$q` | Electronic Format Type | Rép. | Optionnel |  |
| `$r` | Settings | Non rép. | Optionnel |  |
| `$s` | File Size | Rép. | Optionnel |  |
| `$t` | Terminal Emulation | Rép. | Optionnel |  |
| `$u` | Uniform Resource Identifier (URI) | Non rép. | Optionnel |  |
| `$v` | Hours Access Method Available | Rép. | Optionnel |  |
| `$w` | Record Identifier | Rép. | Optionnel |  |
| `$x` | Non-Public Note | Rép. | Optionnel |  |
| `$y` | Access Method | Non rép. | Optionnel |  |
| `$z` | Public Note | Rép. | Optionnel |  |
| `$2` | Link Text | Rép. | Optionnel |  |

### Zone 857 — ELECTRONIC ARCHIVE LOCATION AND ACCESS

**Répétabilité :** — &nbsp;|&nbsp; **Occurrence :** —

| Sous-zone | Nom | Rép. | Occurrence | Description |
|-----------|-----|------|------------|-------------|
| `$a` | Name of Archiving Agency | Non rép. | Optionnel |  |
| `$c` | Name of Web Archive or Digital Archive Repository | Non rép. | Optionnel |  |
| `$d` | Date Range of Archived Material | Non rép. | Optionnel |  |
| `$f` | Archive Completeness | Non rép. | Optionnel |  |
| `$g` | Persistent Identifier | Rép. | Oblig. si appl. |  |
| `$h` | Non-Functioning Uniform Resource Identifier | Rép. | Oblig. si appl. |  |
| `$l` | Standardized Information Governing Access | Rép. | Optionnel |  |
| `$m` | Contact for Access Assistance | Rép. | Optionnel |  |
| `$n` | Terms Governing Access | Rép. | Optionnel |  |
| `$o` | Access Status | Non rép. | Optionnel |  |
| `$p` | Access Method | Non rép. | Optionnel |  |
| `$q` | Electronic Format Type | Rép. | Optionnel |  |
| `$s` | File Size | Rép. | Optionnel |  |
| `$t` | Terms Governing Use and Reproduction | Rép. | Optionnel |  |
| `$u` | Uniform Resource Identifier (URI) | Rép. | Oblig. si appl. |  |
| `$x` | Nonpublic Note | Rép. | Optionnel |  |
| `$y` | Link Text | Rép. | Optionnel |  |
| `$z` | Public Note | Rép. | Optionnel |  |
| `$2` | Source | Non rép. | Optionnel |  |
| `$5` | Institution to which the Field Applies | Non rép. | Optionnel |  |
| `$6` | Interfield Linking Data | Rép. | Optionnel |  |
| `$8` | Materials Specified | Rép. | Optionnel |  |

### Zone 886 — DATA NOT CONVERTED FROM SOURCE FORMAT

**Répétabilité :** — &nbsp;|&nbsp; **Occurrence :** —

| Sous-zone | Nom | Rép. | Occurrence | Description |
|-----------|-----|------|------------|-------------|
| `$a` | Tag of the Source Format Field | Rép. | Optionnel | This subfield will not be present if Indicator 1 has the value 0. This subfield can be used only once with the value Tag of the Source Format Field. All other $a subfields in this field have the va... |