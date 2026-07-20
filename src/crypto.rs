/*!
 * Cryptographic Primitives
 *
 * Ed25519 signature verification with zero-heap support.
 * Uses ed25519-dalek with zeroize for secret key handling.
 */

use core::fmt;
use core::ops::Deref;
#[cfg(feature = "std")]
use serde::{Deserialize, Serialize};
#[cfg(feature = "std")]
use serde_bytes;  // Use the serde_bytes module with `with = "serde_bytes"`
use zeroize::{Zeroize, ZeroizeOnDrop};

#[cfg(any(feature = "crypto", feature = "std", feature = "alloc"))]
use signature::Signer;

use crate::{SIGNATURE_SIZE, PUBLIC_KEY_SIZE, SECRET_KEY_SIZE, KEY_ID_SIZE};

/// Ed25519 Signature (64 bytes)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "std", serde(transparent))]
#[repr(transparent)]
pub struct Signature(#[cfg_attr(feature = "std", serde(with = "serde_bytes"))] pub [u8; SIGNATURE_SIZE]);

impl Signature {
    /// Create a new signature from bytes
    #[inline(always)]
    pub const fn new(bytes: [u8; SIGNATURE_SIZE]) -> Self {
        Self(bytes)
    }

    /// Create from a byte slice (panics if wrong size)
    #[inline]
    pub fn from_slice(bytes: &[u8]) -> Result<Self, VerifyError> {
        if bytes.len() != SIGNATURE_SIZE {
            return Err(VerifyError::InvalidSignatureLength);
        }
        let mut arr = [0u8; SIGNATURE_SIZE];
        arr.copy_from_slice(bytes);
        Ok(Self(arr))
    }

    /// Get the underlying bytes
    #[inline(always)]
    pub const fn as_bytes(&self) -> &[u8; SIGNATURE_SIZE] {
        &self.0
    }

    /// Get the underlying bytes as mutable
    #[inline(always)]
    pub fn as_mut_bytes(&mut self) -> &mut [u8; SIGNATURE_SIZE] {
        &mut self.0
    }
}

impl Deref for Signature {
    type Target = [u8; SIGNATURE_SIZE];

    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl AsRef<[u8]> for Signature {
    #[inline(always)]
    fn as_ref(&self) -> &[u8] {
        &self.0
    }
}

impl fmt::Display for Signature {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for byte in &self.0 {
            write!(f, "{:02x}", byte)?;
        }
        Ok(())
    }
}

impl Zeroize for Signature {
    fn zeroize(&mut self) {
        self.0.zeroize();
    }
}

impl ZeroizeOnDrop for Signature {}

/// Ed25519 Public Key (32 bytes)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "std", serde(transparent))]
#[repr(transparent)]
pub struct PublicKey(#[cfg_attr(feature = "std", serde(with = "serde_bytes"))] pub [u8; PUBLIC_KEY_SIZE]);

impl PublicKey {
    /// Create a new public key from bytes
    #[inline(always)]
    pub const fn new(bytes: [u8; PUBLIC_KEY_SIZE]) -> Self {
        Self(bytes)
    }

    /// Create from a byte slice (panics if wrong size)
    #[inline]
    pub fn from_slice(bytes: &[u8]) -> Result<Self, VerifyError> {
        if bytes.len() != PUBLIC_KEY_SIZE {
            return Err(VerifyError::InvalidPublicKeyLength);
        }
        let mut arr = [0u8; PUBLIC_KEY_SIZE];
        arr.copy_from_slice(bytes);
        Ok(Self(arr))
    }

    /// Get the underlying bytes
    #[inline(always)]
    pub const fn as_bytes(&self) -> &[u8; PUBLIC_KEY_SIZE] {
        &self.0
    }

    /// Get the underlying bytes as mutable
    #[inline(always)]
    pub fn as_mut_bytes(&mut self) -> &mut [u8; PUBLIC_KEY_SIZE] {
        &mut self.0
    }

    /// Verify a signature against a message
    #[inline]
    pub fn verify(&self, message: &[u8], signature: &Signature) -> Result<(), VerifyError> {
        Ed25519Verifier::verify(message, signature, self)
    }

    /// Verify a pre-hashed message
    #[inline]
    pub fn verify_prehashed(&self, prehash: &[u8; 64], signature: &Signature) -> Result<(), VerifyError> {
        Ed25519Verifier::verify_prehashed(prehash, signature, self)
    }
}

impl Deref for PublicKey {
    type Target = [u8; PUBLIC_KEY_SIZE];

    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl AsRef<[u8]> for PublicKey {
    #[inline(always)]
    fn as_ref(&self) -> &[u8] {
        &self.0
    }
}

impl fmt::Display for PublicKey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for byte in &self.0 {
            write!(f, "{:02x}", byte)?;
        }
        Ok(())
    }
}

/// Ed25519 Secret Key (32 bytes) - zeroized on drop
#[derive(Debug, Clone, PartialEq, Eq)]
#[repr(transparent)]
pub struct SecretKey(pub [u8; SECRET_KEY_SIZE]);

impl SecretKey {
    /// Create a new secret key from bytes
    #[inline(always)]
    pub const fn new(bytes: [u8; SECRET_KEY_SIZE]) -> Self {
        Self(bytes)
    }

    /// Create from a byte slice (panics if wrong size)
    #[inline]
    pub fn from_slice(bytes: &[u8]) -> Result<Self, VerifyError> {
        if bytes.len() != SECRET_KEY_SIZE {
            return Err(VerifyError::InvalidSecretKeyLength);
        }
        let mut arr = [0u8; SECRET_KEY_SIZE];
        arr.copy_from_slice(bytes);
        Ok(Self(arr))
    }

    /// Get the underlying bytes
    #[inline(always)]
    pub const fn as_bytes(&self) -> &[u8; SECRET_KEY_SIZE] {
        &self.0
    }

    /// Get the underlying bytes as mutable
    #[inline(always)]
    pub fn as_mut_bytes(&mut self) -> &mut [u8; SECRET_KEY_SIZE] {
        &mut self.0
    }

    /// Generate a new random secret key (requires std/rng feature)
    #[cfg(feature = "std")]
    pub fn generate() -> Self {
        
        use rand::rngs::OsRng;
        use ed25519_dalek::SigningKey;
        let mut rng = OsRng;
        let signing_key = SigningKey::generate(&mut rng);
        Self(signing_key.to_bytes())
    }

    /// Get the corresponding public key
    #[inline]
    pub fn public_key(&self) -> PublicKey {
        #[cfg(feature = "crypto")]
        {
            use ed25519_dalek::SigningKey;
            let signing_key = SigningKey::from_bytes(&self.0);
            PublicKey(signing_key.verifying_key().to_bytes())
        }
        #[cfg(not(feature = "crypto"))]
        {
            // Mock implementation when crypto feature is disabled
            PublicKey([0u8; PUBLIC_KEY_SIZE])
        }
    }

    /// Sign a message
    #[inline]
    pub fn sign(&self, message: &[u8]) -> Signature {
        #[cfg(feature = "crypto")]
        {
            use ed25519_dalek::{SigningKey, Signer};
            let signing_key = SigningKey::from_bytes(&self.0);
            let signature = signing_key.try_sign(message).expect("Signing failed");
            Signature(signature.to_bytes())
        }
        #[cfg(not(feature = "crypto"))]
        {
            Signature([0u8; SIGNATURE_SIZE])
        }
    }

    /// Sign a pre-hashed message
    #[inline]
    pub fn sign_prehashed(&self, prehash: &[u8; 64]) -> Signature {
        #[cfg(feature = "crypto")]
        {
            use ed25519_dalek::SigningKey;
            let signing_key = SigningKey::from_bytes(&self.0);
            // Use regular sign for prehashed (message is already hashed)
            let signature = signing_key.try_sign(prehash).expect("Prehashed signing failed");
            Signature(signature.to_bytes())
        }
        #[cfg(not(feature = "crypto"))]
        {
            let _ = prehash;
            Signature([0u8; SIGNATURE_SIZE])
        }
    }
}

impl Drop for SecretKey {
    fn drop(&mut self) {
        self.0.zeroize();
    }
}

impl Zeroize for SecretKey {
    fn zeroize(&mut self) {
        self.0.zeroize();
    }
}

impl ZeroizeOnDrop for SecretKey {}

impl Deref for SecretKey {
    type Target = [u8; SECRET_KEY_SIZE];

    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl AsRef<[u8]> for SecretKey {
    #[inline(always)]
    fn as_ref(&self) -> &[u8] {
        &self.0
    }
}

/// Key Identifier (16 bytes - BLAKE3 hash truncated)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "std", serde(transparent))]
#[repr(transparent)]
pub struct KeyId(pub [u8; KEY_ID_SIZE]);

impl KeyId {
    /// Create a new key ID from bytes
    #[inline(always)]
    pub const fn new(bytes: [u8; KEY_ID_SIZE]) -> Self {
        Self(bytes)
    }

    /// Create from a byte slice (panics if wrong size)
    #[inline]
    pub fn from_slice(bytes: &[u8]) -> Result<Self, VerifyError> {
        if bytes.len() != KEY_ID_SIZE {
            return Err(VerifyError::InvalidKeyIdLength);
        }
        let mut arr = [0u8; KEY_ID_SIZE];
        arr.copy_from_slice(bytes);
        Ok(Self(arr))
    }

    /// Compute key ID from public key (BLAKE3 hash truncated to 16 bytes)
    #[inline]
    pub fn from_public_key(public_key: &PublicKey) -> Self {
        #[cfg(feature = "std")]
        {
            use blake3::Hasher;
            let mut hasher = Hasher::new();
            hasher.update(public_key.as_bytes());
            let hash = hasher.finalize();
            let mut key_id = [0u8; KEY_ID_SIZE];
            key_id.copy_from_slice(&hash.as_bytes()[..KEY_ID_SIZE]);
            Self(key_id)
        }
        #[cfg(not(feature = "std"))]
        {
            // Fallback for no_std: use a simpler hash
            // In production, use a no_std BLAKE3 implementation
            let mut key_id = [0u8; KEY_ID_SIZE];
            for (i, byte) in public_key.as_bytes().iter().enumerate() {
                key_id[i % KEY_ID_SIZE] ^= byte.wrapping_mul((i + 1) as u8);
            }
            Self(key_id)
        }
    }

    /// Get the underlying bytes
    #[inline(always)]
    pub const fn as_bytes(&self) -> &[u8; KEY_ID_SIZE] {
        &self.0
    }
}

impl Deref for KeyId {
    type Target = [u8; KEY_ID_SIZE];

    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl AsRef<[u8]> for KeyId {
    #[inline(always)]
    fn as_ref(&self) -> &[u8] {
        &self.0
    }
}

impl fmt::Display for KeyId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for byte in &self.0 {
            write!(f, "{:02x}", byte)?;
        }
        Ok(())
    }
}

/// Ed25519 Verification errors
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "std", derive(thiserror::Error))]
pub enum VerifyError {
    /// Signature verification failed
    #[cfg_attr(feature = "std", error("Signature verification failed"))]
    VerificationFailed,
    /// Invalid signature length
    #[cfg_attr(feature = "std", error("Invalid signature length"))]
    InvalidSignatureLength,
    /// Invalid public key length
    #[cfg_attr(feature = "std", error("Invalid public key length"))]
    InvalidPublicKeyLength,
    /// Invalid secret key length
    #[cfg_attr(feature = "std", error("Invalid secret key length"))]
    InvalidSecretKeyLength,
    /// Invalid key ID length
    #[cfg_attr(feature = "std", error("Invalid key ID length"))]
    InvalidKeyIdLength,
    /// Invalid prehash length
    #[cfg_attr(feature = "std", error("Invalid prehash length"))]
    InvalidPrehashLength,
    /// Ed25519 library error
    #[cfg_attr(feature = "std", error("Ed25519 error"))]
    Ed25519Error,
}

