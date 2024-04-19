// Copyright 2018-2019 Cryptape Technologies LLC.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Implement built-in traits in [`::core::convert`].
//!
//! [`::core::convert`]: https://doc.rust-lang.org/core/convert/index.html#traits

use crate::fixed_hash::HashConstructor;
use quote::quote;

impl HashConstructor {
    pub fn impl_traits_std_convert(&self) {
        self.impl_traits_std_convert_from_as();
        self.impl_traits_std_convert_from_array();
    }

    fn impl_traits_std_convert_from_as(&self) {
        let name = &self.ts.name;
        let part = quote!(
            impl ::core::convert::AsRef<[u8]> for #name {
                #[inline]
                fn as_ref(&self) -> &[u8] {
                    &self.inner()[..]
                }
            }
            impl ::core::convert::AsMut<[u8]> for #name {
                #[inline]
                fn as_mut(&mut self) -> &mut [u8] {
                    &mut self.mut_inner()[..]
                }
            }
        );
        self.implt(part);
    }

    fn impl_traits_std_convert_from_array(&self) {
        let name = &self.ts.name;
        let inner_type = &self.ts.inner_type;
        let part = quote!(
            impl ::core::convert::From<#inner_type> for #name {
                #[inline]
                fn from(bytes: #inner_type) -> Self {
                    Self::new(bytes)
                }
            }
            impl<'a> ::core::convert::From<&'a #inner_type> for #name {
                #[inline]
                fn from(bytes: &'a #inner_type) -> Self {
                    Self::new(*bytes)
                }
            }
            impl ::core::convert::From<#name> for #inner_type {
                #[inline]
                fn from(hash: #name) -> Self {
                    hash.into_inner()
                }
            }
        );
        self.implt(part);
    }
}
