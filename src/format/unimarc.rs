use crate::encoding::Encoding;
use crate::error::MarcError;
use crate::format::{apply_mappings, build_iso2709, ReverseFieldData};
use crate::raw::RawRecord;
use crate::record::{
    Agent, CatalogingSource, Classification, ClassificationScheme, CorporateBody, Coden,
    ElectronicLocation, Language, LocationCallNumber, MarcDate, Person, Publication,
    PublisherNumber, Record, Specimen, TargetAudience, Title,
};

pub fn detect_encoding(record: &RawRecord<'_>) -> Result<Encoding, MarcError> {
    // UNIMARC: field 100$a positions 26-27 = character set.
    let mut encoding = Encoding::Utf8;
    if let Ok(fields) = record.fields() {
        for f in fields {
            if let crate::raw::RawField::Data { tag, body, .. } = f {
                if &tag == b"100" {
                    // find $a
                    let mut pos = 0;
                    while pos < body.len() {
                        if body[pos] == 0x1F {
                            if pos + 1 >= body.len() {
                                break;
                            }
                            let code = body[pos + 1];
                            let start = pos + 2;
                            let mut end = start;
                            while end < body.len() && body[end] != 0x1F && body[end] != 0x1E {
                                end += 1;
                            }
                            if code == b'a' && end >= start + 28 {
                                encoding = match &body[start + 26..start + 28] {
                                    b"50" => Encoding::Utf8,
                                    b"01" => Encoding::Iso5426,
                                    b"02" => Encoding::Other(encoding_rs::ISO_8859_2),
                                    b"03" => Encoding::Other(encoding_rs::ISO_8859_3),
                                    b"05" => Encoding::Other(encoding_rs::ISO_8859_5),
                                    _ => Encoding::Utf8,
                                };
                                break; 
                            }
                            pos = end;
                        } else if body[pos] == 0x1E {
                            break;
                        } else {
                            pos += 1;
                        }
                    }
                }
            }
        }
    }
    Ok(encoding)
}

pub fn to_record(encoding: &Encoding, record: &RawRecord<'_>) -> Result<Record, MarcError> {
    apply_mappings(encoding, record, MAPPINGS)
}

pub fn to_raw(encoding: &Encoding, record: &Record) -> Result<Vec<u8>, MarcError> {
    let mut fields: Vec<([u8; 3], ReverseFieldData)> = Vec::new();
    for mapping in REVERSE_MAPPINGS {
        for data in (mapping.build)(record) {
            fields.push((mapping.tag, data));
        }
    }
    build_iso2709(encoding, &record.leader, &fields)
}

fn marc_date_to_string(d: &MarcDate) -> String {
    match d {
        MarcDate::Exact(date) => date.format("%Y-%m-%d").to_string(),
        MarcDate::YearMonth { year, month } => format!("{:04}-{:02}", year, month),
        MarcDate::Year(y) => format!("{:04}", y),
        MarcDate::Range { start, end } => format!("{:04}-{:04}", start, end),
        MarcDate::Approximate(s) => s.clone(),
        MarcDate::Unknown => String::new(),
    }
}

fn specimen_to_995(s: &Specimen) -> Option<ReverseFieldData> {
    let mut subfields: Vec<(u8, String)> = Vec::new();
    if let Some(v) = &s.library { if !v.trim().is_empty() { subfields.push((b'a', v.clone())); } }
    if let Some(v) = &s.section { if !v.trim().is_empty() { subfields.push((b'b', v.clone())); } }
    if let Some(v) = &s.sub_library { if !v.trim().is_empty() { subfields.push((b'c', v.clone())); } }
    if let Some(v) = &s.section_code { if !v.trim().is_empty() { subfields.push((b'd', v.clone())); } }
    if let Some(v) = &s.barcode { if !v.trim().is_empty() { subfields.push((b'f', v.clone())); } }
    if let Some(v) = &s.call_number { if !v.trim().is_empty() { subfields.push((b'k', v.clone())); } }
    if let Some(v) = &s.inventory_number { if !v.trim().is_empty() { subfields.push((b'l', v.clone())); } }
    if let Some(v) = &s.creation_date {
        let val = marc_date_to_string(v);
        if !val.trim().is_empty() { subfields.push((b'm', val)); }
    }
    if let Some(v) = &s.modification_date {
        let val = marc_date_to_string(v);
        if !val.trim().is_empty() { subfields.push((b'n', val)); }
    }
    if let Some(v) = &s.item_type { if !v.trim().is_empty() { subfields.push((b'r', v.clone())); } }
    if let Some(v) = &s.record_control_number { if !v.trim().is_empty() { subfields.push((b't', v.clone())); } }
    if let Some(v) = &s.document_type { if !v.trim().is_empty() { subfields.push((b'v', v.clone())); } }
    if let Some(v) = &s.circulation_status { if !v.trim().is_empty() { subfields.push((b'w', v.clone())); } }
    if subfields.is_empty() { None }
    else { Some(ReverseFieldData::Data { ind1: b' ', ind2: b' ', subfields }) }
}

