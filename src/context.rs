// @author:    olinex
// @time:      2022/07/18

// self mods

// use other mods
use quote::quote;
use std::collections::HashMap;
use std::vec::Vec;
use syn::{punctuated, spanned::Spanned, token};

// use self mods

type Variants = punctuated::Punctuated<syn::Variant, token::Comma>;
type MetaNested = punctuated::Punctuated<syn::NestedMeta, token::Comma>;

pub struct EnumGroupContext<'a> {
    visible: &'a syn::Visibility,
    name: &'a syn::Ident,
    generics: &'a syn::Generics,
    variants: &'a Variants,
    groups: HashMap<syn::Ident, Vec<syn::Variant>>,
}

impl<'a> EnumGroupContext<'a> {
    // Extract the label name ident from path and check the ident format
    fn extract_label_ident(path: &syn::Path) -> syn::Result<syn::Ident> {
        if let Some(i) = path.get_ident() {
            let s = i.to_string();
            if s.chars()
                .any(|c| !(c.is_alphanumeric() || c == '_') || c.is_ascii_uppercase())
            {
                return Err(syn::Error::new_spanned(
                    path,
                    "groups attribute ident can only contain the characters a-z0-9_",
                ));
            }
            if s.starts_with('_') {
                return Err(syn::Error::new_spanned(
                    path,
                    "groups attribute ident must starts wtih characters a-z",
                ));
            }
            if s.ends_with('_') {
                return Err(syn::Error::new_spanned(
                    path,
                    "groups attribute ident must ends wtih characters a-z",
                ));
            }
            Ok(i.clone())
        } else {
            Err(syn::Error::new_spanned(
                path,
                "unknown item in sub groups attribute",
            ))
        }
    }

    // Extract the label name idents in the group attribute
    fn extract_label_idents(nested: MetaNested) -> syn::Result<Vec<syn::Ident>> {
        let mut labels: Vec<_> = Vec::new();
        for nest in nested.into_iter() {
            match nest {
                syn::NestedMeta::Meta(syn::Meta::Path(path)) => {
                    let label = Self::extract_label_ident(&path)?;
                    labels.push(label);
                }
                syn::NestedMeta::Meta(syn::Meta::List(syn::MetaList {
                    ref path, nested, ..
                })) => {
                    let suffix_ident = Self::extract_label_ident(path)?;
                    let sub_label_idents = Self::extract_label_idents(nested)?;
                    for sub_label_ident in sub_label_idents.into_iter() {
                        let concat_ident = syn::Ident::new(
                            &format!(
                                "{}_{}",
                                suffix_ident.to_string(),
                                sub_label_ident.to_string()
                            ),
                            sub_label_ident.span(),
                        );
                        labels.push(concat_ident);
                    }
                }
                _ => {
                    return Err(syn::Error::new_spanned(
                        nest,
                        "unknown item in groups attribute",
                    ))
                }
            }
        }
        Ok(labels)
    }

    // Extract variants of enumeration types that have the `groups` attribute declared
    fn extract_nested_meta(attributes: &Vec<syn::Attribute>) -> syn::Result<Option<MetaNested>> {
        let mut result = None;
        for attribute in attributes.iter() {
            let meta = attribute.parse_meta()?;
            let nested = match meta {
                syn::Meta::List(syn::MetaList {
                    ref path, nested, ..
                }) if path.is_ident("groups") => nested,
                _ => continue,
            };
            if nested.is_empty() {
                return Err(syn::Error::new_spanned(
                    nested,
                    "must have group ident in groups attribute",
                ));
            }
            if result.is_some() {
                return Err(syn::Error::new_spanned(
                    attribute,
                    "duplicate groups attribute",
                ));
            }
            result = Some(nested);
        }
        Ok(result)
    }

    // Extract enumerated group label names and their associated variants
    fn extract_groups(variants: &Variants) -> syn::Result<HashMap<syn::Ident, Vec<syn::Variant>>> {
        let mut groups: HashMap<syn::Ident, Vec<syn::Variant>> = HashMap::new();
        for variant in variants.iter() {
            let meta = Self::extract_nested_meta(&variant.attrs)?;
            if let Some(nested) = meta {
                let idents = Self::extract_label_idents(nested)?;
                for ident in idents.iter() {
                    if ident.to_string().to_lowercase() == variant.ident.to_string().to_lowercase()
                    {
                        return Err(syn::Error::new_spanned(
                            variant,
                            "conflict group label name and viriant name. group name cannot equal to variant name",
                        ));
                    }
                    match groups.get_mut(ident) {
                        Some(items) => items.push(variant.clone()),
                        None => {
                            let items = vec![variant.clone()];
                            groups.insert(ident.clone(), items);
                        }
                    }
                }
            }
        }
        Ok(groups)
    }

