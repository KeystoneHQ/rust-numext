// Copyright 2018-2019 Cryptape Technologies LLC.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Implement built-in traits in [`::core::str`].
//!
//! [`::core::str`]: https://doc.rust-lang.org/core/str/index.html#traits

use crate::fixed_hash::HashConstructor;
use quote::quote;

impl HashConstructor {
    pub fn impl_traits_std_str(&self) {
        let name = &self.ts.name;
        let error_name = &self.ts.error_name;
        let part = quote!(
            impl ::core::str::FromStr for #name {
                type Err = #error_name;
                /// Convert from a hexadecimal string.
                #[inline]
                fn from_str(s: &str) -> Result<Self, Self::Err> {
                    Self::from_hex_str(s)
                }
            }
        );
        self.implt(part);
    }
}
