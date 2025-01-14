use std::collections::{BTreeMap, BTreeSet};
use std::sync::atomic::AtomicU64;
use std::sync::Arc;

use arrow2::datatypes::DataType as ArrowDataType;
use nohash_hasher::IntMap;

use re_chunk::{Chunk, ChunkId, RowId};
use re_log_types::{EntityPath, StoreId, TimeInt, Timeline};
use re_types_core::ComponentName;

use crate::ChunkStoreChunkStats;

// ---

// TODO(cmc): empty for now but soon will contain compaction settings, so preemptively
// avoid breaking changes everywhere.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ChunkStoreConfig {}

impl Default for ChunkStoreConfig {
    #[inline]
    fn default() -> Self {
        Self::DEFAULT
    }
}

impl ChunkStoreConfig {
    pub const DEFAULT: Self = Self {};
}

// ---

pub type ChunkIdSet = BTreeSet<ChunkId>;

#[derive(Default, Debug, Clone)]
pub struct ChunkIdSetPerTime {
    /// Keeps track of the longest interval being currently stored in the two maps below.
    ///
    /// This is used to bound the backwards linear walk when looking for overlapping chunks in
    /// latest-at queries.
    ///
    /// See [`ChunkStore::latest_at_relevant_chunks`] implementation comments for more details.
    pub(crate) max_interval_length: u64,

    pub(crate) per_start_time: BTreeMap<TimeInt, ChunkIdSet>,
    pub(crate) per_end_time: BTreeMap<TimeInt, ChunkIdSet>,
}

pub type ChunkIdSetPerTimePerComponent = BTreeMap<ComponentName, ChunkIdSetPerTime>;

pub type ChunkIdSetPerTimePerComponentPerTimeline =
    BTreeMap<Timeline, ChunkIdSetPerTimePerComponent>;

pub type ChunkIdSetPerTimePerComponentPerTimelinePerEntity =
    BTreeMap<EntityPath, ChunkIdSetPerTimePerComponentPerTimeline>;

pub type ChunkIdPerComponent = BTreeMap<ComponentName, ChunkId>;

pub type ChunkIdPerComponentPerEntity = BTreeMap<EntityPath, ChunkIdPerComponent>;

// ---

/// Incremented on each edit.
#[derive(Clone, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct ChunkStoreGeneration {
    insert_id: u64,
    gc_id: u64,
}

/// A complete chunk store: covers all timelines, all entities, everything.
///
/// The chunk store _always_ works at the chunk level, whether it is for write & read queries or
/// garbage collection. It is completely oblivious to individual rows.
///
/// Use the `Display` implementation for a detailed view of the internals.
#[derive(Debug)]
pub struct ChunkStore {
    pub(crate) id: StoreId,

    /// The configuration of the chunk store (e.g. compaction settings).
    pub(crate) config: ChunkStoreConfig,

    /// Keeps track of the _latest_ datatype information for all component types that have been written
    /// to the store so far.
    ///
    /// See also [`Self::lookup_datatype`].
    //
    // TODO(#1809): replace this with a centralized Arrow registry.
    // TODO(cmc): this would become fairly problematic in a world where each chunk can use a
    // different datatype for a given component.
    pub(crate) type_registry: IntMap<ComponentName, ArrowDataType>,

    pub(crate) chunks_per_chunk_id: BTreeMap<ChunkId, Arc<Chunk>>,

    /// All [`ChunkId`]s currently in the store, indexed by the smallest [`RowId`] in each of them.
    ///
    /// This is effectively all chunks in global data order. Used for garbage collection.
    ///
    /// This is a map of vecs instead of individual [`ChunkId`] in order to better support
    /// duplicated [`RowId`]s.
    pub(crate) chunk_ids_per_min_row_id: BTreeMap<RowId, Vec<ChunkId>>,

    /// All temporal [`ChunkId`]s for all entities on all timelines.
    ///
    /// See also [`Self::static_chunk_ids_per_entity`].
    pub(crate) temporal_chunk_ids_per_entity: ChunkIdSetPerTimePerComponentPerTimelinePerEntity,

    /// Accumulated size statitistics for all temporal [`Chunk`]s currently present in the store.
    ///
    /// This is too costly to be computed from scratch every frame, and is required by e.g. the GC.
    pub(crate) temporal_chunks_stats: ChunkStoreChunkStats,

    /// Static data. Never garbage collected.
    ///
    /// Static data unconditionally shadows temporal data at query time.
    ///
    /// Existing temporal will not be removed. Events won't be fired.
    pub(crate) static_chunk_ids_per_entity: ChunkIdPerComponentPerEntity,

    /// Accumulated size statitistics for all static [`Chunk`]s currently present in the store.
    ///
    /// This is too costly to be computed from scratch every frame, and is required by e.g. the GC.
    pub(crate) static_chunks_stats: ChunkStoreChunkStats,