    // Extract each variant declaration of the enum
    fn extract_variants(input: &syn::DeriveInput) -> syn::Result<&Variants> {
        if let syn::Data::Enum(syn::DataEnum { ref variants, .. }) = input.data {
            Ok(variants)
        } else {
            return Err(syn::Error::new_spanned(input, "must define on enum"));
        }
    }

    pub fn new(input: &'a syn::DeriveInput) -> syn::Result<Self> {
        let visible = &input.vis;
        let name = &input.ident;
        let generics = &input.generics;
        let variants = Self::extract_variants(input)?;
        let groups = Self::extract_groups(variants)?;
        Ok(Self {
            visible,
            name,
            groups,
            generics,
            variants,
        })
    }

    // Generate a new ident by prefixing the group label name with "is_"
    fn gen_group_label_ident(label: &syn::Ident) -> syn::Ident {
        syn::Ident::new(
            &format!("is_{}", label.to_string().to_lowercase()),
            label.span(),
        )
    }

    // Generate different arms based on three different variant patterns of the enum type
    fn gen_variant_arm(variant: &syn::Variant) -> proc_macro2::TokenStream {
        let name = &variant.ident;
        match variant.fields {
            syn::Fields::Named(_) => {
                quote!(
                    #name{..}
                )
            }
            syn::Fields::Unnamed(_) => {
                quote!(
                    #name(..)
                )
            }
            syn::Fields::Unit => {
                quote!(
                    #name
                )
            }
        }
    }

    // Generate a function that returns the name of each variant
    fn gen_variant_name_fn_expr(&self) -> proc_macro2::TokenStream {
        let visible = self.visible;
        let variant_arms: Vec<_> = self
            .variants
            .iter()
            .map(|v| Self::gen_variant_arm(v))
            .collect();
        let variant_names: Vec<_> = self
            .variants
            .iter()
            .map(|v| syn::LitStr::new(&v.ident.to_string(), v.span()))
            .collect();
        quote!(
            #[inline]
            #visible fn variant_name(&self) -> &str {
                match self {
                    #(Self::#variant_arms => #variant_names),*
                }
            }
        )
    }

    // Generate a judgment function for a group label name
    fn gen_group_fn_expr(
        &self,
        label: &syn::Ident,
        variants: &Vec<syn::Variant>,
    ) -> proc_macro2::TokenStream {
        let visible = self.visible;
        let label_ident = Self::gen_group_label_ident(label);
        let variant_arms: Vec<_> = variants.iter().map(|v| Self::gen_variant_arm(v)).collect();
        quote!(
            #[inline]
            #visible fn #label_ident(&self) -> bool {
                match self {
                    #(Self::#variant_arms)|* => true,
                    _ => false
                }
            }
        )
    }

    // Generate a corresponding judgment function for each group label name
    fn gen_group_fn_exprs(&self) -> Vec<proc_macro2::TokenStream> {
        self.groups
            .iter()
            .map(|(label, variants)| self.gen_group_fn_expr(label, variants))
            .collect()
    }

    // Generate a serial of judgment functions for a group label name
    fn gen_variant_group_fn_exprs(&self) -> Vec<proc_macro2::TokenStream> {
        self.variants
            .iter()
            .map(|variant| self.gen_group_fn_expr(&variant.ident, &vec![variant.clone()]))
            .collect()
    }

    // Generate a trait inheritance declaration that includes a group label name judgment function
    pub fn generate(&self) -> syn::Result<proc_macro2::TokenStream> {
        let name = self.name;
        let generics = self.generics;
        let group_fn_exprs = self.gen_group_fn_exprs();
        let variant_group_fn_exprs = self.gen_variant_group_fn_exprs();
        let variant_name_fn_expr = self.gen_variant_name_fn_expr();
        let st = quote!(
            impl #generics #name #generics {
                #(#group_fn_exprs)*

                #(#variant_group_fn_exprs)*

                #variant_name_fn_expr
            }
        );
        Ok(st)
    }
}
