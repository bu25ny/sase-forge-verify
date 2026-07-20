/*!
 * Merkle DAG Audit Log
 *
 * Append-only Merkle DAG with SQLite WAL backend (std feature only).
 */

use crate::MAX_MERKLE_PROOF_DEPTH;
use core::fmt;
#[cfg(feature = "std")]
use serde::{Deserialize, Serialize};
use zeroize::{Zeroize, ZeroizeOnDrop};

/// Merkle node hash (32 bytes - SHA-256)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "std", serde(transparent))]
#[repr(transparent)]
pub struct MerkleHash(pub [u8; 32]);

impl MerkleHash {
    /// Create a new merkle hash
    #[inline(always)]
    pub const fn new(bytes: [u8; 32]) -> Self {
        Self(bytes)
    }

    /// Create from slice
    #[inline]
    pub fn from_slice(bytes: &[u8]) -> Result<Self, MerkleError> {
        if bytes.len() != 32 {
            return Err(MerkleError::InvalidHashLength);
        }
        let mut arr = [0u8; 32];
        arr.copy_from_slice(bytes);
        Ok(Self(arr))
    }

    /// Get the hash bytes
    #[inline(always)]
    pub const fn as_bytes(&self) -> &[u8; 32] {
        &self.0
    }

    /// Compute hash of two child hashes
    #[inline]
    pub fn combine(left: &Self, right: &Self) -> Self {
        use sha2::{Sha256, Digest};
        let mut hasher = Sha256::new();
        hasher.update(left.0);
        hasher.update(right.0);
        let result = hasher.finalize();
        let mut hash = [0u8; 32];
        hash.copy_from_slice(&result);
        Self(hash)
    }

    /// Compute hash of leaf data
    #[inline]
    pub fn from_data(data: &[u8]) -> Self {
        use sha2::{Sha256, Digest};
        let mut hasher = Sha256::new();
        hasher.update(data);
        let result = hasher.finalize();
        let mut hash = [0u8; 32];
        hash.copy_from_slice(&result);
        Self(hash)
    }

    /// Zero hash (empty node)
    #[inline(always)]
    pub const fn zero() -> Self {
        Self([0u8; 32])
    }
}

impl fmt::Display for MerkleHash {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for byte in &self.0 {
            write!(f, "{:02x}", byte)?;
        }
        Ok(())
    }
}

impl Zeroize for MerkleHash {
    fn zeroize(&mut self) {
        self.0.zeroize();
    }
}

impl ZeroizeOnDrop for MerkleHash {}

/// Merkle tree node
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
#[repr(C)]
pub struct MerkleNode {
    /// Node hash
    pub hash: MerkleHash,
    /// Left child hash (None for leaf)
    pub left: Option<MerkleHash>,
    /// Right child hash (None for leaf)
    pub right: Option<MerkleHash>,
    /// Node depth (0 = leaf)
    pub depth: u8,
    /// Node index at this depth
    pub index: u64,
    /// Timestamp when node was created
    pub timestamp: u64,
    /// Optional payload hash (for audit data)
    pub payload_hash: Option<MerkleHash>,
}

impl MerkleNode {
    /// Create a new leaf node
    #[inline]
    pub fn new_leaf(data: &[u8], index: u64, timestamp: u64) -> Self {
        Self {
            hash: MerkleHash::from_data(data),
            left: None,
            right: None,
            depth: 0,
            index,
            timestamp,
            payload_hash: Some(MerkleHash::from_data(data)),
        }
    }

    /// Create a new internal node
    #[inline]
    pub fn new_internal(left: MerkleHash, right: MerkleHash, depth: u8, index: u64, timestamp: u64) -> Self {
        let hash = MerkleHash::combine(&left, &right);
        Self {
            hash,
            left: Some(left),
            right: Some(right),
            depth,
            index,
            timestamp,
            payload_hash: None,
        }
    }

    /// Check if this is a leaf node
    #[inline(always)]
    pub const fn is_leaf(&self) -> bool {
        self.left.is_none() && self.right.is_none()
    }

