// @author:    olinex
// @time:      2022/07/18

// self mods

// use other mods
use quote::quote;
use std::collections::HashMap;
use std::vec::Vec;
use syn::{punctuated, token};

// use self mods

type Variants = punctuated::Punctuated<syn::Variant, token::Comma>;
type MetaNested = punctuated::Punctuated<syn::NestedMeta, token::Comma>;

pub struct EnumGroupContext<'a> {
    visible: &'a syn::Visibility,
    name: &'a syn::Ident,
    generics: &'a syn::Generics,
    groups: HashMap<syn::Ident, Vec<syn::Variant>>,
}

impl<'a> EnumGroupContext<'a> {

    // Extract the label name ident in the group attribute
    fn extract_label_idents(nested: MetaNested) -> syn::Result<Vec<syn::Ident>> {
        let mut labels: Vec<_> = Vec::new();
        for nest in nested.iter() {
            if let syn::NestedMeta::Meta(syn::Meta::Path(path)) = nest {
                if let Some(i) = path.get_ident() {
                    let s = i.to_string();
                    if s.chars().any(|c| !(c.is_alphanumeric() || c == '_')) {
                        return Err(syn::Error::new_spanned(
                            nest,
                            "groups attribute ident can only contain the characters a-zA-Z0-9_",
                        ));
                    }
                    if s.starts_with('_') {
                        return Err(syn::Error::new_spanned(
                            nest,
                            "groups attribute ident must starts wtih characters a-zA-Z",
                        ));
                    }
                    if s.ends_with('_') {
                        return Err(syn::Error::new_spanned(
                            nest,
                            "groups attribute ident must ends wtih characters a-zA-Z",
                        ));
                    }
                    labels.push(i.clone());
                    continue;
                }
            }
            return Err(syn::Error::new_spanned(
                nest,
                "unknown item in groups attribute",
            ));
        }
        Ok(labels)
    }

    // Extract variants of enumeration types that have the `groups` attribute declared
    fn extract_nested_meta(attributes: &'a Vec<syn::Attribute>) -> syn::Result<Option<MetaNested>> {
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
    fn extract_groups(
        variants: &'a Variants,
    ) -> syn::Result<HashMap<syn::Ident, Vec<syn::Variant>>> {
        let mut groups: HashMap<syn::Ident, Vec<syn::Variant>> = HashMap::new();
        for variant in variants.iter() {
            let meta = Self::extract_nested_meta(&variant.attrs)?;
            if let Some(nested) = meta {
                let idents = Self::extract_label_idents(nested)?;
                for ident in idents.iter() {
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
        })
    }

    // Generate a new ident by prefixing the group label name with "is_"
    fn gen_group_label_ident(label: &syn::Ident) -> syn::Ident {
        syn::Ident::new(&format!("is_{}", label.to_string()), label.span())
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

    // Generate a trait inheritance declaration that includes a group label name judgment function
    pub fn generate(&self) -> syn::Result<proc_macro2::TokenStream> {
        let name = self.name;
        let generics = self.generics;
        let group_fn_exprs = self.gen_group_fn_exprs();
        let st = quote!(
            impl #generics #name #generics {
                #(#group_fn_exprs)*
            }
        );
        Ok(st)
    }
}
