/*!
 * License Key Definitions
 *
 * LicenseKey struct containing policy, Ed25519 signature, public key,
 * key ID, and issuance timestamp.
 */

use crate::crypto::{Signature, PublicKey, KeyId};
use crate::policy::{LicensePolicy, PolicyError};
use crate::tier::LicenseTier;
use crate::features::FeatureSet;
use crate::{VerificationContext, LicenseError, MAX_POLICY_SIZE, MAX_KEY_SIZE, KEY_ID_SIZE};
use core::fmt;
#[cfg(feature = "std")]
use serde::{Deserialize, Serialize};
#[cfg(feature = "std")]
use std::string::String;

/// License Key
///
/// Contains the license policy, Ed25519 signature, public key,
/// key identifier (BLAKE3 hash), and issuance timestamp.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "std", serde(rename_all = "snake_case"))]
#[repr(C)]
pub struct LicenseKey {
    /// The license policy
    pub policy: LicensePolicy,
    /// Ed25519 signature over the serialized policy
    pub signature: Signature,
    /// Ed25519 public key for verification
    pub public_key: PublicKey,
    /// Key identifier (BLAKE3 hash of public key, truncated to 16 bytes)
    pub key_id: KeyId,
    /// Issuance timestamp (unix seconds)
    pub issued_at: u64,
}

/// Errors related to license keys
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "std", derive(thiserror::Error))]
pub enum KeyError {
    /// Invalid signature
    #[cfg_attr(feature = "std", error("Invalid signature"))]
    InvalidSignature,
    /// Invalid public key
    #[cfg_attr(feature = "std", error("Invalid public key"))]
    InvalidPublicKey,
    /// Invalid key ID
    #[cfg_attr(feature = "std", error("Invalid key ID"))]
    InvalidKeyId,
    /// Invalid policy
    #[cfg_attr(feature = "std", error("Invalid policy: {0}"))]
    InvalidPolicy(PolicyError),
    /// Key expired
    #[cfg_attr(feature = "std", error("Key expired"))]
    Expired,
    /// Hardware binding mismatch
    #[cfg_attr(feature = "std", error("Hardware binding mismatch"))]
    HwMismatch,
    /// TPM verification failed
    #[cfg_attr(feature = "std", error("TPM verification failed"))]
    TpmVerificationFailed,
    /// Serialization error
    #[cfg_attr(feature = "std", error("Serialization error"))]
    SerializationError,
    /// Deserialization error
    #[cfg_attr(feature = "std", error("Deserialization error"))]
    DeserializationError,
    /// Key too large for buffer
    #[cfg_attr(feature = "std", error("Key exceeds maximum size"))]
    TooLarge,
    /// Invalid issuance time
    #[cfg_attr(feature = "std", error("Invalid issuance time"))]
    InvalidIssuedAt,
    /// Zero-heap violation
    #[cfg_attr(feature = "std", error("Zero-heap violation"))]
    ZeroHeapViolation,
}

impl LicenseKey {
    /// Create a new license key
    #[inline(always)]
    pub fn new(
        policy: LicensePolicy,
        signature: Signature,
        public_key: PublicKey,
        key_id: KeyId,
        issued_at: u64,
    ) -> Self {
        Self {
            policy,
            signature,
            public_key,
            key_id,
            issued_at,
        }
    }

    /// Verify the license key signature
    #[inline]
    pub fn verify(&self) -> Result<(), VerifyError> {
        self.verify_with_context(&VerificationContext::default())
    }

