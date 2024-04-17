// Copyright 2018-2019 Cryptape Technologies LLC.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Implement built-in traits in [`::std::fmt`].
//!
//! [`::std::fmt`]: https://doc.rust-lang.org/std/fmt/index.html#traits

use crate::fixed_uint::UintConstructor;
use crate::utils;
use alloc::{format, vec};
use quote::quote;

impl UintConstructor {
    pub fn impl_traits_std_fmt(&self) {
        self.impl_traits_std_fmt_debug();
        self.impl_traits_std_fmt_binary();
        self.impl_traits_std_fmt_octal();
        self.impl_traits_std_fmt_lowerhex();
        self.impl_traits_std_fmt_upperhex();
        self.impl_traits_std_fmt_display();
    }

    pub fn impl_traits_std_fmt_debug(&self) {
        let name = &self.ts.name;
        let width = self.info.unit_bytes_size * 2 + 2;
        let width = &utils::pure_uint_to_ts(width);
        let loop_width = &vec![width; self.info.unit_amount as usize];
        let loop_unit_amount = &utils::pure_uint_list_to_ts(0..self.info.unit_amount);
        let loop_unit_amount_skip_first = &utils::pure_uint_list_to_ts(1..self.info.unit_amount);
        let part = quote!(
            impl ::std::fmt::Debug for #name {
                #[inline]
                fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    let data = self.inner();
                    let alternate = f.alternate();
                    write!(f, stringify!(#name))?;
                    write!(f, " ( [")?;
                    if alternate {
                        writeln!(f)?;
                        #(
                            writeln!(f, "    {:#0width$x},", data[#loop_unit_amount], width=#loop_width)?;
                        )*
                        writeln!(f, "]")
                    } else {
                        write!(f, " {:#0width$x}", data[0], width=#width)?;
                        #(
                            write!(f, ", {:#0width$x}", data[#loop_unit_amount_skip_first], width=#loop_width)?;
                        )*
                        write!(f, " ] )")
                    }
                }
            }
        );
        self.implt(part);
    }

    fn impl_traits_std_fmt_base_2pow2n(
        &self,
        trait_name: &str,
        prefix_char: char,
        format_char: char,
        exp: u64,
    ) {
        let name = &self.ts.name;
        let unit_amount = &self.ts.unit_amount;
        let trait_name = utils::ident_to_ts(trait_name);
        let prefix = format!("0{}", prefix_char);
        let write_tpl = format!("{{:{}}}", format_char);
        let write_tpl_padded = format!("{{:0width${}}}", format_char);
        let width = self.info.unit_bits_size / exp;
        let width = utils::pure_uint_to_ts(width);
        let part = quote!(
            impl ::std::fmt::#trait_name for #name {
                #[inline]
                fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    let data = self.inner();
                    if f.alternate() {
                        write!(f, #prefix)?;
                    }
                    let mut idx = #unit_amount - 1;
                    while idx > 0 {
                        if data[idx] == 0 {
                            idx -= 1;
                            continue;
                        }
                        break;
                    }
                    if idx == 0 {
                        write!(f, #write_tpl, data[0])
                    } else {
                        write!(f, #write_tpl, data[idx])?;
                        idx -= 1;
                        while idx > 0 {
                            write!(f, #write_tpl_padded, data[idx], width = #width)?;
                            idx -= 1;
                        }
                        write!(f, #write_tpl_padded, data[0], width = #width)
                    }
                }
            }
        );
        self.implt(part);
    }

    fn impl_traits_std_fmt_base_lt10(&self, trait_name: &str, prefix: &str, num: u64) {
        let name = &self.ts.name;
        let trait_name = utils::ident_to_ts(trait_name);
        if num == 0 {
            unreachable!();
        }
        let num = utils::pure_uint_to_ts(num);
        let part = quote!(
            impl ::std::fmt::#trait_name for #name {
                #[inline]
                fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    if f.alternate() {
                        write!(f, #prefix)?;
                    }
                    let mut v = Vec::new();
                    let (mut q, r) = self._div_unit_with_rem(#num);
                    v.push(r);
                    while !q.is_zero() {
                        let (q_new, r) = q._div_unit_with_rem(#num);
                        v.push(r);
                        q = q_new;
                    }
                    for n in v.iter().rev() {
                        write!(f, "{}", n)?;
                    }
                    write!(f, "")
                }
            }
        );
        self.implt(part);
    }

    pub fn impl_traits_std_fmt_binary(&self) {
        self.impl_traits_std_fmt_base_2pow2n("Binary", 'b', 'b', 1);
    }

    pub fn impl_traits_std_fmt_octal(&self) {
        self.impl_traits_std_fmt_base_lt10("Octal", "0o", 8);
    }

    pub fn impl_traits_std_fmt_lowerhex(&self) {
        self.impl_traits_std_fmt_base_2pow2n("LowerHex", 'x', 'x', 4);
    }

    pub fn impl_traits_std_fmt_upperhex(&self) {
        self.impl_traits_std_fmt_base_2pow2n("UpperHex", 'x', 'X', 4);
    }

    pub fn impl_traits_std_fmt_display(&self) {
        self.impl_traits_std_fmt_base_lt10("Display", "", 10);
    }
}
