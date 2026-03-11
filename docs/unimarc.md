# Manuel UNIMARC – Format Bibliographique
## Structure : Zone → Sous-zone → Champs → Sous-champs

---

## Bloc 0 – Identification

### 001 – Identifiant de la notice

| Sous-zone | Libellé | Notes |
|---|---|---|
| — | Identifiant (valeur directe) | Chaîne attribuée par le système, sans sous-zone |

---

### 003 – Identifiant persistant de la notice

| Sous-zone | Libellé | Notes |
|---|---|---|
| — | URI / ISIL | Identifiant unique et persistant de l'agence |

---

### 005 – Version de la notice

| Sous-zone | Libellé | Notes |
|---|---|---|
| — | Date et heure | Format : AAAAMMJJHHMMSS.F |

---

### 010 – ISBN

| Sous-zone | Libellé | Notes |
|---|---|---|
| `$a` | ISBN valide | Format normalisé ISBN-13 |
| `$b` | Modalités d'obtention / prix | Optionnel |
| `$d` | Termes de disponibilité | Optionnel |
| `$z` | ISBN erroné ou annulé | |

---

### 011 – ISSN

| Sous-zone | Libellé | Notes |
|---|---|---|
| `$a` | ISSN valide | 8 chiffres avec tiret |
| `$b` | Modalités d'obtention | Optionnel |
| `$d` | Termes de disponibilité | Optionnel |
| `$f` | ISSN-L (ISSN de liaison) | |
| `$g` | ISSN erroné annulé | |
| `$y` | ISSN provisoire | |
| `$z` | ISSN erroné ou annulé | |

---

### 012 – Fingerprint

| Sous-zone | Libellé | Notes |
|---|---|---|
| `$a` | Fingerprint | Empreinte pour livres anciens |
| `$b` | Date de publication (fingerprint) | |
| `$c` | Code de source du fingerprint | |
| `$d` | Code non standardisé | |

---

### 013 – Numéro ISRN

| Sous-zone | Libellé | Notes |
|---|---|---|
| `$a` | ISRN valide | |
| `$b` | Modalités d'obtention | |
| `$z` | ISRN erroné ou annulé | |

---

### 015 – Numéro de bibliographie nationale

| Sous-zone | Libellé | Notes |
|---|---|---|
| `$a` | Numéro | |
| `$b` | Source | Code de la bibliographie nationale |
| `$z` | Numéro annulé | |

---

### 016 – Numéro de notice de bibliothèque nationale

| Sous-zone | Libellé | Notes |
|---|---|---|
| `$a` | Numéro de notice | |
| `$b` | Source (code ISIL) | |
| `$z` | Numéro annulé | |

---

### 017 – Numéro de dépôt légal

| Sous-zone | Libellé | Notes |
|---|---|---|
| `$a` | Numéro | |
| `$b` | Organisme | |
| `$d` | Date | |

---

### 020 – Numéro LCCN

| Sous-zone | Libellé | Notes |
|---|---|---|
| `$a` | LCCN valide | |
| `$b` | Numéro de notice d'autorité NACO | |
| `$z` | LCCN erroné ou annulé | |

---

### 022 – Numéro de document gouvernemental

| Sous-zone | Libellé | Notes |
|---|---|---|
| `$a` | Numéro | |
| `$b` | Organisme émetteur | |

---

### 035 – Autre numéro de notice

| Sous-zone | Libellé | Notes |
|---|---|---|
| `$a` | Numéro de notice | |
| `$b` | Source / réseau | Ex. : OCLC, Sudoc |
| `$z` | Numéro annulé | |

---

### 036 – Numéro de musique (éditeur)

| Sous-zone | Libellé | Notes |
|---|---|---|
| `$a` | Numéro de plaque | |
| `$b` | Nom de l'éditeur musical | |

---

### 040 – Numéro CODEN

| Sous-zone | Libellé | Notes |
|---|---|---|
| `$a` | CODEN valide | |
| `$z` | CODEN erroné ou annulé | |

---

### 071 – Numéro d'éditeur (musique)

| Sous-zone | Libellé | Notes |
|---|---|---|
| `$a` | Numéro | |
| `$b` | Nom de l'éditeur | |
| `$c` | Qualificatif | |

---

### 072 – ISMN

| Sous-zone | Libellé | Notes |
|---|---|---|
| `$a` | ISMN valide | |
| `$b` | Modalités d'obtention | |
| `$z` | ISMN erroné ou annulé | |

---

### 073 – EAN

| Sous-zone | Libellé | Notes |
|---|---|---|
| `$a` | EAN / GTIN | Code-barres 13 chiffres |
| `$z` | EAN erroné | |

---

## Bloc 1 – Informations codées

### 100 – Données générales de traitement

| Sous-zone | Libellé | Notes |
|---|---|---|
| `$a` | Données codées (40 caractères) | Positions : 00-07 date saisie, 08-10 type date pub., 09-12 date 1, 13-16 date 2, 17-19 pays pub., 20-21 langue, 22 alphabet, 23-25 langue de catalogage |

---

### 101 – Langue de la ressource

| Sous-zone | Libellé | Notes |
|---|---|---|
| `$a` | Langue de la ressource | Code ISO 639-2, répétable |
| `$b` | Langue du résumé | Répétable |
| `$c` | Langue de la table des matières | |
| `$d` | Langue du titre propre | |
| `$e` | Langue de l'original | |
| `$f` | Langue de la version intermédiaire | |
| `$g` | Langue du livret | |
| `$h` | Langue des notes | Répétable |
| `$i` | Langue de la couverture | |
| `$j` | Langue des sous-titres | Répétable |

