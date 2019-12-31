// Copyright 2018-2019 Cryptape Technologies LLC.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use criterion::{criterion_group, criterion_main, Criterion};

fn serde(c: &mut Criterion) {
    let x = nfhash::H256::repeat_byte(2);
    let y = etypes::H256([2; 32]);

    let x_json = serde_json::to_string(&x).unwrap();
    let y_json = serde_json::to_string(&y).unwrap();

    c.bench_function("serde/ser/nfhash/h256", move |b| {
        b.iter(|| {
            let _ = serde_json::to_string(&x);
        })
    });

    c.bench_function("serde/ser/etypes/h256", move |b| {
        b.iter(|| {
            let _ = serde_json::to_string(&y);
        })
    });

    c.bench_function("serde/de/nfhash/h256", move |b| {
        b.iter(|| {
            let _: nfhash::H256 = serde_json::from_str(&x_json).unwrap();
        })
    });

    c.bench_function("serde/de/etypes/h256", move |b| {
        b.iter(|| {
            let _: etypes::H256 = serde_json::from_str(&y_json).unwrap();
        })
    });
}

criterion_group!(ext, serde);
criterion_main!(ext);
