/*!
 * SASE License Core - SASE Licensing Engine
 *
 * A `no_std` Rust crate for SASE licensing with:
 * - Zero-heap hot paths (`-DSASE_ZERO_HEAP=1` compatible)
 * - WASM compatible (wasm32-unknown-unknown)
 * - Ed25519 verification (heap-free via ed25519-dalek + zeroize)
 * - TPM2 PCR sealing (PCR 0-7 + 16)
 * - Merkle-DAG audit log (SQLite WAL backend, std feature)
 * - postcard (no_std) + serde (std feature) serialization
 * - PyO3 bindings (python feature)
 */

#![no_std]
#![cfg_attr(feature = "const_fn", feature(const_fn))]
#![cfg_attr(all(not(feature = "std"), not(test)), no_main)]
#![deny(
    trivial_casts,
    trivial_numeric_casts,
    unused_import_braces
)]
#![warn(rust_2018_idioms, unused_lifetimes, unused_qualifications)]
#![cfg_attr(docsrs, feature(doc_cfg))]

// Core modules
pub mod tier;
pub mod features;
pub mod policy;
pub mod crypto;
pub mod key;
pub mod tpm;
pub mod merkle;

// Re-exports for convenience
pub use tier::{LicenseTier, TierError};
pub use features::{FeatureSet, Feature, FeatureError};
pub use policy::{LicensePolicy, PolicyError};
pub use crypto::{Ed25519Verifier, VerifyError, Signature, PublicKey, SecretKey, KeyId};
pub use key::{LicenseKey, KeyError};
pub use tpm::{TpmSealer, TpmError, MockTpmSealer, PcrMask, PcrIndices};
pub use merkle::{MerkleNode, MerkleLog, MerkleProof, MerkleError};

// Constants
/// Size of key identifier (BLAKE3 hash truncated to 16 bytes)
pub const KEY_ID_SIZE: usize = 16;

/// Ed25519 signature size in bytes
pub const SIGNATURE_SIZE: usize = 64;

/// Ed25519 public key size in bytes
pub const PUBLIC_KEY_SIZE: usize = 32;

/// Ed25519 secret key size in bytes
pub const SECRET_KEY_SIZE: usize = 32;

/// Maximum serialized LicensePolicy size in bytes
pub const MAX_POLICY_SIZE: usize = 512;

/// Maximum serialized LicenseKey size in bytes
pub const MAX_KEY_SIZE: usize = 1024;

/// Maximum Merkle node size in bytes
pub const MAX_MERKLE_NODE_SIZE: usize = 256;

/// Maximum Merkle proof depth
pub const MAX_MERKLE_PROOF_DEPTH: usize = 32;

/// Maximum number of PCRs supported (PCR 0-7 + 16)
pub const MAX_PCR_COUNT: usize = 9;

/// PCR mask for supported PCRs (0-7, 16)
pub const PCR_MASK: u32 = 0x0001_00FF;

/// PCR indices supported (0-7, 16)
pub const PCR_INDICES: [u32; MAX_PCR_COUNT] = [0, 1, 2, 3, 4, 5, 6, 7, 16];

/// Current crate version as a packed u32 (major << 22 | minor << 12 | patch)
#[cfg(feature = "const_fn")]
pub const VERSION: u32 = {
    let major = env!("CARGO_PKG_VERSION_MAJOR").parse::<u32>().unwrap_or(0);
    let minor = env!("CARGO_PKG_VERSION_MINOR").parse::<u32>().unwrap_or(0);
    let patch = env!("CARGO_PKG_VERSION_PATCH").parse::<u32>().unwrap_or(0);
    (major << 22) | (minor << 12) | patch
};

/// Version string
pub const VERSION_STR: &str = env!("CARGO_PKG_VERSION");

/// Feature flags as a bitmask
pub const FEATURE_STD: u32 = 1 << 0;
pub const FEATURE_PYTHON: u32 = 1 << 1;
pub const FEATURE_TPM: u32 = 1 << 2;
pub const FEATURE_SQLITE: u32 = 1 << 3;
pub const FEATURE_WASM: u32 = 1 << 4;
pub const FEATURE_CONST_FN: u32 = 1 << 5;
pub const FEATURE_ZEROIZE: u32 = 1 << 6;

