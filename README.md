# simdutf8 - High-speed UTF-8 validation for Rust

## Quick start
Put `simdutf8 = "0.1.0"` in your Cargo.toml file and use `simdutf8::basic::from_utf8` as a drop-in replacement for
`std::str::from_utf8()`. If you need the extended information on validation failures use `simdutf8::compat::from_utf8`
instead.

## Features
* Written in pure Rust.
* Supports AVX2 and SIMD implementations on x86 and x86-64, ARM neon support is planned.
* Selects the fastest implementation at runtime based on CPU support.
* No dependencies.
* No-std support
* `Basic` API for the fastest validation, optimized for valid UTF-8
* `Compat` API as a plug-in replacement for `std::str::from_utf8()`.
* Fallback to the excellent std implementation if SIMD extensions are not supported.

## APIs

### Basic flavor
For maximum speed on valid UTF-8 use the `basic` api flavor. It is fastest on valid UTF-8 but only checks
for errors after processing the whole byte sequence and does not provide detailed information if the data
is not valid UTF-8. `simdutf8::basic::Utf8Error` is a zero-sized error struct.

### Compat flavor
The `compat` flavor is fully API-compatible with `std::str::from_utf8`. In particular `simdutf8::compat::from_utf8()`
returns a `simdutf8::compat::Utf8Error` which has the `valid_up_to()` and `error_len()` methods. The first is useful
for verification of streamed data. Also it fails fast: Errors are checked on-the-fly as the string is processed so
if there is an invalid UTF-8 sequence at the beginning of the data it returns without processing the rest of the data.

## Implementation selection
The fastest implementation is selected at runtime using the `std::is_x86_feature_detected!` macro unless the targeted
CPU supports AVX 2. Since this is the fastest implementation it is called directly. So if you compile with
`RUSTFLAGS=-C target-cpu=native` on a recent machine the AVX 2 implementation is used automatically.

For non-std support (compiled with `--no-default-features`) the implementation is selected based on the supported
target features, use `RUSTFLAGS=-C target-cpu=avx2` to use the AVX 2 implementation or `RUSTFLAGS=-C target-cpu=sse4.2`
for the SSE 4.2 implementation.

If you want to be able to call the individual implementation directly use the `public_imp` feature flag. The validation
implementations are then accessible via `simdutf8::(compat|pure)::imp::x86::(avx2|sse42)::validate_utf8()`.

## When not to use
If you are only processing short byte sequences (less than 64 bytes) the excellent scalar algorithm in standard
library is likely faster. If there is no native implementation for your platform (yet) use the standard library
instead.

## Benchmarks

## Technical details
The implementation is similar to the one in simdjson except that it aligns reads to the block size of the
SIMD extension leading to better peak performance compared to the implementation in simdjson. Since this alignment
means that an incomplete block needs to be processed before the aligned data is read this would lead to worse
performance on short byte sequences. Thus aligned reads are only used with 2048 bytes data or more. Incomplete
reads for the first unaligned and the last incomplete block are done in two aligned 64-byte buffers.

For the compat API we need to check the error buffer on each 64-byte block instead of just aggregating it. If an
error is found the last bytes of the previous block are checked for a cross-block continuation and then
`std::str::from_utf8()` is run to find the exact location of the error.

## Thanks
* to Daniel Lemire and the autors of [simdjson] for coming up with the high-performance SIMD implementation.
* to the authors of the [simdjson Rust port]() for doing the main work by porting the C++ code to Rust.


## License
This code is made available under the [Apache License 2.0](https://www.apache.org/licenses/LICENSE-2.0.html).

It is based on code distributed with [simd-json.rs, the Rust port of simdjson. Simdjson itself is distributed under
the Apache License 2.0.

* std API uses autodetection of CPU features to select the best implementation.
* All functions are fully inlined.
* Use hints features on nightly (test if any faster) to make use of likely/unlikely intrinsics
* fallback uses the standard core/std implementation, which is quite fast for a scalar implementation, in particular on ASCII
* fuzz-tested
* 10 GiB/sec. performance on non-ASCII strings, xx times faster than stdlib
* 50+ Gib/sec. performance on ASCII strings, xx times faster than stdlib
* SIMD implementations for x86/x86-64 AVX 2 and SSE 4.2, ports of the neon SIMD implementations for aarch64
  and armv7 are planned.
* document `RUSTFLAGS="-C target-feature=+avx2"` and `RUSTFLAGS="-C target-cpu=native"` std code selection