    // pub(crate) static_tables: BTreeMap<EntityPathHash, StaticTable>,
    /// Monotonically increasing ID for insertions.
    pub(crate) insert_id: u64,

    /// Monotonically increasing ID for queries.
    pub(crate) query_id: AtomicU64,

    /// Monotonically increasing ID for GCs.
    pub(crate) gc_id: u64,

    /// Monotonically increasing ID for store events.
    pub(crate) event_id: AtomicU64,
}

impl Clone for ChunkStore {
    #[inline]
    fn clone(&self) -> Self {
        Self {
            id: self.id.clone(),
            config: self.config.clone(),
            type_registry: self.type_registry.clone(),
            chunks_per_chunk_id: self.chunks_per_chunk_id.clone(),
            chunk_ids_per_min_row_id: self.chunk_ids_per_min_row_id.clone(),
            temporal_chunk_ids_per_entity: self.temporal_chunk_ids_per_entity.clone(),
            temporal_chunks_stats: self.temporal_chunks_stats,
            static_chunk_ids_per_entity: self.static_chunk_ids_per_entity.clone(),
            static_chunks_stats: self.static_chunks_stats,
            insert_id: Default::default(),
            query_id: Default::default(),
            gc_id: Default::default(),
            event_id: Default::default(),
        }
    }
}

impl std::fmt::Display for ChunkStore {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let Self {
            id,
            config,
            type_registry: _,
            chunks_per_chunk_id,
            chunk_ids_per_min_row_id: chunk_id_per_min_row_id,
            temporal_chunk_ids_per_entity: _,
            temporal_chunks_stats,
            static_chunk_ids_per_entity: _,
            static_chunks_stats,
            insert_id: _,
            query_id: _,
            gc_id: _,
            event_id: _,
        } = self;

        f.write_str("ChunkStore {\n")?;

        f.write_str(&indent::indent_all_by(4, format!("id: {id}\n")))?;
        f.write_str(&indent::indent_all_by(4, format!("config: {config:?}\n")))?;

        f.write_str(&indent::indent_all_by(4, "stats: {\n"))?;
        f.write_str(&indent::indent_all_by(
            8,
            format!("{}", *static_chunks_stats + *temporal_chunks_stats),
        ))?;
        f.write_str(&indent::indent_all_by(4, "}\n"))?;

        f.write_str(&indent::indent_all_by(4, "chunks: [\n"))?;
        for chunk_id in chunk_id_per_min_row_id.values().flatten() {
            if let Some(chunk) = chunks_per_chunk_id.get(chunk_id) {
                f.write_str(&indent::indent_all_by(8, format!("{chunk}\n")))?;
            } else {
                f.write_str(&indent::indent_all_by(8, "<not_found>\n"))?;
            }
        }
        f.write_str(&indent::indent_all_by(4, "]\n"))?;

        f.write_str("}")?;

        Ok(())
    }
}

// ---

impl ChunkStore {
    #[inline]
    pub fn new(id: StoreId, config: ChunkStoreConfig) -> Self {
        Self {
            id,
            config,
            type_registry: Default::default(),
            chunk_ids_per_min_row_id: Default::default(),
            chunks_per_chunk_id: Default::default(),
            temporal_chunk_ids_per_entity: Default::default(),
            temporal_chunks_stats: Default::default(),
            static_chunk_ids_per_entity: Default::default(),
            static_chunks_stats: Default::default(),
            insert_id: 0,
            query_id: AtomicU64::new(0),
            gc_id: 0,
            event_id: AtomicU64::new(0),
        }
    }

    #[inline]
    pub fn id(&self) -> &StoreId {
        &self.id
    }

    /// Return the current [`ChunkStoreGeneration`]. This can be used to determine whether the
    /// database has been modified since the last time it was queried.
    #[inline]
    pub fn generation(&self) -> ChunkStoreGeneration {
        ChunkStoreGeneration {
            insert_id: self.insert_id,
            gc_id: self.gc_id,
        }
    }

    /// See [`ChunkStoreConfig`] for more information about configuration.
    #[inline]
    pub fn config(&self) -> &ChunkStoreConfig {
        &self.config
    }

    /// Iterate over all chunks in the store, in ascending [`ChunkId`] order.
    #[inline]
    pub fn iter_chunks(&self) -> impl Iterator<Item = &Arc<Chunk>> + '_ {
        self.chunks_per_chunk_id.values()
    }

    /// Lookup the _latest_ arrow [`ArrowDataType`] used by a specific [`re_types_core::Component`].
    #[inline]
    pub fn lookup_datatype(&self, component_name: &ComponentName) -> Option<&ArrowDataType> {
        self.type_registry.get(component_name)
    }
}