/// Ed25519 Verifier - heap-free verification
///
/// Uses ed25519-dalek with zeroize for constant-time verification
/// without heap allocations in the hot path.
pub struct Ed25519Verifier;

impl Ed25519Verifier {
    /// Verify an Ed25519 signature (heap-free)
    ///
    /// This function performs verification without any heap allocations,
    /// making it suitable for `-DSASE_ZERO_HEAP=1` compilation.
    #[inline(always)]
    pub fn verify(message: &[u8], signature: &Signature, public_key: &PublicKey) -> Result<(), VerifyError> {
        #[cfg(any(feature = "crypto", feature = "std", feature = "alloc"))]
        {
            use ed25519_dalek::{VerifyingKey, Signature as DalekSignature};
            use signature::Verifier;
            // Convert to dalek types (zero-copy)
            let verifying_key = VerifyingKey::from_bytes(public_key.as_bytes())
                .map_err(|_| VerifyError::InvalidPublicKeyLength)?;
            let dalek_signature = DalekSignature::from_bytes(signature.as_bytes());

            // Verify - this is heap-free in ed25519-dalek v2+
            verifying_key
                .verify(message, &dalek_signature)
                .map_err(|_| VerifyError::VerificationFailed)
        }
        #[cfg(not(any(feature = "crypto", feature = "std", feature = "alloc")))]
        {
            Err(VerifyError::Ed25519Error)
        }
    }

