//! Video frame rates, stored as exact rationals.
//!
//! Common broadcast rates have named variants; any other rate is kept in
//! [`FrameRate::FrCustom`]. Two frame rates are equal when their ratios are
//! equal, whatever the variant.
//!
//! ```
//! use frame_rate::{FrameRate, Ratio};
//!
//! let frame_rate = FrameRate::new(30000, 1001);
//! assert_eq!(frame_rate, FrameRate::_29_97);
//! assert_eq!(Ratio::from(frame_rate), Ratio::new(30000, 1001));
//! assert!((f64::from(&frame_rate) - 29.97).abs() < 0.001);
//! ```

pub use num_rational::Ratio;

#[cfg(doctest)]
#[doc = include_str!("../README.md")]
struct ReadmeDoctests;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[derive(Clone, Copy, Debug)]
pub enum FrameRate {
  _24_00,
  _25_00,
  _30_00,
  _50_00,
  _60_00,
  _120_00,
  _23_97,
  _24_97,
  _29_97,
  _59_94,
  FrCustom(Ratio<u32>),
}

#[cfg(feature = "utoipa")]
impl utoipa::ToSchema for FrameRate {
  fn name() -> std::borrow::Cow<'static, str> {
    std::borrow::Cow::Borrowed("FrameRate")
  }
}

#[cfg(feature = "utoipa")]
impl utoipa::PartialSchema for FrameRate {
  fn schema() -> utoipa::openapi::RefOr<utoipa::openapi::schema::Schema> {
    SerializeRational::schema()
  }
}

/// Reduces `rational`, leaving it untouched if its denominator is zero, as a
/// [`FrameRate::FrCustom`] built from [`Ratio::new_raw`] may be unreduced or
/// have a zero denominator.
fn reduce(rational: Ratio<u32>) -> Ratio<u32> {
  match (*rational.numer(), *rational.denom()) {
    (_, 0) => rational,
    (numer, denom) => Ratio::new(numer, denom),
  }
}

/// Compares the reduced ratios field by field, rather than with `Ratio`'s
/// `==`, which panics on a zero denominator.
impl PartialEq for FrameRate {
  fn eq(&self, other: &Self) -> bool {
    let (left, right) = (Ratio::<u32>::from(*self), Ratio::<u32>::from(*other));
    (left.numer(), left.denom()) == (right.numer(), right.denom())
  }
}

impl Eq for FrameRate {}

impl FrameRate {
  /// Panics if `den` is zero, see [`FrameRate::try_new`].
  pub fn new(num: u32, den: u32) -> Self {
    Ratio::new(num, den).into()
  }

  /// Returns `None` if `den` is zero.
  pub fn try_new(num: u32, den: u32) -> Option<Self> {
    (den != 0).then(|| Self::new(num, den))
  }
}

impl From<FrameRate> for f64 {
  fn from(frame_rate: FrameRate) -> Self {
    let ratio: Ratio<u32> = frame_rate.into();
    *ratio.numer() as f64 / *ratio.denom() as f64
  }
}

impl From<&FrameRate> for f64 {
  fn from(frame_rate: &FrameRate) -> Self {
    (*frame_rate).into()
  }
}

impl From<FrameRate> for Ratio<u32> {
  fn from(frame_rate: FrameRate) -> Self {
    match frame_rate {
      FrameRate::_24_00 => Self::from_integer(24),
      FrameRate::_25_00 => Self::from_integer(25),
      FrameRate::_30_00 => Self::from_integer(30),
      FrameRate::_50_00 => Self::from_integer(50),
      FrameRate::_60_00 => Self::from_integer(60),
      FrameRate::_120_00 => Self::from_integer(120),
      FrameRate::_23_97 => Self::new(24000, 1001),
      FrameRate::_24_97 => Self::new(25000, 1001),
      FrameRate::_29_97 => Self::new(30000, 1001),
      FrameRate::_59_94 => Self::new(60000, 1001),
      FrameRate::FrCustom(rational) => reduce(rational),
    }
  }
}

impl From<Ratio<u32>> for FrameRate {
  fn from(rational: Ratio<u32>) -> Self {
    let rational = reduce(rational);
    match (rational.numer(), rational.denom()) {
      (24, 1) => Self::_24_00,
      (25, 1) => Self::_25_00,
      (30, 1) => Self::_30_00,
      (50, 1) => Self::_50_00,
      (60, 1) => Self::_60_00,
      (120, 1) => Self::_120_00,
      (24000, 1001) => Self::_23_97,
      (25000, 1001) => Self::_24_97,
      (30000, 1001) => Self::_29_97,
      (60000, 1001) => Self::_59_94,
      _ => Self::FrCustom(rational),
    }
  }
}

#[derive(Serialize, Deserialize)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
struct SerializeRational {
  num: u32,
  den: u32,
}

impl From<Ratio<u32>> for SerializeRational {
  fn from(rational: Ratio<u32>) -> Self {
    Self {
      num: *rational.numer(),
      den: *rational.denom(),
    }
  }
}

