//! This module defines constants and types that are very specific to the
//! ArangoDB REST API.
//!
//! These constants and types are used by implementations of the `rincon_core`
//! API. **The average application will not need to use anything from this
//! module directly**.

#[cfg(test)]
mod tests;

use std::fmt;
use std::marker::PhantomData;

use regex::Regex;
use serde::de::{Deserialize, DeserializeOwned, Deserializer, MapAccess, Visitor};
use serde::ser::{Serialize, Serializer};

use api::{self, method};

#[allow(missing_docs)] pub const FIELD_CODE: &str = "code";
#[allow(missing_docs)] pub const FIELD_COLLECTIONS: &str = "collections";
#[allow(missing_docs)] pub const FIELD_EDGE: &str = "edge";
#[allow(missing_docs)] pub const FIELD_EDGE_DEFINITIONS: &str = "edgeDefinitions";
#[allow(missing_docs)] pub const FIELD_ENTITY_FROM: &str = "_from";
#[allow(missing_docs)] pub const FIELD_ENTITY_ID: &str = "_id";
#[allow(missing_docs)] pub const FIELD_ENTITY_KEY: &str = "_key";
#[allow(missing_docs)] pub const FIELD_ENTITY_REVISION: &str = "_rev";
#[allow(missing_docs)] pub const FIELD_ENTITY_NEW: &str = "new";
#[allow(missing_docs)] pub const FIELD_ENTITY_OLD: &str = "old";
#[allow(missing_docs)] pub const FIELD_ENTITY_OLD_REVISION: &str = "_oldRev";
#[allow(missing_docs)] pub const FIELD_ENTITY_TO: &str = "_to";
#[allow(missing_docs)] pub const FIELD_ERROR: &str = "error";
#[allow(missing_docs)] pub const FIELD_ERROR_MESSAGE: &str = "errorMessage";
#[allow(missing_docs)] pub const FIELD_ERROR_NUMBER: &str = "errorNum";
#[allow(missing_docs)] pub const FIELD_GRAPH: &str = "graph";
#[allow(missing_docs)] pub const FIELD_GRAPHS: &str = "graphs";
#[allow(missing_docs)] pub const FIELD_ID: &str = "id";
#[allow(missing_docs)] pub const FIELD_IS_SMART: &str = "isSmart";
#[allow(missing_docs)] pub const FIELD_NAME: &str = "name";
#[allow(missing_docs)] pub const FIELD_NUMBER_OF_SHARDS: &str = "numberOfShards";
#[allow(missing_docs)] pub const FIELD_ORPHAN_COLLECTIONS: &str = "orphanCollections";
#[allow(missing_docs)] pub const FIELD_REMOVED: &str = "removed";
#[allow(missing_docs)] pub const FIELD_REPLICATION_FACTOR: &str = "replicationFactor";
#[allow(missing_docs)] pub const FIELD_RESULT: &str = "result";
#[allow(missing_docs)] pub const FIELD_SMART_GRAPH_ATTRIBUTE: &str = "smartGraphAttribute";
#[allow(missing_docs)] pub const FIELD_VERTEX: &str = "vertex";

#[allow(missing_docs)] pub const HEADER_IF_MATCH: &str = "If-Match";
#[allow(missing_docs)] pub const HEADER_IF_NON_MATCH: &str = "If-None-Match";

#[allow(missing_docs)] pub const PARAM_COLLECTION: &str = "collection";
#[allow(missing_docs)] pub const PARAM_DETAILS: &str = "details";
#[allow(missing_docs)] pub const PARAM_EXCLUDE_SYSTEM: &str = "excludeSystem";
#[allow(missing_docs)] pub const PARAM_IGNORE_REVISIONS: &str = "ignoreRevs";
#[allow(missing_docs)] pub const PARAM_KEEP_NULL: &str = "keepNull";
#[allow(missing_docs)] pub const PARAM_MERGE_OBJECTS: &str = "mergeObjects";
#[allow(missing_docs)] pub const PARAM_ONLY_GET: &str = "onlyget";
#[allow(missing_docs)] pub const PARAM_RETURN_NEW: &str = "returnNew";
#[allow(missing_docs)] pub const PARAM_RETURN_OLD: &str = "returnOld";
#[allow(missing_docs)] pub const PARAM_WAIT_FOR_SYNC: &str = "waitForSync";
#[allow(missing_docs)] pub const PARAM_WAIT_FOR_SYNC_REPLICATION: &str = "waitForSyncReplication";
#[allow(missing_docs)] pub const PARAM_WITH_REVISIONS: &str = "withRevisions";
#[allow(missing_docs)] pub const PARAM_WITH_DATA: &str = "withData";