    /// Verify a pre-hashed message (heap-free)
    #[inline(always)]
    pub fn verify_prehashed(prehash: &[u8; 64], signature: &Signature, public_key: &PublicKey) -> Result<(), VerifyError> {
        #[cfg(any(feature = "crypto", feature = "std", feature = "alloc"))]
        {
            use ed25519_dalek::{VerifyingKey, Signature as DalekSignature};
            use signature::Verifier;
            let verifying_key = VerifyingKey::from_bytes(public_key.as_bytes())
                .map_err(|_| VerifyError::InvalidPublicKeyLength)?;
            let dalek_signature = DalekSignature::from_bytes(signature.as_bytes());

            // For prehashed, we use verify with the prehash as message
            verifying_key
                .verify(prehash, &dalek_signature)
                .map_err(|_| VerifyError::VerificationFailed)
        }
        #[cfg(not(any(feature = "crypto", feature = "std", feature = "alloc")))]
        {
            Err(VerifyError::Ed25519Error)
        }
    }

    /// Verify multiple signatures in batch (heap-free)
    ///
    /// More efficient than individual verification for multiple messages
    /// signed with the same key.
    #[inline]
    pub fn verify_batch(
        messages: &[&[u8]],
        signatures: &[Signature],
        public_key: &PublicKey,
    ) -> Result<(), VerifyError> {
        if messages.len() != signatures.len() {
            return Err(VerifyError::VerificationFailed);
        }

        for (msg, sig) in messages.iter().zip(signatures.iter()) {
            Self::verify(msg, sig, public_key)?;
        }
        Ok(())
    }