/// Compile-time feature flags
#[cfg(feature = "const_fn")]
pub const COMPILE_TIME_FEATURES: u32 = {
    let mut flags = FEATURE_CONST_FN | FEATURE_ZEROIZE;
    #[cfg(feature = "std")]
    {
        flags |= FEATURE_STD;
    }
    #[cfg(feature = "python")]
    {
        flags |= FEATURE_PYTHON;
    }
    #[cfg(feature = "tpm")]
    {
        flags |= FEATURE_TPM;
    }
    #[cfg(feature = "sqlite")]
    {
        flags |= FEATURE_SQLITE;
    }
    #[cfg(feature = "wasm")]
    {
        flags |= FEATURE_WASM;
    }
    flags
};

/// Error type for the crate
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "std", derive(thiserror::Error))]
#[non_exhaustive]
pub enum LicenseError {
    /// Invalid license tier
    #[cfg_attr(feature = "std", error("Invalid license tier"))]
    InvalidTier,
    /// Invalid feature set
    #[cfg_attr(feature = "std", error("Invalid feature set"))]
    InvalidFeature,
    /// Policy validation failed
    #[cfg_attr(feature = "std", error("Policy validation failed: {0}"))]
    PolicyError(PolicyError),
    /// Key verification failed
    #[cfg_attr(feature = "std", error("Key verification failed: {0}"))]
    KeyError(KeyError),
    /// Ed25519 verification failed
    #[cfg_attr(feature = "std", error("Ed25519 verification failed: {0}"))]
    VerifyError(VerifyError),
    /// TPM operation failed
    #[cfg_attr(feature = "std", error("TPM error: {0}"))]
    TpmError(TpmError),
    /// Merkle DAG error
    #[cfg_attr(feature = "std", error("Merkle error: {0}"))]
    MerkleError(MerkleError),
    /// Serialization error
    #[cfg_attr(feature = "std", error("Serialization error"))]
    SerializationError,
    /// License expired
    #[cfg_attr(feature = "std", error("License expired"))]
    Expired,
    /// Hardware binding mismatch
    #[cfg_attr(feature = "std", error("Hardware binding mismatch"))]
    HwMismatch,
    /// Invalid PCR selection
    #[cfg_attr(feature = "std", error("Invalid PCR selection"))]
    InvalidPcr,
    /// Buffer overflow / capacity exceeded
    #[cfg_attr(feature = "std", error("Buffer capacity exceeded"))]
    CapacityExceeded,
    /// Zero-heap violation detected
    #[cfg_attr(feature = "std", error("Zero-heap violation in hot path"))]
    ZeroHeapViolation,
}

/// Result type for license operations
pub type LicenseResult<T> = Result<T, LicenseError>;

// Implement From traits for error conversion
impl From<TierError> for LicenseError {
    fn from(_err: TierError) -> Self {
        LicenseError::InvalidTier
    }
}

impl From<FeatureError> for LicenseError {
    fn from(_err: FeatureError) -> Self {
        LicenseError::InvalidFeature
    }
}

impl From<PolicyError> for LicenseError {
    fn from(err: PolicyError) -> Self {
        LicenseError::PolicyError(err)
    }
}

impl From<VerifyError> for LicenseError {
    fn from(err: VerifyError) -> Self {
        LicenseError::VerifyError(err)
    }
}

impl From<TpmError> for LicenseError {
    fn from(err: TpmError) -> Self {
        LicenseError::TpmError(err)
    }
}

impl From<MerkleError> for LicenseError {
    fn from(err: MerkleError) -> Self {
        LicenseError::MerkleError(err)
    }
}

/// Context for zero-heap license verification
#[derive(Debug, Clone, Copy, Default)]
pub struct VerificationContext {
    /// Current timestamp (unix seconds)
    pub current_time: u64,
    /// Hardware fingerprint for hw_bound licenses
    pub hw_fingerprint: Option<[u8; 32]>,
    /// PCR values for TPM-bound licenses
    pub pcr_values: Option<[u8; 32]>,
}