impl Serialize for FrameRate {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: Serializer,
  {
    let rational = Ratio::<u32>::from(*self);
    if *rational.denom() == 0 {
      return Err(serde::ser::Error::custom(
        "frame rate denominator must not be zero",
      ));
    }
    SerializeRational::from(rational).serialize(serializer)
  }
}

impl<'de> Deserialize<'de> for FrameRate {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: Deserializer<'de>,
  {
    let SerializeRational { num, den } = SerializeRational::deserialize(deserializer)?;
    Self::try_new(num, den)
      .ok_or_else(|| serde::de::Error::custom("frame rate denominator must not be zero"))
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use serde_json::json;

  /// Every named variant with its exact ratio and approximate value.
  const NAMED: [(FrameRate, u32, u32, f64); 10] = [
    (FrameRate::_24_00, 24, 1, 24.0),
    (FrameRate::_25_00, 25, 1, 25.0),
    (FrameRate::_30_00, 30, 1, 30.0),
    (FrameRate::_50_00, 50, 1, 50.0),
    (FrameRate::_60_00, 60, 1, 60.0),
    (FrameRate::_120_00, 120, 1, 120.0),
    (FrameRate::_23_97, 24000, 1001, 23.976),
    (FrameRate::_24_97, 25000, 1001, 24.975),
    (FrameRate::_29_97, 30000, 1001, 29.97),
    (FrameRate::_59_94, 60000, 1001, 59.94),
  ];

  /// Inputs that are not in reduced form, with the frame rate they reduce to.
  const UNREDUCED: [(u32, u32, FrameRate); 4] = [
    (200, 4, FrameRate::_50_00),
    (48, 2, FrameRate::_24_00),
    (60000, 2002, FrameRate::_29_97),
    (6, 9, FrameRate::FrCustom(Ratio::new_raw(2, 3))),
  ];

  #[test]
  fn named_to_rational() {
    for (frame_rate, num, den, _) in NAMED {
      assert_eq!(
        Ratio::from(frame_rate),
        Ratio::new(num, den),
        "{frame_rate:?}"
      );
    }
  }

  /// `==` compares ratios, so this checks that the variant itself matches.
  fn assert_variant(actual: FrameRate, expected: FrameRate) {
    assert_eq!(
      std::mem::discriminant(&actual),
      std::mem::discriminant(&expected),
      "{actual:?} is not {expected:?}"
    );
    assert_eq!(actual, expected);
  }

  #[test]
  fn rational_to_named() {
    for (frame_rate, num, den, _) in NAMED {
      assert_variant(FrameRate::from(Ratio::new(num, den)), frame_rate);
    }
  }

  #[test]
  fn new_reduces_to_named() {
    for (frame_rate, num, den, _) in NAMED {
      assert_variant(FrameRate::new(num, den), frame_rate);
    }
    for (num, den, frame_rate) in UNREDUCED {
      assert_variant(FrameRate::new(num, den), frame_rate);
    }
  }

  #[test]
  fn custom() {
    let frame_rate = FrameRate::new(2, 3);
    assert!(matches!(frame_rate, FrameRate::FrCustom(_)));
    assert_eq!(Ratio::from(frame_rate), Ratio::new(2, 3));
    assert_eq!(
      Ratio::from(FrameRate::FrCustom(Ratio::new(6, 9))),
      Ratio::new(2, 3)
    );
  }

  #[test]
  fn to_f64() {
    for (frame_rate, _, _, value) in NAMED {
      assert!(
        (f64::from(&frame_rate) - value).abs() < 0.001,
        "{frame_rate:?}"
      );
    }
    assert!((f64::from(&FrameRate::new(2, 3)) - 2.0 / 3.0).abs() < f64::EPSILON);
  }

  #[test]
  fn owned_and_borrowed_f64_agree() {
    for (frame_rate, ..) in NAMED {
      assert_eq!(f64::from(frame_rate), f64::from(&frame_rate));
    }
    let custom = FrameRate::new(2, 3);
    assert_eq!(f64::from(custom), f64::from(&custom));
    assert_eq!(
      f64::from(FrameRate::FrCustom(Ratio::new_raw(1, 0))),
      f64::INFINITY
    );
    assert!(f64::from(FrameRate::FrCustom(Ratio::new_raw(0, 0))).is_nan());
  }

  #[test]
  fn equality_compares_ratios() {
    for (frame_rate, num, den, _) in NAMED {
      assert_eq!(FrameRate::FrCustom(Ratio::new(num, den)), frame_rate);
      assert_eq!(
        FrameRate::FrCustom(Ratio::new_raw(num * 2, den * 2)),
        frame_rate
      );
    }
    for (i, (a, ..)) in NAMED.iter().enumerate() {
      for (b, ..) in &NAMED[i + 1..] {
        assert_ne!(a, b);
      }
    }
  }

