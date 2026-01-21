use std::str::FromStr;

use redis::AsyncTypedCommands;
use uuid::Uuid;

use crate::{errors::RouteError, routes::node::Node};

pub const KEY_PREFIX: &str = "node";

pub const DEPLOYED_BUNDLE_CACHE_PREFIX: &str = "bundle_deployment";

pub trait CacheAccessor {
    async fn node(&self, id: Uuid) -> Result<Node, RouteError>;

    async fn node_set(&self, node: &Node) -> Result<(), RouteError>;

    async fn node_del(&self, id: Uuid) -> Result<(), RouteError>;

    async fn deployed_bundle_node_id(&self, bundle_id: Uuid) -> Result<Uuid, RouteError>;

    async fn deployed_bundle_set(&self, bundle_id: Uuid, node_id: Uuid) -> Result<(), RouteError>;

    async fn deployed_bundle_del(&self, bundle_id: Uuid) -> Result<(), RouteError>;
}

fn node_key(id: Uuid) -> String {
    format!("{}:{}", KEY_PREFIX, id)
}

fn deployed_bundle_key(id: Uuid) -> String {
    format!("{}:{}", DEPLOYED_BUNDLE_CACHE_PREFIX, id)
}

impl CacheAccessor for redis::Client {
    async fn node(&self, id: Uuid) -> Result<Node, RouteError> {
        let mut connection = self.get_multiplexed_async_connection().await?;

        let node = serde_json::from_str(
            &connection
                .get::<String>(node_key(id))
                .await?
                .ok_or(RouteError::NotFound("node with specified id"))?,
        )
        .map_err(|_| RouteError::Unexpected("corrupted data".to_owned()))?;

        Ok(node)
    }

    async fn node_set(&self, node: &Node) -> Result<(), RouteError> {
        let mut connection = self.get_multiplexed_async_connection().await?;

        connection
            .set(
                node_key(node.id),
                serde_json::to_string(&node).expect("serde can't fail"),
            )
            .await?;

        Ok(())
    }

    async fn node_del(&self, id: Uuid) -> Result<(), RouteError> {
        let mut connection = self.get_multiplexed_async_connection().await?;

        connection.del(node_key(id)).await?;

        Ok(())
    }

    async fn deployed_bundle_node_id(&self, bundle_id: Uuid) -> Result<Uuid, RouteError> {
        let mut connection = self.get_multiplexed_async_connection().await?;

        let node_id = connection
            .get(deployed_bundle_key(bundle_id))
            .await?
            .map(|this| Uuid::from_str(&this).ok())
            .flatten()
            .ok_or(RouteError::NotFound("node with this bundle"))?;

        Ok(node_id)
    }

    async fn deployed_bundle_set(&self, bundle_id: Uuid, node_id: Uuid) -> Result<(), RouteError> {
        let mut connection = self.get_multiplexed_async_connection().await?;

        connection
            .set(deployed_bundle_key(bundle_id), node_id.to_string())
            .await?;

        Ok(())
    }

    async fn deployed_bundle_del(&self, bundle_id: Uuid) -> Result<(), RouteError> {
        let mut connection = self.get_multiplexed_async_connection().await?;

        connection.del(deployed_bundle_key(bundle_id)).await?;

        Ok(())
    }
}
