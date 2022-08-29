use num_rational::Ratio;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FrameRate {
  Fr24,
  Fr25,
  Fr30,
  Fr50,
  Fr60,
  Fr120,
  Fr23_97,
  Fr24_97,
  Fr29_97,
  Fr59_94,
  FrCustom(Ratio<u32>),
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
      FrameRate::Fr24 => Self::from_integer(24),
      FrameRate::Fr25 => Self::from_integer(25),
      FrameRate::Fr30 => Self::from_integer(30),
      FrameRate::Fr50 => Self::from_integer(50),
      FrameRate::Fr60 => Self::from_integer(60),
      FrameRate::Fr120 => Self::from_integer(120),
      FrameRate::Fr23_97 => Self::new(24000, 1001),
      FrameRate::Fr24_97 => Self::new(25000, 1001),
      FrameRate::Fr29_97 => Self::new(30000, 1001),
      FrameRate::Fr59_94 => Self::new(60000, 1001),
      FrameRate::FrCustom(rational) => rational,
    }
  }
}

impl From<Ratio<u32>> for FrameRate {
  fn from(rational: Ratio<u32>) -> Self {
    match (rational.numer(), rational.denom()) {
      (24, 1) => Self::Fr24,
      (25, 1) => Self::Fr25,
      (30, 1) => Self::Fr30,
      (50, 1) => Self::Fr50,
      (60, 1) => Self::Fr60,
      (120, 1) => Self::Fr120,
      (24000, 1001) => Self::Fr23_97,
      (25000, 1001) => Self::Fr24_97,
      (30000, 1001) => Self::Fr29_97,
      (60000, 1001) => Self::Fr59_94,
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
    assert_eq!(Ratio::from(FrameRate::Fr24), Ratio::from_integer(24));
    assert_eq!(Ratio::from(FrameRate::Fr25), Ratio::from_integer(25));
    assert_eq!(Ratio::from(FrameRate::Fr30), Ratio::from_integer(30));
    assert_eq!(Ratio::from(FrameRate::Fr50), Ratio::from_integer(50));
    assert_eq!(Ratio::from(FrameRate::Fr60), Ratio::from_integer(60));
    assert_eq!(Ratio::from(FrameRate::Fr120), Ratio::from_integer(120));
    assert_eq!(Ratio::from(FrameRate::Fr23_97), Ratio::new(24000, 1001));
    assert_eq!(Ratio::from(FrameRate::Fr24_97), Ratio::new(25000, 1001));
    assert_eq!(Ratio::from(FrameRate::Fr29_97), Ratio::new(30000, 1001));
    assert_eq!(Ratio::from(FrameRate::Fr59_94), Ratio::new(60000, 1001));
    let rational_2_3 = Ratio::new(2, 3);
    let rational_6_9 = Ratio::new(6, 9);
    assert_eq!(Ratio::from(FrameRate::FrCustom(rational_2_3)), rational_2_3);
    assert_eq!(Ratio::from(FrameRate::FrCustom(rational_6_9)), rational_2_3);
  }

  #[test]
  fn frame_rate_from_rational() {
    assert_eq!(FrameRate::from(Ratio::from_integer(24)), FrameRate::Fr24);
    assert_eq!(FrameRate::from(Ratio::from_integer(25)), FrameRate::Fr25);
    assert_eq!(FrameRate::from(Ratio::from_integer(30)), FrameRate::Fr30);
    assert_eq!(FrameRate::from(Ratio::from_integer(50)), FrameRate::Fr50);
    assert_eq!(FrameRate::from(Ratio::from_integer(60)), FrameRate::Fr60);
    assert_eq!(FrameRate::from(Ratio::from_integer(120)), FrameRate::Fr120);
    assert_eq!(FrameRate::from(Ratio::new(24000, 1001)), FrameRate::Fr23_97);
    assert_eq!(FrameRate::from(Ratio::new(25000, 1001)), FrameRate::Fr24_97);
    assert_eq!(FrameRate::from(Ratio::new(30000, 1001)), FrameRate::Fr29_97);
    assert_eq!(FrameRate::from(Ratio::new(60000, 1001)), FrameRate::Fr59_94);
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
    assert_eq!(FrameRate::from(Ratio::new(200, 4)), FrameRate::Fr50);
  }

  #[test]
  fn serialize() {
    assert_eq!(
      serde_json::to_value(&FrameRate::Fr24).unwrap(),
      serde_json::json!({
        "num": 24,
        "den": 1
      })
    );
    assert_eq!(
      serde_json::to_value(&FrameRate::Fr25).unwrap(),
      serde_json::json!({
        "num": 25,
        "den": 1
      })
    );
    assert_eq!(
      serde_json::to_value(&FrameRate::Fr30).unwrap(),
      serde_json::json!({
        "num": 30,
        "den": 1
      })
    );
    assert_eq!(
      serde_json::to_value(&FrameRate::Fr50).unwrap(),
      serde_json::json!({
        "num": 50,
        "den": 1
      })
    );
    assert_eq!(
      serde_json::to_value(&FrameRate::Fr60).unwrap(),
      serde_json::json!({
        "num": 60,
        "den": 1
      })
    );
    assert_eq!(
      serde_json::to_value(&FrameRate::Fr120).unwrap(),
      serde_json::json!({
        "num": 120,
        "den": 1
      })
    );
    assert_eq!(
      serde_json::to_value(&FrameRate::Fr23_97).unwrap(),
      serde_json::json!({
        "num": 24000,
        "den": 1001
      })
    );
    assert_eq!(
      serde_json::to_value(&FrameRate::Fr24_97).unwrap(),
      serde_json::json!({
        "num": 25000,
        "den": 1001
      })
    );
    assert_eq!(
      serde_json::to_value(&FrameRate::Fr29_97).unwrap(),
      serde_json::json!({
        "num": 30000,
        "den": 1001
      })
    );
    assert_eq!(
      serde_json::to_value(&FrameRate::Fr59_94).unwrap(),
      serde_json::json!({
        "num": 60000,
        "den": 1001
      })
    );
    assert_eq!(
      serde_json::to_value(&FrameRate::FrCustom(Ratio::new(2, 3))).unwrap(),
      serde_json::json!({
        "num": 2,
        "den": 3
      })
    );
    assert_eq!(
      serde_json::to_value(&FrameRate::FrCustom(Ratio::new(6, 9))).unwrap(),
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
      FrameRate::Fr24
    );
    assert_eq!(
      serde_json::from_value::<FrameRate>(serde_json::json!({
        "num": 25,
        "den": 1
      }))
        .unwrap(),
      FrameRate::Fr25
    );
    assert_eq!(
      serde_json::from_value::<FrameRate>(serde_json::json!({
        "num": 30,
        "den": 1
      }))
        .unwrap(),
      FrameRate::Fr30
    );
    assert_eq!(
      serde_json::from_value::<FrameRate>(serde_json::json!({
        "num": 50,
        "den": 1
      }))
        .unwrap(),
      FrameRate::Fr50
    );
    assert_eq!(
      serde_json::from_value::<FrameRate>(serde_json::json!({
        "num": 60,
        "den": 1
      }))
        .unwrap(),
      FrameRate::Fr60
    );
    assert_eq!(
      serde_json::from_value::<FrameRate>(serde_json::json!({
        "num": 120,
        "den": 1
      }))
        .unwrap(),
      FrameRate::Fr120
    );
    assert_eq!(
      serde_json::from_value::<FrameRate>(serde_json::json!({
        "num": 24000,
        "den": 1001
      }))
        .unwrap(),
      FrameRate::Fr23_97
    );
    assert_eq!(
      serde_json::from_value::<FrameRate>(serde_json::json!({
        "num": 25000,
        "den": 1001
      }))
        .unwrap(),
      FrameRate::Fr24_97
    );
    assert_eq!(
      serde_json::from_value::<FrameRate>(serde_json::json!({
        "num": 30000,
        "den": 1001
      }))
        .unwrap(),
      FrameRate::Fr29_97
    );
    assert_eq!(
      serde_json::from_value::<FrameRate>(serde_json::json!({
        "num": 60000,
        "den": 1001
      }))
        .unwrap(),
      FrameRate::Fr59_94
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
      FrameRate::Fr50
    );
  }
}
