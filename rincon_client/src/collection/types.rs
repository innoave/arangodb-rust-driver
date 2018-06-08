//! Types used in methods for managing collections.

use std::mem;
use std::i32;

use serde::de::{Deserialize, Deserializer};
use serde::ser::{Serialize, Serializer};

const COLLECTION_TYPE_DOCUMENTS: i32 = 2;
const COLLECTION_TYPE_EDGES: i32 = 3;

const COLLECTION_STATUS_NEW_BORN: i32 = 1;
const COLLECTION_STATUS_UNLOADED: i32 = 2;
const COLLECTION_STATUS_LOADED: i32 = 3;
const COLLECTION_STATUS_BEING_UNLOADED: i32 = 4;
const COLLECTION_STATUS_DELETED: i32 = 5;
const COLLECTION_STATUS_BEING_LOADED: i32 = 6;
const COLLECTION_STATUS_CORRUPTED: i32 = i32::MAX;

const KEY_GENERATOR_TYPE_TRADITIONAL: &str = "traditional";
const KEY_GENERATOR_TYPE_AUTO_INCREMENT: &str = "autoincrement";

/// This struct holds attributes of a collection.
///
/// It is returned by the `GetCollection` and `ListCollection` methods.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Collection {
    /// The id of the collection.
    id: String,

    /// The name of the collection.
    name: String,

    /// The type of the collection.
    #[serde(rename = "type")]
    kind: CollectionType,

    /// The status of the collection.
    status: CollectionStatus,

    /// Whether the collection is a system collection or regular collection.
    is_system: bool,
}

impl Collection {
    /// Returns the id of this collection.
    pub fn id(&self) -> &str {
        &self.id
    }

    /// Returns the name of this collection.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns the type of this collection.
    pub fn kind(&self) -> CollectionType {
        self.kind
    }

    /// Returns the status of this collection.
    pub fn status(&self) -> CollectionStatus {
        self.status
    }

    /// Returns whether this collection is a system collection or regular
    /// collection.
    pub fn is_system(&self) -> bool {
        self.is_system
    }

    /// Unwraps the id of this collection
    pub fn unwrap_id(self) -> String {
        self.id
    }

    /// Unwraps the name of this collection.
    pub fn unwrap_name(self) -> String {
        self.name
    }
}

impl From<CollectionChecksum> for Collection {
    fn from(properties: CollectionChecksum) -> Self {
        Collection {
            id: properties.id,
            name: properties.name,
            kind: properties.kind,
            status: properties.status,
            is_system: properties.is_system,
        }
    }
}

impl From<CollectionRevision> for Collection {
    fn from(properties: CollectionRevision) -> Self {
        Collection {
            id: properties.id,
            name: properties.name,
            kind: properties.kind,
            status: properties.status,
            is_system: properties.is_system,
        }
    }
}

impl From<BasicCollectionProperties> for Collection {
    fn from(basic_properties: BasicCollectionProperties) -> Self {
        Collection {
            id: basic_properties.id,
            name: basic_properties.name,
            kind: basic_properties.kind,
            status: basic_properties.status,
            is_system: basic_properties.is_system,
        }
    }
}

impl From<CollectionProperties> for Collection {
    fn from(properties: CollectionProperties) -> Self {
        Collection {
            id: properties.id,
            name: properties.name,
            kind: properties.kind,
            status: properties.status,
            is_system: properties.is_system,
        }
    }
}

