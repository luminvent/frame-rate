# frame-rate

[![CI](https://github.com/luminvent/frame-rate/actions/workflows/ci.yml/badge.svg)](https://github.com/luminvent/frame-rate/actions/workflows/ci.yml)
[![crates.io](https://img.shields.io/crates/v/frame-rate.svg)](https://crates.io/crates/frame-rate)
[![docs.rs](https://docs.rs/frame-rate/badge.svg)](https://docs.rs/frame-rate)

A Rust library for video frame rates, stored as exact rationals.

Rates like 29.97 are really `30000/1001`, so a float can't hold them exactly. `FrameRate` keeps the exact ratio, gives
the common broadcast rates named variants, and keeps any other rate as a custom ratio.

## Installation

```toml
[dependencies]
frame-rate = "0.2"
```

## Usage

```rust
use frame_rate::{FrameRate, Ratio};

// Build from a numerator and denominator. The ratio is reduced, and
// known rates become their named variant.
let frame_rate = FrameRate::new(60000, 2002);
assert_eq!(frame_rate, FrameRate::_29_97);

// Convert to an exact ratio or an approximate float.
assert_eq!(Ratio::from(frame_rate), Ratio::new(30000, 1001));
let fps = f64::from(frame_rate); // 29.97002997..., also works on `&FrameRate`

// Any other rate is kept as a custom ratio.
let custom = FrameRate::new(2, 3);
assert!(matches!(custom, FrameRate::FrCustom(_)));

// Untrusted input: reject a zero denominator instead of panicking.
assert_eq!(FrameRate::try_new(1, 0), None);
```

`FrameRate::new` panics if the denominator is zero, as `Ratio::new` does. Use `FrameRate::try_new` for input you
don't control.

### Named rates

| Variant   | Ratio        | Approx. fps |
|-----------|--------------|-------------|
| `_23_97`  | 24000 / 1001 | 23.976      |
| `_24_00`  | 24 / 1       | 24          |
| `_24_97`  | 25000 / 1001 | 24.975      |
| `_25_00`  | 25 / 1       | 25          |
| `_29_97`  | 30000 / 1001 | 29.97       |
| `_30_00`  | 30 / 1       | 30          |
| `_50_00`  | 50 / 1       | 50          |
| `_59_94`  | 60000 / 1001 | 59.94       |
| `_60_00`  | 60 / 1       | 60          |
| `_120_00` | 120 / 1      | 120         |

Equality compares the ratios, not the variants: `FrameRate::FrCustom(Ratio::from_integer(24)) == FrameRate::_24_00`.

### Serialization

`FrameRate` implements serde's `Serialize` and `Deserialize`, as an object with a reduced numerator and denominator:

```json
{ "num": 30000, "den": 1001 }
```

The ratio is always reduced, in both directions: `{"num": 200, "den": 4}` deserializes to `_50_00`, and
`FrCustom(Ratio::new_raw(48, 2))` serializes as `{"num": 24, "den": 1}`. A zero denominator is an error both ways,
so whatever this crate writes, it can read back.

## Features

| Feature  | Description                                                                         |
|----------|-------------------------------------------------------------------------------------|
| `utoipa` | Implements [`utoipa::ToSchema`](https://docs.rs/utoipa) for OpenAPI schema generation. |

```toml
[dependencies]
frame-rate = { version = "0.2", features = ["utoipa"] }
```

The schema is named `FrameRate`: an object with the required, non-negative integer properties `num` and `den`.

## Minimum supported Rust version

Rust 1.85 (edition 2024). The `utoipa` feature needs Rust 1.88.

## License

MIT, see [LICENSE](LICENSE).