---

### 102 – Pays de publication

| Sous-zone | Libellé | Notes |
|---|---|---|
| `$a` | Pays de publication | Code ISO 3166, répétable |
| `$b` | Province / État | |

---

### 105 – Zone codée : textes, monographies

| Sous-zone | Libellé | Notes |
|---|---|---|
| `$a` | Données codées (13 caractères) | Pos. 00-03 : illustrations ; 04-07 : contenu ; 08 : conférence ; 09 : festschrift ; 10 : index ; 11 : littérature ; 12 : biographie |

---

### 106 – Zone codée : forme du document

| Sous-zone | Libellé | Notes |
|---|---|---|
| `$a` | Forme du document (1 caractère) | a=microfilm, b=microfiche, c=microopaque, d=grand format, f=braille, g=ressource électronique, r=reprint ordinaire |

---

### 110 – Zone codée : publications en série

| Sous-zone | Libellé | Notes |
|---|---|---|
| `$a` | Données codées (11 caractères) | Pos. 00 : fréquence ; 01 : régularité ; 02 : type de ressource continue ; 03 : type de contenu ; 04 : nature du contenu ; 05 : nature du travail original ; 06 : cumul d'index ; 07 : titre-clé ; 08 : dépouillement |

---

### 115 – Zone codée : images animées

| Sous-zone | Libellé | Notes |
|---|---|---|
| `$a` | Données codées (14 caractères) | Pos. 00 : type document ; 01-02 : support ; 03 : couleur ; 04 : présence de son ; 05-06 : largeur film ; 07 : configuration canaux son ; 08 : dimension image ; 09 : vitesse projection |
| `$b` | Données codées de la vidéo (12 caractères) | Standard vidéo, couleur, son, dimensions |

---

### 116 – Zone codée : images fixes

| Sous-zone | Libellé | Notes |
|---|---|---|
| `$a` | Données codées (6 caractères) | Pos. 00 : type document ; 01 : support primaire ; 02 : support secondaire ; 03 : couleur ; 04 : technique (image) ; 05 : fonctionnalité |

---

### 117 – Zone codée : matériaux tridimensionnels

| Sous-zone | Libellé | Notes |
|---|---|---|
| `$a` | Données codées (2 caractères) | Pos. 00 : type objet ; 01 : support |

---

### 120 – Zone codée : cartographie (généralités)

| Sous-zone | Libellé | Notes |
|---|---|---|
| `$a` | Données codées (8 caractères) | Pos. 00-01 : type de document ; 02-03 : couleur ; 04 : support physique ; 05 : type de reproduction ; 06 : technique de reproduction |

---

### 121 – Zone codée : cartographie (images terrestres)

| Sous-zone | Libellé | Notes |
|---|---|---|
| `$a` | Données codées (8 caractères) | Pos. 00-01 : altitude/profondeur ; 02 : attitude de l'objet ; 03 : nuages ; 04 : plateforme de construction ; 05-06 : instrument de détection ; 07 : données de données |

---

### 122 – Zone codée : période de temps

| Sous-zone | Libellé | Notes |
|---|---|---|
| `$a` | Type de période | Code unique ou plage |
| `$b` | Date de début | AAAAMMJJ |
| `$c` | Date de fin | AAAAMMJJ |

---

### 123 – Zone codée : cartographie (échelles)

| Sous-zone | Libellé | Notes |
|---|---|---|
| `$a` | Type d'échelle | |
| `$b` | Échelle horizontale constante | |
| `$c` | Échelle horizontale min | |
| `$d` | Échelle horizontale max | |
| `$e` | Échelle verticale | |
| `$f` | Série (identifiant) | |
| `$g` | Feuille de la série | |

---

### 124 – Zone codée : cartographie (données spécifiques)

| Sous-zone | Libellé | Notes |
|---|---|---|
| `$a` | Données codées (8 caractères) | Pos. 00 : forme de relief ; 01-07 : type de carte ; projection ; méridien ; tracé ; |
| `$b` | Projection cartographique | Code MARC |
| `$c` | Méridien | |
| `$d` | Déclinaison magnétique | |
| `$e` | Type d'équipement | |

---

### 125 – Zone codée : musique notée

| Sous-zone | Libellé | Notes |
|---|---|---|
| `$a` | Données codées (2 caractères) | Pos. 00 : format de composition ; 01 : format musical (parties, conducteur…) |
| `$b` | Données codées secondaires | |

---

### 126 – Zone codée : enregistrements musicaux

| Sous-zone | Libellé | Notes |
|---|---|---|
| `$a` | Données codées (13 caractères) | Pos. 00 : type de composition ; 01 : vitesse ; 02-03 : largeur sillon/piste ; 04 : durée ; 05 : canaux son ; 06-07 : support ; 08 : type d'enregistrement |
| `$b` | Données codées secondaires | |

---

### 127 – Zone codée : enregistrements non musicaux

| Sous-zone | Libellé | Notes |
|---|---|---|
| `$a` | Données codées (3 caractères) | Pos. 00 : vitesse ; 01 : canaux son ; 02 : support |

---

### 128 – Zone codée : musique (généralités)

| Sous-zone | Libellé | Notes |
|---|---|---|
| `$a` | Type de composition musicale | Code à 2 caractères |
| `$b` | Format de la musique | |
| `$c` | Tonalité | |
| `$d` | Voix | Répétable |

---

### 130 – Zone codée : microformes