/// This struct defines all attributes that may be specified when creating a
/// new collection.
///
/// The `name` attribute is mandatory. All other attributes are optional and
/// if not specified are assigned to their default values as defined by the
/// ArangoDB server.
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct NewCollection {
    /// The name of the collection.
    name: String,

    /// The type of the collection to create.
    /// (The default is 'Documents')
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    kind: Option<CollectionType>,

    /// If true, create a system collection. In this case collection-name
    /// should start with an underscore.
    /// (The default is false)
    ///
    /// End users should normally create non-system collections only. API
    /// implementors may be required to create system collections in very
    /// special occasions, but normally a regular collection will do.
    #[serde(skip_serializing_if = "Option::is_none")]
    is_system: Option<bool>,

    /// Key options.
    #[serde(skip_serializing_if = "Option::is_none")]
    key_options: Option<NewKeyOptions>,

    /// If true then the data is synchronized to disk before returning from a
    /// document create, update, replace or removal operation.
    /// (The default is false)
    #[serde(skip_serializing_if = "Option::is_none")]
    wait_for_sync: Option<bool>,

    #[cfg(feature = "cluster")]
    /// In a cluster, this value determines the number of shards to create for
    /// the collection.
    /// (The default is 1)
    ///
    /// In a single server setup, this option is meaningless.
    #[serde(skip_serializing_if = "Option::is_none")]
    number_of_shards: Option<u16>,

    #[cfg(feature = "cluster")]
    /// (The default is [ "_key" ])
    ///
    /// In a cluster, this attribute determines which document attributes are
    /// used to determine the target shard for documents. Documents are sent to
    /// shards based on the values of their shard key attributes. The values of
    /// all shard key attributes in a document are hashed, and the hash value
    /// is used to determine the target shard.
    ///
    /// Note: Values of shard key attributes cannot be changed once set.
    ///
    /// In a single server setup, this option is meaningless.
    #[serde(skip_serializing_if = "Option::is_none")]
    shard_keys: Option<String>,

    #[cfg(feature = "cluster")]
    /// The replication factor.
    /// (The default is 1)
    ///
    /// In a cluster, this attribute determines how many copies of each shard
    /// are kept on different DBServers. The value 1 means that only one copy
    /// (no synchronous replication) is kept. A value of k means that k-1
    /// replicas are kept. Any two copies reside on different DBServers.
    /// Replication between them is synchronous, that is, every write operation
    /// to the "leader" copy will be replicated to all "follower" replicas,
    /// before the write operation is reported successful. If a server fails,
    /// this is detected automatically and one of the servers holding copies
    /// take over, usually without an error being reported.
    ///
    /// In a single server setup, this option is meaningless.
    #[serde(skip_serializing_if = "Option::is_none")]
    replication_factor: Option<u16>,

    #[cfg(feature = "mmfiles")]
    /// If true then the collection data is kept in-memory only and not made
    /// persistent.
    /// (The default is false)
    ///
    /// Unloading the collection will cause the collection data to be
    /// discarded. Stopping or re-starting the server will also cause full
    /// loss of data in the collection. Setting this option will make the
    /// resulting collection be slightly faster than regular collections
    /// because ArangoDB does not enforce any synchronization to disk and does
    /// not calculate any CRC checksum for datafiles (as there are no
    /// datafiles). This option should therefore be used for cache-type
    /// collections only, and not for data that cannot be re-created otherwise.
    ///
    /// This option is meaningful for the MMFiles storage engine only.
    #[serde(skip_serializing_if = "Option::is_none")]
    is_volatile: Option<bool>,

    #[cfg(feature = "mmfiles")]
    /// Whether or not the collection will be compacted.
    /// (default is true)
    ///
    /// This option is meaningful for the MMFiles storage engine only.
    #[serde(skip_serializing_if = "Option::is_none")]
    do_compact: Option<bool>,

    #[cfg(feature = "mmfiles")]
    /// The number of buckets into which indexes using a hash table are split.
    /// (The default is 16)
    ///
    /// This number has to be a power of 2 and less than or equal to 1024.
    ///
    /// For very large collections one should increase this to avoid long
    /// pauses when the hash table has to be initially built or resized, since
    /// buckets are resized individually and can be initially built in
    /// parallel. For example, 64 might be a sensible value for a collection
    /// with 100 000 000 documents. Currently, only the edge index respects
    /// this value, but other index types might follow in future ArangoDB
    /// versions. Changes (see below) are applied when the collection is loaded
    /// the next time.
    ///
    /// This option is meaningful for the MMFiles storage engine only.
    #[serde(skip_serializing_if = "Option::is_none")]
    index_buckets: Option<u16>,

    #[cfg(feature = "mmfiles")]
    /// The maximal size of a journal or datafile in bytes.
    /// (The default is a configuration parameter)
    ///
    /// The value must be at least 1048576 (1 MiB).
    ///
    /// This option is meaningful for the MMFiles storage engine only.
    #[serde(skip_serializing_if = "Option::is_none")]
    journal_size: Option<u64>,
}

impl NewCollection {
    fn new<N, K, S>(name: N, kind: K, is_system: S) -> Self
        where N: Into<String>, K: Into<Option<CollectionType>>, S: Into<Option<bool>>
    {
        NewCollection {
            name: name.into(),
            kind: kind.into(),
            is_system: is_system.into(),
            key_options: None,
            wait_for_sync: None,
            #[cfg(feature = "cluster")]
            number_of_shards: None,
            #[cfg(feature = "cluster")]
            shard_keys: None,
            #[cfg(feature = "cluster")]
            replication_factor: None,
            #[cfg(feature = "mmfiles")]
            is_volatile: None,
            #[cfg(feature = "mmfiles")]
            do_compact: None,
            #[cfg(feature = "mmfiles")]
            index_buckets: None,
            #[cfg(feature = "mmfiles")]
            journal_size: None,
        }
    }

