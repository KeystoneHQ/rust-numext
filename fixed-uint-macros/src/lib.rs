// Copyright 2018 Rust-NumExt Developers
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#![recursion_limit = "512"]

extern crate proc_macro;
extern crate proc_macro2;

#[macro_use]
extern crate syn;
#[macro_use]
extern crate quote;

mod core;

mod funclike;

mod parsed;

#[proc_macro]
pub fn construct_fixed_uints(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let inputs = parse_macro_input!(input as funclike::UintDefinitions);
    let expanded = {
        let mut inputs_iter = inputs.inner.into_iter();
        if let Some(input) = inputs_iter.next() {
            let parsed: parsed::UintDefinition = input.into();
            let constructor = core::UintConstructor::new(parsed);
            let (one_uint, common) = constructor.construct_all();
            let all_uints = inputs_iter
                .map(|input| {
                    let parsed: parsed::UintDefinition = input.into();
                    core::UintConstructor::new(parsed).construct_all()
                }).fold(one_uint, |uints, (uint, _)| quote!(#uints #uint));
            quote!(#common #all_uints)
        } else {
            quote!()
        }
    };
    expanded.into()
}