| Sous-zone | Libellé | Notes |
|---|---|---|
| `$a` | Données codées (13 caractères) | Pos. 00 : type de microforme ; 01 : polarité ; 02 : dimensions ; 03 : réduction ; 04-05 : largeur film ; 06 : couleur ; 07 : émulsion ; 08 : génération ; 09 : support |

---

### 131 – Zone codée : cartographie électronique

| Sous-zone | Libellé | Notes |
|---|---|---|
| `$a` | Données codées | Type de document ; support |

---

### 135 – Zone codée : ressources électroniques

| Sous-zone | Libellé | Notes |
|---|---|---|
| `$a` | Données codées (13 caractères) | Pos. 00 : type de fichier ; 01-02 : nombre de fichiers ; 03 : accès ; 04-05 : type accès distant ; 06-08 : qualité d'image |

---

### 140 – Zone codée : livres anciens (généralités)

| Sous-zone | Libellé | Notes |
|---|---|---|
| `$a` | Données codées (14 caractères) | Pos. 00-01 : format bibliographique ; 02 : reliure ; 03-04 : illustrations ; 05 : enluminures ; 06 : incunables |

---

### 141 – Zone codée : livres anciens (copies)

| Sous-zone | Libellé | Notes |
|---|---|---|
| `$a` | Données codées (14 caractères) | Informations propres à l'exemplaire |

---

## Bloc 2 – Description bibliographique

### 200 – Titre et mention de responsabilité

| Sous-zone | Libellé | Notes |
|---|---|---|
| `$a` | Titre propre | Obligatoire |
| `$b` | Désignation générale du document | |
| `$c` | Titre propre du document hôte | |
| `$d` | Titre parallèle | Répétable |
| `$e` | Complément du titre | Répétable |
| `$f` | Première mention de responsabilité | |
| `$g` | Mentions de responsabilité suivantes | Répétable |
| `$h` | Numéro de partie | Répétable |
| `$i` | Titre de partie | Répétable |
| `$v` | Désignation du volume | |
| `$z` | Langue du titre parallèle | |

---

### 205 – Mention d'édition

| Sous-zone | Libellé | Notes |
|---|---|---|
| `$a` | Mention d'édition | |
| `$b` | Mention d'édition parallèle | Répétable |
| `$d` | Mention supplémentaire d'édition | Répétable |
| `$f` | Mention de responsabilité relative à l'édition | Répétable |
| `$g` | Mention de responsabilité suivante relative à l'édition | Répétable |

---

### 206 – Données mathématiques (cartographie)

| Sous-zone | Libellé | Notes |
|---|---|---|
| `$a` | Énoncé d'échelle | |
| `$b` | Énoncé de projection | |
| `$c` | Énoncé des coordonnées | |
| `$d` | Zone d'équinoxe et époque | |
| `$e` | Déclaration de l'objet céleste | |

---

### 207 – Numérotation (publications en série)

| Sous-zone | Libellé | Notes |
|---|---|---|
| `$a` | Numérotation | Répétable |
| `$z` | Source de l'information | |

---

### 208 – Données spécifiques à la musique

| Sous-zone | Libellé | Notes |
|---|---|---|
| `$a` | Présentation musicale | |
| `$d` | Présentation musicale parallèle | |

---

### 210 – Adresse bibliographique

| Sous-zone | Libellé | Notes |
|---|---|---|
| `$a` | Lieu de publication | Répétable |
| `$b` | Adresse de l'éditeur | Répétable |
| `$c` | Nom de l'éditeur | Répétable |
| `$d` | Date de publication | |
| `$e` | Lieu de fabrication | Répétable |
| `$f` | Nom du fabricant | Répétable |
| `$g` | Date de fabrication | |
| `$h` | Date de copyright | |

---

### 215 – Description physique

| Sous-zone | Libellé | Notes |
|---|---|---|
| `$a` | Désignation spécifique du document et étendue | |
| `$b` | Autres caractéristiques physiques | Répétable |
| `$c` | Dimensions | Répétable |
| `$d` | Matériel d'accompagnement | Répétable |

---

### 225 – Collection

| Sous-zone | Libellé | Notes |
|---|---|---|
| `$a` | Titre de la collection | |
| `$b` | Désignation générale du document | |
| `$d` | Titre parallèle de la collection | Répétable |
| `$e` | Complément du titre de la collection | Répétable |
| `$f` | Mention de responsabilité relative à la collection | Répétable |
| `$h` | Numéro de partie | Répétable |
| `$i` | Titre de partie | Répétable |
| `$v` | Numérotation dans la collection | |
| `$x` | ISSN de la collection | |
| `$z` | Langue du titre parallèle | |

---

### 230 – Données spécifiques aux ressources électroniques

| Sous-zone | Libellé | Notes |
|---|---|---|
| `$a` | Désignation spécifique du document | |

---

## Bloc 3 – Notes

### 300 – Note générale

| Sous-zone | Libellé | Notes |
|---|---|---|
| `$a` | Texte de la note | Répétable (zone) |

---

### 302 – Note sur le contenu

| Sous-zone | Libellé | Notes |
|---|---|---|
| `$a` | Texte de la note | |

---

### 303 – Note sur les sources bibliographiques

| Sous-zone | Libellé | Notes |
|---|---|---|
| `$a` | Texte de la note | |

---

### 304 – Note sur le titre et la responsabilité

| Sous-zone | Libellé | Notes |
|---|---|---|
| `$a` | Texte de la note | |

---

### 305 – Note sur l'édition et l'historique