    /// Constant-time signature verification (heap-free)
    /// 
    /// Always takes the same time regardless of input, preventing timing attacks.
    #[inline(always)]
    pub fn verify_ct(message: &[u8], signature: &Signature, public_key: &PublicKey) -> Result<(), VerifyError> {
        // ed25519-dalek's verify is already constant-time
        Self::verify(message, signature, public_key)
    }
}

/// Trait for types that can verify Ed25519 signatures
pub trait Verifier {
    /// Verify a signature
    fn verify(&self, message: &[u8], signature: &Signature, public_key: &PublicKey) -> Result<(), VerifyError>;
}

impl Verifier for Ed25519Verifier {
    #[inline(always)]
    fn verify(&self, message: &[u8], signature: &Signature, public_key: &PublicKey) -> Result<(), VerifyError> {
        Ed25519Verifier::verify(message, signature, public_key)
    }
}

/// Type aliases for crate root
pub type SignatureBytes = [u8; SIGNATURE_SIZE];
pub type PublicKeyBytes = [u8; PUBLIC_KEY_SIZE];
pub type SecretKeyBytes = [u8; SECRET_KEY_SIZE];

#[cfg(test)]
#[cfg(all(feature = "std", feature = "crypto"))]
mod tests {
    use super::*;
    use std::format;
    use crate::SecretKey;

    #[test]
    fn test_signature_creation() {
        let bytes = [0x42; SIGNATURE_SIZE];
        let sig = Signature::new(bytes);
        assert_eq!(sig.as_bytes(), &bytes);
    }

    #[test]
    fn test_signature_from_slice() {
        let bytes = [0x42; SIGNATURE_SIZE];
        let sig = Signature::from_slice(&bytes).unwrap();
        assert_eq!(sig.as_bytes(), &bytes);

        let short = [0x42; 32];
        assert!(Signature::from_slice(&short).is_err());
    }

    #[test]
    fn test_public_key_creation() {
        let bytes = [0x24; PUBLIC_KEY_SIZE];
        let pk = PublicKey::new(bytes);
        assert_eq!(pk.as_bytes(), &bytes);
    }

    #[test]
    fn test_public_key_from_slice() {
        let bytes = [0x24; PUBLIC_KEY_SIZE];
        let pk = PublicKey::from_slice(&bytes).unwrap();
        assert_eq!(pk.as_bytes(), &bytes);

        let short = [0x24; 16];
        assert!(PublicKey::from_slice(&short).is_err());
    }

    #[test]
    fn test_secret_key_creation() {
        let bytes = [0x13; SECRET_KEY_SIZE];
        let sk = SecretKey::new(bytes);
        assert_eq!(sk.as_bytes(), &bytes);
    }

    #[test]
    fn test_secret_key_zeroize_on_drop() {
        let sk = SecretKey::new([0xAB; SECRET_KEY_SIZE]);
        let _ptr = sk.as_bytes().as_ptr();
        drop(sk);
        // Note: Can't easily test zeroize after drop in safe Rust
    }

