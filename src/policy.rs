/*!
 * License Policy Definitions
 *
 * Defines the LicensePolicy struct with all policy fields.
 */

use crate::features::FeatureSet;
use crate::tier::LicenseTier;
use crate::MAX_POLICY_SIZE;
use core::fmt;
#[cfg(any(feature = "std", feature = "alloc"))]
use heapless::Vec as HeaplessVec;
#[cfg(any(feature = "std", feature = "alloc"))]
use serde::{Deserialize, Serialize};
#[cfg(feature = "std")]
use std::string::{String, ToString};

/// License Policy
///
/// Defines the complete policy for a license key including:
/// - Resource limits (tokens, requests, compute)
/// - Hardware binding
/// - Feature flags
/// - Expiration
/// - Tier
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(any(feature = "std", feature = "alloc"), derive(Serialize, Deserialize))]
#[cfg_attr(any(feature = "std", feature = "alloc"), serde(rename_all = "snake_case"))]
#[repr(C)]
pub struct LicensePolicy {
    /// Maximum tokens allowed per period (0 = unlimited)
    pub max_tokens: u64,
    /// Maximum requests allowed per period (0 = unlimited)
    pub max_requests: u64,
    /// Maximum compute resources (threads/CPU %) (0 = unlimited)
    pub max_compute: u32,
    /// Whether license is bound to hardware
    pub hw_bound: bool,
    /// Feature set bitflags
    pub features: FeatureSet,
    /// Expiration timestamp (unix seconds, 0 = never expires)
    pub expiry: u64,
    /// License tier
    pub tier: LicenseTier,
}

/// Errors related to license policy
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "std", derive(thiserror::Error))]
pub enum PolicyError {
    /// Invalid max_tokens value
    #[cfg_attr(feature = "std", error("Invalid max_tokens: {0}"))]
    InvalidMaxTokens(u64),
    /// Invalid max_requests value
    #[cfg_attr(feature = "std", error("Invalid max_requests: {0}"))]
    InvalidMaxRequests(u64),
    /// Invalid max_compute value
    #[cfg_attr(feature = "std", error("Invalid max_compute: {0}"))]
    InvalidMaxCompute(u32),
    /// Invalid expiry timestamp
    #[cfg_attr(feature = "std", error("Invalid expiry timestamp: {0}"))]
    InvalidExpiry(u64),
    /// Feature set not compatible with tier
    #[cfg_attr(feature = "std", error("Feature set incompatible with tier"))]
    FeatureTierMismatch,
    /// Policy too large for serialization buffer
    #[cfg_attr(feature = "std", error("Policy exceeds maximum size"))]
    TooLarge,
    /// Hardware binding required but not supported by tier
    #[cfg_attr(feature = "std", error("Hardware binding not supported by tier"))]
    HwBindingUnsupported,
    /// TPM sealing required but not supported by tier
    #[cfg_attr(feature = "std", error("TPM sealing not supported by tier"))]
    TpmUnsupported,
    /// Serialization error
    #[cfg_attr(feature = "std", error("Serialization failed"))]
    SerializationError,
    /// Deserialization error
    #[cfg_attr(feature = "std", error("Deserialization failed"))]
    DeserializationError,
}

impl LicensePolicy {
    /// Create a new license policy with defaults
    #[inline(always)]
    pub const fn new(tier: LicenseTier) -> Self {
        Self {
            max_tokens: 0,
            max_requests: 0,
            max_compute: 0,
            hw_bound: false,
            features: tier.default_features(),
            expiry: 0,
            tier,
        }
    }

    /// Create a new license policy with custom values
    #[inline(always)]
    pub const fn with_values(
        max_tokens: u64,
        max_requests: u64,
        max_compute: u32,
        hw_bound: bool,
        features: FeatureSet,
        expiry: u64,
        tier: LicenseTier,
    ) -> Self {
        Self {
            max_tokens,
            max_requests,
            max_compute,
            hw_bound,
            features,
            expiry,
            tier,
        }
    }

