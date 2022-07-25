// @author:    olinex
// @time:      2022/07/18


extern crate proc_macro;

// self mods
mod context;

// use other mods
use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput};

// use self mods
use context::EnumGroupContext;

#[proc_macro_derive(EnumGroup, attributes(groups))]
pub fn derive_enum_group(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let ctx = match EnumGroupContext::new(&input) {
        Ok(ctx) => ctx,
        Err(e) => return e.to_compile_error().into(),
    };
    match ctx.generate() {
        Ok(stream) => stream.into(),
        Err(e) => e.to_compile_error().into(),
    }
}

#[cfg(doctest)]
mod test_readme {
  macro_rules! external_doc_test {
    ($x:expr) => {
        #[doc = $x]
        extern {}
    };
  }

  external_doc_test!(include_str!("../README.md"));
}