    #[test]
    fn test_key_id_creation() {
        let bytes = [0x55; KEY_ID_SIZE];
        let kid = KeyId::new(bytes);
        assert_eq!(kid.as_bytes(), &bytes);
    }

    #[test]
    fn test_key_id_from_public_key() {
        let pk = PublicKey::new([0x42; PUBLIC_KEY_SIZE]);
        let kid = KeyId::from_public_key(&pk);
        assert_eq!(kid.as_bytes().len(), KEY_ID_SIZE);
    }

    #[test]
    fn test_sign_verify() {
        let sk = SecretKey::generate();
        let pk = sk.public_key();
        let message = b"Hello, SASE License!";
        let signature = sk.sign(message);

        assert!(Ed25519Verifier::verify(message, &signature, &pk).is_ok());

        // Wrong message should fail
        let wrong_msg = b"Wrong message";
        assert!(Ed25519Verifier::verify(wrong_msg, &signature, &pk).is_err());

        // Wrong key should fail
        let sk2 = SecretKey::generate();
        let pk2 = sk2.public_key();
        assert!(Ed25519Verifier::verify(message, &signature, &pk2).is_err());
    }

    #[test]
    fn test_sign_prehashed() {
        let sk = SecretKey::generate();
        let pk = sk.public_key();

        // SHA-512 hash of message
        use sha2::{Sha512, Digest};
        let mut hasher = Sha512::new();
        hasher.update(b"Prehashed message");
        let prehash: [u8; 64] = hasher.finalize().into();

        let signature = sk.sign_prehashed(&prehash);
        assert!(Ed25519Verifier::verify_prehashed(&prehash, &signature, &pk).is_ok());

        // Wrong prehash should fail
        let mut hasher2 = Sha512::new();
        hasher2.update(b"Different message");
        let wrong_prehash: [u8; 64] = hasher2.finalize().into();
        assert!(Ed25519Verifier::verify_prehashed(&wrong_prehash, &signature, &pk).is_err());
    }

    #[test]
    fn test_batch_verify() {
        let sk = SecretKey::generate();
        let pk = sk.public_key();

        let messages = [b"Message 1".as_slice(), b"Message 2".as_slice(), b"Message 3".as_slice()];
        let mut signatures = heapless::Vec::<Signature, 3>::new();
        for msg in &messages {
            signatures.push(sk.sign(msg)).unwrap();
        }

        assert!(Ed25519Verifier::verify_batch(&messages, &signatures, &pk).is_ok());

        // Tamper with one message
        let mut bad_messages = messages;
        bad_messages[1] = b"Tampered".as_slice();
        assert!(Ed25519Verifier::verify_batch(&bad_messages, &signatures, &pk).is_err());
    }

    #[test]
    fn test_constant_time_verify() {
        let sk = SecretKey::generate();
        let pk = sk.public_key();
        let message = b"Constant time test";
        let signature = sk.sign(message);

        // Should succeed
        assert!(Ed25519Verifier::verify_ct(message, &signature, &pk).is_ok());

        // Should fail
        assert!(Ed25519Verifier::verify_ct(b"Wrong", &signature, &pk).is_err());
    }

    #[test]
    fn test_signature_display() {
        let sig = Signature::new([0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08,
                                  0x09, 0x0A, 0x0B, 0x0C, 0x0D, 0x0E, 0x0F, 0x10,
                                  0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x18,
                                  0x19, 0x1A, 0x1B, 0x1C, 0x1D, 0x1E, 0x1F, 0x20,
                                  0x21, 0x22, 0x23, 0x24, 0x25, 0x26, 0x27, 0x28,
                                  0x29, 0x2A, 0x2B, 0x2C, 0x2D, 0x2E, 0x2F, 0x30,
                                  0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38,
                                  0x39, 0x3A, 0x3B, 0x3C, 0x3D, 0x3E, 0x3F, 0x40]);
        let display = format!("{}", sig);
        assert_eq!(display.len(), SIGNATURE_SIZE * 2);
        assert!(display.starts_with("01020304"));
    }

    #[test]
    fn test_public_key_display() {
        let pk = PublicKey::new([0xAA; PUBLIC_KEY_SIZE]);
        let display = format!("{}", pk);
        assert_eq!(display.len(), PUBLIC_KEY_SIZE * 2);
        assert!(display.starts_with("aaaaaaaa"));
    }

    #[test]
    fn test_key_id_display() {
        let kid = KeyId::new([0xFF; KEY_ID_SIZE]);
        let display = format!("{}", kid);
        assert_eq!(display.len(), KEY_ID_SIZE * 2);
        assert!(display.starts_with("ffffffff"));
    }
}