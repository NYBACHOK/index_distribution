mod create;
mod delete;
mod status;

use uuid::Uuid;

pub use self::{create::*, delete::*, status::*};

#[derive(Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
pub struct DeployBundleModel {
    #[serde(with = "crate::utils::serde::uuid_as_base64")]
    pub id: Uuid,
}