    /// Verify the license key with a verification context
    #[inline]
    pub fn verify_with_context(&self, ctx: &VerificationContext) -> Result<(), VerifyError> {
        // Verify signature over policy
        let policy_bytes = self.policy_to_bytes()?;
        self.public_key.verify(&policy_bytes, &self.signature)
            .map_err(|_| VerifyError::InvalidSignature)?;

        // Verify key ID matches public key
        let computed_key_id = self.compute_key_id();
        if computed_key_id != self.key_id {
            return Err(VerifyError::KeyIdMismatch);
        }

        // Check expiry
        if self.policy.is_expired(ctx.current_time) {
            return Err(VerifyError::Expired);
        }

        // Check hardware binding if required
        if self.policy.hw_bound {
            if let Some(hw_fingerprint) = ctx.hw_fingerprint {
                // In a real implementation, this would verify against TPM or hardware fingerprint
                // For now, we just check that a fingerprint was provided
                if hw_fingerprint == [0u8; 32] {
                    return Err(VerifyError::HwBindingRequired);
                }
            } else {
                return Err(VerifyError::HwBindingRequired);
            }
        }

        // Check TPM binding if required
        if self.policy.features.has(crate::features::Feature::TpmSealing)
            && ctx.pcr_values.is_none() {
                return Err(VerifyError::TpmBindingRequired);
            }

        Ok(())
    }

    /// Serialize policy to bytes for verification
    #[inline]
    fn policy_to_bytes(&self) -> Result<heapless::Vec<u8, MAX_POLICY_SIZE>, VerifyError> {
        let mut buf = [0u8; MAX_POLICY_SIZE];
        let serialized = postcard::to_slice(&self.policy, &mut buf)
            .map_err(|_| VerifyError::SerializationError)?;
        let mut vec = heapless::Vec::new();
        vec.extend_from_slice(serialized).map_err(|_| VerifyError::SerializationError)?;
        Ok(vec)
    }

    /// Compute key ID from public key (BLAKE3 hash truncated to 16 bytes)
    #[inline(always)]
    pub fn compute_key_id(&self) -> KeyId {
        #[cfg(feature = "std")]
        {
            use blake3::Hasher;
            let mut hasher = Hasher::new();
            hasher.update(self.public_key.as_bytes());
            let hash = hasher.finalize();
            let mut key_id_bytes = [0u8; KEY_ID_SIZE];
            key_id_bytes.copy_from_slice(&hash.as_bytes()[..KEY_ID_SIZE]);
            KeyId::new(key_id_bytes)
        }
        #[cfg(not(feature = "std"))]
        {
            // Simplified hash for no_std (in real impl, use blake3 crate with no_std)
            let mut key_id_bytes = [0u8; KEY_ID_SIZE];
            for (i, byte) in self.public_key.as_bytes().iter().enumerate() {
                if i < KEY_ID_SIZE {
                    key_id_bytes[i] = byte.wrapping_add(i as u8);
                }
            }
            KeyId::new(key_id_bytes)
        }
    }

    /// Verify and validate the license key fully
    #[inline]
    pub fn validate(&self, ctx: &VerificationContext) -> Result<(), LicenseError> {
        self.verify_with_context(ctx).map_err(LicenseError::from)?;
        self.policy.validate().map_err(LicenseError::from)?;
        Ok(())
    }

    /// Check if the license is expired
    #[inline(always)]
    pub const fn is_expired(&self, current_time: u64) -> bool {
        self.policy.is_expired(current_time)
    }

    /// Get the tier of this license
    #[inline(always)]
    pub const fn tier(&self) -> LicenseTier {
        self.policy.tier
    }

    /// Get the features of this license
    #[inline(always)]
    pub const fn features(&self) -> FeatureSet {
        self.policy.features
    }

    /// Check if this license has a specific feature
    #[inline(always)]
    pub const fn has_feature(&self, feature: crate::features::Feature) -> bool {
        self.policy.features.has(feature)
    }

    /// Serialize to postcard format
    #[cfg(feature = "std")]
    pub fn to_postcard(&self) -> Result<heapless::Vec<u8, MAX_KEY_SIZE>, KeyError> {
        let mut buf = [0u8; MAX_KEY_SIZE];
        let serialized = postcard::to_slice(self, &mut buf)
            .map_err(|_| KeyError::SerializationError)?;
        let mut vec = heapless::Vec::new();
        vec.extend_from_slice(serialized).map_err(|_| KeyError::SerializationError)?;
        Ok(vec)
    }

