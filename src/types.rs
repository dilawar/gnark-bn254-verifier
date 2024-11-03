use num_bigint::BigUint;
use num_traits::Num;
use serde::de::{Deserialize, Deserializer};
use serde::Serializer;

/// Proof
#[derive(Debug, serde::Serialize, serde::Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub struct VerificationKey {
    protocol: Protocol,
    curve: Curve,
    vk_alpha_1: G2Point,
    vk_beta_2: Vec<G1Point>,
    vk_gamma_2: Vec<G1Point>,
    vk_delta_2: Vec<G1Point>,
    vk_alphabeta_12: Vec<Vec<G1Point>>,
}

#[derive(Debug, serde::Serialize, serde::Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum Protocol {
    Groth16,
}

#[derive(Debug, serde::Serialize, serde::Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum Curve {
    Bn128,
}

///  G1 Point.
#[derive(Debug, serde::Serialize, serde::Deserialize, PartialEq)]
pub struct G1Point {
    /// Field x
    pub x: U252,
    /// Field y
    pub y: U252,
}

///  G2 Point.
#[derive(Debug, serde::Serialize, serde::Deserialize, PartialEq)]
pub struct G2Point {
    /// Field x
    pub x: U252,
    /// Field y
    pub y: U252,
    /// Field z
    pub z: U252,
}

#[derive(Debug, serde::Serialize, serde::Deserialize, PartialEq)]
pub struct U252(
    #[serde(
        deserialize_with = "deserialize_big_uint",
        serialize_with = "serialize_big_uint"
    )]
    BigUint,
);

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

fn serialize_big_uint<S>(v: &BigUint, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    serializer.serialize_str(&v.to_str_radix(10))
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

        println!("{:#?}", serde_json::to_string(&p1).unwrap());
    }

    #[test]
    fn test_parse_verification() {
        let json = std::fs::read_to_string("./data/verification_key.json").unwrap();
        let _verif: VerificationKey = serde_json::from_str(&json).unwrap();
        eprintln!("verification key: {_verif:?}");
    }
}
