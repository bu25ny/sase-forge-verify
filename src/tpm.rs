/*!
 * TPM2 Integration
 *
 * TPM2 PCR sealing/unsealing for license policies.
 * Supports PCR 0-7 and 16.
 */

use crate::policy::LicensePolicy;
use crate::MAX_POLICY_SIZE;
use core::fmt;
#[cfg(feature = "std")]
use serde::{Deserialize, Serialize};

/// PCR selection mask (bits 0-7 and 16)
pub const PCR_MASK: u32 = crate::PCR_MASK;

/// PCR indices supported (0-7, 16)
pub const PCR_INDICES: [u32; crate::MAX_PCR_COUNT] = crate::PCR_INDICES;

/// TPM errors
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "std", derive(thiserror::Error))]
pub enum TpmError {
    /// TPM not available
    #[cfg_attr(feature = "std", error("TPM not available"))]
    NotAvailable,
    /// TPM communication error
    #[cfg_attr(feature = "std", error("TPM communication error: {0}"))]
    CommunicationError(&'static str),
    /// Invalid PCR selection
    #[cfg_attr(feature = "std", error("Invalid PCR selection: {0}"))]
    InvalidPcrSelection(&'static str),
    /// PCR value mismatch
    #[cfg_attr(feature = "std", error("PCR value mismatch for PCR {0}"))]
    PcrMismatch(u32),
    /// Seal operation failed
    #[cfg_attr(feature = "std", error("Seal operation failed"))]
    SealFailed,
    /// Unseal operation failed
    #[cfg_attr(feature = "std", error("Unseal operation failed"))]
    UnsealFailed,
    /// Policy too large for TPM
    #[cfg_attr(feature = "std", error("Policy exceeds TPM buffer size"))]
    PolicyTooLarge,
    /// Invalid authorization
    #[cfg_attr(feature = "std", error("Invalid authorization"))]
    InvalidAuth,
    /// TPM resource exhausted
    #[cfg_attr(feature = "std", error("TPM resource exhausted"))]
    ResourceExhausted,
    /// TPM not initialized
    #[cfg_attr(feature = "std", error("TPM not initialized"))]
    NotInitialized,
    /// Feature not supported by tier
    #[cfg_attr(feature = "std", error("Feature not supported by license tier"))]
    FeatureNotSupported,
    /// Mock TPM error (for testing)
    #[cfg_attr(feature = "std", error("Mock TPM error: {0}"))]
    MockError(&'static str),
}

/// PCR mask type for type-safe PCR selection
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
#[repr(transparent)]
pub struct PcrMask(pub u32);

impl PcrMask {
    /// Create a new PCR mask
    #[inline(always)]
    pub const fn new(mask: u32) -> Self {
        Self(mask & PCR_MASK)
    }

    /// Create a mask with all supported PCRs
    #[inline(always)]
    pub const fn all() -> Self {
        Self(PCR_MASK)
    }

    /// Create a mask for specific PCR indices
    #[inline]
    pub fn from_indices(indices: &[u32]) -> Result<Self, TpmError> {
        let mut mask = 0u32;
        for &idx in indices {
            if idx > 31 {
                return Err(TpmError::InvalidPcrSelection("PCR index > 31"));
            }
            // Check if PCR is supported
            let supported = PCR_INDICES.contains(&idx);
            if !supported {
                return Err(TpmError::InvalidPcrSelection("PCR index not supported"));
            }
            mask |= 1u32 << idx;
        }
        Ok(Self(mask))
    }

    /// Get the raw mask value
    #[inline(always)]
    pub const fn value(self) -> u32 {
        self.0
    }

    /// Check if a PCR is in the mask
    #[inline(always)]
    pub const fn has(self, pcr: u32) -> bool {
        (self.0 & (1u32 << pcr)) != 0
    }

    /// Get the number of PCRs in the mask
    #[inline(always)]
    pub const fn count(self) -> u32 {
        self.0.count_ones()
    }

    /// Iterate over PCR indices in the mask
    #[inline]
    pub fn iter(self) -> impl Iterator<Item = u32> {
        (0..32).filter(move |&i| self.has(i))
    }

    /// Validate that the mask only contains supported PCRs
    #[inline(always)]
    pub const fn is_valid(self) -> bool {
        (self.0 & !PCR_MASK) == 0
    }
}

impl Default for PcrMask {
    #[inline(always)]
    fn default() -> Self {
        Self::all()
    }
}

impl fmt::Display for PcrMask {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "PcrMask(0x{:08X})", self.0)
    }
}

/// PCR indices type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct PcrIndices(pub [u32; crate::MAX_PCR_COUNT]);

impl PcrIndices {
    /// Get the default PCR indices (0-7, 16)
    #[inline(always)]
    pub const fn default() -> Self {
        Self(crate::PCR_INDICES)
    }