    /// Get the node size estimate
    #[inline]
    pub fn size_estimate(&self) -> usize {
        32 + 33 + 33 + 1 + 8 + 8 + 33 // hash + left + right + depth + index + timestamp + payload
    }
}

/// Merkle proof for verifying inclusion
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub struct MerkleProof {
    /// Root hash
    pub root: MerkleHash,
    /// Leaf index
    pub leaf_index: u64,
    /// Sibling hashes from leaf to root
    pub siblings: heapless::Vec<MerkleHash, MAX_MERKLE_PROOF_DEPTH>,
    /// Directions (true = left, false = right)
    pub directions: heapless::Vec<bool, MAX_MERKLE_PROOF_DEPTH>,
    /// Tree size (number of leaves)
    pub tree_size: u64,
}

impl MerkleProof {
    /// Verify a leaf hash against this proof
    #[inline]
    pub fn verify(&self, leaf_hash: &MerkleHash) -> bool {
        let mut current = *leaf_hash;

        for (i, sibling) in self.siblings.iter().enumerate() {
            if i >= self.directions.len() {
                return false;
            }

            if *sibling == MerkleHash::zero() {
                // Promoted node with no sibling: skip hashing at this level
                continue;
            }

            if self.directions[i] {
                // current is left child
                current = MerkleHash::combine(&current, sibling);
            } else {
                // current is right child
                current = MerkleHash::combine(sibling, &current);
            }
        }

        current == self.root
    }

    /// Verify data directly
    #[inline]
    pub fn verify_data(&self, data: &[u8]) -> bool {
        let leaf_hash = MerkleHash::from_data(data);
        self.verify(&leaf_hash)
    }
}

/// Merkle DAG errors
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "std", derive(thiserror::Error))]
pub enum MerkleError {
    /// Invalid hash length
    #[cfg_attr(feature = "std", error("Invalid hash length"))]
    InvalidHashLength,
    /// Invalid proof depth
    #[cfg_attr(feature = "std", error("Invalid proof depth"))]
    InvalidProofDepth,
    /// Proof verification failed
    #[cfg_attr(feature = "std", error("Proof verification failed"))]
    VerificationFailed,
    /// Tree is empty
    #[cfg_attr(feature = "std", error("Tree is empty"))]
    EmptyTree,
    /// Index out of bounds
    #[cfg_attr(feature = "std", error("Index out of bounds"))]
    IndexOutOfBounds,
    /// Database error
    #[cfg_attr(feature = "std", error("Database error: {0}"))]
    DatabaseError(&'static str),
    /// Serialization error
    #[cfg_attr(feature = "std", error("Serialization error"))]
    SerializationError,
    /// Capacity exceeded
    #[cfg_attr(feature = "std", error("Capacity exceeded"))]
    CapacityExceeded,
    /// Invalid node
    #[cfg_attr(feature = "std", error("Invalid node"))]
    InvalidNode,
}

/// Merkle Log trait for append-only audit logging
pub trait MerkleLog: Send + Sync {
    /// Append data to the log, returning the leaf index
    fn append(&mut self, data: &[u8]) -> Result<u64, MerkleError>;

    /// Get the current root hash
    fn root(&self) -> Option<MerkleHash>;

    /// Get the current tree size (number of leaves)
    fn size(&self) -> u64;

    /// Get a Merkle proof for a leaf index
    fn proof(&self, index: u64) -> Result<MerkleProof, MerkleError>;

    /// Verify a proof for a leaf index
    fn verify(&self, index: u64, data: &[u8]) -> Result<bool, MerkleError>;

    /// Get a node by index
    fn get_node(&self, index: u64) -> Result<MerkleNode, MerkleError>;

    /// Get all nodes at a specific depth
    fn get_level(&self, depth: u8) -> Result<heapless::Vec<MerkleNode, 64>, MerkleError>;

    /// Persist the log to storage
    fn persist(&mut self) -> Result<(), MerkleError>;

