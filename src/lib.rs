pub use num_rational::Ratio;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
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

impl utoipa::ToSchema for FrameRate {
  fn name() -> std::borrow::Cow<'static, str> {
    std::borrow::Cow::Borrowed("FrameRate")
  }
}

impl utoipa::PartialSchema for FrameRate {
  fn schema() -> utoipa::openapi::RefOr<utoipa::openapi::schema::Schema> {
    utoipa::openapi::ObjectBuilder::new().into()
  }
}

impl FrameRate {
  pub fn new(num: u32, den: u32) -> Self {
    Ratio::new(num, den).into()
  }
}

impl From<&FrameRate> for f64 {
  fn from(frame_rate: &FrameRate) -> Self {
    let ratio: Ratio<u32> = (*frame_rate).into();
    *ratio.numer() as f64 / *ratio.denom() as f64
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
      FrameRate::FrCustom(rational) => rational,
    }
  }
}

impl From<Ratio<u32>> for FrameRate {
  fn from(rational: Ratio<u32>) -> Self {
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

impl From<SerializeRational> for Ratio<u32> {
  fn from(serialize_rational: SerializeRational) -> Self {
    Self::new(serialize_rational.num, serialize_rational.den)
  }
}

impl Serialize for FrameRate {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: Serializer,
  {
    SerializeRational::from(Ratio::<u32>::from(*self)).serialize(serializer)
  }
}

impl<'de> Deserialize<'de> for FrameRate {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: Deserializer<'de>,
  {
    Ok(Self::from(Ratio::<u32>::from(
      SerializeRational::deserialize(deserializer)?,
    )))
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn rational_from_frame_rate() {
    assert_eq!(Ratio::from(FrameRate::_24_00), Ratio::from_integer(24));
    assert_eq!(Ratio::from(FrameRate::_25_00), Ratio::from_integer(25));
    assert_eq!(Ratio::from(FrameRate::_30_00), Ratio::from_integer(30));
    assert_eq!(Ratio::from(FrameRate::_50_00), Ratio::from_integer(50));
    assert_eq!(Ratio::from(FrameRate::_60_00), Ratio::from_integer(60));
    assert_eq!(Ratio::from(FrameRate::_120_00), Ratio::from_integer(120));
    assert_eq!(Ratio::from(FrameRate::_23_97), Ratio::new(24000, 1001));
    assert_eq!(Ratio::from(FrameRate::_24_97), Ratio::new(25000, 1001));
    assert_eq!(Ratio::from(FrameRate::_29_97), Ratio::new(30000, 1001));
    assert_eq!(Ratio::from(FrameRate::_59_94), Ratio::new(60000, 1001));
    let rational_2_3 = Ratio::new(2, 3);
    let rational_6_9 = Ratio::new(6, 9);
    assert_eq!(Ratio::from(FrameRate::FrCustom(rational_2_3)), rational_2_3);
    assert_eq!(Ratio::from(FrameRate::FrCustom(rational_6_9)), rational_2_3);
  }

  #[test]
  fn frame_rate_from_rational() {
    assert_eq!(FrameRate::from(Ratio::from_integer(24)), FrameRate::_24_00);
    assert_eq!(FrameRate::from(Ratio::from_integer(25)), FrameRate::_25_00);
    assert_eq!(FrameRate::from(Ratio::from_integer(30)), FrameRate::_30_00);
    assert_eq!(FrameRate::from(Ratio::from_integer(50)), FrameRate::_50_00);
    assert_eq!(FrameRate::from(Ratio::from_integer(60)), FrameRate::_60_00);
    assert_eq!(
      FrameRate::from(Ratio::from_integer(120)),
      FrameRate::_120_00
    );
    assert_eq!(FrameRate::from(Ratio::new(24000, 1001)), FrameRate::_23_97);
    assert_eq!(FrameRate::from(Ratio::new(25000, 1001)), FrameRate::_24_97);
    assert_eq!(FrameRate::from(Ratio::new(30000, 1001)), FrameRate::_29_97);
    assert_eq!(FrameRate::from(Ratio::new(60000, 1001)), FrameRate::_59_94);
    let rational_2_3 = Ratio::new(2, 3);
    let rational_6_9 = Ratio::new(6, 9);
    assert_eq!(
      FrameRate::from(rational_2_3),
      FrameRate::FrCustom(rational_2_3)
    );
    assert_eq!(
      FrameRate::from(rational_2_3),
      FrameRate::FrCustom(rational_6_9)
    );
    assert_eq!(FrameRate::from(Ratio::new(200, 4)), FrameRate::_50_00);
  }

  #[test]
  fn serialize() {
    assert_eq!(
      serde_json::to_value(FrameRate::_24_00).unwrap(),
      serde_json::json!({
        "num": 24,
        "den": 1
      })
    );
    assert_eq!(
      serde_json::to_value(FrameRate::_25_00).unwrap(),
      serde_json::json!({
        "num": 25,
        "den": 1
      })
    );
    assert_eq!(
      serde_json::to_value(FrameRate::_30_00).unwrap(),
      serde_json::json!({
        "num": 30,
        "den": 1
      })
    );
    assert_eq!(
      serde_json::to_value(FrameRate::_50_00).unwrap(),
      serde_json::json!({
        "num": 50,
        "den": 1
      })
    );
    assert_eq!(
      serde_json::to_value(FrameRate::_60_00).unwrap(),
      serde_json::json!({
        "num": 60,
        "den": 1
      })
    );
    assert_eq!(
      serde_json::to_value(FrameRate::_120_00).unwrap(),
      serde_json::json!({
        "num": 120,
        "den": 1
      })
    );
    assert_eq!(
      serde_json::to_value(FrameRate::_23_97).unwrap(),
      serde_json::json!({
        "num": 24000,
        "den": 1001
      })
    );
    assert_eq!(
      serde_json::to_value(FrameRate::_24_97).unwrap(),
      serde_json::json!({
        "num": 25000,
        "den": 1001
      })
    );
    assert_eq!(
      serde_json::to_value(FrameRate::_29_97).unwrap(),
      serde_json::json!({
        "num": 30000,
        "den": 1001
      })
    );
    assert_eq!(
      serde_json::to_value(FrameRate::_59_94).unwrap(),
      serde_json::json!({
        "num": 60000,
        "den": 1001
      })
    );
    assert_eq!(
      serde_json::to_value(FrameRate::FrCustom(Ratio::new(2, 3))).unwrap(),
      serde_json::json!({
        "num": 2,
        "den": 3
      })
    );
    assert_eq!(
      serde_json::to_value(FrameRate::FrCustom(Ratio::new(6, 9))).unwrap(),
      serde_json::json!({
        "num": 2,
        "den": 3
      })
    );
  }

  #[test]
  fn deserialize() {
    assert_eq!(
      serde_json::from_value::<FrameRate>(serde_json::json!({
        "num": 24,
        "den": 1
      }))
      .unwrap(),
      FrameRate::_24_00
    );
    assert_eq!(
      serde_json::from_value::<FrameRate>(serde_json::json!({
        "num": 25,
        "den": 1
      }))
      .unwrap(),
      FrameRate::_25_00
    );
    assert_eq!(
      serde_json::from_value::<FrameRate>(serde_json::json!({
        "num": 30,
        "den": 1
      }))
      .unwrap(),
      FrameRate::_30_00
    );
    assert_eq!(
      serde_json::from_value::<FrameRate>(serde_json::json!({
        "num": 50,
        "den": 1
      }))
      .unwrap(),
      FrameRate::_50_00
    );
    assert_eq!(
      serde_json::from_value::<FrameRate>(serde_json::json!({
        "num": 60,
        "den": 1
      }))
      .unwrap(),
      FrameRate::_60_00
    );
    assert_eq!(
      serde_json::from_value::<FrameRate>(serde_json::json!({
        "num": 120,
        "den": 1
      }))
      .unwrap(),
      FrameRate::_120_00
    );
    assert_eq!(
      serde_json::from_value::<FrameRate>(serde_json::json!({
        "num": 24000,
        "den": 1001
      }))
      .unwrap(),
      FrameRate::_23_97
    );
    assert_eq!(
      serde_json::from_value::<FrameRate>(serde_json::json!({
        "num": 25000,
        "den": 1001
      }))
      .unwrap(),
      FrameRate::_24_97
    );
    assert_eq!(
      serde_json::from_value::<FrameRate>(serde_json::json!({
        "num": 30000,
        "den": 1001
      }))
      .unwrap(),
      FrameRate::_29_97
    );
    assert_eq!(
      serde_json::from_value::<FrameRate>(serde_json::json!({
        "num": 60000,
        "den": 1001
      }))
      .unwrap(),
      FrameRate::_59_94
    );
    assert_eq!(
      serde_json::from_value::<FrameRate>(serde_json::json!({
        "num": 2,
        "den": 3
      }))
      .unwrap(),
      FrameRate::FrCustom(Ratio::new(2, 3))
    );
    assert_eq!(
      serde_json::from_value::<FrameRate>(serde_json::json!({
        "num": 6,
        "den": 9
      }))
      .unwrap(),
      FrameRate::FrCustom(Ratio::new(2, 3))
    );
    assert_eq!(
      serde_json::from_value::<FrameRate>(serde_json::json!({
        "num": 200,
        "den": 4
      }))
      .unwrap(),
      FrameRate::_50_00
    );
  }
}