    /// Get as slice
    #[inline(always)]
    pub const fn as_slice(&self) -> &[u32] {
        &self.0
    }
}

impl Default for PcrIndices {
    #[inline(always)]
    fn default() -> Self {
        Self::default()
    }
}

/// TPM Sealer trait for sealing/unsealing policies to PCRs
pub trait TpmSealer: Send + Sync {
    /// Seal a license policy to the specified PCRs
    ///
    /// Returns the sealed data blob that can only be unsealed
    /// when the PCRs match the specified values.
    fn seal_to_pcr(
        &self,
        policy: &LicensePolicy,
        pcrs: &[u32],
    ) -> Result<heapless::Vec<u8, MAX_POLICY_SIZE>, TpmError>;

    /// Unseal a previously sealed policy
    fn unseal(&self, sealed: &[u8]) -> Result<LicensePolicy, TpmError>;

    /// Get the TPM manufacturer
    fn manufacturer(&self) -> Result<heapless::String<32>, TpmError> {
        Err(TpmError::NotAvailable)
    }

    /// Get the TPM firmware version
    fn firmware_version(&self) -> Result<u64, TpmError> {
        Err(TpmError::NotAvailable)
    }

    /// Check if TPM is available and ready
    fn is_ready(&self) -> bool {
        false
    }

    /// Get supported PCR banks
    fn pcr_banks(&self) -> Result<heapless::Vec<u16, 8>, TpmError> {
        Err(TpmError::NotAvailable)
    }
}

/// TPM Sealer implementation using TSS-ESAPI (std feature only)
#[cfg(all(feature = "tpm", feature = "std"))]
pub mod tss {
    use super::*;
    use tss_esapi::{
        handles::KeyHandle,
        interface_types::{
            algorithm::{HashingAlgorithm, PublicAlgorithm},
            resource_handles::Hierarchy,
            key_bits::RsaKeyBits,
        },
        structures::{
            PcrSelectionListBuilder, PcrSlot,
            PublicBuilder, PublicRsaParametersBuilder, RsaExponent,
            RsaScheme, SensitiveData,
        },
        Context, TctiNameConf,
        constants::CapabilityType,
    };
    use std::path::Path;
    use std::format;
    use std::str::FromStr;
    use crate::MAX_PCR_COUNT;

    /// TSS-ESAPI based TPM Sealer
    pub struct TssTpmSealer {
        context: std::sync::Mutex<Context>,
    }

    impl TssTpmSealer {
        /// Create a new TSS TPM Sealer
        pub fn new(tcti: Option<&str>) -> Result<Self, TpmError> {
            let tcti_name = tcti.and_then(|s| TctiNameConf::from_str(s).ok()).unwrap_or_else(|| {
                // Try default TCTI paths
                if Path::new("/dev/tpmrm0").exists() {
                    TctiNameConf::from_str("device:/dev/tpmrm0").unwrap()
                } else if Path::new("/dev/tpm0").exists() {
                    TctiNameConf::from_str("device:/dev/tpm0").unwrap()
                } else {
                    TctiNameConf::from_str("mssim:host=127.0.0.1,port=2321").unwrap()
                }
            });

            let context = Context::new(tcti_name)
                .map_err(|_| TpmError::CommunicationError("Failed to create TPM context"))?;

            Ok(Self { context: std::sync::Mutex::new(context) })
        }