| Sous-zone | Libellé | Notes |
|---|---|---|
| `$a` | Texte de la note | |

---

### 306 – Note sur la publication

| Sous-zone | Libellé | Notes |
|---|---|---|
| `$a` | Texte de la note | |

---

### 307 – Note sur la description physique

| Sous-zone | Libellé | Notes |
|---|---|---|
| `$a` | Texte de la note | |

---

### 308 – Note sur la collection

| Sous-zone | Libellé | Notes |
|---|---|---|
| `$a` | Texte de la note | |

---

### 310 – Note sur la publication en série

| Sous-zone | Libellé | Notes |
|---|---|---|
| `$a` | Texte de la note | |

---

### 311 – Note sur les liens

| Sous-zone | Libellé | Notes |
|---|---|---|
| `$a` | Texte de la note | |

---

### 312 – Note sur le titre

| Sous-zone | Libellé | Notes |
|---|---|---|
| `$a` | Texte de la note | |

---

### 313 – Note sur les sujets

| Sous-zone | Libellé | Notes |
|---|---|---|
| `$a` | Texte de la note | |

---

### 314 – Note sur les personnes et organismes

| Sous-zone | Libellé | Notes |
|---|---|---|
| `$a` | Texte de la note | |

---

### 315 – Note sur les caractéristiques du document

| Sous-zone | Libellé | Notes |
|---|---|---|
| `$a` | Texte de la note | |

---

### 316 – Note sur l'exemplaire

| Sous-zone | Libellé | Notes |
|---|---|---|
| `$a` | Texte de la note | |
| `$5` | Code de l'organisme | |

---

### 317 – Note de provenance

| Sous-zone | Libellé | Notes |
|---|---|---|
| `$a` | Texte de la note | |
| `$5` | Code de l'organisme | |

---

### 318 – Note sur l'action

| Sous-zone | Libellé | Notes |
|---|---|---|
| `$a` | Texte de la note | |
| `$5` | Code de l'organisme | |

---

### 320 – Note bibliographique interne

| Sous-zone | Libellé | Notes |
|---|---|---|
| `$a` | Texte de la note | |

---

### 321 – Note de résumé

| Sous-zone | Libellé | Notes |
|---|---|---|
| `$a` | Texte du résumé | |
| `$z` | Langue du résumé | Code ISO 639-2 |

---

### 322 – Note sur les crédits

| Sous-zone | Libellé | Notes |
|---|---|---|
| `$a` | Texte de la note | |

---

### 323 – Note sur les interprètes

| Sous-zone | Libellé | Notes |
|---|---|---|
| `$a` | Texte de la note | |

---

### 324 – Note de reproduction

| Sous-zone | Libellé | Notes |
|---|---|---|
| `$a` | Texte de la note | |

---

### 325 – Note sur l'original

| Sous-zone | Libellé | Notes |
|---|---|---|
| `$a` | Texte de la note | |

---

### 326 – Note sur la fréquence

| Sous-zone | Libellé | Notes |
|---|---|---|
| `$a` | Fréquence courante | |
| `$b` | Dates de la fréquence courante | |

---

### 327 – Note de contenu

| Sous-zone | Libellé | Notes |
|---|---|---|
| `$a` | Texte de la note | Répétable |
| `$z` | Langue | |

---

### 328 – Note de thèse

| Sous-zone | Libellé | Notes |
|---|---|---|
| `$a` | Texte de la note | |
| `$b` | Degré | |
| `$c` | Établissement | |
| `$d` | Date | |
| `$e` | Pays de soutenance | |

---

### 330 – Résumé

| Sous-zone | Libellé | Notes |
|---|---|---|
| `$a` | Texte du résumé | |
| `$z` | Langue | |

---

### 332 – Citation préférée

| Sous-zone | Libellé | Notes |
|---|---|---|
| `$a` | Texte de la citation | |

---

### 333 – Public visé

| Sous-zone | Libellé | Notes |
|---|---|---|
| `$a` | Texte | |

---

### 336 – Type de contenu

| Sous-zone | Libellé | Notes |
|---|---|---|
| `$a` | Terme de type de contenu | Code RDA |
| `$b` | Code de type de contenu | |
| `$2` | Source | |

---

### 337 – Type de média

| Sous-zone | Libellé | Notes |
|---|---|---|
| `$a` | Terme de type de média | Code RDA |
| `$b` | Code de type de média | |
| `$2` | Source | |

---

### 338 – Type de support

| Sous-zone | Libellé | Notes |
|---|---|---|
| `$a` | Terme de type de support | Code RDA |
| `$b` | Code de type de support | |
| `$2` | Source | |

---

## Bloc 4 – Liens

### 410 – Lien à la collection

| Sous-zone | Libellé | Notes |
|---|---|---|
| `$0` | Numéro de notice de la collection | |
| `$1` | Données de la collection liée | Bloc structuré |
| `$t` | Titre | |
| `$v` | Numérotation | |
| `$x` | ISSN | |

---

### 411 – Lien à la sous-collection

| Sous-zone | Libellé | Notes |
|---|---|---|
| `$0` | Numéro de notice | |
| `$1` | Données liées | |
| `$t` | Titre | |
| `$v` | Numérotation | |
| `$x` | ISSN | |

---

### 421 – Lien au supplément

| Sous-zone | Libellé | Notes |
|---|---|---|
| `$0` | Numéro de notice | |
| `$1` | Données liées | |
| `$t` | Titre | |
| `$x` | ISSN | |

---

### 422 – Lien à la ressource mère (supplément)