    /// Constructs a new instance of `NewCollection` with the given name as
    /// the name of the collection going to be created. The newly created
    /// collection will be a documents collection.
    ///
    /// All other attributes of the collection are set to their default values
    /// by the ArangoDB server.
    pub fn with_name<N>(name: N) -> Self
        where N: Into<String>
    {
        NewCollection::new(name.into(), None, None)
    }

    /// Constructs a new instance of `NewCollection` with the given name as
    /// the name of the collection going to be created. The kind attribute
    /// is set to `Documents` explicitly.
    ///
    /// All other attributes of the collection are set to their default values
    /// by the ArangoDB server.
    pub fn documents_with_name<N>(name: N) -> Self
        where N: Into<String>
    {
        NewCollection::new(name.into(), Some(CollectionType::Documents), None)
    }

    /// Constructs a new instance of `NewCollection` with the given name as
    /// the name of the collection going to be created. The newly created
    /// collection will be an edges collection.
    ///
    /// All other attributes of the collection are set to their default values
    /// by the ArangoDB server.
    pub fn edges_with_name<N>(name: N) -> Self
        where N: Into<String>
    {
        NewCollection::new(name.into(), Some(CollectionType::Edges), None)
    }

    /// Constructs a new instance of `NewCollection` with the given name as
    /// the name of the collection going to be created. The created collection
    /// will be a system collection holding documents.
    ///
    /// All other attributes of the collection are set to their default values
    /// by the ArangoDB server.
    pub fn system_documents_with_name<N>(name: N) -> Self
        where N: Into<String>
    {
        NewCollection::new(name.into(), Some(CollectionType::Documents), Some(true))
    }

    /// Constructs a new instance of `NewCollection` with the given name as
    /// the name of the collection going to be created. The created collection
    /// will be a system collection for edges.
    ///
    /// All other attributes of the collection are set to their default values
    /// by the ArangoDB server.
    pub fn system_edges_with_name<N>(name: N) -> Self
        where N: Into<String>
    {
        NewCollection::new(name.into(), Some(CollectionType::Edges), Some(true))
    }

    /// Returns the name of the collection to be created.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns the type of the collection to be created.
    pub fn kind(&self) -> Option<CollectionType> {
        self.kind
    }

    /// Returns whether the collection is going to be a system or regular
    /// collection.
    pub fn is_system(&self) -> Option<bool> {
        self.is_system
    }

    /// Returns the key options as mutable reference for changing key options
    /// in place.
    pub fn key_options_mut<K>(&mut self) -> &mut NewKeyOptions {
        self.key_options.get_or_insert_with(NewKeyOptions::new)
    }

    /// Removes the currently set key options from this struct and returns them.
    pub fn remove_key_options(&mut self) -> Option<NewKeyOptions> {
        mem::replace(&mut self.key_options, None)
    }

    /// Returns the key options of the collection to be created.
    pub fn key_options(&self) -> Option<&NewKeyOptions> {
        self.key_options.as_ref()
    }

    /// Sets whether the server waits for sync to the filesystem before
    /// sending the response.
    pub fn set_wait_for_sync<W>(&mut self, wait_for_sync: W)
        where W: Into<Option<bool>>
    {
        self.wait_for_sync = wait_for_sync.into();
    }

    /// Returns whether the server waits for sync to the filesystem before
    /// sending the response.
    pub fn is_wait_for_sync(&self) -> Option<bool> {
        self.wait_for_sync
    }

    #[cfg(feature = "cluster")]
    /// Sets the number of shards that shall be created for this collection.
    pub fn set_number_of_shards<S>(&mut self, number_of_shards: S)
        where S: Into<Option<u16>>
    {
        self.number_of_shards = number_of_shards.into();
    }

    #[cfg(feature = "cluster")]
    /// Returns the number of shards that shall be created for this collection.
    pub fn number_of_shards(&self) -> Option<u16> {
        self.number_of_shards
    }

    #[cfg(feature = "cluster")]
    /// Sets the keys to determine the shard for a collection.
    pub fn set_shard_keys<K>(&mut self, shard_keys: K)
        where K: Into<Option<String>>
    {
        self.shard_keys = shard_keys.into();
    }

    #[cfg(feature = "cluster")]
    /// Returns the keys to determine the shard for a collection.
    pub fn shard_keys(&self) -> Option<&String> {
        self.shard_keys.as_ref()
    }

    #[cfg(feature = "cluster")]
    /// Sets the number of copies that are kept of each shard.
    pub fn set_replication_factor<R>(&mut self, replication_factor: R)
        where R: Into<Option<u16>>
    {
        self.replication_factor = replication_factor.into();
    }

    #[cfg(feature = "cluster")]
    /// Returns the number of copies that are kept of each shard.
    pub fn replication_factor(&self) -> Option<u16> {
        self.replication_factor
    }