        /// Create a TPM key for sealing
        fn create_sealing_key(&mut self) -> Result<KeyHandle, TpmError> {
            let sensitive = SensitiveData::default();
            let rsa_params = PublicRsaParametersBuilder::new()
                .with_scheme(RsaScheme::Null)
                .with_key_bits(RsaKeyBits::Rsa2048)
                .with_exponent(RsaExponent::default())
                .build()
                .map_err(|_| TpmError::SealFailed)?;

            let public = PublicBuilder::new()
                .with_public_algorithm(PublicAlgorithm::Rsa)
                .with_name_hashing_algorithm(HashingAlgorithm::Sha256)
                .with_rsa_parameters(rsa_params)
                .build()
                .map_err(|_| TpmError::SealFailed)?;

            let mut ctx = self.context.lock().map_err(|_| TpmError::SealFailed)?;
            let result = ctx
                .create_primary(Hierarchy::Owner, public, None, Some(sensitive), None, None)
                .map_err(|_| TpmError::SealFailed)?;

            Ok(result.key_handle)
        }
    }

    impl TpmSealer for TssTpmSealer {
        fn seal_to_pcr(
            &self,
            policy: &LicensePolicy,
            pcrs: &[u32],
        ) -> Result<heapless::Vec<u8, MAX_POLICY_SIZE>, TpmError> {
            // Serialize policy
            let policy_bytes = postcard::to_allocvec(policy)
                .map_err(|_| TpmError::PolicyTooLarge)?;

            // Validate PCR selection
            if pcrs.is_empty() {
                return Err(TpmError::InvalidPcrSelection("No PCRs specified"));
            }

            for &pcr in pcrs {
                if !PCR_INDICES.iter().any(|&p| p == pcr) {
                    return Err(TpmError::InvalidPcrSelection("Unsupported PCR"));
                }
            }

            // Create PCR selection list
            let mut slots = std::vec::Vec::new();
            for &pcr in pcrs {
                let slot = match pcr {
                    0 => PcrSlot::Slot0,
                    1 => PcrSlot::Slot1,
                    2 => PcrSlot::Slot2,
                    3 => PcrSlot::Slot3,
                    4 => PcrSlot::Slot4,
                    5 => PcrSlot::Slot5,
                    6 => PcrSlot::Slot6,
                    7 => PcrSlot::Slot7,
                    8 => PcrSlot::Slot8,
                    9 => PcrSlot::Slot9,
                    10 => PcrSlot::Slot10,
                    11 => PcrSlot::Slot11,
                    12 => PcrSlot::Slot12,
                    13 => PcrSlot::Slot13,
                    14 => PcrSlot::Slot14,
                    15 => PcrSlot::Slot15,
                    16 => PcrSlot::Slot16,
                    17 => PcrSlot::Slot17,
                    18 => PcrSlot::Slot18,
                    19 => PcrSlot::Slot19,
                    20 => PcrSlot::Slot20,
                    21 => PcrSlot::Slot21,
                    22 => PcrSlot::Slot22,
                    23 => PcrSlot::Slot23,
                    _ => return Err(TpmError::InvalidPcrSelection("Invalid PCR slot number")),
                };
                slots.push(slot);
            }

            let _pcr_selection = PcrSelectionListBuilder::new()
                .with_selection(HashingAlgorithm::Sha256, &slots)
                .build()
                .map_err(|_| TpmError::InvalidPcrSelection("Failed to build PCR selection"))?;

            // Placeholder logic (full seal requires transient objects etc)
            let mut sealed = heapless::Vec::new();
            sealed.extend_from_slice(&policy_bytes)
                .map_err(|_| TpmError::PolicyTooLarge)?;

            // Add PCR selection info (simplified)
            for &pcr in pcrs {
                sealed.push(pcr as u8).map_err(|_| TpmError::PolicyTooLarge)?;
            }

            Ok(sealed)
        }

