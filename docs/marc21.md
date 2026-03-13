# Structure MARC21 — Référence complète des zones et sous-zones

> **Source :** *MARC21 Code List for Field Identification*, 24 septembre 2001.
>
> **Répétabilité :** R = Répétable · NR = Non répétable
>
> Pour les indicateurs et le caractère obligatoire/optionnel de chaque zone, se référer au *MARC21 Concise Format*.

## Table des matières

- [BLOC 00X — CHAMPS DE CONTRÔLE](#00x)
- [BLOC 01X–09X — NUMÉROS ET CODES](#01x-09x)
- [BLOC 1XX — ENTRÉE PRINCIPALE](#1xx)
- [BLOC 2XX — TITRE, ÉDITION, PUBLICATION](#2xx)
- [BLOC 3XX — DESCRIPTION PHYSIQUE](#3xx)
- [BLOC 4XX — MENTIONS DE COLLECTION](#4xx)
- [BLOC 5XX — NOTES](#5xx)
- [BLOC 6XX — ACCÈS SUJET](#6xx)
- [BLOC 7XX — ENTRÉES SECONDAIRES (NOMS & TITRES)](#7xx)
- [BLOC 8XX — ENTRÉES SECONDAIRES SÉRIE & LOCALISATION](#8xx)

---

## BLOC 00X — CHAMPS DE CONTRÔLE

### Zone 001 — Control Number

**Répétabilité :** Non rép.

*Champ de contrôle — données en position fixe, sans sous-zones.*

### Zone 003 — Control Number Identifier

**Répétabilité :** Non rép.

*Champ de contrôle — données en position fixe, sans sous-zones.*

### Zone 005 — Date & Time of Latest Transaction

**Répétabilité :** Non rép.

*Champ de contrôle — données en position fixe, sans sous-zones.*

### Zone 006 — Fixed Length Data Elements – AMC

**Répétabilité :** Rép.

*Champ de contrôle — données en position fixe, sans sous-zones.*

### Zone 007 — Physical Description Fixed Field

**Répétabilité :** Rép.

*Champ de contrôle — données en position fixe, sans sous-zones.*

### Zone 008 — Fixed Length Data Elements – GI

**Répétabilité :** Non rép.

*Champ de contrôle — données en position fixe, sans sous-zones.*

---

## BLOC 01X–09X — NUMÉROS ET CODES

### Zone 013 — Patent Control Number

**Répétabilité :** Rép.

| Sous-zone | Nom | Rép. |
|-----------|-----|------|
| `$a` | Number | Non rép. |
| `$b` | Country | Non rép. |
| `$c` | Type of Number | Non rép. |
| `$d` | Date | Rép. |
| `$e` | Status | Rép. |
| `$f` | Party to document | Rép. |

### Zone 015 — National Bibliography Number

**Répétabilité :** Non rép.

| Sous-zone | Nom | Rép. |
|-----------|-----|------|
| `$a` | National bibliography number | Rép. |

### Zone 016 — National Bibliography Agency Control Number

**Répétabilité :** Rép.

| Sous-zone | Nom | Rép. |
|-----------|-----|------|
| `$a` | Record control number | Non rép. |
| `$z` | Canceled or invalid control number | Rép. |
| `$2` | Source | Non rép. |

### Zone 020 — ISBN

**Répétabilité :** Rép.

| Sous-zone | Nom | Rép. |
|-----------|-----|------|
| `$a` | ISBN | Non rép. |
| `$c` | Terms of availability | Non rép. |
| `$z` | Canceled/invalid ISBN | Rép. |

### Zone 022 — ISSN

**Répétabilité :** Rép. &nbsp;|&nbsp; **Usage :** Périodiques (serials)

| Sous-zone | Nom | Rép. |
|-----------|-----|------|
| `$a` | ISSN | Non rép. |
| `$y` | Incorrect ISSN | Rép. |
| `$z` | Canceled ISSN | Rép. |

### Zone 027 — Standard Technical Report Number

**Répétabilité :** Rép. &nbsp;|&nbsp; **Usage :** Périodiques (serials)

| Sous-zone | Nom | Rép. |
|-----------|-----|------|
| `$a` | Standard technical report number | Non rép. |
| `$z` | Canceled/invalid number | Rép. |

### Zone 028 — Publisher Number

**Répétabilité :** Rép. &nbsp;|&nbsp; **Usage :** Ressources électroniques

| Sous-zone | Nom | Rép. |
|-----------|-----|------|
| `$a` | Publisher number | Non rép. |
| `$b` | Source | Non rép. |

### Zone 030 — CODEN Designation

**Répétabilité :** Rép. &nbsp;|&nbsp; **Usage :** Périodiques (serials)

| Sous-zone | Nom | Rép. |
|-----------|-----|------|
| `$a` | CODEN | Non rép. |
| `$z` | Canceled/invalid CODEN | Rép. |

### Zone 033 — Date/Time Place of Event

**Répétabilité :** Rép.

| Sous-zone | Nom | Rép. |
|-----------|-----|------|
| `$a` | Formatted date/time | Rép. |
| `$3` | Materials specified | Non rép. |

### Zone 034 — Coded Cartographic Mathematical Data

**Répétabilité :** Rép. &nbsp;|&nbsp; **Usage :** Matériel cartographique

| Sous-zone | Nom | Rép. |
|-----------|-----|------|
| `$a` | Category of scale | Non rép. |
| `$b` | Constant ratio linear horizontal scale | Rép. |
| `$c` | Constant ratio linear vertical scale | Rép. |
| `$d` | Coordinates – westernmost longitude | Non rép. |
| `$e` | Coordinates – easternmost longitude | Non rép. |
| `$f` | Coordinates – northernmost latitude | Non rép. |
| `$g` | Coordinates – southernmost latitude | Non rép. |
| `$h` | Angular scale | Rép. |
| `$j` | Declination – northern limit | Non rép. |
| `$k` | Declination – southern limit | Non rép. |
| `$m` | Right ascension – eastern limit | Non rép. |
| `$n` | Right ascension – western limit | Non rép. |
| `$p` | Equinox | Non rép. |
| `$s` | G-ring latitude | Rép. |
| `$t` | G-ring longitude | Rép. |

### Zone 036 — Original Study Number for Computer Data files

**Répétabilité :** Non rép. &nbsp;|&nbsp; **Usage :** Fichiers informatiques

| Sous-zone | Nom | Rép. |
|-----------|-----|------|
| `$a` | Original study number | Non rép. |
| `$b` | Source (agency assigning number) | Non rép. |

### Zone 040 — Cataloguing Source

**Répétabilité :** Non rép.

| Sous-zone | Nom | Rép. |
|-----------|-----|------|
| `$a` | Original cataloguing agency | Non rép. |
| `$b` | Language of cataloguing | Non rép. |
| `$c` | Transcribing agency | Non rép. |
| `$d` | Modifying agency | Rép. |
| `$e` | Describing conventions | Non rép. |

### Zone 041 — Language Code

**Répétabilité :** Non rép.

| Sous-zone | Nom | Rép. |
|-----------|-----|------|
| `$a` | Language code of text/sound track or separate title | Non rép. |
| `$b` | Language code of summary or abstract/overprinted title or subtitle | Non rép. |
| `$d` | Language code of sung or spoken text | Non rép. |
| `$e` | Language code of librettos | Non rép. |
| `$f` | Language code of table of contents | Non rép. |
| `$g` | Language code of original and/or intermediate translations of text | Rép. |
| `$h` | Language code of original and/or intermediate translations of text | Rép. |

### Zone 043 — Geographic Area Code

**Répétabilité :** Non rép.

| Sous-zone | Nom | Rép. |
|-----------|-----|------|
| `$a` | Geographic area code | Rép. |

### Zone 045 — Time Period of Content

**Répétabilité :** Non rép.

| Sous-zone | Nom | Rép. |
|-----------|-----|------|
| `$a` | Time period code | Rép. |
| `$b` | Formatted 9999 B.C. through C.E. time period | Rép. |
| `$c` | Formatted pre-9999 B.C. time period | Rép. |

### Zone 080 — UDC Number

**Répétabilité :** Rép.

| Sous-zone | Nom | Rép. |
|-----------|-----|------|
| `$a` | UDC number | Non rép. |
| `$b` | Item number | Non rép. |
| `$2` | Edition identifier | Non rép. |

### Zone 082 — DDC Number

**Répétabilité :** Rép.

| Sous-zone | Nom | Rép. |
|-----------|-----|------|
| `$a` | Classification number | Rép. |
| `$b` | Item number | Non rép. |
| `$2` | Edition number | Non rép. |

### Zone 084 — Other Classification Number

**Répétabilité :** Rép.

| Sous-zone | Nom | Rép. |
|-----------|-----|------|
| `$a` | Classification number | Rép. |
| `$b` | Item number | Non rép. |
| `$2` | Source of number | Non rép. |

### Zone 086 — Government Document Call Number

**Répétabilité :** Rép.

| Sous-zone | Nom | Rép. |
|-----------|-----|------|
| `$a` | Classification number | Rép. |
| `$z` | Canceled/invalid classification number | Rép. |
| `$2` | Number source | Non rép. |

### Zone 088 — Report Number

**Répétabilité :** Rép.

| Sous-zone | Nom | Rép. |
|-----------|-----|------|
| `$a` | Report number | Rép. |
| `$z` | Canceled/invalid report number | Rép. |

---

## BLOC 1XX — ENTRÉE PRINCIPALE

### Zone 100 — Main Entry – Personal Name

**Répétabilité :** Non rép.

| Sous-zone | Nom | Rép. |
|-----------|-----|------|
| `$a` | Personal name | Non rép. |
| `$b` | Numeration | Non rép. |
| `$c` | Titles & other words associated with a name | Rép. |
| `$d` | Dates associated with name | Non rép. |
| `$f` | Date of work | Non rép. |
| `$k` | Form subheading | Rép. |
| `$l` | Language of a work | Non rép. |
| `$n` | Number of part/section of a work | Rép. |
| `$p` | Name of part/section of a work | Rép. |
| `$q` | Fuller form of name | Non rép. |
| `$t` | Title of a work | Non rép. |

### Zone 110 — Main Entry – Corporate Name

**Répétabilité :** Non rép.

| Sous-zone | Nom | Rép. |
|-----------|-----|------|
| `$a` | Corporate name or jurisdiction name as entry element | Non rép. |
| `$b` | Subordinate unit | Rép. |
| `$c` | Location of meeting | Non rép. |
| `$d` | Date of meeting or treaty signing | Rép. |
| `$f` | Date of work | Non rép. |
| `$k` | Form subheading | Rép. |
| `$l` | Language of a work | Non rép. |
| `$n` | Number of part/section of a work | Rép. |
| `$p` | Name of part/section of a work | Rép. |
| `$t` | Title of a work | Non rép. |

### Zone 111 — Main Entry – Meeting Name

**Répétabilité :** Non rép.

| Sous-zone | Nom | Rép. |
|-----------|-----|------|
| `$a` | Meeting name or jurisdiction name as entry element | Non rép. |
| `$c` | Location of meeting | Non rép. |
| `$d` | Date of meeting | Non rép. |
| `$e` | Subordinate unit | Rép. |
| `$f` | Date of work | Non rép. |
| `$k` | Form subheading | Rép. |
| `$l` | Language of a work | Non rép. |
| `$n` | Number of part/section of a work | Rép. |
| `$p` | Name of part/section of a work | Rép. |
| `$q` | Name of meeting following jurisdiction name entry element | Non rép. |
| `$t` | Title of a work | Non rép. |

### Zone 130 — Main Entry – Uniform Title

**Répétabilité :** Non rép.

| Sous-zone | Nom | Rép. |
|-----------|-----|------|
| `$a` | Uniform title | Non rép. |
| `$d` | Date of treaty signing | Rép. |
| `$f` | Date of work | Non rép. |
| `$k` | Form subheading | Rép. |
| `$l` | Language of a work | Non rép. |
| `$m` | Medium of performance for music | Rép. |
| `$n` | Number of part/section of a work | Rép. |
| `$o` | Arranged statement for music | Non rép. |
| `$p` | Name of part/section of a work | Rép. |
| `$s` | Version | Non rép. |
| `$t` | Title of a work | Non rép. |

---

## BLOC 2XX — TITRE, ÉDITION, PUBLICATION

### Zone 222 — Key Title

**Répétabilité :** Rép.

| Sous-zone | Nom | Rép. |
|-----------|-----|------|
| `$a` | Key title | Non rép. |
| `$b` | Qualifying information | Non rép. |

### Zone 240 — Uniform Title

**Répétabilité :** Non rép.

| Sous-zone | Nom | Rép. |
|-----------|-----|------|
| `$a` | Uniform title | Non rép. |
| `$d` | Date of treaty signing | Rép. |
| `$f` | Date of work | Non rép. |
| `$k` | Form subheading | Rép. |
| `$l` | Language of a work | Non rép. |
| `$m` | Medium of performance for music | Rép. |
| `$o` | Arranged statement for music | Non rép. |
| `$p` | Name of part/section of a work | Rép. |
| `$s` | Version | Non rép. |

### Zone 245 — Title Statement

**Répétabilité :** Non rép.

| Sous-zone | Nom | Rép. |
|-----------|-----|------|
| `$a` | Title | Non rép. |
| `$b` | Remainder of title | Non rép. |
| `$c` | Statement of responsibility, etc. | Non rép. |
| `$f` | Inclusive dates | Non rép. |
| `$g` | Bulk dates | Non rép. |
| `$h` | Medium | Non rép. |
| `$n` | Number of part/section of a work | Rép. |
| `$p` | Name of part/section of a work | Rép. |

### Zone 246 — Varying Form of Title

**Répétabilité :** Rép.

| Sous-zone | Nom | Rép. |
|-----------|-----|------|
| `$a` | Title proper/short title | Non rép. |
| `$i` | Display text | Non rép. |

### Zone 247 — Former Title or Title Variations

**Répétabilité :** Rép.

| Sous-zone | Nom | Rép. |
|-----------|-----|------|
| `$a` | Title proper/short title | Non rép. |
| `$b` | Remainder of title | Non rép. |
| `$f` | Date or sequential designation | Non rép. |
| `$h` | Medium | Non rép. |
| `$n` | Number of part/section of a work | Rép. |
| `$p` | Name of part/section of a work | Rép. |
| `$x` | ISSN | Non rép. |

### Zone 250 — Edition Statement

**Répétabilité :** Non rép.

| Sous-zone | Nom | Rép. |
|-----------|-----|------|
| `$a` | Edition statement | Non rép. |
| `$b` | Remainder of edition statement | Non rép. |

### Zone 254 — Musical Presentation Statement

**Répétabilité :** Non rép.

| Sous-zone | Nom | Rép. |
|-----------|-----|------|
| `$a` | Musical presentation statement | Non rép. |

### Zone 255 — Cartographic Mathematical Data

**Répétabilité :** Rép. &nbsp;|&nbsp; **Usage :** Cartes

| Sous-zone | Nom | Rép. |
|-----------|-----|------|
| `$a` | Statement of scale | Non rép. |
| `$b` | Statement of projection | Non rép. |
| `$c` | Statement of coordinates | Non rép. |
| `$d` | Statement of zone | Non rép. |
| `$e` | Statement of equinox | Non rép. |
| `$f` | Outer G-ring coordinate pairs | Non rép. |
| `$g` | Exclusion G-ring coordinate pairs | Non rép. |

### Zone 256 — Computer File Characteristics

**Répétabilité :** Non rép. &nbsp;|&nbsp; **Usage :** Fichiers informatiques

| Sous-zone | Nom | Rép. |
|-----------|-----|------|
| `$a` | Computer file characteristics | Non rép. |

### Zone 257 — Country of Producing Entity for Archival Films

**Répétabilité :** Non rép. &nbsp;|&nbsp; **Usage :** Films d'archives

| Sous-zone | Nom | Rép. |
|-----------|-----|------|
| `$a` | Country of producing entity | Non rép. |

### Zone 260 — Publication, Distribution, etc. (Imprint)

**Répétabilité :** Non rép.

| Sous-zone | Nom | Rép. |
|-----------|-----|------|
| `$a` | Place of publication, distribution, etc. | Rép. |
| `$b` | Name of publisher, distributor, etc. | Rép. |
| `$c` | Date of publication, distribution, etc. | Rép. |
| `$e` | Place of manufacture | Non rép. |
| `$f` | Manufacturer | Non rép. |
| `$g` | Date of manufacturer | Non rép. |

### Zone 270 — Address

**Répétabilité :** Rép.

| Sous-zone | Nom | Rép. |
|-----------|-----|------|
| `$a` | Address | Rép. |
| `$b` | City | Non rép. |
| `$c` | State or province | Non rép. |
| `$d` | Country | Non rép. |
| `$e` | Postal code | Non rép. |
| `$f` | Attention position | Non rép. |
| `$g` | Attention name | Non rép. |
| `$i` | Type of address | Non rép. |
| `$k` | Telephone number | Rép. |
| `$l` | Fax number | Rép. |
| `$m` | Electronic mail address | Rép. |

---

## BLOC 3XX — DESCRIPTION PHYSIQUE

### Zone 300 — Physical Description

**Répétabilité :** Rép.

| Sous-zone | Nom | Rép. |
|-----------|-----|------|
| `$a` | Extent | Rép. |
| `$b` | Other physical details | Non rép. |
| `$c` | Dimensions | Rép. |
| `$e` | Accompanying material | Non rép. |
| `$f` | Type of unit | Rép. |
| `$g` | Size of unit | Rép. |
| `$3` | Materials specified | Non rép. |

### Zone 306 — Playing Time

**Répétabilité :** Non rép. &nbsp;|&nbsp; **Usage :** Musique

| Sous-zone | Nom | Rép. |
|-----------|-----|------|
| `$a` | Playing time | Rép. |

### Zone 307 — Hours, Etc.

**Répétabilité :** Rép. &nbsp;|&nbsp; **Usage :** Ressources électroniques

| Sous-zone | Nom | Rép. |
|-----------|-----|------|
| `$a` | Hours | Non rép. |
| `$b` | Additional information | Non rép. |

### Zone 310 — Current Publication Frequency

**Répétabilité :** Non rép. &nbsp;|&nbsp; **Usage :** Périodiques (serials)

| Sous-zone | Nom | Rép. |
|-----------|-----|------|
| `$a` | Current publication frequency | Non rép. |
| `$b` | Date of current publication frequency | Non rép. |

### Zone 321 — Former Publication Frequency

**Répétabilité :** Rép. &nbsp;|&nbsp; **Usage :** Périodiques (serials)

| Sous-zone | Nom | Rép. |
|-----------|-----|------|
| `$a` | Former publication frequency | Non rép. |
| `$b` | Dates of former publication frequency | Non rép. |

### Zone 340 — Physical Medium

**Répétabilité :** Rép.

| Sous-zone | Nom | Rép. |
|-----------|-----|------|
| `$a` | Material base and configuration | Rép. |
| `$b` | Dimensions | Rép. |
| `$c` | Materials applied to surface | Rép. |
| `$d` | Information recording technique | Rép. |
| `$e` | Support | Rép. |
| `$f` | Production rate/ratio | Rép. |
| `$h` | Location within medium | Rép. |
| `$i` | Technical specifications of medium | Rép. |
| `$3` | Materials specified | Non rép. |

### Zone 351 — Organization and Arrangement of Materials

**Répétabilité :** Rép.

| Sous-zone | Nom | Rép. |
|-----------|-----|------|
| `$a` | Organization | Rép. |
| `$b` | Arrangement | Rép. |
| `$c` | Hierarchical level | Non rép. |
| `$3` | Materials specified | Non rép. |

### Zone 362 — Dates of Publication and/or Volume Designation

**Répétabilité :** Rép.

| Sous-zone | Nom | Rép. |
|-----------|-----|------|
| `$a` | Dates of publication and/or sequential designation | Non rép. |
| `$z` | Source of information | Non rép. |

---

## BLOC 4XX — MENTIONS DE COLLECTION

### Zone 440 — Series Statement/Added Entry – Title

**Répétabilité :** Rép.

| Sous-zone | Nom | Rép. |
|-----------|-----|------|
| `$a` | Title | Non rép. |
| `$n` | Number of part/section of a work | Rép. |
| `$p` | Name of part/section of a work | Rép. |
| `$v` | Volume/sequential designation | Non rép. |
| `$x` | ISSN | Non rép. |

### Zone 490 — Series Statement

**Répétabilité :** Rép. &nbsp;|&nbsp; **Usage :** Autorités de collection

| Sous-zone | Nom | Rép. |
|-----------|-----|------|
| `$a` | Series statement | Non rép. |
| `$l` | Library of Congress call number | Non rép. |
| `$v` | Volume/sequential designation | Rép. |
| `$x` | ISSN | Non rép. |

---

## BLOC 5XX — NOTES

### Zone 500 — General Note

**Répétabilité :** Rép.

| Sous-zone | Nom | Rép. |
|-----------|-----|------|
| `$a` | General note | Non rép. |

### Zone 501 — With Note

**Répétabilité :** Rép.

| Sous-zone | Nom | Rép. |
|-----------|-----|------|
| `$a` | With note | Non rép. |
| `$5` | Institution to which field applies | Non rép. |

### Zone 502 — Dissertation Note

**Répétabilité :** Rép. &nbsp;|&nbsp; **Usage :** Thèses

| Sous-zone | Nom | Rép. |
|-----------|-----|------|
| `$a` | Dissertation note | Non rép. |

### Zone 504 — Bibliography, Etc. Note

**Répétabilité :** Rép.

| Sous-zone | Nom | Rép. |
|-----------|-----|------|
| `$a` | Bibliography, etc. note | Non rép. |
| `$b` | Number of references | Non rép. |

### Zone 505 — Formatted Contents Note

**Répétabilité :** Rép.

| Sous-zone | Nom | Rép. |
|-----------|-----|------|
| `$a` | Formatted contents note | Non rép. |
| `$g` | Miscellaneous information | Non rép. |
| `$r` | Statement of responsibility | Rép. |
| `$t` | Title | Rép. |

### Zone 508 — Creation/Production Credits Note

**Répétabilité :** Non rép. &nbsp;|&nbsp; **Usage :** Films uniquement

| Sous-zone | Nom | Rép. |
|-----------|-----|------|
| `$a` | Creation/production credits note | Non rép. |

### Zone 513 — Type of Report and Period Covered Note

**Répétabilité :** Rép.

| Sous-zone | Nom | Rép. |
|-----------|-----|------|
| `$a` | Type of report | Non rép. |
| `$b` | Period covered | Non rép. |

### Zone 515 — Numbering Peculiarities Note

**Répétabilité :** Rép. &nbsp;|&nbsp; **Usage :** Périodiques (serials)

| Sous-zone | Nom | Rép. |
|-----------|-----|------|
| `$a` | Numbering peculiarities note | Non rép. |

### Zone 516 — Type of Computer File or Data Note

**Répétabilité :** Rép. &nbsp;|&nbsp; **Usage :** Fichiers informatiques

| Sous-zone | Nom | Rép. |
|-----------|-----|------|
| `$a` | Type of computer file or data note | Non rép. |

### Zone 520 — Summary, Etc.

**Répétabilité :** Rép.

| Sous-zone | Nom | Rép. |
|-----------|-----|------|
| `$a` | Summary, etc. note | Non rép. |
| `$b` | Expansion of summary note | Non rép. |
| `$3` | Materials specified | Non rép. |

### Zone 525 — Supplement Note

**Répétabilité :** Rép.

| Sous-zone | Nom | Rép. |
|-----------|-----|------|
| `$a` | Supplement note | Non rép. |

### Zone 530 — Additional Physical Form Available Note

**Répétabilité :** Rép.

| Sous-zone | Nom | Rép. |
|-----------|-----|------|
| `$a` | Additional physical form available note | Non rép. |
| `$b` | Availability source | Non rép. |
| `$c` | Availability conditions | Non rép. |

### Zone 534 — Original Version Note

**Répétabilité :** Rép.

| Sous-zone | Nom | Rép. |
|-----------|-----|------|
| `$p` | Introductory phrase | Non rép. |
| `$a` | Main entry of original | Non rép. |
| `$t` | Title statement of original | Non rép. |
| `$b` | Edition statement of original | Non rép. |
| `$c` | Publication, distribution, etc. of original | Non rép. |
| `$e` | Physical description, etc. of original | Non rép. |
| `$x` | ISBN | Rép. |

### Zone 536 — Funding Information Note

**Répétabilité :** Rép.

| Sous-zone | Nom | Rép. |
|-----------|-----|------|
| `$a` | Text of note | Non rép. |

### Zone 538 — System Details Note

**Répétabilité :** Rép.

| Sous-zone | Nom | Rép. |
|-----------|-----|------|
| `$a` | System details note | Non rép. |

### Zone 546 — Language Note

**Répétabilité :** Rép.

| Sous-zone | Nom | Rép. |
|-----------|-----|------|
| `$a` | Language note | Non rép. |

### Zone 550 — Issuing Body Note

**Répétabilité :** Rép.

| Sous-zone | Nom | Rép. |
|-----------|-----|------|
| `$a` | Issuing body note | Non rép. |

### Zone 586 — Awards Note

**Répétabilité :** Rép.

| Sous-zone | Nom | Rép. |
|-----------|-----|------|
| `$a` | Awards note | Non rép. |

### Zone 590 — Local Notes

**Répétabilité :** Rép.

| Sous-zone | Nom | Rép. |
|-----------|-----|------|
| `$a` | Local Notes | Rép. |

---

## BLOC 6XX — ACCÈS SUJET

### Zone 600 — Subject Added Entry – Personal Name

**Répétabilité :** Rép.

| Sous-zone | Nom | Rép. |
|-----------|-----|------|
| `$a` | Personal name | Non rép. |
| `$b` | Numeration | Non rép. |
| `$c` | Titles and other words associated with a name | Rép. |
| `$d` | Dates associated with a name | Non rép. |
| `$f` | Date of a work | Non rép. |
| `$l` | Language of a work | Non rép. |
| `$n` | Number of part/section of a work | Rép. |
| `$p` | Name of part/section of a work | Rép. |
| `$q` | Fuller form of name | Non rép. |
| `$t` | Title of a work | Non rép. |
| `$x` | General subdivision | Rép. |
| `$y` | Chronological subdivision | Rép. |
| `$z` | Geographic subdivision | Rép. |

### Zone 610 — Subject Added Entry – Corporate Name

**Répétabilité :** Rép.

| Sous-zone | Nom | Rép. |
|-----------|-----|------|
| `$a` | Corporate name or jurisdiction name as entry element | Non rép. |
| `$b` | Subordinate unit | Rép. |
| `$c` | Location of meeting | Non rép. |
| `$d` | Date of meeting or treaty signing | Rép. |
| `$f` | Date of a work | Non rép. |
| `$l` | Language of a work | Non rép. |
| `$n` | Number of part/section/meeting | Rép. |
| `$p` | Name of part/section of a work | Rép. |
| `$q` | Fuller form of name | Non rép. |
| `$t` | Title of a work | Non rép. |
| `$x` | General subdivision | Rép. |
| `$y` | Chronological subdivision | Rép. |
| `$z` | Geographic subdivision | Rép. |

### Zone 611 — Subject Added Entry – Meeting Name

**Répétabilité :** Rép.

| Sous-zone | Nom | Rép. |
|-----------|-----|------|
| `$a` | Meeting name or jurisdiction name as entry element | Non rép. |
| `$c` | Location of meeting | Non rép. |
| `$d` | Date of meeting | Non rép. |
| `$e` | Subordinate unit | Rép. |
| `$f` | Date of a work | Non rép. |
| `$l` | Language of a work | Non rép. |
| `$n` | Number of part/section/meeting | Rép. |
| `$p` | Name of part/section of a work | Rép. |
| `$q` | Name of meeting following jurisdiction name entry element | Non rép. |
| `$t` | Title of a work | Non rép. |
| `$x` | General subdivision | Rép. |
| `$y` | Chronological subdivision | Rép. |
| `$z` | Geographic subdivision | Rép. |

### Zone 630 — Subject Added Entry – Uniform Title

**Répétabilité :** Rép.

| Sous-zone | Nom | Rép. |
|-----------|-----|------|
| `$a` | Uniform title | Non rép. |
| `$d` | Date of treaty signing | Rép. |
| `$f` | Date of a work | Non rép. |
| `$k` | Form subheading | Rép. |
| `$l` | Language of a work | Non rép. |
| `$m` | Medium of performance for music | Rép. |
| `$n` | Number of part/section of work | Rép. |
| `$o` | Arranged statement for music | Non rép. |
| `$p` | Name of part/section of a work | Rép. |
| `$t` | Title of a work | Non rép. |
| `$x` | General subdivision | Rép. |
| `$y` | Chronological subdivision | Rép. |
| `$z` | Geographic subdivision | Rép. |

### Zone 650 — Subject Added Entry – Topical Term

**Répétabilité :** Rép.

| Sous-zone | Nom | Rép. |
|-----------|-----|------|
| `$a` | Topical term or geographic name as entry element | Non rép. |
| `$b` | Topical term following geographic name as entry element | Non rép. |
| `$x` | General subdivision | Rép. |
| `$y` | Chronological subdivision | Rép. |
| `$z` | Geographic subdivision | Rép. |

### Zone 651 — Subject Added Entry – Geographical Name

**Répétabilité :** Rép.

| Sous-zone | Nom | Rép. |
|-----------|-----|------|
| `$a` | Geographic name | Non rép. |
| `$v` | Form subdivision | Rép. |
| `$x` | General subdivision | Rép. |
| `$y` | Chronological subdivision | Rép. |
| `$z` | Geographic subdivision | Rép. |

### Zone 653 — Index Term – Uncontrolled

**Répétabilité :** Rép.

| Sous-zone | Nom | Rép. |
|-----------|-----|------|
| `$a` | Uncontrolled term | Rép. |

---

## BLOC 7XX — ENTRÉES SECONDAIRES (NOMS & TITRES)

### Zone 700 — Added Entry – Personal Name

**Répétabilité :** Rép.

| Sous-zone | Nom | Rép. |
|-----------|-----|------|
| `$a` | Personal name | Non rép. |
| `$b` | Numeration | Non rép. |
| `$c` | Titles & other words associated with a name | Rép. |
| `$d` | Dates associated with name | Non rép. |
| `$e` | Relator term | Rép. |
| `$f` | Date of work | Non rép. |
| `$k` | Form subheading | Rép. |
| `$l` | Language of a work | Non rép. |
| `$n` | Number of part/section of a work | Rép. |
| `$p` | Name of part/section of a work | Rép. |
| `$q` | Fuller form of name | Non rép. |
| `$t` | Title of a work | Non rép. |

### Zone 710 — Added Entry – Corporate Name

**Répétabilité :** Rép.

| Sous-zone | Nom | Rép. |
|-----------|-----|------|
| `$a` | Corporate name or jurisdiction name as entry element | Non rép. |
| `$b` | Subordinate unit | Non rép. |
| `$c` | Location of meeting | Non rép. |
| `$d` | Date of meeting or treaty signing | Non rép. |
| `$f` | Date of work | Non rép. |
| `$k` | Form subheading | Rép. |
| `$l` | Language of a work | Non rép. |
| `$n` | Number of part/section/meeting | Rép. |
| `$p` | Name of part/section of a work | Rép. |
| `$t` | Title of a work | Non rép. |

### Zone 711 — Added Entry – Meeting Name

**Répétabilité :** Rép.

| Sous-zone | Nom | Rép. |
|-----------|-----|------|
| `$a` | Meeting name or jurisdiction name as entry element | Non rép. |
| `$c` | Location of meeting | Non rép. |
| `$d` | Date of meeting | Non rép. |
| `$e` | Subordinate unit | Rép. |
| `$f` | Date of a work | Non rép. |
| `$h` | Medium | Non rép. |
| `$k` | Form subheading | Rép. |
| `$l` | Language of a work | Non rép. |
| `$n` | Number of part/section/meeting | Rép. |
| `$p` | Name of part/section of a work | Rép. |
| `$q` | Name of meeting following jurisdiction name entry element | Non rép. |
| `$t` | Title of a work | Non rép. |

### Zone 730 — Added Entry – Uniform Title

**Répétabilité :** Rép.

| Sous-zone | Nom | Rép. |
|-----------|-----|------|
| `$a` | Uniform title | Non rép. |
| `$d` | Date of treaty signing | Rép. |
| `$f` | Date of work | Non rép. |
| `$k` | Form subheading | Rép. |
| `$l` | Language of a work | Non rép. |
| `$m` | Medium of performance for music | Rép. |
| `$n` | Number of part/section of work | Rép. |
| `$o` | Arranged statement for music | Non rép. |
| `$p` | Name of part/section of a work | Rép. |
| `$s` | Version | Non rép. |
| `$t` | Title of a work | Non rép. |

### Zone 740 — Added Entry – Uncontrolled Related/Analytical Title

**Répétabilité :** Rép.

| Sous-zone | Nom | Rép. |
|-----------|-----|------|
| `$a` | Uncontrolled related/analytical title | Non rép. |
| `$h` | Medium | Non rép. |
| `$n` | Number of part/section of a work | Rép. |
| `$p` | Name of part/section of a work | Rép. |

### Zone 777 — Issued With Entry

**Répétabilité :** Rép.

| Sous-zone | Nom | Rép. |
|-----------|-----|------|
| `$a` | Main entry heading | Non rép. |
| `$b` | Edition | Non rép. |
| `$c` | Qualifying information | Non rép. |
| `$d` | Place, publisher, and date of publication | Non rép. |
| `$g` | Relationship information | Rép. |
| `$x` | ISSN | Non rép. |

### Zone 780 — Preceding Entry

**Répétabilité :** Rép.

| Sous-zone | Nom | Rép. |
|-----------|-----|------|
| `$a` | Main entry heading | Non rép. |
| `$b` | Edition | Non rép. |
| `$c` | Qualifying information | Non rép. |
| `$d` | Place, publisher, and date of publication | Non rép. |
| `$g` | Relationship information | Rép. |
| `$u` | Standard Technical Report Number | Non rép. |
| `$x` | ISSN | Non rép. |

### Zone 785 — Succeeding Entry

**Répétabilité :** Rép.

| Sous-zone | Nom | Rép. |
|-----------|-----|------|
| `$a` | Main entry heading | Non rép. |
| `$b` | Edition | Non rép. |
| `$c` | Qualifying information | Non rép. |
| `$d` | Place, publisher, and date of publication | Non rép. |
| `$g` | Relationship information | Rép. |
| `$x` | ISSN | Non rép. |

---

## BLOC 8XX — ENTRÉES SECONDAIRES SÉRIE & LOCALISATION

### Zone 800 — Series Added Entry – Personal Name

**Répétabilité :** Rép.

| Sous-zone | Nom | Rép. |
|-----------|-----|------|
| `$a` | Personal name | Non rép. |
| `$b` | Numeration | Non rép. |
| `$c` | Titles and other words associated with a name | Rép. |
| `$d` | Dates associated with a name | Non rép. |
| `$f` | Date of a work | Non rép. |
| `$l` | Language of a work | Non rép. |
| `$n` | Number of part/section of a work | Rép. |
| `$p` | Name of part/section of a work | Rép. |
| `$q` | Fuller form of name | Non rép. |
| `$t` | Title of a work | Non rép. |

### Zone 810 — Series Added Entry – Corporate Name

**Répétabilité :** Rép.

| Sous-zone | Nom | Rép. |
|-----------|-----|------|
| `$a` | Corporate name or jurisdiction name as entry element | Non rép. |
| `$b` | Subordinate unit | Rép. |
| `$c` | Location of meeting | Non rép. |
| `$d` | Date of meeting or treaty signing | Rép. |
| `$f` | Date of a work | Non rép. |
| `$l` | Language of a work | Non rép. |
| `$n` | Number of part/section/meeting | Rép. |
| `$p` | Name of part/section of a work | Rép. |
| `$t` | Title of a work | Non rép. |

### Zone 811 — Series Added Entry – Meeting Name

**Répétabilité :** Rép.

| Sous-zone | Nom | Rép. |
|-----------|-----|------|
| `$a` | Meeting name or jurisdiction name as entry element | Non rép. |
| `$c` | Location of meeting | Non rép. |
| `$d` | Date of meeting | Non rép. |
| `$e` | Subordinate unit | Rép. |
| `$f` | Date of a work | Non rép. |
| `$l` | Language of a work | Non rép. |
| `$n` | Number of part/section/meeting | Rép. |
| `$p` | Name of part/section of a work | Rép. |
| `$q` | Name of meeting following jurisdiction name entry element | Non rép. |
| `$t` | Title of a work | Non rép. |

### Zone 830 — Series Added Entry – Uniform Title

**Répétabilité :** Rép.

| Sous-zone | Nom | Rép. |
|-----------|-----|------|
| `$a` | Uniform title | Non rép. |
| `$b` | Date of treaty signing | Rép. |
| `$f` | Date of a work | Non rép. |
| `$g` | Miscellaneous information | Non rép. |
| `$h` | Medium | Non rép. |
| `$k` | Form subheading | Rép. |
| `$l` | Language of a work | Non rép. |
| `$n` | Number of part/section of a work | Rép. |
| `$p` | Name of part/section of a work | Rép. |
| `$t` | Title of a work | Non rép. |

### Zone 850 — Holding Institution

**Répétabilité :** Rép.

| Sous-zone | Nom | Rép. |
|-----------|-----|------|
| `$a` | Holding institution | Rép. |

### Zone 852 — Location

**Répétabilité :** Rép.

| Sous-zone | Nom | Rép. |
|-----------|-----|------|
| `$a` | Location | Non rép. |
| `$b` | Sublocation or collection | Rép. |
| `$c` | Shelving location | Rép. |
| `$h` | Classification part | Non rép. |
| `$i` | Item part | Rép. |
| `$j` | Shelving control number | Non rép. |
| `$n` | Country code | Non rép. |
| `$p` | Piece Designation | Non rép. |
| `$t` | Copy number | Non rép. |

### Zone 856 — Electronic Location and Access

**Répétabilité :** Rép.

| Sous-zone | Nom | Rép. |
|-----------|-----|------|
| `$a` | Host name | Rép. |
| `$b` | Access number | Rép. |
| `$c` | Compression information | Rép. |
| `$d` | Path | Rép. |
| `$f` | Electronic name | Rép. |
| `$g` | Uniform Resource Name | Rép. |
| `$h` | Processor of request | Non rép. |
| `$i` | Instruction | Rép. |
| `$j` | Bits per second | Non rép. |
| `$k` | Password | Non rép. |
| `$l` | Logon | Non rép. |
| `$m` | Contact for access assistance | Rép. |
| `$n` | Name of location of host | Non rép. |
| `$o` | Operating system | Non rép. |
| `$p` | Port | Non rép. |
| `$q` | Electronic format type | Non rép. |
| `$r` | Settings | Non rép. |
| `$s` | File size | Rép. |
| `$t` | Terminal emulation | Rép. |
| `$u` | Uniform Resource Locator | Non rép. |
| `$v` | Hours access method available | Rép. |
| `$w` | Record control number | Rép. |
| `$x` | Nonpublic note | Rép. |
| `$z` | Public note | Rép. |
| `$2` | Access method | Non rép. |

### Zone 886 — Foreign MARC Information Field

**Répétabilité :** Rép.

| Sous-zone | Nom | Rép. |
|-----------|-----|------|
| `$a` | Tag of the foreign MARC field | Non rép. |
| `$b` | Content of the foreign MARC field | Non rép. |
| `$2` | Source of data | Non rép. |
| `$a-z` | Foreign MARC subfield | Rép. |
| `$0-9` | Foreign MARC subfield | Rép. |