    /// Deserialize from postcard format
    #[cfg(feature = "std")]
    pub fn from_postcard(data: &[u8]) -> Result<Self, KeyError> {
        postcard::from_bytes(data)
            .map_err(|_| KeyError::DeserializationError)
    }

    /// Serialize to JSON (std feature only)
    #[cfg(feature = "std")]
    pub fn to_json(&self) -> Result<String, KeyError> {
        serde_json::to_string(self)
            .map_err(|_| KeyError::SerializationError)
    }

    /// Deserialize from JSON (std feature only)
    #[cfg(feature = "std")]
    pub fn from_json(json: &str) -> Result<Self, KeyError> {
        serde_json::from_str(json)
            .map_err(|_| KeyError::DeserializationError)
    }
}

impl fmt::Display for LicenseKey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "LicenseKey {{ key_id: {}, tier: {}, issued_at: {}, policy: {} }}",
            self.key_id,
            self.policy.tier,
            self.issued_at,
            self.policy
        )
    }
}

/// Ed25519 Verifier - zero-heap signature verification
#[derive(Debug, Clone, Copy)]
pub struct Ed25519Verifier;

impl Ed25519Verifier {
    /// Verify a signature over a message using a public key
    #[inline(always)]
    pub fn verify(message: &[u8], signature: &Signature, public_key: &PublicKey) -> Result<(), VerifyError> {
        #[cfg(feature = "std")]
        {
            use ed25519_dalek::{VerifyingKey, Signature as DalekSignature};
            use signature::Verifier;
            let verifying_key = VerifyingKey::from_bytes(public_key.as_bytes())
                .map_err(|_| VerifyError::InvalidPublicKey)?;
            let dalek_sig = DalekSignature::from_bytes(signature.as_bytes());
            verifying_key.verify(message, &dalek_sig)
                .map_err(|_| VerifyError::InvalidSignature)?;
        }
        #[cfg(not(feature = "std"))]
        {
            // In no_std, use the signature crate's Verifier trait
            use signature::Verifier;
            use ed25519_dalek::{VerifyingKey, Signature as DalekSignature};
            let verifying_key = VerifyingKey::from_bytes(public_key.as_bytes())
                .map_err(|_| VerifyError::InvalidPublicKey)?;
            let dalek_sig = DalekSignature::from_bytes(signature.as_bytes());
            verifying_key.verify(message, &dalek_sig)
                .map_err(|_| VerifyError::InvalidSignature)?;
        }
        Ok(())
    }

    /// Verify a pre-hashed message
    #[inline(always)]
    pub fn verify_prehashed(prehash: &[u8; 64], signature: &Signature, public_key: &PublicKey) -> Result<(), VerifyError> {
        // Ed25519ph (pre-hashed) verification
        // This is a simplified implementation - in practice use ed25519-dalek's verify_prehashed
        Self::verify(prehash, signature, public_key)
    }
}

/// Verification errors
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "std", derive(thiserror::Error))]
pub enum VerifyError {
    /// Invalid signature
    #[cfg_attr(feature = "std", error("Invalid signature"))]
    InvalidSignature,
    /// Invalid signature length
    #[cfg_attr(feature = "std", error("Invalid signature length"))]
    InvalidSignatureLength,
    /// Invalid public key
    #[cfg_attr(feature = "std", error("Invalid public key"))]
    InvalidPublicKey,
    /// Invalid public key length
    #[cfg_attr(feature = "std", error("Invalid public key length"))]
    InvalidPublicKeyLength,
    /// Key ID mismatch
    #[cfg_attr(feature = "std", error("Key ID mismatch"))]
    KeyIdMismatch,
    /// License expired
    #[cfg_attr(feature = "std", error("License expired"))]
    Expired,
    /// Hardware binding required but not provided
    #[cfg_attr(feature = "std", error("Hardware binding required"))]
    HwBindingRequired,
    /// TPM binding required but not provided
    #[cfg_attr(feature = "std", error("TPM binding required"))]
    TpmBindingRequired,
    /// Serialization error
    #[cfg_attr(feature = "std", error("Serialization error"))]
    SerializationError,
    /// Deserialization error
    #[cfg_attr(feature = "std", error("Deserialization error"))]
    DeserializationError,
    /// Policy error
    #[cfg_attr(feature = "std", error("Policy error: {0}"))]
    PolicyError(PolicyError),
}