    /// Load the log from storage
    fn load(&mut self) -> Result<(), MerkleError>;
}

/// In-memory Merkle log implementation (no_std compatible)
#[derive(Debug, Clone)]
pub struct MemoryMerkleLog {
    leaves: heapless::Vec<MerkleNode, 1024>,
    nodes: heapless::Vec<MerkleNode, 2048>,
    root: Option<MerkleHash>,
    size: u64,
    timestamp: u64,
}

impl MemoryMerkleLog {
    /// Create a new empty memory Merkle log
    #[inline(always)]
    pub fn new() -> Self {
        Self {
            leaves: heapless::Vec::new(),
            nodes: heapless::Vec::new(),
            root: None,
            size: 0,
            timestamp: 0,
        }
    }

    /// Create with a custom timestamp
    #[inline(always)]
    pub fn with_timestamp(timestamp: u64) -> Self {
        Self {
            leaves: heapless::Vec::new(),
            nodes: heapless::Vec::new(),
            root: None,
            size: 0,
            timestamp,
        }
    }

    /// Recompute the tree root from leaves
    fn recompute_root(&mut self) {
        if self.leaves.is_empty() {
            self.root = None;
            return;
        }

        // Build tree bottom-up
        let mut current_level: heapless::Vec<MerkleHash, 1024> = heapless::Vec::new();
        for leaf in &self.leaves {
            current_level.push(leaf.hash).ok();
        }

        while current_level.len() > 1 {
            let mut next_level: heapless::Vec<MerkleHash, 1024> = heapless::Vec::new();
            for chunk in current_level.chunks(2) {
                if chunk.len() == 2 {
                    next_level.push(MerkleHash::combine(&chunk[0], &chunk[1])).ok();
                } else {
                    // Odd node, promote
                    next_level.push(chunk[0]).ok();
                }
            }
            current_level = next_level;
        }

        self.root = current_level.first().copied();
    }
}

impl Default for MemoryMerkleLog {
    #[inline(always)]
    fn default() -> Self {
        Self::new()
    }
}

impl MerkleLog for MemoryMerkleLog {
    fn append(&mut self, data: &[u8]) -> Result<u64, MerkleError> {
        let index = self.size;
        let leaf = MerkleNode::new_leaf(data, index, self.timestamp);
        self.leaves.push(leaf).map_err(|_| MerkleError::CapacityExceeded)?;
        self.size += 1;
        self.timestamp += 1;
        self.recompute_root();
        Ok(index)
    }

    fn root(&self) -> Option<MerkleHash> {
        self.root
    }

    fn size(&self) -> u64 {
        self.size
    }

    fn proof(&self, index: u64) -> Result<MerkleProof, MerkleError> {
        if index >= self.size || self.leaves.is_empty() {
            return Err(MerkleError::IndexOutOfBounds);
        }

        let mut siblings = heapless::Vec::new();
        let mut directions = heapless::Vec::new();

        let mut current_index = index;
        let mut _depth = 0;
        let mut current_level: heapless::Vec<MerkleHash, 1024> = heapless::Vec::new();
        for leaf in &self.leaves {
            current_level.push(leaf.hash).ok();
        }
        while current_level.len() > 1 {
            let sibling_index = if current_index.is_multiple_of(2) {
                current_index + 1
            } else {
                current_index - 1
            };

            let direction = current_index.is_multiple_of(2); // true = left

            if sibling_index < current_level.len() as u64 {
                siblings.push(current_level[sibling_index as usize]).map_err(|_| MerkleError::InvalidProofDepth)?;
                directions.push(direction).map_err(|_| MerkleError::InvalidProofDepth)?;
            } else {
                // No sibling (odd node at end), use zero hash
                siblings.push(MerkleHash::zero()).map_err(|_| MerkleError::InvalidProofDepth)?;
                directions.push(direction).map_err(|_| MerkleError::InvalidProofDepth)?;
            }

            // Build next level
            let mut next_level: heapless::Vec<MerkleHash, 1024> = heapless::Vec::new();
            for chunk in current_level.chunks(2) {
                if chunk.len() == 2 {
                    next_level.push(MerkleHash::combine(&chunk[0], &chunk[1])).ok();
                } else {
                    next_level.push(chunk[0]).ok();
                }
            }
            current_level = next_level;
            current_index /= 2;
            _depth += 1;
        }

        Ok(MerkleProof {
            root: self.root.unwrap_or(MerkleHash::zero()),
            leaf_index: index,
            siblings,
            directions,
            tree_size: self.size,
        })
    }