    #[cfg(feature = "mmfiles")]
    /// Sets whether this collection is going to be a volatile collection.
    pub fn set_volatile<V>(&mut self, volatile: V)
        where V: Into<Option<bool>>
    {
        self.is_volatile = volatile.into();
    }

    #[cfg(feature = "mmfiles")]
    /// Returns whether this collection is going to be a volatile collection.
    pub fn is_volatile(&self) -> Option<bool> {
        self.is_volatile
    }

    #[cfg(feature = "mmfiles")]
    /// Sets whether this collection is going to be compacted.
    pub fn set_do_compact<C>(&mut self, do_compact: C)
        where C: Into<Option<bool>>
    {
        self.do_compact = do_compact.into();
    }

    #[cfg(feature = "mmfiles")]
    /// Returns whether this collection is going to be compacted.
    pub fn is_do_compact(&self) -> Option<bool> {
        self.do_compact
    }

    #[cfg(feature = "mmfiles")]
    /// Sets the number of buckets into which indexes using a hash table
    /// are split.
    pub fn set_index_buckets<B>(&mut self, index_buckets: B)
        where B: Into<Option<u16>>
    {
        self.index_buckets = index_buckets.into();
    }

    #[cfg(feature = "mmfiles")]
    /// Returns the number of buckets into which indexes using a hash table
    /// are split.
    pub fn index_buckets(&self) -> Option<u16> {
        self.index_buckets
    }

    #[cfg(feature = "mmfiles")]
    /// Sets the maximal size of a journal or datafile in bytes.
    pub fn set_journal_size<J>(&mut self, journal_size: J)
        where J: Into<Option<u64>>
    {
        self.journal_size = journal_size.into();
    }

    #[cfg(feature = "mmfiles")]
    /// Returns the maximal size of a journal or datafile in bytes.
    pub fn journal_size(&self) -> Option<u64> {
        self.journal_size
    }
}

/// This struct holds the key options to be used when creating a new
/// collection.
#[allow(missing_copy_implementations)]
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct NewKeyOptions {
    /// If set to true, then it is allowed to supply own key values in the _key
    /// attribute of a document. If set to false, then the key generator will
    /// solely be responsible for generating keys and supplying own key values
    /// in the _key attribute of documents is considered an error.
    #[serde(skip_serializing_if = "Option::is_none")]
    allow_user_keys: Option<bool>,

    /// Specifies the type of the key generator.
    ///
    /// The currently available generators are 'Traditional' and
    /// 'AutoIncrement'.
    #[serde(rename = "type")]
    kind: KeyGeneratorType,

    /// Increment value for 'AutoIncrement key generator.
    ///
    /// Used for 'AutoIncrement' key generator only.
    #[serde(skip_serializing_if = "Option::is_none")]
    increment: Option<u64>,

    /// Initial offset value for autoincrement key generator.
    ///
    /// Used for 'AutoIncrement' key generator only.
    #[serde(skip_serializing_if = "Option::is_none")]
    offset: Option<u64>,
}

impl NewKeyOptions {
    /// Constructs a new instance of `NewKeyOptions` with the kind set to
    /// `KeyGeneratorType::Traditional` and all optional fields set to `None`.
    fn new() -> Self {
        NewKeyOptions::default()
    }

    /// Sets the flag indicating whether user keys shall be allowed.
    ///
    /// If set to true, then it is allowed to supply own key values in the _key
    /// attribute of a document. If set to false, then the key generator will
    /// solely be responsible for generating keys and supplying own key values
    /// in the _key attribute of documents is considered an error.
    pub fn set_allow_user_keys<U>(&mut self, allow_user_keys: U)
        where U: Into<Option<bool>>
    {
        self.allow_user_keys = allow_user_keys.into();
    }

    /// Returns whether user keys shall be allowed.
    pub fn is_allow_user_keys(&self) -> Option<bool> {
        self.allow_user_keys
    }

    /// Sets the type of the key generator that shall be used by the new
    /// collection.
    pub fn set_kind(&mut self, kind: KeyGeneratorType)
    {
        self.kind = kind;
    }

    /// Returns the type of the key generator that shall be used.
    pub fn kind(&self) -> &KeyGeneratorType {
        &self.kind
    }

    /// Sets the increment value for 'AutoIncrement key generator.
    ///
    /// Used for 'AutoIncrement' key generator only.
    pub fn set_increment<I>(&mut self, increment: I)
        where I: Into<Option<u64>>
    {
        self.increment = increment.into();
    }

    /// Returns the initial offset value for autoincrement key generator.
    pub fn increment(&self) -> Option<u64> {
        self.increment
    }