impl VerificationContext {
    /// Create a new verification context with current timestamp
    #[inline(always)]
    pub const fn new(current_time: u64) -> Self {
        Self {
            current_time,
            hw_fingerprint: None,
            pcr_values: None,
        }
    }

    /// Set hardware fingerprint for hw_bound verification
    #[inline(always)]
    pub const fn with_hw_fingerprint(mut self, fingerprint: [u8; 32]) -> Self {
        self.hw_fingerprint = Some(fingerprint);
        self
    }

    /// Set PCR values for TPM verification
    #[inline(always)]
    pub const fn with_pcr_values(mut self, pcrs: [u8; 32]) -> Self {
        self.pcr_values = Some(pcrs);
        self
    }
}

// Feature flags for conditional compilation
#[cfg(feature = "std")]
#[cfg_attr(docsrs, doc(cfg(feature = "std")))]
extern crate std;
#[cfg(feature = "std")]
pub use std::vec::Vec;

#[cfg(all(not(feature = "std"), feature = "alloc"))]
extern crate alloc;
#[cfg(all(not(feature = "std"), feature = "alloc"))]
pub use alloc::vec::Vec;

#[cfg(not(any(feature = "std", feature = "alloc")))]
pub use heapless::Vec;

/// Trait for types that can be serialized in no_std environments
pub trait NoStdSerialize {
    /// Serialize to a fixed-size buffer
    fn serialize_to(&self, buf: &mut [u8]) -> Result<usize, LicenseError>;

    /// Get the serialized size
    fn serialized_size(&self) -> usize;
}

/// Trait for types that can be deserialized in no_std environments
pub trait NoStdDeserialize: Sized {
    /// Deserialize from a buffer
    fn deserialize_from(buf: &[u8]) -> Result<Self, LicenseError>;
}

// Compile-time assertions for constant sizes
#[cfg(feature = "const_fn")]
const _: () = {
    assert!(KEY_ID_SIZE == 16);
    assert!(SIGNATURE_SIZE == 64);
    assert!(PUBLIC_KEY_SIZE == 32);
    assert!(SECRET_KEY_SIZE == 32);
    assert!(MAX_POLICY_SIZE == 512);
    assert!(MAX_KEY_SIZE == 1024);
    assert!(MAX_MERKLE_NODE_SIZE == 256);
    assert!(MAX_MERKLE_PROOF_DEPTH == 32);
    assert!(MAX_PCR_COUNT == 9);
    assert!(PCR_MASK == 0x0001_00FF);
    assert!(PCR_INDICES.len() == 9);
};

// Tests
#[cfg(test)]
#[cfg(feature = "std")]
mod tests {
    use super::*;

    #[test]
    fn test_constants() {
        assert_eq!(KEY_ID_SIZE, 16);
        assert_eq!(SIGNATURE_SIZE, 64);
        assert_eq!(PUBLIC_KEY_SIZE, 32);
        assert_eq!(SECRET_KEY_SIZE, 32);
        assert_eq!(MAX_POLICY_SIZE, 512);
        assert_eq!(MAX_KEY_SIZE, 1024);
        assert_eq!(MAX_MERKLE_NODE_SIZE, 256);
        assert_eq!(MAX_MERKLE_PROOF_DEPTH, 32);
        assert_eq!(MAX_PCR_COUNT, 9);
        assert_eq!(PCR_MASK, 0x0001_00FF);
        assert_eq!(PCR_INDICES.len(), 9);
        assert_eq!(PCR_INDICES, [0, 1, 2, 3, 4, 5, 6, 7, 16]);
    }

    #[test]
    fn test_version() {
        assert!(!VERSION_STR.is_empty());
    }

    #[test]
    fn test_verification_context() {
        let ctx = VerificationContext::new(1234567890)
            .with_hw_fingerprint([0xAA; 32])
            .with_pcr_values([0xBB; 32]);

        assert_eq!(ctx.current_time, 1234567890);
        assert_eq!(ctx.hw_fingerprint, Some([0xAA; 32]));
        assert_eq!(ctx.pcr_values, Some([0xBB; 32]));
    }
}