    fn verify(&self, index: u64, data: &[u8]) -> Result<bool, MerkleError> {
        let proof = self.proof(index)?;
        Ok(proof.verify_data(data))
    }

    fn get_node(&self, index: u64) -> Result<MerkleNode, MerkleError> {
        if index < self.leaves.len() as u64 {
            Ok(self.leaves[index as usize].clone())
        } else if index < self.nodes.len() as u64 {
            Ok(self.nodes[index as usize].clone())
        } else {
            Err(MerkleError::IndexOutOfBounds)
        }
    }

    fn get_level(&self, _depth: u8) -> Result<heapless::Vec<MerkleNode, 64>, MerkleError> {
        // Not fully implemented for memory log
        Err(MerkleError::InvalidNode)
    }

    fn persist(&mut self) -> Result<(), MerkleError> {
        // No-op for memory log
        Ok(())
    }

    fn load(&mut self) -> Result<(), MerkleError> {
        // No-op for memory log
        Ok(())
    }
}

/// SQLite-backed Merkle log (std feature only)
#[cfg(all(feature = "sqlite", feature = "std"))]
pub mod sqlite {
    use super::*;
    use rusqlite::{Connection, params, OptionalExtension};
    use std::path::Path;
    use std::sync::{Arc, Mutex};

    /// SQLite Merkle log configuration
    #[derive(Debug, Clone)]
    pub struct SqliteMerkleLogConfig {
        /// Database path
        pub path: String,
        /// Enable WAL mode
        pub wal_mode: bool,
        /// Synchronous mode
        pub synchronous: SynchronousMode,
        /// Cache size (pages)
        pub cache_size: i32,
    }

    impl Default for SqliteMerkleLogConfig {
        fn default() -> Self {
            Self {
                path: ":memory:".to_string(),
                wal_mode: true,
                synchronous: SynchronousMode::Normal,
                cache_size: -2000, // 2MB
            }
        }
    }

    /// SQLite synchronous mode
    #[derive(Debug, Clone, Copy)]
    pub enum SynchronousMode {
        Off = 0,
        Normal = 1,
        Full = 2,
        Extra = 3,
    }

    /// SQLite-backed Merkle log
    pub struct SqliteMerkleLog {
        conn: Arc<Mutex<Connection>>,
        config: SqliteMerkleLogConfig,
        root: Option<MerkleHash>,
        size: u64,
        timestamp: u64,
    }

    impl SqliteMerkleLog {
        /// Create a new SQLite Merkle log
        pub fn new(config: SqliteMerkleLogConfig) -> Result<Self, MerkleError> {
            let conn = Connection::open(&config.path)
                .map_err(|_| MerkleError::DatabaseError("Failed to open database"))?;

            // Configure database
            if config.wal_mode {
                conn.execute("PRAGMA journal_mode=WAL", [])
                    .map_err(|_| MerkleError::DatabaseError("Failed to enable WAL"))?;
            }

            conn.execute(&format!("PRAGMA synchronous={}", config.synchronous as i32), [])
                .map_err(|_| MerkleError::DatabaseError("Failed to set synchronous"))?;

            conn.execute(&format!("PRAGMA cache_size={}", config.cache_size), [])
                .map_err(|_| MerkleError::DatabaseError("Failed to set cache size"))?;

            // Create tables
            conn.execute(
                "CREATE TABLE IF NOT EXISTS merkle_leaves (
                    index INTEGER PRIMARY KEY,
                    hash BLOB NOT NULL,
                    data_hash BLOB,
                    timestamp INTEGER NOT NULL,
                    data BLOB
                )",
                [],
            ).map_err(|_| MerkleError::DatabaseError("Failed to create leaves table"))?;