#[allow(missing_docs)] pub const PATH_ADMIN: &str = "/_admin";
#[allow(missing_docs)] pub const PATH_API_COLLECTION: &str = "/_api/collection";
#[allow(missing_docs)] pub const PATH_API_CURSOR: &str = "/_api/cursor";
#[allow(missing_docs)] pub const PATH_API_DATABASE: &str = "/_api/database";
#[allow(missing_docs)] pub const PATH_API_DOCUMENT: &str = "/_api/document";
#[allow(missing_docs)] pub const PATH_API_EXPLAIN: &str = "/_api/explain";
#[allow(missing_docs)] pub const PATH_API_GHARIAL: &str = "/_api/gharial";
#[allow(missing_docs)] pub const PATH_API_INDEX: &str = "/_api/index";
#[allow(missing_docs)] pub const PATH_API_QUERY: &str = "/_api/query";
#[allow(missing_docs)] pub const PATH_API_USER: &str = "/_api/user";
#[allow(missing_docs)] pub const PATH_API_VERSION: &str = "/_api/version";
#[allow(missing_docs)] pub const PATH_OPEN_AUTH: &str = "/_open/auth";

#[allow(missing_docs)] pub const PATH_CURRENT: &str = "/current";
#[allow(missing_docs)] pub const PATH_DATABASE: &str = "/database";
#[allow(missing_docs)] pub const PATH_DB: &str = "/_db/";
#[allow(missing_docs)] pub const PATH_EDGE: &str = "/edge";
#[allow(missing_docs)] pub const PATH_PROPERTIES: &str = "/properties";
#[allow(missing_docs)] pub const PATH_RENAME: &str = "/rename";
#[allow(missing_docs)] pub const PATH_REVISION: &str = "/revision";
#[allow(missing_docs)] pub const PATH_CHECKSUM: &str = "/checksum";
#[allow(missing_docs)] pub const PATH_DOCUMENT_COUNT: &str = "/count";
#[allow(missing_docs)] pub const PATH_LOAD: &str = "/load";
#[allow(missing_docs)] pub const PATH_UNLOAD: &str = "/unload";
#[allow(missing_docs)] pub const PATH_TARGET_VERSION: &str = "/target-version";
#[allow(missing_docs)] pub const PATH_USER: &str = "/user";
#[allow(missing_docs)] pub const PATH_VERTEX: &str = "/vertex";

#[allow(missing_docs)] pub const SYSTEM_DATABASE: &str = "_system";

const CAPTURE_CONTEXT_NAME: &str = "ctx";
const CAPTURE_ELEMENT_KEY: &str = "key";
const REGEX_ID_CAPTURE: &str = "^((?P<ctx>[^/]+)/)?(?P<key>[^/]+)$";


/// A handle as used by the ArangoDB REST API for identifying entities like
/// documents, indexes and cursors.
///
/// A handle can be either `Qualified` which means it consists of a context and
/// an key or context `Local` which holds the key only. For example a
/// `DocumentId` is qualified and consists of the key of a document and the
/// name of the collection where this document is located in.
///
/// This is the common implementation of such a handle. The methods use
/// specialized types for each entity, like `DocumentIdOption` and
/// `IndexIdOption`.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum HandleOption {
    /// A fully qualified handle with context and key information.
    Qualified(Handle),
    /// A handle key that is local to a context that is specified elsewhere.
    Local(HandleKey),
}

impl HandleOption {
    /// Tries to create `HandleOption` from a handle name and value.
    pub fn from_str(handle_name: &str, value: &str) -> Result<HandleOption, String> {
        let re = Regex::new(REGEX_ID_CAPTURE).unwrap();
        if let Some(caps) = re.captures(value) {
            match (caps.name(CAPTURE_CONTEXT_NAME), caps.name(CAPTURE_ELEMENT_KEY)) {
                (Some(context_name), Some(element_key)) =>
                    Ok(HandleOption::Qualified(Handle {
                        context: context_name.as_str().to_string(),
                        key: element_key.as_str().to_string(),
                    })),
                (None, Some(element_key)) =>
                    Ok(HandleOption::Local(HandleKey(
                        element_key.as_str().to_string()
                    ))),
                (_, None) =>
                    Err(format!("{} does not have a key: {:?}", handle_name, value)),
            }
        } else {
            Err(format!("Invalid {}: {:?}", handle_name, value))
        }
    }
}

impl Serialize for HandleOption {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where S: Serializer
    {
        match *self {
            HandleOption::Qualified(ref handle) => handle.serialize(serializer),
            HandleOption::Local(ref handle_key) => handle_key.serialize(serializer),
        }
    }
}

impl<'de> Deserialize<'de> for HandleOption {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where D: Deserializer<'de>
    {
        use serde::de::Error;
        let value = String::deserialize(deserializer)?;
        HandleOption::from_str("handle", &value).map_err(D::Error::custom)
    }
}

/// A qualified handle with defined context and key.
///
/// see the documentation of `HandleOption` for more details.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Handle {
    context: String,
    key: String,
}

impl Handle {
    /// Tries to create a `Handle` from a handle name and value.
    pub fn from_str(handle_name: &str, value: &str) -> Result<Self, String> {
        let re = Regex::new(REGEX_ID_CAPTURE).unwrap();
        if let Some(caps) = re.captures(value) {
            match (caps.name(CAPTURE_CONTEXT_NAME), caps.name(CAPTURE_ELEMENT_KEY)) {
                (Some(context_name), Some(element_key)) =>
                    Ok(Handle {
                        context: context_name.as_str().to_string(),
                        key: element_key.as_str().to_string(),
                    }),
                (None, _) =>
                    Err(format!("{} does not have a context: {:?}", handle_name, value)),
                (_, None) =>
                    Err(format!("{} does not have a key: {:?}", handle_name, value)),
            }
        } else {
            Err(format!("Invalid {}: {:?}", handle_name, value))
        }
    }