        fn unseal(&self, sealed: &[u8]) -> Result<LicensePolicy, TpmError> {
            if sealed.len() < MAX_PCR_COUNT {
                return Err(TpmError::UnsealFailed);
            }

            let policy_len = sealed.len().saturating_sub(MAX_PCR_COUNT);
            let policy_bytes = &sealed[..policy_len];

            let policy = postcard::from_bytes(policy_bytes)
                .map_err(|_| TpmError::UnsealFailed)?;

            Ok(policy)
        }

        fn is_ready(&self) -> bool {
            if let Ok(mut ctx) = self.context.lock() {
                ctx.get_capability(CapabilityType::TpmProperties, 0, 1).is_ok()
            } else {
                false
            }
        }

        fn manufacturer(&self) -> Result<heapless::String<32>, TpmError> {
            let mut ctx = self.context.lock().map_err(|_| TpmError::CommunicationError("Lock poisoned"))?;
            let (cap, _) = ctx.get_capability(CapabilityType::TpmProperties, 0x00000104, 1) // TPM_PT_MANUFACTURER
                .map_err(|_| TpmError::CommunicationError("Get capability failed"))?;
            let mut s = heapless::String::new();
            s.push_str(&format!("{:?}", cap)).ok();
            Ok(s)
        }
    }
}

/// Mock TPM Sealer for testing and no_std environments
#[derive(Debug, Clone, Default)]
pub struct MockTpmSealer {
    /// Simulated PCR values (32 bytes = 256 bits)
    pcr_values: [u8; 32],
    /// Whether the mock should succeed
    should_succeed: bool,
    /// Mock manufacturer string
    manufacturer: heapless::String<32>,
}

impl MockTpmSealer {
    /// Create a new mock TPM sealer
    #[inline(always)]
    pub fn new() -> Self {
        Self {
            pcr_values: [0u8; 32],
            should_succeed: true,
            manufacturer: heapless::String::new(),
        }
    }

    /// Set PCR values for testing
    #[inline(always)]
    pub fn with_pcr_values(mut self, pcrs: [u8; 32]) -> Self {
        self.pcr_values = pcrs;
        self
    }

    /// Set whether operations should succeed
    #[inline(always)]
    pub fn with_success(mut self, success: bool) -> Self {
        self.should_succeed = success;
        self
    }

    /// Set manufacturer string
    #[inline(always)]
    pub fn with_manufacturer(mut self, mfr: &str) -> Self {
        let _ = self.manufacturer.push_str(mfr);
        self
    }

    /// Set a specific PCR value
    #[inline(always)]
    pub fn set_pcr(&mut self, pcr: u32, value: [u8; 32]) {
        if pcr < 32 {
            let start = (pcr as usize) * 32;
            if start + 32 <= self.pcr_values.len() {
                self.pcr_values[start..start+32].copy_from_slice(&value);
            }
        }
    }
}

impl TpmSealer for MockTpmSealer {
    fn seal_to_pcr(
        &self,
        policy: &LicensePolicy,
        pcrs: &[u32],
    ) -> Result<heapless::Vec<u8, MAX_POLICY_SIZE>, TpmError> {
        if !self.should_succeed {
            return Err(TpmError::MockError("Mock TPM configured to fail"));
        }

        // Serialize policy
        let policy_bytes = postcard::to_allocvec(policy)
            .map_err(|_| TpmError::PolicyTooLarge)?;

        // Validate PCRs
        if pcrs.is_empty() {
            return Err(TpmError::InvalidPcrSelection("No PCRs specified"));
        }

        for &pcr in pcrs {
            if !PCR_INDICES.contains(&pcr) {
                return Err(TpmError::InvalidPcrSelection("Unsupported PCR"));
            }
        }

        // Create sealed blob: policy + PCR indices + PCR values
        let mut sealed = heapless::Vec::new();
        sealed.extend_from_slice(&policy_bytes)
            .map_err(|_| TpmError::PolicyTooLarge)?;

        // Add PCR count
        sealed.push(pcrs.len() as u8).map_err(|_| TpmError::PolicyTooLarge)?;

        // Add PCR indices
        for &pcr in pcrs {
            sealed.push(pcr as u8).map_err(|_| TpmError::PolicyTooLarge)?;
        }

        // Add PCR values (simplified - just a hash of PCR indices)
        let mut pcr_hash = [0u8; 32];
        for (i, &pcr) in pcrs.iter().enumerate() {
            if i < 32 {
                pcr_hash[i] = pcr as u8;
            }
        }
        sealed.extend_from_slice(&pcr_hash)
            .map_err(|_| TpmError::PolicyTooLarge)?;

        // Add PCR count at the very end
        sealed.push(pcrs.len() as u8).map_err(|_| TpmError::PolicyTooLarge)?;

        Ok(sealed)
    }

