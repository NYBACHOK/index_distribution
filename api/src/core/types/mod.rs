use std::str::FromStr;
use uuid::Uuid;

#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    serde::Deserialize,
    serde::Serialize,
    strum::EnumString,
    strum::AsRefStr,
    utoipa::ToSchema,
)]
pub enum BundleKind {
    #[strum(to_string = "static")]
    Static,
    #[strum(to_string = "nodejs")]
    NodeJS,
}

impl TryFrom<String> for BundleKind {
    type Error = &'static str;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::from_str(&value).map_err(|_| "invalid kind")
    }
}

#[derive(Debug)]
pub struct RedeployTask {
    pub bundle_id: Uuid,
}