            conn.execute(
                "CREATE TABLE IF NOT EXISTS merkle_nodes (
                    depth INTEGER NOT NULL,
                    index INTEGER NOT NULL,
                    hash BLOB NOT NULL,
                    left_hash BLOB,
                    right_hash BLOB,
                    timestamp INTEGER NOT NULL,
                    PRIMARY KEY (depth, index)
                )",
                [],
            ).map_err(|_| MerkleError::DatabaseError("Failed to create nodes table"))?;

            conn.execute(
                "CREATE TABLE IF NOT EXISTS merkle_metadata (
                    key TEXT PRIMARY KEY,
                    value BLOB NOT NULL
                )",
                [],
            ).map_err(|_| MerkleError::DatabaseError("Failed to create metadata table"))?;

            // Create indexes
            conn.execute("CREATE INDEX IF NOT EXISTS idx_leaves_timestamp ON merkle_leaves(timestamp)", [])
                .map_err(|_| MerkleError::DatabaseError("Failed to create timestamp index"))?;

            let mut log = Self {
                conn: Arc::new(Mutex::new(conn)),
                config,
                root: None,
                size: 0,
                timestamp: 0,
            };

            log.load()?;
            Ok(log)
        }

        /// Get current timestamp
        fn current_timestamp(&self) -> u64 {
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs()
        }

        /// Recompute root from database
        fn recompute_root(&mut self) -> Result<(), MerkleError> {
            let conn = self.conn.lock().map_err(|_| MerkleError::DatabaseError("Lock poisoned"))?;

            let mut stmt = conn.prepare(
                "SELECT hash FROM merkle_leaves ORDER BY index"
            ).map_err(|_| MerkleError::DatabaseError("Failed to prepare statement"))?;

            let hashes: Vec<[u8; 32]> = stmt.query_map([], |row| {
                let mut hash = [0u8; 32];
                row.get_ref(0)?.as_blob()?.copy_to_slice(&mut hash);
                Ok(hash)
            }).map_err(|_| MerkleError::DatabaseError("Failed to query leaves"))?
            .collect::<Result<Vec<_>, _>>()
            .map_err(|_| MerkleError::DatabaseError("Failed to collect leaves"))?;

            if hashes.is_empty() {
                self.root = None;
                self.size = 0;
                return Ok(());
            }

            let mut current_level = hashes;
            while current_level.len() > 1 {
                let mut next_level = Vec::with_capacity((current_level.len() + 1) / 2);
                for chunk in current_level.chunks(2) {
                    if chunk.len() == 2 {
                        next_level.push(MerkleHash::combine(
                            &MerkleHash(chunk[0]),
                            &MerkleHash(chunk[1])
                        ).0);
                    } else {
                        next_level.push(chunk[0]);
                    }
                }
                current_level = next_level;
            }

            self.root = Some(MerkleHash(current_level[0]));
            self.size = hashes.len() as u64;
            Ok(())
        }
    }

    impl MerkleLog for SqliteMerkleLog {
        fn append(&mut self, data: &[u8]) -> Result<u64, MerkleError> {
            let conn = self.conn.lock().map_err(|_| MerkleError::DatabaseError("Lock poisoned"))?;

            let index = self.size;
            let timestamp = self.current_timestamp();
            let hash = MerkleHash::from_data(data);
            let data_hash = MerkleHash::from_data(data);

            conn.execute(
                "INSERT INTO merkle_leaves (index, hash, data_hash, timestamp, data) VALUES (?, ?, ?, ?, ?)",
                params![index, hash.as_bytes(), data_hash.as_bytes(), timestamp, data],
            ).map_err(|_| MerkleError::DatabaseError("Failed to insert leaf"))?;

            self.size += 1;
            self.timestamp = timestamp;
            self.recompute_root()?;

            Ok(index)
        }

        fn root(&self) -> Option<MerkleHash> {
            self.root
        }

        fn size(&self) -> u64 {
            self.size
        }

        fn proof(&self, index: u64) -> Result<MerkleProof, MerkleError> {
            if index >= self.size {
                return Err(MerkleError::IndexOutOfBounds);
            }

            let conn = self.conn.lock().map_err(|_| MerkleError::DatabaseError("Lock poisoned"))?;

            // Get all leaf hashes
            let mut stmt = conn.prepare(
                "SELECT hash FROM merkle_leaves ORDER BY index"
            ).map_err(|_| MerkleError::DatabaseError("Failed to prepare statement"))?;

            let hashes: Vec<[u8; 32]> = stmt.query_map([], |row| {
                let mut hash = [0u8; 32];
                row.get_ref(0)?.as_blob()?.copy_to_slice(&mut hash);
                Ok(hash)
            }).map_err(|_| MerkleError::DatabaseError("Failed to query leaves"))?
            .collect::<Result<Vec<_>, _>>()
            .map_err(|_| MerkleError::DatabaseError("Failed to collect leaves"))?;

            if hashes.is_empty() {
                return Err(MerkleError::EmptyTree);
            }

            // Build proof
            let mut siblings = heapless::Vec::new();
            let mut directions = heapless::Vec::new();

            let mut current_index = index;
            let mut current_level = hashes;

            while current_level.len() > 1 {
                let sibling_index = if current_index % 2 == 0 {
                    current_index + 1
                } else {
                    current_index - 1
                };

                let direction = current_index % 2 == 0;

                if (sibling_index as usize) < current_level.len() {
                    siblings.push(MerkleHash(current_level[sibling_index as usize]))
                        .map_err(|_| MerkleError::InvalidProofDepth)?;
                } else {
                    siblings.push(MerkleHash::zero())
                        .map_err(|_| MerkleError::InvalidProofDepth)?;
                }

                directions.push(direction)
                    .map_err(|_| MerkleError::InvalidProofDepth)?;

                // Build next level
                let mut next_level = Vec::with_capacity((current_level.len() + 1) / 2);
                for chunk in current_level.chunks(2) {
                    if chunk.len() == 2 {
                        next_level.push(MerkleHash::combine(
                            &MerkleHash(chunk[0]),
                            &MerkleHash(chunk[1])
                        ).0);
                    } else {
                        next_level.push(chunk[0]);
                    }
                }
                current_level = next_level;
                current_index /= 2;
            }

            Ok(MerkleProof {
                root: self.root.unwrap_or(MerkleHash::zero()),
                leaf_index: index,
                siblings,
                directions,
                tree_size: self.size,
            })
        }

        fn verify(&self, index: u64, data: &[u8]) -> Result<bool, MerkleError> {
            let proof = self.proof(index)?;
            Ok(proof.verify_data(data))
        }

        fn get_node(&self, index: u64) -> Result<MerkleNode, MerkleError> {
            let conn = self.conn.lock().map_err(|_| MerkleError::DatabaseError("Lock poisoned"))?;

            if index < self.size {
                let mut stmt = conn.prepare(
                    "SELECT hash, data_hash, timestamp FROM merkle_leaves WHERE index = ?"
                ).map_err(|_| MerkleError::DatabaseError("Failed to prepare statement"))?;

                let row = stmt.query_row(params![index], |row| {
                    let mut hash = [0u8; 32];
                    let mut data_hash = [0u8; 32];
                    row.get_ref(0)?.as_blob()?.copy_to_slice(&mut hash)?;
                    row.get_ref(1)?.as_blob()?.copy_to_slice(&mut data_hash)?;
                    let timestamp: u64 = row.get(2)?;
                    Ok((hash, data_hash, timestamp))
                }).optional().map_err(|_| MerkleError::DatabaseError("Failed to query leaf"))?;

                if let Some((hash, data_hash, timestamp)) = row {
                    Ok(MerkleNode {
                        hash: MerkleHash(hash),
                        left: None,
                        right: None,
                        depth: 0,
                        index,
                        timestamp,
                        payload_hash: Some(MerkleHash(data_hash)),
                    })
                } else {
                    Err(MerkleError::IndexOutOfBounds)
                }
            } else {
                Err(MerkleError::IndexOutOfBounds)
            }
        }

        fn get_level(&self, depth: u8) -> Result<heapless::Vec<MerkleNode, 64>, MerkleError> {
            let conn = self.conn.lock().map_err(|_| MerkleError::DatabaseError("Lock poisoned"))?;

            let mut stmt = conn.prepare(
                "SELECT depth, index, hash, left_hash, right_hash, timestamp FROM merkle_nodes WHERE depth = ? ORDER BY index"
            ).map_err(|_| MerkleError::DatabaseError("Failed to prepare statement"))?;

            let mut nodes = heapless::Vec::new();
            let rows = stmt.query_map(params![depth], |row| {
                let mut hash = [0u8; 32];
                let mut left = [0u8; 32];
                let mut right = [0u8; 32];
                row.get_ref(2)?.as_blob()?.copy_to_slice(&mut hash)?;
                row.get_ref(3)?.as_blob()?.copy_to_slice(&mut left)?;
                row.get_ref(4)?.as_blob()?.copy_to_slice(&mut right)?;
                let index: u64 = row.get(1)?;
                let timestamp: u64 = row.get(5)?;
                Ok((index, hash, left, right, timestamp))
            }).map_err(|_| MerkleError::DatabaseError("Failed to query nodes"))?;

            for row in rows {
                let (index, hash, left, right, timestamp) = row.map_err(|_| MerkleError::DatabaseError("Failed to read node"))?;
                nodes.push(MerkleNode {
                    hash: MerkleHash(hash),
                    left: if left != [0u8; 32] { Some(MerkleHash(left)) } else { None },
                    right: if right != [0u8; 32] { Some(MerkleHash(right)) } else { None },
                    depth,
                    index,
                    timestamp,
                    payload_hash: None,
                }).map_err(|_| MerkleError::CapacityExceeded)?;
            }

            Ok(nodes)
        }

        fn persist(&mut self) -> Result<(), MerkleError> {
            // SQLite auto-commits in WAL mode
            Ok(())
        }

        fn load(&mut self) -> Result<(), MerkleError> {
            self.recompute_root()
        }
    }
}