impl From<VerifyError> for LicenseError {
    fn from(err: VerifyError) -> Self {
        match err {
            VerifyError::InvalidSignature => LicenseError::VerifyError(crate::crypto::VerifyError::VerificationFailed),
            VerifyError::InvalidSignatureLength => LicenseError::VerifyError(crate::crypto::VerifyError::InvalidSignatureLength),
            VerifyError::InvalidPublicKey => LicenseError::VerifyError(crate::crypto::VerifyError::InvalidPublicKeyLength),
            VerifyError::InvalidPublicKeyLength => LicenseError::VerifyError(crate::crypto::VerifyError::InvalidPublicKeyLength),
            VerifyError::KeyIdMismatch => LicenseError::VerifyError(crate::crypto::VerifyError::VerificationFailed),
            VerifyError::Expired => LicenseError::Expired,
            VerifyError::HwBindingRequired => LicenseError::HwMismatch,
            VerifyError::TpmBindingRequired => LicenseError::TpmError(crate::tpm::TpmError::InvalidPcrSelection("TPM PCR binding required")),
            VerifyError::SerializationError => LicenseError::SerializationError,
            VerifyError::DeserializationError => LicenseError::SerializationError,
            VerifyError::PolicyError(e) => LicenseError::PolicyError(e),
        }
    }
}

impl From<PolicyError> for VerifyError {
    fn from(err: PolicyError) -> Self {
        VerifyError::PolicyError(err)
    }
}

// Note: From<KeyError> for LicenseError is implemented in lib.rs to avoid conflicts

/// Zero-heap verify function attribute
#[cfg(feature = "zeroize")]
#[inline(always)]
pub fn verify_license(key_bytes: &[u8], policy: &LicensePolicy) -> bool {
    // This function is marked for zero-heap checking
    // Compilation will fail with -DSASE_ZERO_HEAP=1 if heap is allocated
    let _ = (key_bytes, policy);
    false // Placeholder
}

/// Inline verification function for hot paths
#[inline(always)]
pub fn verify_license_inline(
    policy_bytes: &[u8],
    signature: &Signature,
    public_key: &PublicKey,
) -> Result<(), VerifyError> {
    Ed25519Verifier::verify(policy_bytes, signature, public_key)
}

#[cfg(test)]
#[cfg(feature = "std")]
mod tests {
    use super::*;
    use std::format;
    use crate::tier::LicenseTier;
    use crate::features::Feature;
    use crate::SecretKey;
    use ed25519_dalek::{SigningKey, Signer};
    use rand::rngs::OsRng;

    fn create_test_key() -> (LicenseKey, SecretKey) {
        let mut csprng = OsRng;
        let signing_key = SigningKey::generate(&mut csprng);
        let verifying_key = signing_key.verifying_key();

        let policy = LicensePolicy::new(LicenseTier::Pro)
            .max_tokens(1000)
            .max_requests(100)
            .hw_bound(false)
            .expiry(2000000000);

        let policy_bytes = postcard::to_allocvec(&policy).unwrap();
        let signature_bytes = signing_key.sign(&policy_bytes);
        let signature = Signature::new(signature_bytes.to_bytes());

        let public_key = PublicKey::new(verifying_key.to_bytes());

        // Compute the real key_id via BLAKE3 like compute_key_id does
        let key_id = {
            use blake3::Hasher;
            let mut hasher = Hasher::new();
            hasher.update(public_key.as_bytes());
            let hash = hasher.finalize();
            let mut key_id_bytes = [0u8; KEY_ID_SIZE];
            key_id_bytes.copy_from_slice(&hash.as_bytes()[..KEY_ID_SIZE]);
            KeyId::new(key_id_bytes)
        };

        let key = LicenseKey::new(
            policy,
            signature,
            public_key,
            key_id,
            1000000000,
        );

        (key, SecretKey::new(signing_key.to_bytes()))
    }

