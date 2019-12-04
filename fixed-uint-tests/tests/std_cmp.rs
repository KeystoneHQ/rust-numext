// Copyright 2018-2019 Cryptape Technologies LLC.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use nfuint_tests::props;
use proptest::{
    prelude::{any, any_with},
    proptest,
};

proptest! {
    #[test]
    fn random(ref pair in any_with::<props::U256Pair>(props::U256PairParameters::Random)) {
        let result_etypes = {
            let (ref lhs, ref rhs): (etypes::U256, etypes::U256) = pair.into();
            (lhs > rhs, lhs >= rhs, lhs == rhs, lhs <= rhs, lhs < rhs)
        };
        let result_nfuint = {
            let (ref lhs, ref rhs): (nfuint::U256, nfuint::U256) = pair.into();
            (lhs > rhs, lhs >= rhs, lhs == rhs, lhs <= rhs, lhs < rhs)
        };
        assert_eq!(result_etypes, result_nfuint);
        assert_eq!(result_nfuint.1, result_nfuint.0 || result_nfuint.2);
    }
}

proptest! {
    #[test]
    fn same(ref le in any::<props::U256LeBytes>()) {
        let result_nfuint = {
            let lhs: &nfuint::U256 = &le.into();
            let rhs: &nfuint::U256 = &le.into();
            (lhs > rhs, lhs >= rhs, lhs == rhs, lhs <= rhs, lhs < rhs)
        };
        assert_eq!(result_nfuint, (false, true, true, true, false));
    }
}