/// Audit log entry for the Merkle DAG
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
#[repr(C)]
pub struct AuditEntry {
    /// Entry index
    pub index: u64,
    /// Entry type
    pub entry_type: AuditEntryType,
    /// Timestamp (unix seconds)
    pub timestamp: u64,
    /// License key ID
    pub key_id: crate::KeyId,
    /// Action performed
    pub action: heapless::String<64>,
    /// Result (success/failure)
    pub result: bool,
    /// Additional data hash
    pub data_hash: MerkleHash,
    /// Merkle proof for this entry
    pub proof: MerkleProof,
}

/// Audit entry types
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
#[repr(u8)]
pub enum AuditEntryType {
    /// License verification
    Verification = 0,
    /// License issuance
    Issuance = 1,
    /// License revocation
    Revocation = 2,
    /// Policy update
    PolicyUpdate = 3,
    /// Hardware binding change
    HwBindingChange = 4,
    /// TPM seal/unseal
    TpmOperation = 5,
    /// Access granted
    AccessGranted = 6,
    /// Access denied
    AccessDenied = 7,
    /// Configuration change
    ConfigChange = 8,
    /// Security event
    SecurityEvent = 9,
}

impl fmt::Display for AuditEntryType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AuditEntryType::Verification => write!(f, "verification"),
            AuditEntryType::Issuance => write!(f, "issuance"),
            AuditEntryType::Revocation => write!(f, "revocation"),
            AuditEntryType::PolicyUpdate => write!(f, "policy_update"),
            AuditEntryType::HwBindingChange => write!(f, "hw_binding_change"),
            AuditEntryType::TpmOperation => write!(f, "tpm_operation"),
            AuditEntryType::AccessGranted => write!(f, "access_granted"),
            AuditEntryType::AccessDenied => write!(f, "access_denied"),
            AuditEntryType::ConfigChange => write!(f, "config_change"),
            AuditEntryType::SecurityEvent => write!(f, "security_event"),
        }
    }
}

