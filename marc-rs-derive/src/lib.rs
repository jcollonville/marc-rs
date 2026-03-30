use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::{Attribute, Data, DeriveInput, Field, Fields, Ident, Type};

#[proc_macro_derive(MarcPaths, attributes(marc))]
pub fn derive_marc_paths(input: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(input as DeriveInput);
    match impl_marc_paths(&input) {
        Ok(ts) => ts.into(),
        Err(e) => e.to_compile_error().into(),
    }
}

// ── Field classification ────────────────────────────────────────────────────

enum FieldCat {
    Str,
    OptStr,
    VecStr,
    OptType(Type),
    VecType(Type),
    BareType(Type),
}

struct FInfo {
    ident: Ident,
    cat: FieldCat,
}

fn has_skip(attrs: &[Attribute]) -> bool {
    for attr in attrs {
        if attr.path().is_ident("marc") {
            let mut found = false;
            let _ = attr.parse_nested_meta(|meta| {
                if meta.path.is_ident("skip") {
                    found = true;
                }
                Ok(())
            });
            if found {
                return true;
            }
        }
    }
    false
}

fn classify(ty: &Type) -> FieldCat {
    if let Type::Path(tp) = ty {
        if let Some(seg) = tp.path.segments.last() {
            let id = seg.ident.to_string();
            if id == "String" {
                return FieldCat::Str;
            }
            if id == "Option" || id == "Vec" {
                if let syn::PathArguments::AngleBracketed(ref ab) = seg.arguments {
                    if let Some(syn::GenericArgument::Type(inner)) = ab.args.first() {
                        let is_string = matches!(inner, Type::Path(ip) if ip.path.segments.last().map(|s| s.ident == "String").unwrap_or(false));
                        if is_string {
                            return if id == "Option" {
                                FieldCat::OptStr
                            } else {
                                FieldCat::VecStr
                            };
                        }
                        return if id == "Option" {
                            FieldCat::OptType(inner.clone())
                        } else {
                            FieldCat::VecType(inner.clone())
                        };
                    }
                }
            }
        }
    }
    FieldCat::BareType(ty.clone())
}

fn creator_name(fields: &[FInfo]) -> String {
    for f in fields {
        match &f.cat {
            FieldCat::Str | FieldCat::OptStr => return f.ident.to_string(),
            _ => {}
        }
    }
    String::new()
}

// ── Code generation ─────────────────────────────────────────────────────────

fn impl_marc_paths(input: &DeriveInput) -> syn::Result<TokenStream2> {
    let name = &input.ident;

    let all_fields: Vec<&Field> = match &input.data {
        Data::Struct(ds) => match &ds.fields {
            Fields::Named(nf) => nf.named.iter().collect(),
            _ => return Err(syn::Error::new_spanned(name, "expected named fields")),
        },
        _ => return Err(syn::Error::new_spanned(name, "expected a struct")),
    };

    let active: Vec<FInfo> = all_fields
        .iter()
        .filter(|f| !has_skip(&f.attrs))
        .map(|f| FInfo {
            ident: f.ident.clone().unwrap(),
            cat: classify(&f.ty),
        })
        .collect();

    let creator = creator_name(&active);

    let set_body = gen_set(&active);
    let get_option_body = gen_get_option(&active);
    let get_vec_body = gen_get_vec(&active);
    let path_kind_body = gen_path_kind(&active);
    let has_path_body = gen_has_path(&active);
    let is_vec_leaf_body = gen_is_vec_leaf(&active);
    let _from_marc = gen_from_marc(&active);
    let _to_marc = gen_to_marc(&active);

    Ok(quote! {
        impl crate::record::MarcPaths for #name {
            const IS_LEAF: bool = false;

            fn from_marc_str(_s: &str) -> Self { unreachable!("container") }
            fn to_marc_str(&self) -> String { unreachable!("container") }

            fn marc_set(&mut self, path: &str, value: &str) -> bool {
                #set_body
                false
            }

            fn marc_get_option(&self, path: &str) -> Option<String> {
                #get_option_body
                None
            }

            fn marc_get_vec(&self, path: &str) -> Option<Vec<String>> {
                #get_vec_body
                None
            }

            fn marc_path_kind(path: &str) -> Option<crate::record::PathKind> {
                #path_kind_body
                None
            }

            fn marc_has_path(path: &str) -> bool {
                #has_path_body
                false
            }

            fn marc_is_vec_leaf(path: &str) -> bool {
                #is_vec_leaf_body
                false
            }

            fn marc_creator_field() -> &'static str { #creator }
        }
    })
}

// ── marc_set ────────────────────────────────────────────────────────────────

