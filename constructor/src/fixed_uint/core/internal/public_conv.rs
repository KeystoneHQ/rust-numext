// Copyright 2018-2019 Cryptape Technologies LLC.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Define public methods about convert.

use crate::fixed_uint::UintConstructor;
use crate::utils;
use quote::quote;

impl UintConstructor {
    pub fn defun_pub_conv(&self) {
        self.defun_pub_conv_from_slice();
        self.defun_pub_conv_into_slice();
        self.attach_error_for_conv_from_str();
        self.defun_pub_conv_from_bin_str();
        self.defun_pub_conv_from_oct_str_dict();
        self.defun_pub_conv_from_oct_str();
        self.defun_pub_conv_from_hex_str_dict();
        self.defun_pub_conv_from_hex_str();
        self.defun_pub_conv_from_dec_str_dict();
        self.defun_pub_conv_from_dec_str();
    }

    fn attach_error_for_conv_slice(&self, conv_type: &str, type_explain: &str) {
        let error_item = utils::ident_to_ts(format!("{}Slice", conv_type).as_ref());
        let inner_error_name = utils::ident_to_ts(format!("{}SliceError", conv_type).as_ref());
        let error_explain = format!("failed to convert {} slice since {{0}}", type_explain);
        let part = quote!(
            /// Error for parse from slice.
            #[derive(Debug, Error)]
            pub enum #inner_error_name {
                #[error("invalid length: {0}")]
                InvalidLength(usize),
            }
        );
        self.attach_common(part);
        let part = quote!(
            #[error(#error_explain)]
            #error_item(#[from] #inner_error_name),
        );
        self.error(part);
    }

    fn defun_pub_conv_from_slice(&self) {
        self.attach_error_for_conv_slice("From", "from");
        let error_name = &self.ts.error_name;
        let inner_type = &self.ts.inner_type;
        let bytes_size = &self.ts.bytes_size;
        let unit_amount = &self.ts.unit_amount;
        let unit_bytes_size = &self.ts.unit_bytes_size;
        let part = quote!(
            #[inline]
            fn _from_le_slice_on_le_platform(input: &[u8]) -> Self {
                let mut ret: #inner_type = [0; #unit_amount];
                unsafe {
                    let slice = &mut *(&mut ret as *mut #inner_type as *mut [u8; #bytes_size]);
                    slice[0..input.len()].copy_from_slice(input);
                }
                Self::new(ret)
            }
            #[inline]
            fn _from_be_slice_on_le_platform(input: &[u8]) -> Self {
                let mut ret: #inner_type = [0; #unit_amount];
                unsafe {
                    let slice = &mut *(&mut ret as *mut #inner_type as *mut [u8; #bytes_size]);
                    let mut slice_ptr = slice.as_mut_ptr();
                    let mut input_ptr = input.as_ptr().offset(input.len() as isize - 1);
                    for _ in 0..input.len() {
                        *slice_ptr = *input_ptr;
                        slice_ptr = slice_ptr.offset(1);
                        input_ptr = input_ptr.offset(-1);
                    }
                }
                Self::new(ret)
            }
            // TODO more tests
            #[inline]
            fn _from_le_slice_on_be_platform(input: &[u8]) -> Self {
                let mut ret: #inner_type = [0; #unit_amount];
                unsafe {
                    let slice = &mut *(&mut ret as *mut #inner_type as *mut [u8; #bytes_size]);
                    slice[0..input.len()].copy_from_slice(input);
                }
                let input_units = input.len() / #unit_bytes_size + 1;
                for x in ret.iter_mut().take(input_units) {
                    *x = x.swap_bytes();
                }
                Self::new(ret)
            }
            // TODO more tests
            #[inline]
            fn _from_be_slice_on_be_platform(input: &[u8]) -> Self {
                let mut ret: #inner_type = [0; #unit_amount];
                unsafe {
                    let slice = &mut *(&mut ret as *mut #inner_type as *mut [u8; #bytes_size]);
                    let mut slice_ptr = slice.as_mut_ptr();
                    let mut input_ptr = input.as_ptr().offset(input.len() as isize - 1);
                    for _ in 0..input.len() {
                        *slice_ptr = *input_ptr;
                        slice_ptr = slice_ptr.offset(1);
                        input_ptr = input_ptr.offset(-1);
                    }
                }
                let input_units = input.len() / #unit_bytes_size + 1;
                for x in ret.iter_mut().take(input_units) {
                    *x = x.swap_bytes();
                }
                Self::new(ret)
            }
            /// Convert from little-endian slice.
            #[inline]
            pub fn from_little_endian(input: &[u8]) -> Result<Self, #error_name> {
                if input.len() > #bytes_size {
                    Err(FromSliceError::InvalidLength(input.len()).into())
                } else if cfg!(target_endian = "little") {
                    Ok(Self::_from_le_slice_on_le_platform(input))
                } else {
                    Ok(Self::_from_le_slice_on_be_platform(input))
                }
            }
            /// Convert from big-endian slice.
            #[inline]
            pub fn from_big_endian(input: &[u8]) -> Result<Self, #error_name> {
                if input.len() > #bytes_size {
                    Err(FromSliceError::InvalidLength(input.len()).into())
                } else if cfg!(target_endian = "little") {
                    Ok(Self::_from_be_slice_on_le_platform(input))
                } else {
                    Ok(Self::_from_be_slice_on_be_platform(input))
                }
            }
        );
        self.defun(part);
    }

    fn defun_pub_conv_into_slice(&self) {
        self.attach_error_for_conv_slice("Into", "into");
        let error_name = &self.ts.error_name;
        let inner_type = &self.ts.inner_type;
        let bytes_size = &self.ts.bytes_size;
        let loop_bytes_size = &utils::pure_uint_list_to_ts(0..self.info.bytes_size);
        let loop_unit_amount = &utils::pure_uint_list_to_ts(0..self.info.unit_amount);
        let loop_unit_amount_rev = &utils::pure_uint_list_to_ts((0..self.info.unit_amount).rev());
        let loop_unit_bytes_size = &utils::pure_uint_list_to_ts(0..self.info.unit_bytes_size);
        let copys_unit_bytes_size = &vec![&self.ts.unit_bytes_size; self.info.unit_amount as usize];
        let copys_part_for_into_le_slice_on_be_platform = {
            let part = quote!(
                #({
                    let _ = #loop_unit_bytes_size;
                    slice_ptr = slice_ptr.offset(-1);
                    *output_ptr = *slice_ptr;
                    output_ptr = output_ptr.offset(1);
                })*
            );
            &vec![part; self.info.unit_amount as usize]
        };
        let copys_part_for_into_be_slice_on_be_platform = {
            let part = quote!(
                #({
                    let _ = #loop_unit_bytes_size;
                    *output_ptr = *slice_ptr;
                    slice_ptr = slice_ptr.offset(1);
                    output_ptr = output_ptr.offset(1);
                })*
            );
            &vec![part; self.info.unit_amount as usize]
        };
        let part = quote!(
            #[inline]
            fn _into_le_slice_on_le_platform(&self, output: &mut [u8]) {
                let inner = self.inner();
                unsafe {
                    let slice = &*(inner as *const #inner_type as *const  [u8; #bytes_size]);
                    output[0..#bytes_size].copy_from_slice(slice);
                }
            }
            #[inline]
            fn _into_be_slice_on_le_platform(&self, output: &mut [u8]) {
                let inner = self.inner();
                unsafe {
                    let slice = &*(inner as *const #inner_type as *const  [u8; #bytes_size]);
                    let mut slice_ptr = slice.as_ptr().offset(#bytes_size - 1);
                    let mut output_ptr = output.as_mut_ptr();
                    #({
                        let _ = #loop_bytes_size;
                        *output_ptr = *slice_ptr;
                        slice_ptr = slice_ptr.offset(-1);
                        output_ptr = output_ptr.offset(1);
                    })*
                }
            }
            // TODO more tests
            #[inline]
            fn _into_le_slice_on_be_platform(&self, output: &mut [u8]) {
                let inner = self.inner();
                unsafe {
                    let slice = &*(inner as *const #inner_type as *const  [u8; #bytes_size]);
                    let slice_ptr_tmp = slice.as_ptr();
                    let mut output_ptr = output.as_mut_ptr();
                    #({
                        let idx = (#loop_unit_amount+1) * #copys_unit_bytes_size;
                        let mut slice_ptr = slice_ptr_tmp.offset(idx);
                        #copys_part_for_into_le_slice_on_be_platform
                    })*
                }
            }
            // TODO more tests
            #[inline]
            fn _into_be_slice_on_be_platform(&self, output: &mut [u8]) {
                let inner = self.inner();
                unsafe {
                    let slice = &*(inner as *const #inner_type as *const  [u8; #bytes_size]);
                    let slice_ptr_tmp = slice.as_ptr();
                    let mut output_ptr = output.as_mut_ptr();
                    #({
                        let idx = #loop_unit_amount_rev * #copys_unit_bytes_size;
                        let mut slice_ptr = slice_ptr_tmp.offset(idx);
                        #copys_part_for_into_be_slice_on_be_platform
                    })*
                }
            }
            /// Convert into little-endian slice.
            #[inline]
            pub fn into_little_endian(&self, output: &mut [u8]) -> Result<(), #error_name> {
                if output.len() != #bytes_size {
                    Err(IntoSliceError::InvalidLength(output.len()).into())
                } else if cfg!(target_endian = "little") {
                    self._into_le_slice_on_le_platform(output);
                    Ok(())
                } else {
                    self._into_le_slice_on_be_platform(output);
                    Ok(())
                }
            }
            /// Convert into big-endian slice.
            #[inline]
            pub fn into_big_endian(&self, output: &mut [u8]) -> Result<(), #error_name> {
                if output.len() != #bytes_size {
                    Err(IntoSliceError::InvalidLength(output.len()).into())
                } else if cfg!(target_endian = "little") {
                    self._into_be_slice_on_le_platform(output);
                    Ok(())
                } else {
                    self._into_be_slice_on_be_platform(output);
                    Ok(())
                }
            }
        );
        self.defun(part);
    }

    fn attach_error_for_conv_from_str(&self) {
        let part = quote!(
            /// Error for parse from string.
            #[derive(Debug, Error)]
            pub enum FromStrError {
                #[error("invalid character code `{chr}` at {idx}")]
                InvalidCharacter { chr: u8, idx: usize },
                #[error("invalid length: {0}")]
                InvalidLength(usize),
                #[error("number is too big (length is {0})")]
                Overflow(usize),
            }
        );
        self.attach_common(part);
        #[rustfmt::skip]
        let part = quote!(
            #[error("failed to parse from string since {0}")]
            FromStr(#[from] FromStrError),
        );
        self.error(part);
    }

    fn defun_pub_conv_from_bin_str(&self) {
        let error_name = &self.ts.error_name;
        let bits_size = &self.ts.bits_size;
        let unit_bits_size = &self.ts.unit_bits_size;
        let unit_amount = &self.ts.unit_amount;
        let inner_type = &self.ts.inner_type;
        let loop_unit_bits_size = &utils::pure_uint_list_to_ts(0..self.info.unit_bits_size);
        let part = quote!(
            /// Convert from a binary string.
            #[inline]
            pub fn from_bin_str(input: &str) -> Result<Self, #error_name> {
                let len = input.len();
                if len == 0 || len > #bits_size {
                    return Err(FromStrError::InvalidLength(len).into());
                } else if len != 1 && input.as_bytes()[0] == b'0' {
                    return Err(FromStrError::InvalidCharacter { chr: b'0', idx: 0 }.into());
                }
                let mut src = input.bytes().enumerate();
                let mut ret: #inner_type = [0; #unit_amount];
                let unit_cnt = len / #unit_bits_size;
                let chars_more = len % #unit_bits_size;
                for i in 0..chars_more {
                    let (idx, chr) = src.next().unwrap();
                    ret[unit_cnt] <<= 1;
                    match chr {
                        b'0' => {},
                        b'1' => ret[unit_cnt] |= 1,
                        _ => return Err(FromStrError::InvalidCharacter { chr, idx }.into()),
                    }
                }
                for i in (0..unit_cnt).rev() {
                    #({
                        let _ = #loop_unit_bits_size;
                        let (idx, chr) = src.next().unwrap();
                        ret[i] <<= 1;
                        match chr {
                            b'0' => {},
                            b'1' => ret[i] |= 1,
                            _ => return Err(FromStrError::InvalidCharacter { chr, idx }.into()),
                        }
                    })*
                }
                Ok(Self::new(ret))
            }
        );
        self.defun(part);
    }

    fn defun_pub_conv_from_oct_str_dict(&self) {
        let part = quote!(
            pub(crate) const DICT_OCT_ERROR: u8 = u8::max_value();
            pub(crate) static DICT_OCT: [u8; 256] = {
                const ____: u8 = DICT_OCT_ERROR;
                [
                    ____, ____, ____, ____, ____, ____, ____, ____, ____, ____, ____, ____, ____,
                    ____, ____, ____, ____, ____, ____, ____, ____, ____, ____, ____, ____, ____,
                    ____, ____, ____, ____, ____, ____, ____, ____, ____, ____, ____, ____, ____,
                    ____, ____, ____, ____, ____, ____, ____, ____, ____, 0x00, 0x01, 0x02, 0x03,
                    0x04, 0x05, 0x06, 0x07, ____, ____, ____, ____, ____, ____, ____, ____, ____,
                    ____, ____, ____, ____, ____, ____, ____, ____, ____, ____, ____, ____, ____,
                    ____, ____, ____, ____, ____, ____, ____, ____, ____, ____, ____, ____, ____,
                    ____, ____, ____, ____, ____, ____, ____, ____, ____, ____, ____, ____, ____,
                    ____, ____, ____, ____, ____, ____, ____, ____, ____, ____, ____, ____, ____,
                    ____, ____, ____, ____, ____, ____, ____, ____, ____, ____, ____, ____, ____,
                    ____, ____, ____, ____, ____, ____, ____, ____, ____, ____, ____, ____, ____,
                    ____, ____, ____, ____, ____, ____, ____, ____, ____, ____, ____, ____, ____,
                    ____, ____, ____, ____, ____, ____, ____, ____, ____, ____, ____, ____, ____,
                    ____, ____, ____, ____, ____, ____, ____, ____, ____, ____, ____, ____, ____,
                    ____, ____, ____, ____, ____, ____, ____, ____, ____, ____, ____, ____, ____,
                    ____, ____, ____, ____, ____, ____, ____, ____, ____, ____, ____, ____, ____,
                    ____, ____, ____, ____, ____, ____, ____, ____, ____, ____, ____, ____, ____,
                    ____, ____, ____, ____, ____, ____, ____, ____, ____, ____, ____, ____, ____,
                    ____, ____, ____, ____, ____, ____, ____, ____, ____, ____, ____, ____, ____,
                    ____, ____, ____, ____, ____, ____, ____, ____, ____,
                ]
            };
        );
        self.util(part);
    }

    fn defun_pub_conv_from_oct_str(&self) {
        let name = &self.ts.name;
        let error_name = &self.ts.error_name;
        let utils_name = &self.ts.utils_name;
        let char_amount_max = utils::pure_uint_to_ts(if self.info.bits_size % 3 == 0 {
            self.info.bits_size / 3
        } else {
            (f64::from(self.info.bits_size as u32) / 3f64).ceil() as u64
        });
        let part = quote!(
            /// Convert from a octal string.
            #[inline]
            pub fn from_oct_str(input: &str) -> Result<Self, #error_name> {
                let len = input.len();
                if len == 0 || len > #char_amount_max {
                    return Err(FromStrError::InvalidLength(len).into());
                } else if len != 1 && input.as_bytes()[0] == b'0' {
                    return Err(FromStrError::InvalidCharacter { chr: b'0', idx: 0 }.into());
                }
                let mut ret = Self::zero();
                for (idx, chr) in input.bytes().enumerate() {
                    let v = #utils_name::DICT_OCT[usize::from(chr)];
                    if v == #utils_name::DICT_OCT_ERROR {
                        return Err(FromStrError::InvalidCharacter { chr, idx }.into());
                    }
                    let (ret_new, of) = ret._mul_unit(8);
                    if of {
                        return Err(FromStrError::Overflow(len).into());
                    }
                    let u = #name::from(v);
                    let (ret_new, of) = ret_new._add(&u);
                    if of {
                        return Err(FromStrError::Overflow(len).into());
                    }
                    ret = ret_new;
                }
                Ok(ret)
            }
        );
        self.defun(part);
    }

    fn defun_pub_conv_from_hex_str_dict(&self) {
        let part = quote!(
            pub(crate) const DICT_HEX_ERROR: u8 = u8::max_value();
            pub(crate) static DICT_HEX: [u8; 256] = {
                const ____: u8 = DICT_HEX_ERROR;
                [
                    ____, ____, ____, ____, ____, ____, ____, ____, ____, ____, ____, ____, ____,
                    ____, ____, ____, ____, ____, ____, ____, ____, ____, ____, ____, ____, ____,
                    ____, ____, ____, ____, ____, ____, ____, ____, ____, ____, ____, ____, ____,
                    ____, ____, ____, ____, ____, ____, ____, ____, ____, 0x00, 0x01, 0x02, 0x03,
                    0x04, 0x05, 0x06, 0x07, 0x08, 0x09, ____, ____, ____, ____, ____, ____, ____,
                    0x0a, 0x0b, 0x0c, 0x0d, 0x0e, 0x0f, ____, ____, ____, ____, ____, ____, ____,
                    ____, ____, ____, ____, ____, ____, ____, ____, ____, ____, ____, ____, ____,
                    ____, ____, ____, ____, ____, ____, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e, 0x0f, ____,
                    ____, ____, ____, ____, ____, ____, ____, ____, ____, ____, ____, ____, ____,
                    ____, ____, ____, ____, ____, ____, ____, ____, ____, ____, ____, ____, ____,
                    ____, ____, ____, ____, ____, ____, ____, ____, ____, ____, ____, ____, ____,
                    ____, ____, ____, ____, ____, ____, ____, ____, ____, ____, ____, ____, ____,
                    ____, ____, ____, ____, ____, ____, ____, ____, ____, ____, ____, ____, ____,
                    ____, ____, ____, ____, ____, ____, ____, ____, ____, ____, ____, ____, ____,
                    ____, ____, ____, ____, ____, ____, ____, ____, ____, ____, ____, ____, ____,
                    ____, ____, ____, ____, ____, ____, ____, ____, ____, ____, ____, ____, ____,
                    ____, ____, ____, ____, ____, ____, ____, ____, ____, ____, ____, ____, ____,
                    ____, ____, ____, ____, ____, ____, ____, ____, ____, ____, ____, ____, ____,
                    ____, ____, ____, ____, ____, ____, ____, ____, ____, ____, ____, ____, ____,
                    ____, ____, ____, ____, ____, ____, ____, ____, ____,
                ]
            };
        );
        self.util(part);
    }

    fn defun_pub_conv_from_hex_str(&self) {
        let error_name = &self.ts.error_name;
        let utils_name = &self.ts.utils_name;
        let unit_suffix = &self.ts.unit_suffix;
        let char_amount_max = utils::pure_uint_to_ts(self.info.bytes_size * 2);
        let unit_char_amount_max = utils::pure_uint_to_ts(self.info.unit_bytes_size * 2);
        let part = quote!(
            /// Convert from a hexadecimal string.
            #[inline]
            pub fn from_hex_str(input: &str) -> Result<Self, #error_name> {
                let len = input.len();
                if len == 0 || len > #char_amount_max {
                    return Err(FromStrError::InvalidLength(len).into());
                } else if len != 1 && input.as_bytes()[0] == b'0' {
                    return Err(FromStrError::InvalidCharacter { chr: b'0', idx: 0 }.into());
                }
                let mut ret = Self::zero();
                let mut input_bytes = input.bytes();
                let mut idx = 0;
                let mut unit_idx = len / #unit_char_amount_max;
                let char_offset = len % #unit_char_amount_max;
                if char_offset > 0 {
                    let inner = ret.mut_inner();
                    let mut k = 0;
                    for _ in 0..char_offset {
                        let chr = input_bytes.next().unwrap_or_else(|| unreachable!());
                        let v = #utils_name::DICT_HEX[usize::from(chr)];
                        if v == #utils_name::DICT_HEX_ERROR {
                            return Err(FromStrError::InvalidCharacter { chr, idx }.into());
                        }
                        k *= 16;
                        k += #unit_suffix::from(v);
                        idx += 1;
                    }
                    inner[unit_idx] = k;
                }
                {
                    let inner = ret.mut_inner();
                    let mut k = 0;
                    let mut flag = #unit_char_amount_max - 1;
                    for chr in input_bytes {
                        let v = #utils_name::DICT_HEX[usize::from(chr)];
                        if v == #utils_name::DICT_HEX_ERROR {
                            return Err(FromStrError::InvalidCharacter { chr, idx }.into());
                        }
                        k *= 16;
                        k += #unit_suffix::from(v);
                        if flag == 0 {
                            unit_idx -= 1;
                            inner[unit_idx] = k;
                            k = 0;
                            flag = #unit_char_amount_max - 1;
                            continue;
                        } else {
                            idx += 1;
                            flag -= 1;
                        }
                    }
                }
                Ok(ret)
            }
        );
        self.defun(part);
    }

    fn defun_pub_conv_from_dec_str_dict(&self) {
        let part = quote!(
            pub(crate) const DICT_DEC_ERROR: u8 = u8::max_value();
            pub(crate) static DICT_DEC: [u8; 256] = {
                const ____: u8 = DICT_DEC_ERROR;
                [
                    ____, ____, ____, ____, ____, ____, ____, ____, ____, ____, ____, ____, ____,
                    ____, ____, ____, ____, ____, ____, ____, ____, ____, ____, ____, ____, ____,
                    ____, ____, ____, ____, ____, ____, ____, ____, ____, ____, ____, ____, ____,
                    ____, ____, ____, ____, ____, ____, ____, ____, ____, 0x00, 0x01, 0x02, 0x03,
                    0x04, 0x05, 0x06, 0x07, 0x08, 0x09, ____, ____, ____, ____, ____, ____, ____,
                    ____, ____, ____, ____, ____, ____, ____, ____, ____, ____, ____, ____, ____,
                    ____, ____, ____, ____, ____, ____, ____, ____, ____, ____, ____, ____, ____,
                    ____, ____, ____, ____, ____, ____, ____, ____, ____, ____, ____, ____, ____,
                    ____, ____, ____, ____, ____, ____, ____, ____, ____, ____, ____, ____, ____,
                    ____, ____, ____, ____, ____, ____, ____, ____, ____, ____, ____, ____, ____,
                    ____, ____, ____, ____, ____, ____, ____, ____, ____, ____, ____, ____, ____,
                    ____, ____, ____, ____, ____, ____, ____, ____, ____, ____, ____, ____, ____,
                    ____, ____, ____, ____, ____, ____, ____, ____, ____, ____, ____, ____, ____,
                    ____, ____, ____, ____, ____, ____, ____, ____, ____, ____, ____, ____, ____,
                    ____, ____, ____, ____, ____, ____, ____, ____, ____, ____, ____, ____, ____,
                    ____, ____, ____, ____, ____, ____, ____, ____, ____, ____, ____, ____, ____,
                    ____, ____, ____, ____, ____, ____, ____, ____, ____, ____, ____, ____, ____,
                    ____, ____, ____, ____, ____, ____, ____, ____, ____, ____, ____, ____, ____,
                    ____, ____, ____, ____, ____, ____, ____, ____, ____, ____, ____, ____, ____,
                    ____, ____, ____, ____, ____, ____, ____, ____, ____,
                ]
            };
        );
        self.util(part);
    }

    fn defun_pub_conv_from_dec_str(&self) {
        let name = &self.ts.name;
        let error_name = &self.ts.error_name;
        let utils_name = &self.ts.utils_name;
        let char_amount_max = utils::pure_uint_to_ts(if self.info.bits_size % 10 == 0 {
            self.info.bits_size / 10
        } else {
            (f64::from(self.info.bits_size as u32) / 10f64.log2()).ceil() as u64
        });
        let part = quote!(
            /// Convert from a decimal string.
            #[inline]
            pub fn from_dec_str(input: &str) -> Result<Self, #error_name> {
                let len = input.len();
                if len == 0 || len > #char_amount_max {
                    return Err(FromStrError::InvalidLength(len).into());
                } else if len != 1 && input.as_bytes()[0] == b'0' {
                    return Err(FromStrError::InvalidCharacter { chr: b'0', idx: 0 }.into());
                }
                let mut ret = Self::zero();
                for (idx, chr) in input.bytes().enumerate() {
                    let v = #utils_name::DICT_DEC[usize::from(chr)];
                    if v == #utils_name::DICT_DEC_ERROR {
                        return Err(FromStrError::InvalidCharacter { chr, idx }.into());
                    }
                    let (ret_new, of) = ret._mul_unit(10);
                    if of {
                        return Err(FromStrError::Overflow(len).into());
                    }
                    let u = #name::from(v);
                    let (ret_new, of) = ret_new._add(&u);
                    if of {
                        return Err(FromStrError::Overflow(len).into());
                    }
                    ret = ret_new;
                }
                Ok(ret)
            }
        );
        self.defun(part);
    }
}