  #[test]
  fn raw_custom_is_reduced() {
    for (num, den, frame_rate) in UNREDUCED {
      let raw = Ratio::new_raw(num, den);
      assert_variant(FrameRate::from(raw), frame_rate);
      assert_eq!(
        Ratio::from(FrameRate::FrCustom(raw)),
        Ratio::from(frame_rate)
      );
    }
  }

  #[test]
  fn zero_denominator_equality() {
    let zero_den = [
      Ratio::new_raw(0, 0),
      Ratio::new_raw(1, 0),
      Ratio::new_raw(2, 0),
    ]
    .map(FrameRate::FrCustom);
    for (i, a) in zero_den.iter().enumerate() {
      assert_eq!(a, a);
      for b in &zero_den[i + 1..] {
        assert_ne!(a, b);
      }
      for (frame_rate, ..) in NAMED {
        assert_ne!(*a, frame_rate);
        assert_ne!(frame_rate, *a);
      }
    }
  }

  #[test]
  fn try_new() {
    for (frame_rate, num, den, _) in NAMED {
      assert_eq!(FrameRate::try_new(num, den), Some(frame_rate));
    }
    assert_eq!(FrameRate::try_new(1, 0), None);
    assert_eq!(FrameRate::try_new(0, 0), None);
    assert_eq!(
      FrameRate::try_new(0, 1),
      Some(FrameRate::FrCustom(Ratio::from_integer(0)))
    );
  }

  #[test]
  #[should_panic(expected = "denominator == 0")]
  fn new_zero_denominator_panics() {
    FrameRate::new(1, 0);
  }

  #[test]
  fn serialize() {
    for (frame_rate, num, den, _) in NAMED {
      assert_eq!(
        serde_json::to_value(frame_rate).unwrap(),
        json!({ "num": num, "den": den })
      );
    }
    for (num, den, frame_rate) in UNREDUCED {
      let reduced = Ratio::from(frame_rate);
      assert_eq!(
        serde_json::to_value(FrameRate::FrCustom(Ratio::new_raw(num, den))).unwrap(),
        json!({ "num": reduced.numer(), "den": reduced.denom() })
      );
    }
  }

  #[test]
  fn serialize_zero_denominator() {
    for raw in [Ratio::new_raw(1, 0), Ratio::new_raw(0, 0)] {
      let error = serde_json::to_value(FrameRate::FrCustom(raw)).unwrap_err();
      assert_eq!(error.to_string(), "frame rate denominator must not be zero");
    }
  }

  #[test]
  fn deserialize() {
    for (frame_rate, num, den, _) in NAMED {
      assert_variant(
        serde_json::from_value(json!({ "num": num, "den": den })).unwrap(),
        frame_rate,
      );
    }
    for (num, den, frame_rate) in UNREDUCED {
      assert_variant(
        serde_json::from_value(json!({ "num": num, "den": den })).unwrap(),
        frame_rate,
      );
    }
  }

  #[test]
  fn round_trip() {
    let unreduced = UNREDUCED.map(|(num, den, _)| FrameRate::FrCustom(Ratio::new_raw(num, den)));
    for frame_rate in NAMED
      .map(|(frame_rate, ..)| frame_rate)
      .into_iter()
      .chain(unreduced)
    {
      let json = serde_json::to_string(&frame_rate).unwrap();
      assert_eq!(
        serde_json::from_str::<FrameRate>(&json).unwrap(),
        frame_rate
      );
    }
  }

  #[test]
  fn deserialize_invalid() {
    for value in [
      json!({ "num": 1, "den": 0 }),
      json!({ "num": 0, "den": 0 }),
      json!({ "num": 24 }),
      json!({ "den": 1 }),
      json!({ "num": -24, "den": 1 }),
      json!({ "num": 24.5, "den": 1 }),
      json!({ "num": 4294967296u64, "den": 1 }),
      json!({ "num": "24", "den": 1 }),
      json!(24),
      json!("24/1"),
      json!(null),
    ] {
      assert!(
        serde_json::from_value::<FrameRate>(value.clone()).is_err(),
        "{value}"
      );
    }
  }

  #[test]
  fn deserialize_zero_denominator_message() {
    let error = serde_json::from_value::<FrameRate>(json!({ "num": 1, "den": 0 })).unwrap_err();
    assert_eq!(error.to_string(), "frame rate denominator must not be zero");
  }

  #[cfg(feature = "utoipa")]
  #[test]
  fn schema() {
    use utoipa::{PartialSchema, ToSchema};
    assert_eq!(FrameRate::name(), "FrameRate");
    let schema = serde_json::to_value(FrameRate::schema()).unwrap();
    assert_eq!(schema["type"], "object");
    assert_eq!(schema["required"], json!(["num", "den"]));
    for field in ["num", "den"] {
      assert_eq!(schema["properties"][field]["type"], "integer", "{field}");
      assert_eq!(schema["properties"][field]["minimum"], 0, "{field}");
    }
  }
}