    /// Sets the initial offset value for autoincrement key generator.
    ///
    /// Used for 'AutoIncrement' key generator only.
    pub fn set_offset<O>(&mut self, offset: O)
        where O: Into<Option<u64>>
    {
        self.offset = offset.into();
    }

    /// Returns the initial offset value for autoincrement key generator.
    pub fn offset(&self) -> Option<u64> {
        self.offset
    }
}

impl Default for NewKeyOptions {
    fn default() -> Self {
        NewKeyOptions {
            allow_user_keys: None,
            kind: KeyGeneratorType::Traditional,
            increment: None,
            offset: None,
        }
    }
}

/// This struct holds the checksum of a collection together with some basic
/// attributes.
///
/// It is return by the `GetCollectionChecksum` method.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CollectionChecksum {
    /// The id of the collection.
    id: String,

    /// The name of the collection.
    name: String,

    /// The type of the collection.
    #[serde(rename = "type")]
    kind: CollectionType,

    /// The status of the collection.
    status: CollectionStatus,

    /// Whether the collection is system collection or regular collection.
    is_system: bool,

    /// The revision of the collection
    revision: String,

    /// The checksum of the collection
    checksum: String,
}

impl CollectionChecksum {
    /// Returns the id of the collection.
    pub fn id(&self) -> &str {
        &self.id
    }

    /// Returns the name of the collection.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns the type of the collection
    pub fn kind(&self) -> CollectionType {
        self.kind
    }

    /// Returns the status of the collection
    pub fn status(&self) -> CollectionStatus {
        self.status
    }

    /// Returns whether the collection is a system or regular
    /// collection.
    pub fn is_system(&self) -> bool {
        self.is_system
    }

    /// Returns the revision of the collection.
    pub fn revision(&self) -> &str {
        &self.revision
    }

    /// Returns the checksum of the collection.
    pub fn checksum(&self) -> &str {
        &self.checksum
    }
}

/// This struct holds the document count of a collection together with some
/// basic attributes.
///
/// It is returned by the `GetCollectionDocumentCount` method.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CollectionDocumentCount {
    /// The id of the collection.
    id: String,

    /// The name of the collection.
    name: String,

    /// The type of the collection.
    #[serde(rename = "type")]
    kind: CollectionType,

    /// The status of the collection.
    status: CollectionStatus,

    /// Whether the collection is system collection or regular collection.
    is_system: bool,

    /// The document count of the collection.
    count: u64,
}

impl CollectionDocumentCount {
    /// Returns the id of the collection.
    pub fn id(&self) -> &str {
        &self.id
    }

    /// Returns the name of the collection.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns the type of the collection.
    pub fn kind(&self) -> CollectionType {
        self.kind
    }

    /// Returns the status of the collection.
    pub fn status(&self) -> CollectionStatus {
        self.status
    }

    /// Returns whether the collection is a system or regular
    /// collection.
    pub fn is_system(&self) -> bool {
        self.is_system
    }

    /// Returns the document count of the collection.
    pub fn count(&self) -> u64 {
        self.count
    }
}

/// This struct holds the result of a load operation.
///
/// It is returned by the `LoadCollection` method.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LoadCollectionResult {
    /// The id of the collection.
    id: String,

    /// The name of the collection.
    name: String,

    /// The document count of the collection.
    count: u64,

    /// The status of the collection.
    status: CollectionStatus,

    /// The type of the collection.
    #[serde(rename = "type")]
    kind: CollectionType,

    /// Whether the collection is system collection or regular collection.
    is_system: bool,
}

impl LoadCollectionResult {
    /// Returns the id of the collection.
    pub fn id(&self) -> &str {
        &self.id
    }

    /// Returns the name of the collection.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns the document count of the collection.
    pub fn count(&self) -> u64 {
        self.count
    }

    /// Returns the type of the collection.
    pub fn kind(&self) -> CollectionType {
        self.kind
    }

    /// Returns the status of the collection.
    pub fn status(&self) -> CollectionStatus {
        self.status
    }

    /// Returns whether the collection is a system or regular
    /// collection.
    pub fn is_system(&self) -> bool {
        self.is_system
    }
}

/// This struct holds the result of a unload operation.
///
/// It is returned by the `UnloadCollection` method.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UnloadCollectionResult {
    /// The id of the collection.
    id: String,

    /// The name of the collection.
    name: String,

    /// The status of the collection.
    status: CollectionStatus,

    /// The type of the collection.
    #[serde(rename = "type")]
    kind: CollectionType,

    /// Whether the collection is system collection or regular collection.
    is_system: bool,
}

impl UnloadCollectionResult {
    /// Returns the id of the collection.
    pub fn id(&self) -> &str {
        &self.id
    }

    /// Returns the name of the collection.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns the type of the collection.
    pub fn kind(&self) -> CollectionType {
        self.kind
    }

