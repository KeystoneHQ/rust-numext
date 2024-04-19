// Copyright 2018-2019 Cryptape Technologies LLC.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Implement built-in traits in [`::core::iter`].
//!
//! [`::core::iter`]: https://doc.rust-lang.org/core/iter/index.html#traits

use crate::fixed_uint::UintConstructor;
use quote::quote;

impl UintConstructor {
    pub fn impl_traits_std_iter(&self) {
        self.impl_traits_std_iter_sum();
        self.impl_traits_std_iter_product();
    }

    fn impl_traits_std_iter_sum(&self) {
        let name = &self.ts.name;
        let part = quote!(
            impl<'a> ::core::iter::Sum<&'a #name> for #name {
                #[inline]
                fn sum<I>(iter: I) -> Self
                where
                    I: ::core::iter::Iterator<Item = &'a #name>,
                {
                    iter.fold(Self::zero(), ::core::ops::Add::add)
                }
            }
            impl ::core::iter::Sum<#name> for #name {
                #[inline]
                fn sum<I>(iter: I) -> Self
                where
                    I: ::core::iter::Iterator<Item = #name>,
                {
                    iter.fold(Self::zero(), ::core::ops::Add::add)
                }
            }
        );
        self.implt(part);
    }

    fn impl_traits_std_iter_product(&self) {
        let name = &self.ts.name;
        let part = quote!(
            impl<'a> ::core::iter::Product<&'a #name> for #name {
                #[inline]
                fn product<I>(iter: I) -> Self
                where
                    I: ::core::iter::Iterator<Item = &'a #name>,
                {
                    iter.fold(Self::one(), ::core::ops::Mul::mul)
                }
            }
            impl ::core::iter::Product<#name> for #name {
                #[inline]
                fn product<I>(iter: I) -> Self
                where
                    I: ::core::iter::Iterator<Item = #name>,
                {
                    iter.fold(Self::one(), ::core::ops::Mul::mul)
                }
            }
        );
        self.implt(part);
    }
}