| Sous-zone | Libellé | Notes |
|---|---|---|
| `$0` | Numéro de notice | |
| `$1` | Données liées | |
| `$t` | Titre | |
| `$x` | ISSN | |

---

### 423 – Lien publié avec

| Sous-zone | Libellé | Notes |
|---|---|---|
| `$0` | Numéro de notice | |
| `$1` | Données liées | |
| `$t` | Titre | |

---

### 430 – Autre édition même support

| Sous-zone | Libellé | Notes |
|---|---|---|
| `$0` | Numéro de notice | |
| `$1` | Données liées | |
| `$t` | Titre | |
| `$x` | ISSN | |
| `$z` | ISBN | |

---

### 431 – Autre édition autre support

| Sous-zone | Libellé | Notes |
|---|---|---|
| `$0` | Numéro de notice | |
| `$1` | Données liées | |
| `$t` | Titre | |
| `$x` | ISSN | |
| `$z` | ISBN | |

---

### 432 – Remplacé par

| Sous-zone | Libellé | Notes |
|---|---|---|
| `$0` | Numéro de notice | |
| `$1` | Données liées | |
| `$t` | Titre | |
| `$x` | ISSN | |

---

### 433 – Remplace

| Sous-zone | Libellé | Notes |
|---|---|---|
| `$0` | Numéro de notice | |
| `$1` | Données liées | |
| `$t` | Titre | |
| `$x` | ISSN | |

---

### 434 – Absorbe

| Sous-zone | Libellé | Notes |
|---|---|---|
| `$0` | Numéro de notice | |
| `$1` | Données liées | |
| `$t` | Titre | |

---

### 435 – Absorbé par

| Sous-zone | Libellé | Notes |
|---|---|---|
| `$0` | Numéro de notice | |
| `$1` | Données liées | |
| `$t` | Titre | |

---

### 436 – Scinde en

| Sous-zone | Libellé | Notes |
|---|---|---|
| `$0` | Numéro de notice | |
| `$1` | Données liées | |
| `$t` | Titre | |

---

### 437 – Résulte de la fusion de

| Sous-zone | Libellé | Notes |
|---|---|---|
| `$0` | Numéro de notice | |
| `$1` | Données liées | |
| `$t` | Titre | |

---

### 440 – Titre de section / partie

| Sous-zone | Libellé | Notes |
|---|---|---|
| `$0` | Numéro de notice | |
| `$1` | Données liées | |
| `$t` | Titre | |
| `$x` | ISSN | |

---

### 451 – Même publication autre support

| Sous-zone | Libellé | Notes |
|---|---|---|
| `$0` | Numéro de notice | |
| `$1` | Données liées | |
| `$t` | Titre | |
| `$x` | ISSN | |

---

### 452 – Version électronique

| Sous-zone | Libellé | Notes |
|---|---|---|
| `$0` | Numéro de notice | |
| `$1` | Données liées | |
| `$t` | Titre | |
| `$u` | URI | |

---

### 453 – Traduit en

| Sous-zone | Libellé | Notes |
|---|---|---|
| `$0` | Numéro de notice | |
| `$1` | Données liées | |
| `$t` | Titre | |
| `$z` | ISBN | |

---

### 454 – Traduit de

| Sous-zone | Libellé | Notes |
|---|---|---|
| `$0` | Numéro de notice | |
| `$1` | Données liées | |
| `$t` | Titre | |
| `$z` | ISBN | |

---

### 455 – Reproduction de

| Sous-zone | Libellé | Notes |
|---|---|---|
| `$0` | Numéro de notice | |
| `$1` | Données liées | |
| `$t` | Titre | |

---

### 456 – Reproduit en

| Sous-zone | Libellé | Notes |
|---|---|---|
| `$0` | Numéro de notice | |
| `$1` | Données liées | |
| `$t` | Titre | |

---

### 461 – Ensemble dont fait partie

| Sous-zone | Libellé | Notes |
|---|---|---|
| `$0` | Numéro de notice | |
| `$1` | Données liées | |
| `$t` | Titre | |
| `$v` | Numérotation | |
| `$z` | ISBN | |

---

### 462 – Sous-ensemble

| Sous-zone | Libellé | Notes |
|---|---|---|
| `$0` | Numéro de notice | |
| `$1` | Données liées | |
| `$t` | Titre | |
| `$v` | Numérotation | |

---

### 463 – Partie analytique

| Sous-zone | Libellé | Notes |
|---|---|---|
| `$0` | Numéro de notice | |
| `$1` | Données liées | |
| `$t` | Titre | |
| `$v` | Numérotation | |

---

### 464 – Partie analytique composante

| Sous-zone | Libellé | Notes |
|---|---|---|
| `$0` | Numéro de notice | |
| `$1` | Données liées | |
| `$t` | Titre | |

---

### 470 – Article recensé

| Sous-zone | Libellé | Notes |
|---|---|---|
| `$0` | Numéro de notice | |
| `$1` | Données liées | |
| `$t` | Titre | |

---

### 481 – Publié avec (ensemble)

| Sous-zone | Libellé | Notes |
|---|---|---|
| `$0` | Numéro de notice | |
| `$1` | Données liées | |
| `$t` | Titre | |

---

### 482 – Relié ensemble

| Sous-zone | Libellé | Notes |
|---|---|---|
| `$0` | Numéro de notice | |
| `$1` | Données liées | |
| `$t` | Titre | |

---

### 488 – Autre relation

| Sous-zone | Libellé | Notes |
|---|---|---|
| `$0` | Numéro de notice | |
| `$1` | Données liées | |
| `$t` | Titre | |
| `$i` | Texte explicatif de la relation | |