    /// Builder-style methods for const construction
    #[inline(always)]
    pub const fn max_tokens(mut self, max_tokens: u64) -> Self {
        self.max_tokens = max_tokens;
        self
    }

    #[inline(always)]
    pub const fn max_requests(mut self, max_requests: u64) -> Self {
        self.max_requests = max_requests;
        self
    }

    #[inline(always)]
    pub const fn max_compute(mut self, max_compute: u32) -> Self {
        self.max_compute = max_compute;
        self
    }

    #[inline(always)]
    pub const fn hw_bound(mut self, hw_bound: bool) -> Self {
        self.hw_bound = hw_bound;
        self
    }

    #[inline(always)]
    pub const fn features(mut self, features: FeatureSet) -> Self {
        self.features = features;
        self
    }

    #[inline(always)]
    pub const fn expiry(mut self, expiry: u64) -> Self {
        self.expiry = expiry;
        self
    }

    #[inline(always)]
    pub const fn tier(mut self, tier: LicenseTier) -> Self {
        self.tier = tier;
        self
    }

    /// Validate the policy against its tier
    #[inline]
    pub fn validate(&self) -> Result<(), PolicyError> {
        // Check hardware binding compatibility
        if self.hw_bound && !self.tier.supports_hw_binding() {
            return Err(PolicyError::HwBindingUnsupported);
        }

        // Check if features are compatible with tier
        let tier_features = self.tier.default_features();
        if !self.features.is_subset_of(tier_features) {
            // Allow government tier to have all features
            if self.tier != LicenseTier::Government && !self.features.is_subset_of(tier_features) {
                return Err(PolicyError::FeatureTierMismatch);
            }
        }

        // Check TPM compatibility
        if self.features.has(crate::features::Feature::TpmSealing) && !self.tier.supports_tpm() {
            return Err(PolicyError::TpmUnsupported);
        }

        // Validate expiry (if non-zero, must be in future)
        // Note: actual time check happens at verification time
        Ok(())
    }

    /// Check if the license is expired at the given timestamp
    #[inline(always)]
    pub const fn is_expired(&self, current_time: u64) -> bool {
        self.expiry != 0 && current_time >= self.expiry
    }

    /// Check if the license has unlimited tokens
    #[inline(always)]
    pub const fn unlimited_tokens(&self) -> bool {
        self.max_tokens == 0
    }

    /// Check if the license has unlimited requests
    #[inline(always)]
    pub const fn unlimited_requests(&self) -> bool {
        self.max_requests == 0
    }

    /// Check if the license has unlimited compute
    #[inline(always)]
    pub const fn unlimited_compute(&self) -> bool {
        self.max_compute == 0
    }

    /// Get the remaining time until expiry (0 if never expires or already expired)
    #[inline(always)]
    pub const fn time_until_expiry(&self, current_time: u64) -> u64 {
        if self.expiry == 0 {
            0
        } else { self.expiry.saturating_sub(current_time) }
    }

    /// Serialize to postcard format (no_std compatible)
    #[cfg(any(feature = "std", feature = "alloc"))]
    pub fn to_postcard(&self) -> Result<HeaplessVec<u8, MAX_POLICY_SIZE>, PolicyError> {
        let mut buf = [0u8; MAX_POLICY_SIZE];
        let serialized = postcard::to_slice(self, &mut buf).map_err(|_| PolicyError::SerializationError)?;
        let mut vec = HeaplessVec::new();
        vec.extend_from_slice(serialized).map_err(|_| PolicyError::SerializationError)?;
        Ok(vec)
    }

    /// Deserialize from postcard format
    #[cfg(any(feature = "std", feature = "alloc"))]
    pub fn from_postcard(data: &[u8]) -> Result<Self, PolicyError> {
        postcard::from_bytes(data).map_err(|_| PolicyError::DeserializationError)
    }