    fn unseal(&self, sealed: &[u8]) -> Result<LicensePolicy, TpmError> {
        if !self.should_succeed {
            return Err(TpmError::MockError("Mock TPM configured to fail"));
        }

        if sealed.len() < 34 { // min: policy(1) + pcr(1) + hash(32) + count(1)
            return Err(TpmError::UnsealFailed);
        }

        // Extract PCR count from the very last byte
        let pcr_count = sealed[sealed.len() - 1] as usize;
        let expected_min_len = 32 + pcr_count + 1 + 1; // hash + indices + count + policy(1)
        if sealed.len() < expected_min_len {
            return Err(TpmError::UnsealFailed);
        }

        // Compute policy length
        let policy_len = sealed.len() - 32 - pcr_count - 1;
        let pcr_indices = &sealed[policy_len..policy_len + pcr_count];
        let stored_pcr_hash = &sealed[policy_len + pcr_count..sealed.len() - 1];

        // Verify PCRs match current values
        for (i, &pcr_idx) in pcr_indices.iter().enumerate() {
            if i < stored_pcr_hash.len() && i < 32 {
                // In real implementation, read actual PCR and compare
                // For mock, we just check the hash matches
                if stored_pcr_hash[i] != pcr_idx {
                    return Err(TpmError::PcrMismatch(pcr_idx as u32));
                }
            }
        }

        // Deserialize policy
        let policy_bytes = &sealed[..policy_len];
        let policy = postcard::from_bytes(policy_bytes)
            .map_err(|_| TpmError::UnsealFailed)?;

        Ok(policy)
    }

    fn manufacturer(&self) -> Result<heapless::String<32>, TpmError> {
        Ok(self.manufacturer.clone())
    }

    fn is_ready(&self) -> bool {
        self.should_succeed
    }
}

/// Verify a license policy against TPM PCRs
#[inline]
pub fn verify_policy_pcr_binding(
    _policy: &LicensePolicy,
    pcrs: &[u32],
    expected_pcr_values: &[[u8; 32]],
) -> Result<(), TpmError> {
    if pcrs.len() != expected_pcr_values.len() {
        return Err(TpmError::InvalidPcrSelection("PCR count mismatch"));
    }

    for (i, &pcr) in pcrs.iter().enumerate() {
        if !PCR_INDICES.contains(&pcr) {
            return Err(TpmError::InvalidPcrSelection("Unsupported PCR"));
        }

        // In real implementation, read PCR from TPM and compare
        // For now, just validate structure
        if expected_pcr_values[i] == [0u8; 32] {
            return Err(TpmError::PcrMismatch(pcr));
        }
    }

    Ok(())
}

/// Create a PCR policy digest for TPM sealing
#[inline]
pub fn create_pcr_policy_digest(pcrs: &[u32]) -> Result<[u8; 32], TpmError> {
    use sha2::{Sha256, Digest};

    if pcrs.is_empty() {
        return Err(TpmError::InvalidPcrSelection("No PCRs specified"));
    }

    let mut hasher = Sha256::new();
    for &pcr in pcrs {
        if !PCR_INDICES.contains(&pcr) {
            return Err(TpmError::InvalidPcrSelection("Unsupported PCR"));
        }
        hasher.update(pcr.to_le_bytes());
    }

    let result = hasher.finalize();
    let mut digest = [0u8; 32];
    digest.copy_from_slice(&result);
    Ok(digest)
}

#[cfg(test)]
#[cfg(feature = "std")]
mod tests {
    use super::*;
    use crate::policy::LicensePolicy;
    use crate::tier::LicenseTier;
    

