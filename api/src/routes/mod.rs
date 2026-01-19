pub mod bundle;
pub mod deploy;
pub mod node;

pub const NODE_PREFIX: &str = "node";

#[derive(serde::Deserialize, serde::Serialize)]
pub struct UuidQuery {
    #[serde(with = "crate::utils::serde::uuid_as_base64")]
    pub id: uuid::Uuid,
}