#[cfg(test)]
#[cfg(feature = "std")]
mod tests {
    use super::*;
    use std::format;

    #[test]
    fn test_merkle_hash() {
        let h1 = MerkleHash::from_data(b"test");
        let h2 = MerkleHash::from_data(b"test");
        assert_eq!(h1, h2);

        let h3 = MerkleHash::from_data(b"different");
        assert_ne!(h1, h3);

        let combined = MerkleHash::combine(&h1, &h2);
        assert_ne!(combined, h1);
    }

    #[test]
    fn test_merkle_hash_zero() {
        let zero = MerkleHash::zero();
        assert_eq!(zero.as_bytes(), &[0u8; 32]);
    }

    #[test]
    fn test_memory_merkle_log() {
        let mut log = MemoryMerkleLog::new();
        assert_eq!(log.size(), 0);
        assert_eq!(log.root(), None);

        let idx1 = log.append(b"entry 1").unwrap();
        assert_eq!(idx1, 0);
        assert_eq!(log.size(), 1);
        assert!(log.root().is_some());

        let idx2 = log.append(b"entry 2").unwrap();
        assert_eq!(idx2, 1);
        assert_eq!(log.size(), 2);

        let idx3 = log.append(b"entry 3").unwrap();
        assert_eq!(idx3, 2);
        assert_eq!(log.size(), 3);
    }

