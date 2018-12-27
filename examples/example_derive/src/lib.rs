#![crate_type = "proc-macro"]

extern crate derive_utils;
extern crate proc_macro;

use derive_utils::quick_derive;
use proc_macro::TokenStream;

#[proc_macro_derive(Iterator)]
pub fn derive_iterator(input: TokenStream) -> TokenStream {
    quick_derive! {
        input,
        // trait
        trait Iterator {
            type Item;
            fn next(&mut self) -> Option<Self::Item>;
            fn size_hint(&self) -> (usize, Option<usize>);
        }
    }
}

#[proc_macro_derive(ExactSizeIterator)]
pub fn derive_exact_size_iterator(input: TokenStream) -> TokenStream {
    quick_derive! {
        input,
        // super trait's associated types
        Item,
        // trait
        trait ExactSizeIterator: Iterator {
            fn len(&self) -> usize;
        }
    }
}

#[proc_macro_derive(FusedIterator)]
pub fn derive_fused_iterator(input: TokenStream) -> TokenStream {
    quick_derive! {
        input,
        // super trait's associated types
        Item,
        // path
        (std::iter::FusedIterator),
        // trait
        trait FusedIterator: Iterator {},
    }
}

/* Same as the following:

extern crate derive_utils;
extern crate proc_macro;
extern crate proc_macro2;
extern crate syn;

use derive_utils::{derive_trait, EnumData};
use proc_macro::TokenStream;
use proc_macro2::{Ident, Span};
use syn::DeriveInput;

#[proc_macro_derive(Iterator)]
pub fn derive_iterator(input: TokenStream) -> TokenStream {
    let ast: DeriveInput = syn::parse(input).unwrap();
    let data = EnumData::from_derive(&ast).unwrap();

    derive_trait!(
        data,
        _,
        // trait
        trait Iterator {
            type Item;
            fn next(&mut self) -> Option<Self::Item>;
            fn size_hint(&self) -> (usize, Option<usize>);
        }
    )
    .unwrap()
    .into()
}

#[proc_macro_derive(ExactSizeIterator)]
pub fn derive_exact_size_iterator(input: TokenStream) -> TokenStream {
    let ast: DeriveInput = syn::parse(input).unwrap();
    let data = EnumData::from_derive(&ast).unwrap();

    derive_trait!(
        data,
        // super trait's associated types
        Some(Ident::new("Item", Span::call_site())),
        _,
        // trait
        trait ExactSizeIterator: Iterator {
            fn len(&self) -> usize;
        }
    )
    .unwrap()
    .into()
}

#[proc_macro_derive(FusedIterator)]
pub fn derive_fused_iterator(input: TokenStream) -> TokenStream {
    let ast: DeriveInput = syn::parse(input).unwrap();
    let data = EnumData::from_derive(&ast).unwrap();

    derive_trait!(
        data,
        // super trait's associated types
        Some(Ident::new("Item", Span::call_site())),
        // path
        (std::iter::FusedIterator),
        // trait
        trait FusedIterator: Iterator {}
    )
    .unwrap()
    .into()
}
*/
