use num_bigint::BigUint;
use num_traits::Num;
use serde::de::{Deserialize, Deserializer};

///  G1 Point.
#[derive(Debug, serde::Serialize, serde::Deserialize, PartialEq)]
pub struct G1Point {
    /// Field x
    pub x: U252,
    /// Field y
    pub y: U252,
}

#[derive(Debug, serde::Serialize, serde::Deserialize, PartialEq)]
pub struct U252(#[serde(deserialize_with = "deserialize_big_uint")] BigUint);

impl TryFrom<&str> for U252 {
    type Error = anyhow::Error;

    fn try_from(value: &str) -> anyhow::Result<U252> {
        Ok(U252(BigUint::from_str_radix(value, 10)?))
    }
}

fn deserialize_big_uint<'de, D>(deserializer: D) -> Result<BigUint, D::Error>
where
    D: Deserializer<'de>,
{
    let buf = String::deserialize(deserializer)?;

    BigUint::from_str_radix(buf.as_str(), 10).map_err(serde::de::Error::custom)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_types() {
        let x = BigUint::from_str_radix(
            "18094334872876641868723494105288622412147911500330691749314947389229051047374",
            10,
        )
        .unwrap();
        println!("{x:?}");

        let p1: G1Point = G1Point {
            x: "9840231571876985386317977392848354901467143519108961818080766241279044324906"
                .try_into()
                .unwrap(),
            y: "2271327910224116051156093381945234779599139456906937995159191301263024040071"
                .try_into()
                .unwrap(),
        };

        let p2: G1Point = serde_json::from_str(
            r#"{
            "x": "9840231571876985386317977392848354901467143519108961818080766241279044324906",
            "y": "2271327910224116051156093381945234779599139456906937995159191301263024040071"
        }"#,
        )
        .unwrap();
        assert_eq!(p1, p2);
    }
}
