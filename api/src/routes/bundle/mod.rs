mod create;
mod deploy;
mod deploy_status;
mod list;
mod upload;

pub use self::{create::*, deploy::*, deploy_status::*, list::*, upload::*};

#[derive(serde::Deserialize, serde::Serialize)]
pub struct BundleQuery {
    #[serde(with = "crate::utils::serde::uuid_as_base64")]
    pub bundle_id: uuid::Uuid,
}