    /// Returns the status of the collection.
    pub fn status(&self) -> CollectionStatus {
        self.status
    }

    /// Returns whether the collection is a system or regular
    /// collection.
    pub fn is_system(&self) -> bool {
        self.is_system
    }
}

/// This struct holds the revision of a collection together with some basic
/// attributes.
///
/// It is returned by the `GetCollectionRevision` method.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CollectionRevision {
    /// The id of the collection.
    id: String,

    /// The name of the collection.
    name: String,

    /// The type of the collection.
    #[serde(rename = "type")]
    kind: CollectionType,

    /// The status of the collection.
    status: CollectionStatus,

    /// Whether the collection is system collection or regular collection.
    is_system: bool,

    /// The revision of the collection.
    revision: String,
}

impl CollectionRevision {
    /// Returns the id of the collection.
    pub fn id(&self) -> &str {
        &self.id
    }

    /// Returns the name of the collection.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns the type of the collection.
    pub fn kind(&self) -> CollectionType {
        self.kind
    }

    /// Returns the status of the collection.
    pub fn status(&self) -> CollectionStatus {
        self.status
    }

    /// Returns whether the collection is a system or regular
    /// collection.
    pub fn is_system(&self) -> bool {
        self.is_system
    }

    /// Returns the revision of the collection.
    pub fn revision(&self) -> &str {
        &self.revision
    }
}

/// This struct holds basic attributes of a collection.
///
/// It is returned by the `CreateCollection` method.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BasicCollectionProperties {
    /// The id of the collection.
    id: String,

    /// The name of the collection.
    name: String,

    /// The type of the collection.
    #[serde(rename = "type")]
    kind: CollectionType,

    /// The status of the collection.
    status: CollectionStatus,

    /// Whether the collection is system collection or regular collection.
    is_system: bool,

    /// Whether the server should wait until the collection is synchronized to
    /// the file system before returning the response.
    wait_for_sync: bool,

    #[cfg(feature = "mmfiles")]
    /// Whether this collection is volatile.
    is_volatile: bool,
}

impl BasicCollectionProperties {
    /// Returns the id of the collection.
    pub fn id(&self) -> &str {
        &self.id
    }

    /// Returns the name of the collection.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns the type of the collection.
    pub fn kind(&self) -> CollectionType {
        self.kind
    }

    /// Returns the status of the collection.
    pub fn status(&self) -> CollectionStatus {
        self.status
    }

    /// Returns whether the collection is a system or regular
    /// collection.
    pub fn is_system(&self) -> bool {
        self.is_system
    }

    /// Returns whether the server waits for sync to the filesystem before
    /// sending the response.
    pub fn is_wait_for_sync(&self) -> bool {
        self.wait_for_sync
    }

    #[cfg(feature = "mmfiles")]
    /// Returns whether this collection is a volatile collection.
    pub fn is_volatile(&self) -> bool {
        self.is_volatile
    }
}

impl From<CollectionProperties> for BasicCollectionProperties {
    fn from(properties: CollectionProperties) -> Self {
        BasicCollectionProperties {
            id: properties.id,
            name: properties.name,
            kind: properties.kind,
            status: properties.status,
            is_system: properties.is_system,
            wait_for_sync: properties.wait_for_sync,
            #[cfg(feature = "mmfiles")]
            is_volatile: properties.is_volatile
        }
    }
}

/// This struct holds the properties of a collection.
///
/// It is returned by the `GetCollectionProperties` method.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CollectionProperties {
    /// The id of the collection.
    id: String,

    /// The name of the collection.
    name: String,

    /// The type of the collection.
    #[serde(rename = "type")]
    kind: CollectionType,

    /// The status of the collection.
    status: CollectionStatus,

    /// Whether the collection is system collection or regular collection.
    is_system: bool,

    /// The key options of the collection.
    key_options: KeyOptions,

    /// Whether the server should wait until the collection is synchronized to
    /// the file system before returning the response.
    wait_for_sync: bool,

    #[cfg(feature = "cluster")]
    /// The number of shards of the collection.
    number_of_shards: u16,

    #[cfg(feature = "cluster")]
    /// The keys used to identify the shards of a collection.
    shard_keys: String,

    #[cfg(feature = "cluster")]
    /// The number of copies that are kept of each shard.
    replication_factor: u64,

    #[cfg(feature = "mmfiles")]
    /// Whether this collection is volatile.
    is_volatile: bool,

    #[cfg(feature = "mmfiles")]
    /// Whether this collection is compacted.
    do_compact: bool,

    #[cfg(feature = "mmfiles")]
    /// The number of buckets into which indexes using a hash table are split.
    index_buckets: u16,

    #[cfg(feature = "mmfiles")]
    /// The maximal size of a journal or datafile in bytes.
    journal_size: u64,
}