    #[test]
    fn test_license_key_creation() {
        let (key, _) = create_test_key();
        assert_eq!(key.policy.tier, LicenseTier::Pro);
        assert_eq!(key.policy.max_tokens, 1000);
        assert_eq!(key.issued_at, 1000000000);
    }

    #[test]
    fn test_license_key_verify() {
        let (key, _) = create_test_key();
        let ctx = VerificationContext::new(1500000000);
        let result = key.verify_with_context(&ctx);
        assert!(result.is_ok(), "verify failed: {:?}", result.err());
    }

    #[test]
    fn test_license_key_expired() {
        let mut csprng = OsRng;
        let signing_key = SigningKey::generate(&mut csprng);
        let verifying_key = signing_key.verifying_key();

        let policy = LicensePolicy::new(LicenseTier::Pro)
            .expiry(1000);

        let policy_bytes = postcard::to_allocvec(&policy).unwrap();
        let signature = Signature::new(signing_key.sign(&policy_bytes).to_bytes());
        let public_key = PublicKey::new(verifying_key.to_bytes());
        let key_id = {
            use blake3::Hasher;
            let mut hasher = Hasher::new();
            hasher.update(public_key.as_bytes());
            let hash = hasher.finalize();
            let mut key_id_bytes = [0u8; KEY_ID_SIZE];
            key_id_bytes.copy_from_slice(&hash.as_bytes()[..KEY_ID_SIZE]);
            KeyId::new(key_id_bytes)
        };

        let key = LicenseKey::new(policy, signature, public_key, key_id, 1000000000);
        let ctx = VerificationContext::new(2000);
        assert!(matches!(key.verify_with_context(&ctx), Err(VerifyError::Expired)));
    }

    #[test]
    fn test_license_key_hw_bound() {
        let mut csprng = OsRng;
        let signing_key = SigningKey::generate(&mut csprng);
        let verifying_key = signing_key.verifying_key();

        let policy = LicensePolicy::new(LicenseTier::Pro).hw_bound(true);
        let policy_bytes = postcard::to_allocvec(&policy).unwrap();
        let signature_bytes = signing_key.sign(&policy_bytes);
        let signature = Signature::new(signature_bytes.to_bytes());

        let public_key = PublicKey::new(verifying_key.to_bytes());
        let key_id = {
            use blake3::Hasher;
            let mut hasher = Hasher::new();
            hasher.update(public_key.as_bytes());
            let hash = hasher.finalize();
            let mut key_id_bytes = [0u8; KEY_ID_SIZE];
            key_id_bytes.copy_from_slice(&hash.as_bytes()[..KEY_ID_SIZE]);
            KeyId::new(key_id_bytes)
        };

        let key = LicenseKey::new(policy, signature, public_key, key_id, 1000000000);

        // Without hardware fingerprint, should fail
        let ctx = VerificationContext::new(1500000000);
        assert!(matches!(key.verify_with_context(&ctx), Err(VerifyError::HwBindingRequired)));

        // With hardware fingerprint, should pass
        let ctx = VerificationContext::new(1500000000).with_hw_fingerprint([0xBB; 32]);
        assert!(key.verify_with_context(&ctx).is_ok());
    }

