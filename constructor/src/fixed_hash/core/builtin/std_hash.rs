// Copyright 2018-2019 Cryptape Technologies LLC.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Implement built-in traits in [`::core::hash`].
//!
//! [`::core::hash`]: https://doc.rust-lang.org/core/hash/index.html#traits

use crate::fixed_hash::HashConstructor;
use quote::quote;

impl HashConstructor {
    pub fn impl_traits_std_hash(&self) {
        let name = &self.ts.name;
        let part = quote!(
            impl ::core::hash::Hash for #name {
                #[inline]
                fn hash<H: ::core::hash::Hasher>(&self, state: &mut H) {
                    state.write(&self.inner()[..])
                }
            }
        );
        self.implt(part);
    }
}