fn ensure_publication(rec: &mut Record) {
    if rec.description.publication.is_empty() {
        rec.description.publication.push(Publication {
            place: None, publisher: None, date: None, function: None,
        });
    }
}

fn ensure_cataloging_source(rec: &mut Record) {
    if rec.international.cataloging_sources.is_empty() {
        rec.international.cataloging_sources.push(CatalogingSource::default());
    }
}

fn set_last_specimen<F: FnOnce(&mut Specimen)>(rec: &mut Record, f: F) {
    if rec.local.specimens.is_empty() {
        rec.local.specimens.push(Specimen::default());
    }
    if let Some(s) = rec.local.specimens.last_mut() { f(s); }
}

fn parse_marc_date(s: &str) -> MarcDate {
    let s = s.trim();
    if s.is_empty() { return MarcDate::Unknown; }
    if let Ok(d) = chrono::NaiveDate::parse_from_str(s, "%Y-%m-%d") {
        return MarcDate::Exact(d);
    }
    let digits: String = s.chars().filter(|c| c.is_ascii_digit()).collect();
    if digits.len() >= 4 {
        if let Ok(y) = digits[..4].parse::<u16>() { return MarcDate::Year(y); }
    }
    MarcDate::Approximate(s.to_string())
}

crate::marc_bimap! {
    b"001", control => set identification.record_id;
    b"003", control => set identification.agency_id;
    b"005", control => set identification.record_version_date;

    b"010" => {
        b'a' => |data, rec| {
            let v = data.trim();
            if !v.is_empty() {
                rec.identification.isbn.push(crate::record::Isbn { value: v.to_string(), qualifying: None });
            }
        };
        b'b' => last identification.isbn -> qualifying;
    } reverse |rec| rec.identification.isbn.iter()
        .filter_map(|isbn| {
            if isbn.value.trim().is_empty() { return None; }
            let mut subfields = vec![(b'a', isbn.value.clone())];
            if let Some(q) = &isbn.qualifying {
                if !q.trim().is_empty() { subfields.push((b'b', q.clone())); }
            }
            Some(ReverseFieldData::Data { ind1: b' ', ind2: b' ', subfields })
        })
        .collect();

    b"011" => {
        b'a' => |data, rec| {
            let v = data.trim();
            if !v.is_empty() {
                rec.identification.issn.push(crate::record::Issn { value: v.to_string(), qualifying: None });
            }
        };
        b'b' => last identification.issn -> qualifying;
    } reverse |rec| rec.identification.issn.iter()
        .filter_map(|issn| {
            if issn.value.trim().is_empty() { return None; }
            let mut subfields = vec![(b'a', issn.value.clone())];
            if let Some(q) = &issn.qualifying {
                if !q.trim().is_empty() { subfields.push((b'b', q.clone())); }
            }
            Some(ReverseFieldData::Data { ind1: b' ', ind2: b' ', subfields })
        })
        .collect();

    // 040 - CODEN
    b"040" => {
        b'a' => |data, rec| {
            let v = data.trim();
            if !v.is_empty() {
                rec.identification.codens.push(Coden {
                    value: v.to_string(),
                    canceled_or_invalid: false,
                });
            }
        };
        b'z' => |data, rec| {
            let v = data.trim();
            if !v.is_empty() {
                rec.identification.codens.push(Coden {
                    value: v.to_string(),
                    canceled_or_invalid: true,
                });
            }
        };
    } reverse |rec| rec.identification.codens.iter()
        .filter_map(|coden| {
            let v = coden.value.trim();
            if v.is_empty() { return None; }
            let code = if coden.canceled_or_invalid { b'z' } else { b'a' };
            Some(ReverseFieldData::Data {
                ind1: b' ', ind2: b' ',
                subfields: vec![(code, v.to_string())],
            })
        })
        .collect();

    // 071 - Publisher's number
    b"071" => {
        b'a' => |data, rec| {
            let v = data.trim();
            if !v.is_empty() {
                rec.identification.publisher_numbers.push(PublisherNumber {
                    value: v.to_string(),
                    source: None,
                    canceled_or_invalid: false,
                });
            }
        };
        b'b' => |data, rec| {
            let v = data.trim();
            if !v.is_empty() {
                if let Some(last) = rec.identification.publisher_numbers.last_mut() {
                    last.source = Some(v.to_string());
                }
            }
        };
        b'z' => |data, rec| {
            let v = data.trim();
            if !v.is_empty() {
                rec.identification.publisher_numbers.push(PublisherNumber {
                    value: v.to_string(),
                    source: None,
                    canceled_or_invalid: true,
                });
            }
        };
    } reverse |rec| rec.identification.publisher_numbers.iter()
        .filter_map(|num| {
            let v = num.value.trim();
            if v.is_empty() { return None; }
            let code = if num.canceled_or_invalid { b'z' } else { b'a' };
            let mut subfields = vec![(code, v.to_string())];
            if let Some(src) = &num.source {
                if !src.trim().is_empty() {
                    subfields.push((b'b', src.clone()));
                }
            }
            Some(ReverseFieldData::Data { ind1: b' ', ind2: b' ', subfields })
        })
        .collect();

    b"015", b'a' => push identification.national_bibliography_numbers;
    b"016", b'a' => push identification.national_library_record_numbers;
    b"017", b'a' => push identification.legal_deposit_numbers;
    b"020", b'a' => push identification.lccn;
    b"035", b'a' => push identification.system_control_numbers;

    b"100", b'a', slice 17, 3 => |data, rec| {
        let code = data.trim();
        rec.coded.target_audience = Some(match code {
            "000" | "   " => TargetAudience::General,
            "010" => TargetAudience::Juvenile,
            "020" => TargetAudience::YoungAdult,
            _ => TargetAudience::Other(code.to_string()),
        });
    }, reverse |rec| {
        let Some(audience) = &rec.coded.target_audience else { return Vec::new(); };
        let code = match audience {
            TargetAudience::General => "000",
            TargetAudience::Juvenile => "010",
            TargetAudience::YoungAdult => "020",
            TargetAudience::Specialized => "030",
            TargetAudience::Unknown => "   ",
            TargetAudience::Other(s) => s.as_str(),
        };
        let mut buf = [b' '; 36];
        let bytes = code.as_bytes();
        let len = bytes.len().min(3);
        buf[17..17 + len].copy_from_slice(&bytes[..len]);
        vec![ReverseFieldData::Data {
            ind1: b' ', ind2: b' ',
            subfields: vec![(b'a', String::from_utf8_lossy(&buf).into_owned())],
        }]
    };

    b"101", b'a' => |data, rec| {
        let code = data.trim();
        if !code.is_empty() { rec.coded.languages.push(Language::from(code)); }
    }, reverse |rec| rec.coded.languages.iter()
        .map(|lang| ReverseFieldData::Data {
            ind1: b' ', ind2: b' ',
            subfields: vec![(b'a', lang.code().to_string())],
        })
        .collect();

    b"102", b'a' => |data, rec| {
        let code = data.trim();
        if !code.is_empty() { rec.coded.country = Some(crate::record::Country::from(code)); }
    }, reverse |rec| rec.coded.country.as_ref()
        .map(|c| vec![ReverseFieldData::Data {
            ind1: b' ', ind2: b' ',
            subfields: vec![(b'a', c.code().to_string())],
        }])
        .unwrap_or_default();

    b"200" => {
        b'a' => |data, rec| {
            let title = rec.description.title.get_or_insert_with(|| Title {
                main: String::new(), subtitle: None, parallel: Vec::new(), responsibility: None,
            });
            title.main = data.trim_end_matches('/').trim().to_string();
        };
        b'e' => |data, rec| {
            let title = rec.description.title.get_or_insert_with(|| Title {
                main: String::new(), subtitle: None, parallel: Vec::new(), responsibility: None,
            });
            title.subtitle = Some(data.trim_end_matches('/').trim().to_string());
        };
        b'f' => |data, rec| {
            let title = rec.description.title.get_or_insert_with(|| Title {
                main: String::new(), subtitle: None, parallel: Vec::new(), responsibility: None,
            });
            title.responsibility = Some(data.trim().to_string());
        };
        b'd' => |data, rec| {
            let v = data.trim_end_matches('/').trim();
            if !v.is_empty() {
                let title = rec.description.title.get_or_insert_with(|| Title {
                    main: String::new(), subtitle: None, parallel: Vec::new(), responsibility: None,
                });
                title.parallel.push(v.to_string());
            }
        };
    } reverse |rec| {
        let Some(title) = &rec.description.title else { return Vec::new(); };
        let mut subfields: Vec<(u8, String)> = Vec::new();
        if !title.main.trim().is_empty() { subfields.push((b'a', title.main.clone())); }
        if let Some(e) = &title.subtitle {
            if !e.trim().is_empty() { subfields.push((b'e', e.clone())); }
        }
        for d in &title.parallel {
            if !d.trim().is_empty() { subfields.push((b'd', d.clone())); }
        }
        if let Some(f) = &title.responsibility {
            if !f.trim().is_empty() { subfields.push((b'f', f.clone())); }
        }
        if subfields.is_empty() { Vec::new() }
        else { vec![ReverseFieldData::Data { ind1: b' ', ind2: b' ', subfields }] }
    };

    b"205", b'a' => set description.edition;

    b"210" => {
        b'a' => |data, rec| {
            ensure_publication(rec);
            if let Some(first) = rec.description.publication.first_mut() {
                first.place = Some(data.trim_matches(|c| c == '[' || c == ']').trim().to_string());
            }
        };
        b'c' => |data, rec| {
            ensure_publication(rec);
            if let Some(first) = rec.description.publication.first_mut() {
                first.publisher = Some(data.trim_matches(|c| c == '[' || c == ']').trim().to_string());
            }
        };
        b'd' => |data, rec| {
            ensure_publication(rec);
            if let Some(first) = rec.description.publication.first_mut() {
                first.date = Some(parse_marc_date(data));
            }
        };
    } reverse |rec| {
        let Some(first) = rec.description.publication.first() else { return Vec::new(); };
        let mut subfields = Vec::new();
        if let Some(a) = &first.place {
            if !a.trim().is_empty() { subfields.push((b'a', a.clone())); }
        }
        if let Some(c) = &first.publisher {
            if !c.trim().is_empty() { subfields.push((b'c', c.clone())); }
        }
        if let Some(d) = &first.date {
            let s = marc_date_to_string(d);
            if !s.trim().is_empty() { subfields.push((b'd', s)); }
        }
        if subfields.is_empty() { Vec::new() }
        else { vec![ReverseFieldData::Data { ind1: b' ', ind2: b' ', subfields }] }
    };

    b"215" => init description.physical_description {
        b'a' => extent, b'b' => other_physical_details,
        b'c' => dimensions, b'd' => accompanying_material,
    };

    b"225", b'a' => set description.series_statement;

    b"676", b'a' => |data, rec| {
        let v = data.trim();
        if !v.is_empty() {
            rec.indexing.classifications.push(Classification {
                scheme: ClassificationScheme::Dewey, number: v.to_string(),
            });
        }
    }, reverse |rec| rec.indexing.classifications.iter()
        .filter_map(|c| match c.scheme {
            ClassificationScheme::Dewey if !c.number.trim().is_empty() =>
                Some(ReverseFieldData::Data { ind1: b' ', ind2: b' ', subfields: vec![(b'a', c.number.clone())] }),
            _ => None,
        })
        .collect();

    b"700" => {
        b'a' => |data, rec| {
            let v = data.trim();
            if !v.is_empty() {
                rec.responsibility.main_entry =
                    Some(Agent::Person(Person { name: v.to_string(), forename: None, dates: None }));
            }
        };
        b'b' => |data, rec| {
            let v = data.trim();
            if !v.is_empty() {
                if let Some(Agent::Person(p)) = &mut rec.responsibility.main_entry {
                    p.forename = Some(v.to_string());
                }
            }
        };
    } reverse |rec| match &rec.responsibility.main_entry {
        Some(Agent::Person(p)) if !p.name.trim().is_empty() => {
            let mut subfields = vec![(b'a', p.name.clone())];
            if let Some(f) = &p.forename {
                if !f.trim().is_empty() { subfields.push((b'b', f.clone())); }
            }
            vec![ReverseFieldData::Data { ind1: b' ', ind2: b' ', subfields }]
        }
        _ => Vec::new(),
    };

    b"701" => {
        b'a' => |data, rec| {
            let v = data.trim();
            if !v.is_empty() {
                rec.responsibility.added_entries.push(
                    Agent::Person(Person { name: v.to_string(), forename: None, dates: None })
                );
            }
        };
        b'b' => |data, rec| {
            let v = data.trim();
            if !v.is_empty() {
                if let Some(Agent::Person(p)) = rec.responsibility.added_entries.last_mut() {
                    p.forename = Some(v.to_string());
                }
            }
        };
    } reverse |rec| rec.responsibility.added_entries.iter()
        .filter_map(|a| match a {
            Agent::Person(p) if !p.name.trim().is_empty() => {
                let mut subfields = vec![(b'a', p.name.clone())];
                if let Some(f) = &p.forename {
                    if !f.trim().is_empty() { subfields.push((b'b', f.clone())); }
                }
                Some(ReverseFieldData::Data { ind1: b' ', ind2: b' ', subfields })
            }
            _ => None,
        })
        .collect();

    b"702", b'a' => |data, rec| {
        let v = data.trim();
        if !v.is_empty() {
            rec.responsibility.added_entries.push(
                Agent::CorporateBody(CorporateBody { name: v.to_string() })
            );
        }
    }, reverse |rec| rec.responsibility.added_entries.iter()
        .filter_map(|a| match a {
            Agent::CorporateBody(c) if !c.name.trim().is_empty() =>
                Some(ReverseFieldData::Data { ind1: b' ', ind2: b' ', subfields: vec![(b'a', c.name.clone())] }),
            _ => None,
        })
        .collect();

    b"710", b'a' => |data, rec| {
        let v = data.trim();
        if !v.is_empty() {
            rec.responsibility.main_entry =
                Some(Agent::CorporateBody(CorporateBody { name: v.to_string() }));
        }
    }, reverse |rec| match &rec.responsibility.main_entry {
        Some(Agent::CorporateBody(c)) if !c.name.trim().is_empty() =>
            vec![ReverseFieldData::Data { ind1: b' ', ind2: b' ', subfields: vec![(b'a', c.name.clone())] }],
        _ => Vec::new(),
    };

    b"801" => {
        b'a' => |data, rec| {
            let v = data.trim();
            if !v.is_empty() {
                rec.international.cataloging_sources.push(CatalogingSource {
                    country: Some(v.to_string()), ..CatalogingSource::default()
                });
            }
        };
        b'b' => |data, rec| {
            let v = data.trim();
            if !v.is_empty() {
                ensure_cataloging_source(rec);
                if let Some(last) = rec.international.cataloging_sources.last_mut() {
                    last.agency = Some(v.to_string());
                }
            }
        };
        b'c' => |data, rec| {
            let v = data.trim();
            if !v.is_empty() {
                ensure_cataloging_source(rec);
                if let Some(last) = rec.international.cataloging_sources.last_mut() {
                    last.date = Some(parse_marc_date(v));
                }
            }
        };
        b'g' => |data, rec| {
            let v = data.trim();
            if !v.is_empty() {
                ensure_cataloging_source(rec);
                if let Some(last) = rec.international.cataloging_sources.last_mut() {
                    last.transcription_conventions = Some(v.to_string());
                }
            }
        };
    } reverse |rec| rec.international.cataloging_sources.iter()
        .filter_map(|s| {
            let mut subfields = Vec::new();
            if let Some(a) = &s.country { if !a.trim().is_empty() { subfields.push((b'a', a.clone())); } }
            if let Some(b) = &s.agency { if !b.trim().is_empty() { subfields.push((b'b', b.clone())); } }
            if let Some(c) = &s.date {
                let v = marc_date_to_string(c);
                if !v.trim().is_empty() { subfields.push((b'c', v)); }
            }
            if let Some(g) = &s.transcription_conventions { if !g.trim().is_empty() { subfields.push((b'g', g.clone())); } }
            if subfields.is_empty() { None }
            else { Some(ReverseFieldData::Data { ind1: b' ', ind2: b' ', subfields }) }
        })
        .collect();

    b"852" => {
        b'a' => |data, rec| {
            let v = data.trim();
            if !v.is_empty() {
                rec.international.location_call_numbers.push(LocationCallNumber {
                    location: Some(v.to_string()), sublocation: None, call_number: None,
                });
            }
        };
        b'b' => |data, rec| {
            let v = data.trim();
            if !v.is_empty() {
                if rec.international.location_call_numbers.is_empty() {
                    rec.international.location_call_numbers.push(LocationCallNumber::default());
                }
                if let Some(last) = rec.international.location_call_numbers.last_mut() {
                    last.sublocation = Some(v.to_string());
                }
            }
        };
        b'j' => |data, rec| {
            let v = data.trim();
            if !v.is_empty() {
                if rec.international.location_call_numbers.is_empty() {
                    rec.international.location_call_numbers.push(LocationCallNumber::default());
                }
                if let Some(last) = rec.international.location_call_numbers.last_mut() {
                    last.call_number = Some(v.to_string());
                }
            }
        };
    } reverse |rec| rec.international.location_call_numbers.iter()
        .filter_map(|l| {
            let mut subfields = Vec::new();
            if let Some(a) = &l.location { if !a.trim().is_empty() { subfields.push((b'a', a.clone())); } }
            if let Some(b) = &l.sublocation { if !b.trim().is_empty() { subfields.push((b'b', b.clone())); } }
            if let Some(j) = &l.call_number { if !j.trim().is_empty() { subfields.push((b'j', j.clone())); } }
            if subfields.is_empty() { None }
            else { Some(ReverseFieldData::Data { ind1: b' ', ind2: b' ', subfields }) }
        })
        .collect();

    b"856" => {
        b'u' => |data, rec| {
            let v = data.trim();
            if !v.is_empty() {
                rec.international.electronic_locations.push(ElectronicLocation {
                    uri: Some(v.to_string()), public_note: None,
                });
            }
        };
        b'z' => |data, rec| {
            let v = data.trim();
            if !v.is_empty() {
                if let Some(last) = rec.international.electronic_locations.last_mut() {
                    last.public_note = Some(v.to_string());
                }
            }
        };
    } reverse |rec| rec.international.electronic_locations.iter()
        .filter_map(|e| {
            let mut subfields = Vec::new();
            if let Some(u) = &e.uri { if !u.trim().is_empty() { subfields.push((b'u', u.clone())); } }
            if let Some(z) = &e.public_note { if !z.trim().is_empty() { subfields.push((b'z', z.clone())); } }
            if subfields.is_empty() { None }
            else { Some(ReverseFieldData::Data { ind1: b' ', ind2: b' ', subfields }) }
        })
        .collect();

    b"995" => {
        b'a' => |data, rec| {
            let v = data.trim();
            if !v.is_empty() {
                rec.local.specimens.push(Specimen { library: Some(v.to_string()), ..Specimen::default() });
            }
        };
        b'b' => |data, rec| { set_last_specimen(rec, |s| s.section = Some(data.trim().to_string())); };
        b'c' => |data, rec| { set_last_specimen(rec, |s| s.sub_library = Some(data.trim().to_string())); };
        b'd' => |data, rec| { set_last_specimen(rec, |s| s.section_code = Some(data.trim().to_string())); };
        b'f' => |data, rec| { set_last_specimen(rec, |s| s.barcode = Some(data.trim().to_string())); };
        b'k' => |data, rec| { set_last_specimen(rec, |s| s.call_number = Some(data.trim().to_string())); };
        b'l' => |data, rec| { set_last_specimen(rec, |s| s.inventory_number = Some(data.trim().to_string())); };
        b'm' => |data, rec| { set_last_specimen(rec, |s| s.creation_date = Some(parse_marc_date(data))); };
        b'n' => |data, rec| { set_last_specimen(rec, |s| s.modification_date = Some(parse_marc_date(data))); };
        b'r' => |data, rec| { set_last_specimen(rec, |s| s.item_type = Some(data.trim().to_string())); };
        b't' => |data, rec| { set_last_specimen(rec, |s| s.record_control_number = Some(data.trim().to_string())); };
        b'v' => |data, rec| { set_last_specimen(rec, |s| s.document_type = Some(data.trim().to_string())); };
        b'w' => |data, rec| { set_last_specimen(rec, |s| s.circulation_status = Some(data.trim().to_string())); };
    } reverse |rec| rec.local.specimens.iter()
        .filter_map(|s| specimen_to_995(s))
        .collect();
}