    /// Serialize to JSON (std feature only)
    #[cfg(feature = "std")]
    pub fn to_json(&self) -> Result<String, PolicyError> {
        serde_json::to_string(self).map_err(|_| PolicyError::SerializationError)
    }

    /// Deserialize from JSON (std feature only)
    #[cfg(feature = "std")]
    pub fn from_json(json: &str) -> Result<Self, PolicyError> {
        serde_json::from_str(json).map_err(|_| PolicyError::DeserializationError)
    }

    /// Get the serialized size (approximate)
    #[inline]
    pub fn serialized_size(&self) -> usize {
        // Rough estimate for postcard encoding
        8 + 8 + 4 + 1 + 8 + 8 + 1 + 16 // fields + feature set + tier
    }
}

impl Default for LicensePolicy {
    #[inline(always)]
    fn default() -> Self {
        Self::new(LicenseTier::Free)
    }
}

impl fmt::Display for LicensePolicy {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        #[cfg(feature = "std")]
        {
            write!(
                f,
                "LicensePolicy {{ tier: {}, max_tokens: {}, max_requests: {}, max_compute: {}, hw_bound: {}, features: {:?}, expiry: {} }}",
                self.tier,
                self.max_tokens,
                self.max_requests,
                self.max_compute,
                self.hw_bound,
                self.features,
                if self.expiry == 0 { "never".to_string() } else { self.expiry.to_string() }
            )
        }
        #[cfg(not(feature = "std"))]
        {
            let expiry_str = if self.expiry == 0 { "never" } else { "expires" };
            write!(
                f,
                "LicensePolicy {{ tier: {}, max_tokens: {}, max_requests: {}, max_compute: {}, hw_bound: {}, features: {:?}, expiry: {} }}",
                self.tier,
                self.max_tokens,
                self.max_requests,
                self.max_compute,
                self.hw_bound,
                self.features,
                expiry_str
            )
        }
    }
}

#[cfg(feature = "const_fn")]
impl LicensePolicy {
    /// Const version of validate
    #[inline(always)]
    pub const fn const_validate(&self) -> Result<(), PolicyError> {
        // Check hardware binding compatibility
        if self.hw_bound && !self.tier.supports_hw_binding() {
            return Err(PolicyError::HwBindingUnsupported);
        }

        // Check if features are compatible with tier
        let tier_features = self.tier.default_features();
        if !self.features.is_subset_of(tier_features) {
            // Use .value() comparison — const-compatible since LicenseTier is #[repr(u8)]
            if self.tier.value() != LicenseTier::Government.value() {
                return Err(PolicyError::FeatureTierMismatch);
            }
        }

        Ok(())
    }

    /// Const version of is_expired
    #[inline(always)]
    pub const fn const_is_expired(&self, current_time: u64) -> bool {
        self.expiry != 0 && current_time >= self.expiry
    }
}

#[cfg(test)]
#[cfg(feature = "std")]
mod tests {
    use super::*;
    use std::format;
    use crate::features::{Feature, FeatureSet};
    use crate::tier::LicenseTier;

    #[test]
    fn test_policy_new() {
        let policy = LicensePolicy::new(LicenseTier::Pro);
        assert_eq!(policy.tier, LicenseTier::Pro);
        assert_eq!(policy.max_tokens, 0);
        assert_eq!(policy.max_requests, 0);
        assert_eq!(policy.max_compute, 0);
        assert!(!policy.hw_bound);
        assert_eq!(policy.expiry, 0);
        assert!(policy.features.has(Feature::ZeroTrust));
    }

    #[test]
    fn test_policy_builder() {
        let policy = LicensePolicy::new(LicenseTier::Base)
            .max_tokens(1000)
            .max_requests(100)
            .max_compute(4)
            .hw_bound(true)
            .expiry(2000000000);

        assert_eq!(policy.max_tokens, 1000);
        assert_eq!(policy.max_requests, 100);
        assert_eq!(policy.max_compute, 4);
        assert!(policy.hw_bound);
        assert_eq!(policy.expiry, 2000000000);
    }