---

## Bloc 5 – Titres associés

### 500 – Titre uniforme

| Sous-zone | Libellé | Notes |
|---|---|---|
| `$a` | Titre uniforme | |
| `$b` | Désignation générale du document | |
| `$d` | Titre parallèle | |
| `$e` | Complément du titre | |
| `$f` | Date de l'œuvre | |
| `$g` | Divers | |
| `$h` | Numéro de partie | Répétable |
| `$i` | Titre de partie | Répétable |
| `$j` | Qualificatif de forme | |
| `$k` | Date de publication | |
| `$l` | Sous-vedette de forme | |
| `$m` | Langue | |
| `$n` | Clé de tri | |
| `$q` | Version | |
| `$r` | Tonalité | |
| `$s` | Mention d'édition | |
| `$u` | URI | |

---

### 501 – Titre uniforme collectif

| Sous-zone | Libellé | Notes |
|---|---|---|
| `$a` | Titre uniforme collectif | |
| `$j` | Qualificatif de forme | |
| `$l` | Sous-vedette de forme | |
| `$m` | Langue | |

---

### 503 – Variante de titre uniforme

| Sous-zone | Libellé | Notes |
|---|---|---|
| `$a` | Titre uniforme | |
| `$j` | Qualificatif | |

---

### 510 – Titre propre (accès)

| Sous-zone | Libellé | Notes |
|---|---|---|
| `$a` | Titre propre | |
| `$e` | Complément du titre | |
| `$h` | Numéro de partie | |
| `$i` | Titre de partie | |
| `$v` | Numérotation | |
| `$x` | ISSN | |
| `$z` | Langue du titre parallèle | |

---

### 512 à 520 – Variantes de titre

| Sous-zone | Libellé | Notes |
|---|---|---|
| `$a` | Variante de titre | |
| `$e` | Complément | |
| `$h` | Numéro de partie | |
| `$i` | Titre de partie | |
| `$z` | Langue | |

---

### 530 – Titre-clé

| Sous-zone | Libellé | Notes |
|---|---|---|
| `$a` | Titre-clé | Attribué par réseau ISSN |
| `$b` | Qualificatif | |

---

### 531 – Titre abrégé (ISSN)

| Sous-zone | Libellé | Notes |
|---|---|---|
| `$a` | Titre abrégé | |
| `$z` | Langue | |

---

### 532 – Titre développé

| Sous-zone | Libellé | Notes |
|---|---|---|
| `$a` | Titre développé | |

---

### 540 – Titre uniforme (entrée secondaire)

| Sous-zone | Libellé | Notes |
|---|---|---|
| `$a` | Titre uniforme | |
| `$f` | Date de l'œuvre | |
| `$h` | Numéro de partie | |
| `$i` | Titre de partie | |
| `$m` | Langue | |
| `$n` | Clé de tri | |
| `$r` | Tonalité | |

---

### 541 – Titre de traduction

| Sous-zone | Libellé | Notes |
|---|---|---|
| `$a` | Titre | |
| `$e` | Complément | |
| `$z` | Langue | |

---

### 545 – Section / partie

| Sous-zone | Libellé | Notes |
|---|---|---|
| `$a` | Titre de section | |
| `$e` | Complément | |
| `$v` | Numérotation | |

---

## Bloc 6 – Analyse matière

### 600 – Vedette matière – personne

| Sous-zone | Libellé | Notes |
|---|---|---|
| `$a` | Élément d'entrée (nom) | |
| `$b` | Partie de nom autre que l'élément d'entrée | |
| `$c` | Titre de noblesse / qualificatif | |
| `$d` | Numéros romains | |
| `$f` | Dates | |
| `$g` | Développement des initiales | |
| `$j` | Qualificatif de forme | Répétable |
| `$t` | Titre de l'œuvre | |
| `$x` | Subdivision thématique | Répétable |
| `$y` | Subdivision géographique | Répétable |
| `$z` | Subdivision chronologique | Répétable |
| `$2` | Code de système d'indexation | Ex. : rameau, lcsh |
| `$3` | Numéro d'autorité | |

---

### 601 – Vedette matière – collectivité

| Sous-zone | Libellé | Notes |
|---|---|---|
| `$a` | Élément d'entrée (nom) | |
| `$b` | Sous-division | Répétable |
| `$c` | Ajout qualificatif | |
| `$d` | Numéro et date de réunion | |
| `$e` | Lieu de réunion | |
| `$f` | Date de l'œuvre | |
| `$g` | Développement des initiales | |
| `$t` | Titre de l'œuvre | |
| `$x` | Subdivision thématique | Répétable |
| `$y` | Subdivision géographique | Répétable |
| `$z` | Subdivision chronologique | Répétable |
| `$2` | Code du système d'indexation | |
| `$3` | Numéro d'autorité | |

---

### 602 – Vedette matière – famille

| Sous-zone | Libellé | Notes |
|---|---|---|
| `$a` | Nom de famille | |
| `$f` | Dates | |
| `$x` | Subdivision thématique | Répétable |
| `$y` | Subdivision géographique | Répétable |
| `$z` | Subdivision chronologique | Répétable |
| `$2` | Code du système | |

---

### 604 – Vedette matière – nom et titre

| Sous-zone | Libellé | Notes |
|---|---|---|
| `$a` | Nom | |
| `$t` | Titre | |
| `$x` | Subdivision thématique | |
| `$2` | Code du système | |
| `$3` | Numéro d'autorité | |

