Pure & Fastest Rust implementation of Base64.

Example
---

```rust
extern crate base64cr;

use base64cr::{encode, decode};

fn main() {
    let a = b"hello world";
    let b = b"aGVsbG8gd29ybGQ=";

    assert_eq!(a.encode().unwrap(), b);
    assert_eq!(a, &b.decode().unwrap());
}
```

Rust version compatibility
---

The minimum required Rust version is 1.47.0.

# Contributing

Contributions are very welcome. However, because this library is used widely, and in security-sensitive contexts, all PRs will be carefully scrutinized. Beyond that, this sort of low level library simply needs to be 100% correct. Nobody wants to chase bugs in encoding of any sort.

All this means that it takes me a fair amount of time to review each PR, so it might take quite a while to carve out the free time to give each PR the attention it deserves. I will get to everyone eventually!

Developing
---

Benchmarks are in `benches/`. Running them requires nightly rust, but `rustup` makes it easy:

```bash
rustup run nightly cargo bench
```

Benchmarks
---

```bash
running 2 tests
test do_bench_base64   ... bench:  23,965,866 ns/iter (+/- 5,789,520)
test do_bench_base64cr ... bench:  20,147,797 ns/iter (+/- 184,003)
```