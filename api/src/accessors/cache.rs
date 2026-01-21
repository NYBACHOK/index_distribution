use std::str::FromStr;

use redis::AsyncTypedCommands;
use uuid::Uuid;

use crate::{errors::RouteError, routes::node::Node};

pub const KEY_PREFIX: &str = "node";

pub const DEPLOYED_BUNDLE_CACHE_PREFIX: &str = "bundle_deployment";
pub const DEPLOYED_NODE_CACHE_PREFIX: &str = "node_deployment";

#[derive(Debug, Clone, Copy)]
pub enum FindBy {
    Bundle(Uuid),
    Node(Uuid),
}

pub trait CacheAccessor {
    async fn node(&self, id: Uuid) -> Result<Node, RouteError>;

    async fn node_set(&self, node: &Node) -> Result<(), RouteError>;

    async fn node_del(&self, id: Uuid) -> Result<(), RouteError>;

    async fn deployed_bundle(&self, by: FindBy) -> Result<Uuid, RouteError>;

    async fn deployed_bundle_set(&self, bundle_id: Uuid, node_id: Uuid) -> Result<(), RouteError>;

    async fn deployed_bundle_del_by(&self, by: FindBy) -> Result<Uuid, RouteError>;
}

pub fn node_key(id: Uuid) -> String {
    format!("{}:{}", KEY_PREFIX, id)
}

pub fn deployed_bundle_key(id: Uuid) -> String {
    format!("{}:{}", DEPLOYED_BUNDLE_CACHE_PREFIX, id)
}

pub fn deployed_node_key(id: Uuid) -> String {
    format!("{}:{}", DEPLOYED_NODE_CACHE_PREFIX, id)
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

    async fn deployed_bundle(&self, by: FindBy) -> Result<Uuid, RouteError> {
        let mut connection = self.get_multiplexed_async_connection().await?;

        let key = match by {
            FindBy::Bundle(uuid) => deployed_bundle_key(uuid),
            FindBy::Node(uuid) => deployed_node_key(uuid),
        };

        let node_id = connection
            .get(key)
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

        connection
            .set(deployed_node_key(node_id), bundle_id.to_string())
            .await?;

        Ok(())
    }

    async fn deployed_bundle_del_by(&self, by: FindBy) -> Result<Uuid, RouteError> {
        let mut connection = self.get_multiplexed_async_connection().await?;

        let key = match by {
            FindBy::Bundle(uuid) => deployed_bundle_key(uuid),
            FindBy::Node(uuid) => deployed_node_key(uuid),
        };

        let uuid: Uuid = connection
            .get_del(key)
            .await?
            .ok_or(RouteError::NotFound("node or bundle deployment record"))?
            .parse()
            .map_err(|_| {
                RouteError::Unexpected(format!("corrupted data for deployment records in cache"))
            })?;

        match by {
            FindBy::Bundle(_) => connection.del(deployed_node_key(uuid)).await?,
            FindBy::Node(_) => connection.del(deployed_bundle_key(uuid)).await?,
        };

        Ok(uuid)
    }
}