---

### 605 – Vedette matière – titre

| Sous-zone | Libellé | Notes |
|---|---|---|
| `$a` | Titre uniforme | |
| `$h` | Numéro de partie | |
| `$i` | Titre de partie | |
| `$j` | Qualificatif | |
| `$x` | Subdivision thématique | Répétable |
| `$y` | Subdivision géographique | Répétable |
| `$z` | Subdivision chronologique | Répétable |
| `$2` | Code du système | |
| `$3` | Numéro d'autorité | |

---

### 606 – Vedette matière – nom commun

| Sous-zone | Libellé | Notes |
|---|---|---|
| `$a` | Terme thématique | |
| `$j` | Qualificatif de forme | Répétable |
| `$x` | Subdivision thématique | Répétable |
| `$y` | Subdivision géographique | Répétable |
| `$z` | Subdivision chronologique | Répétable |
| `$2` | Code du système d'indexation | Ex. : rameau |
| `$3` | Numéro d'autorité | |

---

### 607 – Vedette matière – nom géographique

| Sous-zone | Libellé | Notes |
|---|---|---|
| `$a` | Nom géographique | |
| `$j` | Qualificatif | |
| `$x` | Subdivision thématique | Répétable |
| `$y` | Subdivision géographique | Répétable |
| `$z` | Subdivision chronologique | Répétable |
| `$2` | Code du système | |
| `$3` | Numéro d'autorité | |

---

### 608 – Vedette matière – forme / genre

| Sous-zone | Libellé | Notes |
|---|---|---|
| `$a` | Terme de forme / genre | |
| `$j` | Qualificatif | |
| `$x` | Subdivision thématique | |
| `$2` | Code du système | |
| `$3` | Numéro d'autorité | |

---

### 610 – Vedette matière non structurée

| Sous-zone | Libellé | Notes |
|---|---|---|
| `$a` | Terme non contrôlé | Répétable |

---

### 615 – Vedette matière (Rameau / LCSH)

| Sous-zone | Libellé | Notes |
|---|---|---|
| `$a` | Terme | |
| `$x` | Subdivision thématique | Répétable |
| `$y` | Subdivision géographique | Répétable |
| `$z` | Subdivision chronologique | Répétable |
| `$2` | Code du thésaurus | |
| `$3` | Numéro d'autorité | |

---

### 620 – Lieu

| Sous-zone | Libellé | Notes |
|---|---|---|
| `$a` | Continent | |
| `$b` | Pays | |
| `$c` | Province / État | |
| `$d` | Comté / Département | |
| `$e` | Ville | |
| `$f` | Lieu-dit | |

---

### 621 – Lieu de l'action

| Sous-zone | Libellé | Notes |
|---|---|---|
| `$a` | Lieu | |
| `$b` | Pays | |

---

### 660 – Zone géographique (code)

| Sous-zone | Libellé | Notes |
|---|---|---|
| `$a` | Code de zone géographique | |

---

### 661 – Époque

| Sous-zone | Libellé | Notes |
|---|---|---|
| `$a` | Période | |

---

### 676 – Classification Dewey (CDD)

| Sous-zone | Libellé | Notes |
|---|---|---|
| `$a` | Indice Dewey | |
| `$b` | Numéro de table auxiliaire | |
| `$v` | Édition | |
| `$z` | Langue | |

---

### 680 – Classification LC (LCC)

| Sous-zone | Libellé | Notes |
|---|---|---|
| `$a` | Cote LC | |
| `$b` | Numéro Cutter | |

---

### 686 – Autre classification

| Sous-zone | Libellé | Notes |
|---|---|---|
| `$a` | Indice | |
| `$b` | Sous-division | |
| `$c` | Libellé de l'indice | |
| `$2` | Source / système | |

---

## Bloc 7 – Responsabilité intellectuelle

### 700 – Entrée principale – personne physique

| Sous-zone | Libellé | Notes |
|---|---|---|
| `$a` | Élément d'entrée (nom) | |
| `$b` | Partie du nom autre que l'élément d'entrée | |
| `$c` | Qualificatif (titre de noblesse, fonction) | |
| `$d` | Numéros romains | |
| `$f` | Dates | |
| `$g` | Développement des initiales | |
| `$o` | Numéro d'opus | |
| `$p` | Clé musicale | |
| `$r` | Tonalité | |
| `$t` | Titre de l'œuvre | |
| `$3` | Numéro d'autorité | |
| `$4` | Code de relation | Ex. : 070=auteur |

---

### 701 – Entrée secondaire – personne physique

| Sous-zone | Libellé | Notes |
|---|---|---|
| `$a` | Élément d'entrée | |
| `$b` | Partie du nom | |
| `$c` | Qualificatif | |
| `$d` | Numéros romains | |
| `$f` | Dates | |
| `$g` | Initiales | |
| `$t` | Titre de l'œuvre | |
| `$3` | Numéro d'autorité | |
| `$4` | Code de relation | Ex. : 080=éditeur intellectuel, 440=traducteur |

---

### 702 – Entrée secondaire – personne physique (relation alternative)

| Sous-zone | Libellé | Notes |
|---|---|---|
| `$a` | Élément d'entrée | |
| `$b` | Partie du nom | |
| `$c` | Qualificatif | |
| `$f` | Dates | |
| `$3` | Numéro d'autorité | |
| `$4` | Code de relation | |

---

### 710 – Entrée principale – collectivité