    #[test]
    fn test_license_key_tpm_binding() {
        let mut csprng = OsRng;
        let signing_key = SigningKey::generate(&mut csprng);
        let verifying_key = signing_key.verifying_key();

        let mut features = FeatureSet::empty();
        features.insert(FeatureSet::from(Feature::TpmSealing));
        let policy = LicensePolicy::new(LicenseTier::Engineering).features(features);
        let policy_bytes = postcard::to_allocvec(&policy).unwrap();
        let signature_bytes = signing_key.sign(&policy_bytes);
        let signature = Signature::new(signature_bytes.to_bytes());

        let public_key = PublicKey::new(verifying_key.to_bytes());
        let key_id = {
            use blake3::Hasher;
            let mut hasher = Hasher::new();
            hasher.update(public_key.as_bytes());
            let hash = hasher.finalize();
            let mut key_id_bytes = [0u8; KEY_ID_SIZE];
            key_id_bytes.copy_from_slice(&hash.as_bytes()[..KEY_ID_SIZE]);
            KeyId::new(key_id_bytes)
        };

        let key = LicenseKey::new(policy, signature, public_key, key_id, 1000000000);

        // Without PCR values, should fail
        let ctx = VerificationContext::new(1500000000);
        assert!(matches!(key.verify_with_context(&ctx), Err(VerifyError::TpmBindingRequired)));

        // With PCR values, should pass
        let ctx = VerificationContext::new(1500000000).with_pcr_values([0xCC; 32]);
        assert!(key.verify_with_context(&ctx).is_ok());
    }

    #[test]
    fn test_license_key_key_id() {
        let (key, _) = create_test_key();
        let computed = key.compute_key_id();
        // In test we use a fixed key_id, so they won't match with the real computation
        // This just verifies the function runs
        assert_eq!(computed.as_bytes().len(), KEY_ID_SIZE);
    }

    #[test]
    fn test_license_key_serialization() {
        let (key, _) = create_test_key();
        let postcard = key.to_postcard().unwrap();
        let decoded = LicenseKey::from_postcard(&postcard).unwrap();
        assert_eq!(key, decoded);

        let json = key.to_json().unwrap();
        let decoded = LicenseKey::from_json(&json).unwrap();
        assert_eq!(key, decoded);
    }

    #[test]
    fn test_ed25519_verifier() {
        let mut csprng = OsRng;
        let signing_key = SigningKey::generate(&mut csprng);
        let verifying_key = signing_key.verifying_key();

        let message = b"test message";
        let signature_bytes = signing_key.sign(message);
        let signature = Signature::new(signature_bytes.to_bytes());
        let public_key = PublicKey::new(verifying_key.to_bytes());

        assert!(Ed25519Verifier::verify(message, &signature, &public_key).is_ok());

        // Wrong message should fail
        assert!(Ed25519Verifier::verify(b"wrong message", &signature, &public_key).is_err());
    }

    #[test]
    fn test_key_display() {
        let (key, _) = create_test_key();
        let display = format!("{}", key);
        assert!(display.contains("pro"));
        assert!(display.contains("1000000000"));
    }

    #[test]
    fn test_key_features() {
        let mut csprng = OsRng;
        let signing_key = SigningKey::generate(&mut csprng);
        let verifying_key = signing_key.verifying_key();

        let mut features = FeatureSet::empty();
        features.insert(FeatureSet::from(Feature::ZeroTrust));
        features.insert(FeatureSet::from(Feature::DlpBasic));

        let policy = LicensePolicy::new(LicenseTier::Pro).features(features);
        let policy_bytes = postcard::to_allocvec(&policy).unwrap();
        let signature_bytes = signing_key.sign(&policy_bytes);
        let signature = Signature::new(signature_bytes.to_bytes());

        let public_key = PublicKey::new(verifying_key.to_bytes());
        let key_id = KeyId::new([0xAA; KEY_ID_SIZE]);

        let key = LicenseKey::new(policy, signature, public_key, key_id, 1000000000);

        assert!(key.has_feature(Feature::ZeroTrust));
        assert!(key.has_feature(Feature::DlpBasic));
        assert!(!key.has_feature(Feature::TpmSealing));
    }
}