fn gen_set(fields: &[FInfo]) -> TokenStream2 {
    let arms: Vec<TokenStream2> = fields.iter().map(|f| {
        let id = &f.ident;
        let name_s = id.to_string();
        let prefix = format!("{}.", name_s);
        match &f.cat {
            FieldCat::Str => quote! {
                if path == #name_s { self.#id = value.to_string(); return true; }
            },
            FieldCat::OptStr => quote! {
                if path == #name_s { self.#id = Some(value.to_string()); return true; }
            },
            FieldCat::VecStr => quote! {
                if path == #name_s { self.#id.push(value.to_string()); return true; }
            },
            FieldCat::OptType(inner) => quote! {
                if <#inner as crate::record::MarcPaths>::IS_LEAF {
                    if path == #name_s {
                        self.#id = Some(<#inner as crate::record::MarcPaths>::from_marc_str(value));
                        return true;
                    }
                } else if let Some(rest) = path.strip_prefix(#prefix) {
                    let inner = self.#id.get_or_insert_with(#inner::default);
                    return inner.marc_set(rest, value);
                }
            },
            FieldCat::VecType(inner) => quote! {
                if <#inner as crate::record::MarcPaths>::IS_LEAF {
                    if path == #name_s {
                        self.#id.push(<#inner as crate::record::MarcPaths>::from_marc_str(value));
                        return true;
                    }
                } else if let Some(rest) = path.strip_prefix(#prefix) {
                    if rest == <#inner as crate::record::MarcPaths>::marc_creator_field() {
                        let mut item = #inner::default();
                        item.marc_set(rest, value);
                        self.#id.push(item);
                        return true;
                    } else if let Some(last) = self.#id.last_mut() {
                        return last.marc_set(rest, value);
                    }
                    return false;
                }
            },
            FieldCat::BareType(inner) => quote! {
                if <#inner as crate::record::MarcPaths>::IS_LEAF {
                    if path == #name_s {
                        self.#id = <#inner as crate::record::MarcPaths>::from_marc_str(value);
                        return true;
                    }
                } else if let Some(rest) = path.strip_prefix(#prefix) {
                    return self.#id.marc_set(rest, value);
                }
            },
        }
    }).collect();
    quote! { #(#arms)* }
}

// ── marc_get_option ─────────────────────────────────────────────────────────

fn gen_get_option(fields: &[FInfo]) -> TokenStream2 {
    let arms: Vec<TokenStream2> = fields.iter().map(|f| {
        let id = &f.ident;
        let name_s = id.to_string();
        let prefix = format!("{}.", name_s);
        match &f.cat {
            FieldCat::Str => quote! {
                if path == #name_s { return Some(self.#id.clone()); }
            },
            FieldCat::OptStr => quote! {
                if path == #name_s { return self.#id.clone(); }
            },
            FieldCat::VecStr => quote! {},
            FieldCat::OptType(inner) => quote! {
                if <#inner as crate::record::MarcPaths>::IS_LEAF {
                    if path == #name_s {
                        return self.#id.as_ref().map(|v| v.to_marc_str());
                    }
                } else if let Some(rest) = path.strip_prefix(#prefix) {
                    return self.#id.as_ref().and_then(|v| v.marc_get_option(rest));
                }
            },
            FieldCat::VecType(_) => quote! {},
            FieldCat::BareType(inner) => quote! {
                if <#inner as crate::record::MarcPaths>::IS_LEAF {
                    if path == #name_s {
                        return Some(self.#id.to_marc_str());
                    }
                } else if let Some(rest) = path.strip_prefix(#prefix) {
                    return self.#id.marc_get_option(rest);
                }
            },
        }
    }).collect();
    quote! { #(#arms)* }
}

// ── marc_get_vec ────────────────────────────────────────────────────────────

fn gen_get_vec(fields: &[FInfo]) -> TokenStream2 {
    let arms: Vec<TokenStream2> = fields.iter().map(|f| {
        let id = &f.ident;
        let name_s = id.to_string();
        let prefix = format!("{}.", name_s);
        match &f.cat {
            FieldCat::Str | FieldCat::OptStr => quote! {},
            FieldCat::VecStr => quote! {
                if path == #name_s { return Some(self.#id.clone()); }
            },
            FieldCat::OptType(inner) => quote! {
                if !<#inner as crate::record::MarcPaths>::IS_LEAF {
                    if let Some(rest) = path.strip_prefix(#prefix) {
                        return self.#id.as_ref().and_then(|v| v.marc_get_vec(rest));
                    }
                }
            },
            FieldCat::VecType(inner) => quote! {
                if <#inner as crate::record::MarcPaths>::IS_LEAF {
                    if path == #name_s {
                        return Some(self.#id.iter().map(|v| v.to_marc_str()).collect());
                    }
                } else if let Some(rest) = path.strip_prefix(#prefix) {
                    return Some(
                        self.#id.iter().filter_map(|v| v.marc_get_option(rest)).collect()
                    );
                }
            },
            FieldCat::BareType(inner) => quote! {
                if !<#inner as crate::record::MarcPaths>::IS_LEAF {
                    if let Some(rest) = path.strip_prefix(#prefix) {
                        return self.#id.marc_get_vec(rest);
                    }
                }
            },
        }
    }).collect();
    quote! { #(#arms)* }
}

