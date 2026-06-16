use crate::errors::ErrorResponse;

#[derive(utoipa::OpenApi)]
#[openapi(
    paths(
        crate::routes::bundle::create,
        crate::routes::bundle::list,
        crate::routes::bundle::upload,
        crate::routes::deploy::create,
        crate::routes::deploy::delete,
        crate::routes::deploy::status,
        crate::routes::node::connect,
        crate::routes::node::disconnect,
    ),
    components(
        schemas(
            crate::routes::bundle::CreateBundleRequest,
            crate::routes::bundle::CreateBundleResponse,
            crate::routes::bundle::Bundle,
            crate::routes::bundle::ListResponse,
            crate::routes::deploy::DeployBundleModel,
            crate::routes::node::Node,
            crate::routes::node::NodeKind,
            crate::routes::node::DisconnectNode,
            crate::routes::UuidQuery,
            crate::core::types::BundleKind,
        ),
        responses(ErrorResponse)
    )
)]
pub struct ApiDoc;