    #[test]
    fn test_policy_validate() {
        // Valid policy
        let policy = LicensePolicy::new(LicenseTier::Pro)
            .max_tokens(1000)
            .hw_bound(true);
        assert!(policy.validate().is_ok());

        // Invalid: hw_bound on Free tier
        let policy = LicensePolicy::new(LicenseTier::Free).hw_bound(true);
        assert!(matches!(policy.validate(), Err(PolicyError::HwBindingUnsupported)));

        // Invalid: features not in tier
        let mut features = FeatureSet::empty();
        features.insert(FeatureSet::from(Feature::ZeroTrust)); // Pro feature
        let policy = LicensePolicy::new(LicenseTier::Base).features(features);
        assert!(matches!(policy.validate(), Err(PolicyError::FeatureTierMismatch)));

        // Valid: Government tier can have any feature
        let features = FeatureSet::all();
        let policy = LicensePolicy::new(LicenseTier::Government).features(features);
        assert!(policy.validate().is_ok());
    }

    #[test]
    fn test_policy_expiry() {
        let policy = LicensePolicy::new(LicenseTier::Pro).expiry(2000000000);
        assert!(!policy.is_expired(1000000000));
        assert!(policy.is_expired(3000000000));

        let never_expires = LicensePolicy::new(LicenseTier::Pro);
        assert!(!never_expires.is_expired(u64::MAX));
    }

    #[test]
    fn test_policy_unlimited() {
        let policy = LicensePolicy::new(LicenseTier::Pro);
        assert!(policy.unlimited_tokens());
        assert!(policy.unlimited_requests());
        assert!(policy.unlimited_compute());

        let limited = LicensePolicy::new(LicenseTier::Pro)
            .max_tokens(100)
            .max_requests(50)
            .max_compute(2);
        assert!(!limited.unlimited_tokens());
        assert!(!limited.unlimited_requests());
        assert!(!limited.unlimited_compute());
    }

    #[test]
    fn test_policy_time_until_expiry() {
        let policy = LicensePolicy::new(LicenseTier::Pro).expiry(2000);
        assert_eq!(policy.time_until_expiry(1000), 1000);
        assert_eq!(policy.time_until_expiry(2000), 0);
        assert_eq!(policy.time_until_expiry(3000), 0);

        let never = LicensePolicy::new(LicenseTier::Pro);
        assert_eq!(never.time_until_expiry(1000), 0);
    }

    #[test]
    #[cfg(feature = "std")]
    fn test_policy_serialization() {
        let policy = LicensePolicy::new(LicenseTier::Pro)
            .max_tokens(1000)
            .max_requests(100)
            .hw_bound(true)
            .expiry(2000000000);

        // First test direct postcard::to_allocvec to confirm struct serializes
        let alloc_bytes = postcard::to_allocvec(&policy).unwrap();
        let decoded: LicensePolicy = postcard::from_bytes(&alloc_bytes).unwrap();
        assert_eq!(policy, decoded);

        // Now test heapless-based to_postcard
        let postcard_result = policy.to_postcard();
        let postcard = postcard_result.expect("to_postcard should work");
        let decoded = LicensePolicy::from_postcard(&postcard).unwrap();
        assert_eq!(policy, decoded);
        let decoded = LicensePolicy::from_postcard(&postcard).unwrap();
        assert_eq!(policy, decoded);

        let json = policy.to_json().unwrap();
        let decoded = LicensePolicy::from_json(&json).unwrap();
        assert_eq!(policy, decoded);
    }

    #[test]
    fn test_policy_default() {
        let policy = LicensePolicy::default();
        assert_eq!(policy.tier, LicenseTier::Free);
        assert!(policy.features.has(Feature::SaseBasic));
    }

    #[test]
    fn test_policy_display() {
        let policy = LicensePolicy::new(LicenseTier::Pro).expiry(1234567890);
        let display = format!("{}", policy);
        assert!(display.contains("pro"));
        assert!(display.contains("1234567890"));
    }
}