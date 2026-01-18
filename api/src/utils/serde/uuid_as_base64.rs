use serde::de::Error;
use uuid::Uuid;

struct Visitor;

impl serde::de::Visitor<'_> for Visitor {
    type Value = String;

    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "expected decimal")
    }

    fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(v)
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(v.to_string())
    }
}

pub fn serialize<S: ::serde::Serializer>(id: &Uuid, serializer: S) -> Result<S::Ok, S::Error> {
    serializer.serialize_str(&data_encoding::BASE64URL.encode(id.as_bytes()))
}

pub fn deserialize<'a, D: ::serde::Deserializer<'a>>(deserializer: D) -> Result<Uuid, D::Error> {
    let s = deserializer.deserialize_str(Visitor)?;
    let uuid = Uuid::try_from(
        data_encoding::BASE64URL
            .decode(s.as_bytes())
            .map_err(D::Error::custom)?,
    )
    .map_err(D::Error::custom)?;

    Ok(uuid)
}