impl CollectionProperties {
    /// Returns the id of the collection.
    pub fn id(&self) -> &str {
        &self.id
    }

    /// Returns the name of the collection.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns the type of the collection.
    pub fn kind(&self) -> CollectionType {
        self.kind
    }

    /// Returns the status of the collection.
    pub fn status(&self) -> CollectionStatus {
        self.status
    }

    /// Returns whether the collection is a system or regular
    /// collection.
    pub fn is_system(&self) -> bool {
        self.is_system
    }

    /// Returns the key options of the collection.
    pub fn key_options(&self) -> &KeyOptions {
        &self.key_options
    }

    /// Returns whether the server waits for sync to the filesystem before
    /// sending the response.
    pub fn is_wait_for_sync(&self) -> bool {
        self.wait_for_sync
    }

    #[cfg(feature = "cluster")]
    /// Returns the number of shards for this collection.
    pub fn number_of_shards(&self) -> u16 {
        self.number_of_shards
    }

    #[cfg(feature = "cluster")]
    /// Returns the keys to determine the shard for a collection.
    pub fn shard_keys(&self) -> &str {
        &self.shard_keys
    }

    #[cfg(feature = "cluster")]
    /// Returns the number of copies that are kept of each shard.
    pub fn replication_factor(&self) -> u64 {
        self.replication_factor
    }

    #[cfg(feature = "mmfiles")]
    /// Returns whether this collection is a volatile collection.
    pub fn is_volatile(&self) -> bool {
        self.is_volatile
    }

    #[cfg(feature = "mmfiles")]
    /// Returns whether this collection is compacted.
    pub fn is_do_compact(&self) -> bool {
        self.do_compact
    }

    #[cfg(feature = "mmfiles")]
    /// Returns the number of buckets into which indexes using a hash table
    /// are split.
    pub fn index_buckets(&self) -> u16 {
        self.index_buckets
    }

    #[cfg(feature = "mmfiles")]
    /// Returns the maximal size of a journal or datafile in bytes.
    pub fn journal_size(&self) -> u64 {
        self.journal_size
    }
}

/// This struct holds optional values for the properties of a collection which
/// shall be changed.
#[allow(missing_copy_implementations)]
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CollectionPropertiesUpdate {
    /// Whether the server should wait until the collection is synchronized to
    /// the file system before returning the response.
    wait_for_sync: Option<bool>,

    #[cfg(feature = "mmfiles")]
    /// The maximal size of a journal or datafile in bytes.
    journal_size: Option<u64>,
}

#[cfg_attr(feature = "cargo-clippy", allow(new_without_default_derive))]
impl CollectionPropertiesUpdate {
    /// Constructs a new instance of `CollectionPropertiesUpdate` with no
    /// options set.
    pub fn new() -> Self {
        CollectionPropertiesUpdate {
            wait_for_sync: None,
            #[cfg(feature = "mmfiles")]
            journal_size: None,
        }
    }

    /// Sets whether the server waits for sync to the filesystem before
    /// sending the response.
    pub fn set_wait_for_sync<W>(&mut self, wait_for_sync: W)
        where W: Into<Option<bool>>
    {
        self.wait_for_sync = wait_for_sync.into();
    }

    /// Returns whether the server waits for sync to the filesystem before
    /// sending the response.
    pub fn is_wait_for_sync(&self) -> Option<bool> {
        self.wait_for_sync
    }

    #[cfg(feature = "mmfiles")]
    /// Sets the maximal size of a journal or datafile in bytes.
    pub fn set_journal_size<J>(&mut self, journal_size: J)
        where J: Into<Option<u64>>
    {
        self.journal_size = journal_size.into();
    }

    #[cfg(feature = "mmfiles")]
    /// Returns the maximal size of a journal or datafile in bytes.
    pub fn journal_size(&self) -> Option<u64> {
        self.journal_size
    }
}

/// This struct holds the new name for rename methods.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct RenameTo {
    /// The new name to rename the object to.
    name: String,
}

impl RenameTo {
    /// Constructs a new instance of the `RenameTo` struct with the given
    /// new name.
    pub fn new<N>(new_name: N) -> Self
        where N: Into<String>
    {
        RenameTo {
            name: new_name.into(),
        }
    }

    /// Returns the new name.
    pub fn name(&self) -> &str {
        &self.name
    }
}

/// This struct holds the key related properties.
#[allow(missing_copy_implementations)]
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct KeyOptions {
    /// If set to true, then it is allowed to supply own key values in the _key
    /// attribute of a document. If set to false, then the key generator will
    /// solely be responsible for generating keys and supplying own key values
    /// in the _key attribute of documents is considered an error.
    allow_user_keys: bool,

    /// Specifies the type of the key generator.
    ///
    /// The currently available generators are 'Traditional' and
    /// 'AutoIncrement'.
    #[serde(rename = "type")]
    kind: KeyGeneratorType,

    /// Last used key value.
    last_value: u64,
}