// ── marc_path_kind ──────────────────────────────────────────────────────────

fn gen_path_kind(fields: &[FInfo]) -> TokenStream2 {
    let arms: Vec<TokenStream2> = fields.iter().map(|f| {
        let id = &f.ident;
        let name_s = id.to_string();
        let prefix = format!("{}.", name_s);
        match &f.cat {
            FieldCat::Str | FieldCat::OptStr => quote! {
                if path == #name_s { return Some(crate::record::PathKind::OptionSet); }
            },
            FieldCat::VecStr => quote! {
                if path == #name_s { return Some(crate::record::PathKind::VecPush); }
            },
            FieldCat::OptType(inner) => quote! {
                if <#inner as crate::record::MarcPaths>::IS_LEAF {
                    if path == #name_s { return Some(crate::record::PathKind::OptionSet); }
                } else if let Some(rest) = path.strip_prefix(#prefix) {
                    if <#inner as crate::record::MarcPaths>::marc_is_vec_leaf(rest) {
                        return Some(crate::record::PathKind::VecPush);
                    }
                    if <#inner as crate::record::MarcPaths>::marc_has_path(rest) {
                        return Some(crate::record::PathKind::OptionInit);
                    }
                }
            },
            FieldCat::VecType(inner) => quote! {
                if <#inner as crate::record::MarcPaths>::IS_LEAF {
                    if path == #name_s { return Some(crate::record::PathKind::VecPush); }
                } else if let Some(rest) = path.strip_prefix(#prefix) {
                    if rest == <#inner as crate::record::MarcPaths>::marc_creator_field() {
                        return Some(crate::record::PathKind::VecStructCreator);
                    }
                    if <#inner as crate::record::MarcPaths>::marc_has_path(rest) {
                        return Some(crate::record::PathKind::VecStructField);
                    }
                }
            },
            FieldCat::BareType(inner) => quote! {
                if <#inner as crate::record::MarcPaths>::IS_LEAF {
                    if path == #name_s { return Some(crate::record::PathKind::OptionSet); }
                } else if let Some(rest) = path.strip_prefix(#prefix) {
                    return <#inner as crate::record::MarcPaths>::marc_path_kind(rest);
                }
            },
        }
    }).collect();
    quote! { #(#arms)* }
}

// ── marc_has_path ───────────────────────────────────────────────────────────

fn gen_has_path(fields: &[FInfo]) -> TokenStream2 {
    let arms: Vec<TokenStream2> = fields.iter().map(|f| {
        let id = &f.ident;
        let name_s = id.to_string();
        let prefix = format!("{}.", name_s);
        match &f.cat {
            FieldCat::Str | FieldCat::OptStr | FieldCat::VecStr => quote! {
                if path == #name_s { return true; }
            },
            FieldCat::OptType(inner) | FieldCat::VecType(inner) | FieldCat::BareType(inner) => quote! {
                if <#inner as crate::record::MarcPaths>::IS_LEAF {
                    if path == #name_s { return true; }
                } else if let Some(rest) = path.strip_prefix(#prefix) {
                    if <#inner as crate::record::MarcPaths>::marc_has_path(rest) { return true; }
                }
            },
        }
    }).collect();
    quote! { #(#arms)* }
}

// ── marc_is_vec_leaf ────────────────────────────────────────────────────────

fn gen_is_vec_leaf(fields: &[FInfo]) -> TokenStream2 {
    let arms: Vec<TokenStream2> = fields.iter().filter_map(|f| {
        let id = &f.ident;
        let name_s = id.to_string();
        let prefix = format!("{}.", name_s);
        match &f.cat {
            FieldCat::VecStr => Some(quote! {
                if path == #name_s { return true; }
            }),
            FieldCat::VecType(inner) => Some(quote! {
                if <#inner as crate::record::MarcPaths>::IS_LEAF {
                    if path == #name_s { return true; }
                }
            }),
            FieldCat::OptType(inner) | FieldCat::BareType(inner) => Some(quote! {
                if !<#inner as crate::record::MarcPaths>::IS_LEAF {
                    if let Some(rest) = path.strip_prefix(#prefix) {
                        if <#inner as crate::record::MarcPaths>::marc_is_vec_leaf(rest) { return true; }
                    }
                }
            }),
            _ => None,
        }
    }).collect();
    quote! { #(#arms)* }
}

// ── from_marc_str / to_marc_str (unused for containers) ────────────────────

fn gen_from_marc(_fields: &[FInfo]) -> TokenStream2 {
    quote! {}
}

fn gen_to_marc(_fields: &[FInfo]) -> TokenStream2 {
    quote! {}
}