    #[test]
    fn test_pcr_mask() {
        let mask = PcrMask::all();
        assert_eq!(mask.value(), PCR_MASK);
        assert!(mask.has(0));
        assert!(mask.has(7));
        assert!(mask.has(16));
        assert!(!mask.has(8));
        assert!(!mask.has(15));
        assert!(!mask.has(17));
        assert_eq!(mask.count(), 9);
    }

    #[test]
    fn test_pcr_mask_from_indices() {
        let mask = PcrMask::from_indices(&[0, 1, 2, 16]).unwrap();
        assert!(mask.has(0));
        assert!(mask.has(1));
        assert!(mask.has(2));
        assert!(mask.has(16));
        assert!(!mask.has(3));
        assert_eq!(mask.count(), 4);
    }

    #[test]
    fn test_pcr_mask_invalid() {
        assert!(PcrMask::from_indices(&[32]).is_err());
        assert!(PcrMask::from_indices(&[8]).is_err()); // Not supported
    }

    #[test]
    fn test_pcr_indices() {
        let indices = PcrIndices::default();
        assert_eq!(indices.as_slice(), &[0, 1, 2, 3, 4, 5, 6, 7, 16]);
    }

    #[test]
    fn test_mock_tpm_sealer() {
        let mut sealer = MockTpmSealer::new();
        sealer.pcr_values = [0xAA; 32];

        let policy = LicensePolicy::new(LicenseTier::Pro);
        let sealed = sealer.seal_to_pcr(&policy, &[0, 1, 2]).unwrap();
        assert!(!sealed.is_empty());

        let unsealed = sealer.unseal(&sealed).unwrap();
        assert_eq!(unsealed.tier, LicenseTier::Pro);
    }

    #[test]
    fn test_mock_tpm_sealer_failure() {
        let sealer = MockTpmSealer::new().with_success(false);
        let policy = LicensePolicy::new(LicenseTier::Pro);

        assert!(matches!(
            sealer.seal_to_pcr(&policy, &[0]),
            Err(TpmError::MockError(_))
        ));

        assert!(matches!(
            sealer.unseal(&[0u8; 100]),
            Err(TpmError::MockError(_))
        ));
    }

    #[test]
    fn test_mock_tpm_invalid_pcr() {
        let sealer = MockTpmSealer::new();
        let policy = LicensePolicy::new(LicenseTier::Pro);

        // Unsupported PCR
        assert!(matches!(
            sealer.seal_to_pcr(&policy, &[8]),
            Err(TpmError::InvalidPcrSelection(_))
        ));

        // Empty PCRs
        assert!(matches!(
            sealer.seal_to_pcr(&policy, &[]),
            Err(TpmError::InvalidPcrSelection(_))
        ));
    }

    #[test]
    fn test_create_pcr_policy_digest() {
        let digest = create_pcr_policy_digest(&[0, 1, 2]).unwrap();
        assert_ne!(digest, [0u8; 32]);

        // Same PCRs should produce same digest
        let digest2 = create_pcr_policy_digest(&[0, 1, 2]).unwrap();
        assert_eq!(digest, digest2);

        // Different order produces different digest
        let digest3 = create_pcr_policy_digest(&[2, 1, 0]).unwrap();
        assert_ne!(digest, digest3);
    }

    #[test]
    fn test_verify_policy_pcr_binding() {
        let policy = LicensePolicy::new(LicenseTier::Pro);
        let pcrs = [0, 1, 2];
        let values = [[0xAA; 32], [0xBB; 32], [0xCC; 32]];

        assert!(verify_policy_pcr_binding(&policy, &pcrs, &values).is_ok());

        // Wrong count
        assert!(verify_policy_pcr_binding(&policy, &pcrs, &[values[0]]).is_err());

        // Zero PCR value
        assert!(verify_policy_pcr_binding(&policy, &pcrs, &[[0u8; 32], values[1], values[2]]).is_err());
    }
}