impl KeyOptions {
    /// Returns whether the collection allows user supplied key values.
    pub fn is_allow_user_keys(&self) -> bool {
        self.allow_user_keys
    }

    /// Returns the type of the key generator.
    pub fn kind(&self) -> KeyGeneratorType {
        self.kind
    }

    /// Returns the last value assigned by the key generator.
    pub fn last_value(&self) -> u64 {
        self.last_value
    }
}

/// This enum defines the different types of collections.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CollectionType {
    /// Documents collection
    Documents,
    /// Edges collection
    Edges,
}

impl Serialize for CollectionType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where S: Serializer
    {
        use self::CollectionType::*;
        let type_id = match *self {
            Documents => COLLECTION_TYPE_DOCUMENTS,
            Edges => COLLECTION_TYPE_EDGES,
        };
        serializer.serialize_i32(type_id)
    }
}

impl<'de> Deserialize<'de> for CollectionType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where D: Deserializer<'de>
    {
        use serde::de::Error;
        use self::CollectionType::*;
        let value = i32::deserialize(deserializer)?;
        match value {
            COLLECTION_TYPE_DOCUMENTS => Ok(Documents),
            COLLECTION_TYPE_EDGES => Ok(Edges),
            _ => Err(D::Error::custom(format!("Unknown collection type: {:?}", value))),
        }
    }
}

/// This enum defines the possible states a collection can be in.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CollectionStatus {
    /// New born
    NewBorn,
    /// Unloaded
    Unloaded,
    /// Loaded
    Loaded,
    /// In the process of being unloaded
    BeingUnloaded,
    /// Deleted
    Deleted,
    /// In the process of being loaded
    BeingLoaded,
    /// Indicates a corrupted collection
    Corrupted,
}

impl Serialize for CollectionStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where S: Serializer
    {
        use self::CollectionStatus::*;
        let status_id = match *self {
            NewBorn => COLLECTION_STATUS_NEW_BORN,
            Unloaded => COLLECTION_STATUS_UNLOADED,
            Loaded => COLLECTION_STATUS_LOADED,
            BeingUnloaded => COLLECTION_STATUS_BEING_UNLOADED,
            Deleted => COLLECTION_STATUS_DELETED,
            BeingLoaded => COLLECTION_STATUS_BEING_LOADED,
            Corrupted => COLLECTION_STATUS_CORRUPTED,
        };
        serializer.serialize_i32(status_id)
    }
}

impl<'de> Deserialize<'de> for CollectionStatus {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where D: Deserializer<'de>
    {
        use self::CollectionStatus::*;
        let value = i32::deserialize(deserializer)?;
        match value {
            COLLECTION_STATUS_NEW_BORN => Ok(NewBorn),
            COLLECTION_STATUS_UNLOADED => Ok(Unloaded),
            COLLECTION_STATUS_LOADED => Ok(Loaded),
            COLLECTION_STATUS_BEING_UNLOADED => Ok(BeingUnloaded),
            COLLECTION_STATUS_DELETED => Ok(Deleted),
            COLLECTION_STATUS_BEING_LOADED => Ok(BeingLoaded),
            _ => Ok(Corrupted),
        }
    }
}

/// This enum defines the types of key generators that are available.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum KeyGeneratorType {
    /// Traditional key generator
    //TODO clarify what the `Traditional` key generator actually is.
    Traditional,
    //TODO clarify what the `AutoIncrement` key generator actually is.
    /// Auto incrementing key generator
    AutoIncrement,
}

impl Serialize for KeyGeneratorType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where S: Serializer
    {
        use self::KeyGeneratorType::*;
        let type_str = match *self {
            Traditional => KEY_GENERATOR_TYPE_TRADITIONAL,
            AutoIncrement => KEY_GENERATOR_TYPE_AUTO_INCREMENT,
        };
        serializer.serialize_str(type_str)
    }
}

impl<'de> Deserialize<'de> for KeyGeneratorType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where D: Deserializer<'de>
    {
        use serde::de::Error;
        use self::KeyGeneratorType::*;
        let value = String::deserialize(deserializer)?;
        match &value[..] {
            KEY_GENERATOR_TYPE_TRADITIONAL => Ok(Traditional),
            KEY_GENERATOR_TYPE_AUTO_INCREMENT => Ok(AutoIncrement),
            _ => Err(D::Error::custom(format!("Unknown KeyGeneratorType: {:?}", value))),
        }
    }
}
