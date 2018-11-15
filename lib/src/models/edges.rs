use super::types::Type;
use super::ids::Id;
use chrono::offset::Utc;
use chrono::DateTime;

/// Represents a uniquely identifiable key to an edge.
#[derive(Clone, Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub struct EdgeKey {
    /// The id of the outbound vertex.
    pub outbound_id: Id,

    /// The type of the edge.
    pub t: Type,

    /// The id of the inbound vertex.
    pub inbound_id: Id,
}

impl EdgeKey {
    /// Creates a new edge key.
    ///
    /// # Arguments
    ///
    /// * `outbound_id` - The id of the outbound vertex.
    /// * `t` - The type of the edge.
    /// * `inbound_id` - The id of the inbound vertex.
    pub fn new(outbound_id: Id, t: Type, inbound_id: Id) -> EdgeKey {
        EdgeKey {
            outbound_id,
            t,
            inbound_id,
        }
    }
}

/// An edge.
///
/// Edges are how you would represent a verb or a relationship in the
/// datastore. An example might be "liked" or "reviewed". Edges are typed
/// and directed.
#[derive(Clone, Debug)]
pub struct Edge {
    /// The key to the edge.
    pub key: EdgeKey,

    /// When the edge was created.
    pub created_datetime: DateTime<Utc>,
}

impl Edge {
    /// Creates a new edge with the current datetime in UTC.
    ///
    /// # Arguments
    /// * `key` - The key to the edge.
    pub fn new_with_current_datetime(key: EdgeKey) -> Edge {
        Self::new(key, Utc::now())
    }

    /// Creates a new edge with a specified datetime.
    ///
    /// # Arguments
    /// * `key` - The key to the edge.
    /// * `created_datetime` - When the edge was created.
    pub fn new(key: EdgeKey, created_datetime: DateTime<Utc>) -> Edge {
        Edge { key, created_datetime }
    }
}

#[cfg(test)]
mod tests {
    use super::{Edge, EdgeKey, Id};
    use chrono::Utc;
    use models::Type;

    #[test]
    fn should_create_edge_with_current_datetime() {
        let start_datetime = Utc::now();

        let edge = Edge::new_with_current_datetime(EdgeKey::new(
            Id::default(),
            Type::default(),
            Id::default(),
        ));

        let end_datetime = Utc::now();

        assert!(edge.created_datetime >= start_datetime);
        assert!(edge.created_datetime <= end_datetime);
    }
}
