mod create;
mod delete;
mod status;

use uuid::Uuid;

pub use self::{create::*, delete::*, status::*};

#[derive(Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
pub struct DeployBundleModel {
    pub bundle_id: Uuid,
    pub node_id: Uuid,
}