| Sous-zone | Libellé | Notes |
|---|---|---|
| `$a` | Élément d'entrée (nom) | |
| `$b` | Sous-division (département, section) | Répétable |
| `$c` | Ajout qualificatif | |
| `$d` | Numéro et date de réunion | |
| `$e` | Lieu de réunion | |
| `$f` | Date de l'œuvre | |
| `$g` | Développement des initiales | |
| `$t` | Titre de l'œuvre | |
| `$3` | Numéro d'autorité | |
| `$4` | Code de relation | |

---

### 711 – Entrée secondaire – collectivité

| Sous-zone | Libellé | Notes |
|---|---|---|
| `$a` | Élément d'entrée | |
| `$b` | Sous-division | Répétable |
| `$c` | Qualificatif | |
| `$d` | Numéro et date de réunion | |
| `$e` | Lieu de réunion | |
| `$f` | Date | |
| `$t` | Titre | |
| `$3` | Numéro d'autorité | |
| `$4` | Code de relation | |

---

### 712 – Entrée secondaire – collectivité (relation alternative)

| Sous-zone | Libellé | Notes |
|---|---|---|
| `$a` | Élément d'entrée | |
| `$b` | Sous-division | |
| `$c` | Qualificatif | |
| `$3` | Numéro d'autorité | |
| `$4` | Code de relation | |

---

### 720 – Entrée principale – famille

| Sous-zone | Libellé | Notes |
|---|---|---|
| `$a` | Nom de famille | |
| `$f` | Dates | |
| `$3` | Numéro d'autorité | |
| `$4` | Code de relation | |

---

### 721 – Entrée secondaire – famille

| Sous-zone | Libellé | Notes |
|---|---|---|
| `$a` | Nom de famille | |
| `$f` | Dates | |
| `$3` | Numéro d'autorité | |
| `$4` | Code de relation | |

---

### 730 – Entrée principale – titre uniforme

| Sous-zone | Libellé | Notes |
|---|---|---|
| `$a` | Titre uniforme | |
| `$f` | Date de l'œuvre | |
| `$h` | Numéro de partie | |
| `$i` | Titre de partie | |
| `$m` | Langue | |
| `$n` | Clé de tri | |
| `$r` | Tonalité | |
| `$3` | Numéro d'autorité | |

---

## Bloc 8 – Usage international

### 801 – Source de la notice

| Sous-zone | Libellé | Notes |
|---|---|---|
| `$a` | Pays | Code ISO 3166 |
| `$b` | Organisme | Code ISIL ou NCP |
| `$c` | Date de la transaction | AAAAMMJJ |
| `$g` | Règles de catalogage | Ex. : AFNOR, ISBD, RDA |
| `$2` | Source des données codées | |

---

### 802 – Identifiant ISSN de l'édition

| Sous-zone | Libellé | Notes |
|---|---|---|
| `$a` | ISSN | |

---

### 830 – Informations générales de gestion

| Sous-zone | Libellé | Notes |
|---|---|---|
| `$a` | Texte | |

---

### 852 – Localisation et cote

| Sous-zone | Libellé | Notes |
|---|---|---|
| `$a` | Institution / bibliothèque | Code ISIL |
| `$b` | Localisation (rayon, salle) | |
| `$c` | Sous-localisation | |
| `$d` | Ancien numéro de rangement | |
| `$e` | Adresse | |
| `$f` | Indice de classement | |
| `$g` | Parties non classées par cote | |
| `$h` | Cote (partie classification) | |
| `$i` | Cote (partie item) | |
| `$j` | Cote libre | |
| `$k` | Préfixe de cote | |
| `$l` | Suffixe de cote | |
| `$m` | Note de transaction | |
| `$n` | Numéro de pays | |
| `$p` | Code-barres | |
| `$q` | Qualificatif de cote | |
| `$r` | Numéro de séquence | |
| `$s` | Statut (Copyright) | |
| `$t` | Numéro d'exemplaire | |
| `$u` | URI | |
| `$x` | Note non publique | |
| `$z` | Note publique | |
| `$2` | Source du système de cotation | Ex. : ddc, lcc, udc |
| `$9` | Numéro interne | |

---

### 856 – Localisation et accès électronique

| Sous-zone | Libellé | Notes |
|---|---|---|
| `$a` | Nom d'hôte | |
| `$b` | Numéro d'accès | |
| `$c` | Informations de compression | |
| `$d` | Chemin d'accès | |
| `$f` | Nom de fichier | Répétable |
| `$g` | URI persistant (PURL) | |
| `$i` | Instruction | |
| `$j` | Bits par seconde | |
| `$k` | Mot de passe | |
| `$l` | Login | |
| `$m` | Contact pour accès | |
| `$n` | Nom de réseau | |
| `$o` | Système opérateur requis | |
| `$p` | Port | |
| `$q` | Format électronique | |
| `$r` | Paramètres | |
| `$s` | Taille du fichier | Répétable |
| `$t` | Terminal requis | |
| `$u` | URI (URL) | Répétable |
| `$v` | Heures d'accès | |
| `$w` | Numéro de notice | |
| `$x` | Note non publique | |
| `$y` | Texte du lien | |
| `$z` | Note publique | |
| `$2` | Méthode d'accès | |
| `$3` | Matériaux spécifiés | |

---

## Bloc 9 – Usage local

### 9xx – Zones locales

| Sous-zone | Libellé | Notes |
|---|---|---|
| `$a`–`$z` | Définition locale | Entièrement à la discrétion de l'institution |
| `$0`–`$9` | Définition locale | |

---

*Source : Manuel UNIMARC Format Bibliographique – Transition Bibliographique / IFLA*