    #[test]
    fn test_memory_merkle_proof() {
        let mut log = MemoryMerkleLog::new();
        log.append(b"entry 1").unwrap();
        log.append(b"entry 2").unwrap();
        log.append(b"entry 3").unwrap();

        // Verify each entry
        assert!(log.verify(0, b"entry 1").unwrap());
        assert!(log.verify(1, b"entry 2").unwrap());
        assert!(log.verify(2, b"entry 3").unwrap());

        // Wrong data should fail
        assert!(!log.verify(0, b"wrong").unwrap());
    }

    #[test]
    fn test_merkle_proof_verification() {
        let mut log = MemoryMerkleLog::new();
        log.append(b"data").unwrap();
        let proof = log.proof(0).unwrap();

        assert!(proof.verify_data(b"data"));
        assert!(!proof.verify_data(b"wrong"));
    }

    #[test]
    fn test_merkle_node() {
        let leaf = MerkleNode::new_leaf(b"leaf data", 0, 1000);
        assert!(leaf.is_leaf());
        assert_eq!(leaf.depth, 0);
        assert_eq!(leaf.index, 0);
        assert_eq!(leaf.timestamp, 1000);
        assert!(leaf.payload_hash.is_some());

        let left = MerkleHash::from_data(b"left");
        let right = MerkleHash::from_data(b"right");
        let internal = MerkleNode::new_internal(left, right, 1, 0, 2000);
        assert!(!internal.is_leaf());
        assert_eq!(internal.depth, 1);
        assert!(internal.left.is_some());
        assert!(internal.right.is_some());
    }

    #[test]
    fn test_audit_entry_type() {
        assert_eq!(format!("{}", AuditEntryType::Verification), "verification");
        assert_eq!(format!("{}", AuditEntryType::TpmOperation), "tpm_operation");
        assert_eq!(format!("{}", AuditEntryType::SecurityEvent), "security_event");
    }

    #[test]
    fn test_pcr_mask_display() {
        let mask = MerkleHash::zero();
        // Just test it doesn't panic
        let _ = format!("{}", mask);
    }

    #[cfg(feature = "sqlite")]
    #[test]
    fn test_sqlite_merkle_log() {
        let config = sqlite::SqliteMerkleLogConfig::default();
        let mut log = sqlite::SqliteMerkleLog::new(config).unwrap();

        let idx1 = log.append(b"sqlite entry 1").unwrap();
        assert_eq!(idx1, 0);

        let idx2 = log.append(b"sqlite entry 2").unwrap();
        assert_eq!(idx2, 1);

        assert_eq!(log.size(), 2);
        assert!(log.root().is_some());

        assert!(log.verify(0, b"sqlite entry 1").unwrap());
        assert!(log.verify(1, b"sqlite entry 2").unwrap());
        assert!(!log.verify(0, b"wrong").unwrap());
    }
}