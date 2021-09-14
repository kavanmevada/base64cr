#![feature(test)]
extern crate base64;
extern crate base64cr;
extern crate test;

use base64cr::Base64;
use test::{black_box, Bencher};

// Benchmarks over these byte sizes take longer so we will run fewer samples to
// keep the benchmark runtime reasonable.
const LARGE_BYTE_SIZES: [usize; 3] = [3 * 1024 * 1024, 10 * 1024 * 1024, 30 * 1024 * 1024];

#[bench]
fn do_bench_base64(b: &mut Bencher) {
    for size in LARGE_BYTE_SIZES {
        let mut v = vec![0u8; size * 3 / 4];
        fill(&mut v);
        let encoded = base64::encode(&v);

        b.iter(|| {
            black_box(&base64::decode(&encoded));
        });
    }
}

#[bench]
fn do_bench_base64cr(b: &mut Bencher) {
    for size in LARGE_BYTE_SIZES {
        let mut v = vec![0u8; size * 3 / 4];
        fill(&mut v);
        let encoded = v.encode();

        b.iter(|| {
            black_box(&encoded.decode());
        });
    }
}

fn fill(v: &mut Vec<u8>) {
    let seed = v.capacity();
    let mut x = (0xed as u8, seed as u8, seed as u8, seed as u8);
    v.fill_with(|| {
        let e = x.0 - (((x.1) << (1)) | ((x.1) >> (8 - (1))));
        x.0 = x.1 ^ ((x.2) << (4)) | ((x.2) >> (8 - (4)));
        x.1 = x.2 + x.3;
        x.2 = x.3 + e;
        x.3 = e + x.0;
        x.3
    });
}
