//! The Rerun chunk store, implemented on top of [Apache Arrow](https://arrow.apache.org/)
//! using the [`arrow2`] crate.
//!
//! This crate is an in-memory time series database for Rerun log data.
//! It is indexed by Entity path, component, timeline, and time.
//! It supports out-of-order insertions, and fast `O(log(N))` queries.
//!
//! * See [`ChunkStore`] for an overview of the core data structures.
//! * See [`ChunkStore::latest_at_relevant_chunks`] and [`ChunkStore::range_relevant_chunks`]
//!   for the documentation of the public read APIs.
//! * See [`ChunkStore::insert_chunk`] for the documentation of the public write APIs.
//!
//! ## Feature flags
#![doc = document_features::document_features!()]
//!

mod events;
mod gc;
mod query;
mod stats;
mod store;
mod subscribers;
mod writes;

pub use self::events::{ChunkStoreDiff, ChunkStoreDiffKind, ChunkStoreEvent};
pub use self::gc::{GarbageCollectionOptions, GarbageCollectionTarget};
pub use self::stats::{ChunkStoreChunkStats, ChunkStoreStats};
pub use self::store::{ChunkStore, ChunkStoreConfig, ChunkStoreGeneration};
pub use self::subscribers::{ChunkStoreSubscriber, ChunkStoreSubscriberHandle};

// Re-exports
#[doc(no_inline)]
pub use re_chunk::{Chunk, ChunkId, LatestAtQuery, RangeQuery, RowId};
#[doc(no_inline)]
pub use re_log_types::{ResolvedTimeRange, TimeInt, TimeType, Timeline};

pub mod external {
    pub use re_chunk;
}

// ---

#[derive(thiserror::Error, Debug)]
pub enum ChunkStoreError {
    #[error("Chunks must be sorted before insertion in the chunk store")]
    UnsortedChunk,
}

pub type ChunkStoreResult<T> = ::std::result::Result<T, ChunkStoreError>;