    /// Unwraps this `Handle` into its context and key.
    pub fn unwrap(self) -> (String, String) {
        (self.context, self.key)
    }

    /// Formats this `Handle` as its string representation.
    pub fn to_string(&self) -> String {
        self.context.to_owned() + "/" + &self.key
    }
}

impl From<Handle> for HandleOption {
    fn from(handle: Handle) -> Self {
        HandleOption::Qualified(handle)
    }
}

impl Serialize for Handle {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where S: Serializer
    {
        serializer.serialize_str(&self.to_string())
    }
}

impl<'de> Deserialize<'de> for Handle {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where D: Deserializer<'de>
    {
        use serde::de::Error;
        let value = String::deserialize(deserializer)?;
        Handle::from_str("handle", &value).map_err(D::Error::custom)
    }
}

/// A local handle specifying the key only. The context must be specified by
/// other means, like for example another parameter of a method.
///
/// This is basically a new type for key strings.
///
/// see also the documentation of `HandleOption` for more details.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct HandleKey(String);

impl HandleKey {
    /// Tries to create a `HandleKey` from a handle name and value.
    pub fn from_string(handle_name: &str, value: String) -> Result<Self, String> {
        if value.contains('/') {
            Err(format!("A {} key must not contain any '/' character, but got: {:?}",
                handle_name, value))
        } else {
            Ok(HandleKey(value))
        }
    }

    /// Unwraps this `HandleKey` into its key string.
    pub fn unwrap(self) -> String {
        self.0
    }
}

impl From<HandleKey> for HandleOption {
    fn from(handle_key: HandleKey) -> Self {
        HandleOption::Local(handle_key)
    }
}

impl Serialize for HandleKey {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where S: Serializer
    {
        serializer.serialize_str(&self.0)
    }
}

impl<'de> Deserialize<'de> for HandleKey {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where D: Deserializer<'de>
    {
        use serde::de::Error;
        let value = String::deserialize(deserializer)?;
        HandleKey::from_string("handle", value).map_err(D::Error::custom)
    }
}

impl<'de, T> Deserialize<'de> for method::Result<T>
    where T: DeserializeOwned
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where D: Deserializer<'de>
    {
        use serde::de::Error;
        use serde_json::{Map, Value, from_value};

        #[derive(Debug)]
        enum Field {
            Code,
            Error,
            ErrorNumber,
            ErrorMessage,
            Other(String),
        }

        impl<'de> Deserialize<'de> for Field {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
                where D: Deserializer<'de>
            {
                struct FieldVisitor;

                impl<'de> Visitor<'de> for FieldVisitor {
                    type Value = Field;

                    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                        formatter.write_str("a valid object or an error object with fields 'code', 'errorNum' and 'errorMessage'")
                    }

                    fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
                        where E: Error
                    {
                        Ok(match value {
                            FIELD_CODE => Field::Code,
                            FIELD_ERROR => Field::Error,
                            FIELD_ERROR_NUMBER => Field::ErrorNumber,
                            FIELD_ERROR_MESSAGE => Field::ErrorMessage,
                            _ => Field::Other(value.to_owned()),
                        })
                    }
                }

                deserializer.deserialize_str(FieldVisitor)
            }
        }

        struct ResultVisitor<T>(PhantomData<T>);

        impl<'de, T> Visitor<'de> for ResultVisitor<T>
            where T: DeserializeOwned
        {
            type Value = method::Result<T>;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("enum method::Result")
            }

            fn visit_map<A>(self, map: A) -> Result<Self::Value, A::Error>
                where A: MapAccess<'de>,
            {
                let mut error_code: Option<api::ErrorCode> = None;
                let mut message: Option<String> = None;
                let mut other = Map::new();
                let mut fields = map;
                while let Some(name) = fields.next_key()? {
                    match name {
                        Field::Code => {
                            let _: i32 = fields.next_value()?;
                        },
                        Field::Error => {
                            let _: bool = fields.next_value()?;
                        },
                        Field::ErrorNumber => {
                            error_code = fields.next_value()?;
                        },
                        Field::ErrorMessage => {
                            message = fields.next_value()?;
                        },
                        Field::Other(name) => {
                            other.insert(name, fields.next_value()?);
                        },
                    }
                }
                match (error_code, message) {
                    (Some(error_code), Some(message)) => {
                        let error = method::Error::new(error_code, message);
                        Ok(method::Result::Failed(error))
                    },
                    (Some(_), None) => Err(A::Error::missing_field(FIELD_ERROR_MESSAGE)),
                    (None, _) => {
                        let result_value = from_value(Value::Object(other)).map_err(A::Error::custom)?;
                        Ok(method::Result::Success(result_value))
                    },
                }
            }
        }

        deserializer.deserialize_map(ResultVisitor(PhantomData))
    }
}
