/*!
 * Feature Set Definitions
 *
 * Bitflags-based feature set for SASE licensing.
 */

use core::fmt;
#[cfg(any(feature = "std", feature = "alloc"))]
use serde::{Deserialize, Serialize};

bitflags::bitflags! {
    /// SASE License Features as bitflags
    ///
    /// Each feature represents a specific capability in the SASE platform.
    /// Features are organized hierarchically by tier.
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    pub struct FeatureSet: u64 {
        // SASE Core Features
        /// Basic SASE access (Free tier)
        const SaseBasic         = 1 << 0;
        /// Advanced SASE features (Base tier+)
        const SaseAdvanced      = 1 << 1;
        /// Enterprise SASE features (Pro tier+)
        const SaseEnterprise    = 1 << 2;

        // VPN Features
        /// Basic VPN connectivity (Base tier+)
        const VpnBasic          = 1 << 3;
        /// Advanced VPN features (Pro tier+)
        const VpnAdvanced       = 1 << 4;
        /// SD-WAN integration (Engineering tier+)
        const SdWan             = 1 << 5;
        /// Multi-cloud networking (Engineering tier+)
        const MultiCloud        = 1 << 6;

        // Zero Trust Features
        /// Zero Trust Network Access (Pro tier+)
        const ZeroTrust         = 1 << 7;
        /// Identity-based access (Pro tier+)
        const IdentityAccess    = 1 << 8;
        /// Device posture checking (Engineering tier+)
        const DevicePosture     = 1 << 9;
        /// Continuous verification (Engineering tier+)
        const ContinuousVerify  = 1 << 10;

        // DLP Features
        /// Basic DLP (Pro tier+)
        const DlpBasic          = 1 << 11;
        /// Advanced DLP (Engineering tier+)
        const DlpAdvanced       = 1 << 12;
        /// Custom DLP policies (Engineering tier+)
        const DlpCustom         = 1 << 13;
        /// ML-based DLP (Government tier+)
        const DlpMl             = 1 << 14;

        // API & Integration
        /// API access (Engineering tier+)
        const ApiAccess         = 1 << 15;
        /// Custom policies (Engineering tier+)
        const CustomPolicies    = 1 << 16;
        /// Webhook integrations (Engineering tier+)
        const Webhooks          = 1 << 17;
        /// SCIM provisioning (Sovereign tier+)
        const Scim              = 1 << 18;

        // Data Sovereignty
        /// Data sovereignty controls (Sovereign tier+)
        const DataSovereignty   = 1 << 19;
        /// Geo-fencing (Sovereign tier+)
        const GeoFencing        = 1 << 20;
        /// Data residency (Sovereign tier+)
        const DataResidency     = 1 << 21;
        /// Cross-border controls (Sovereign tier+)
        const CrossBorder       = 1 << 22;

        // Government/Compliance
        /// Compliance audit logging (Government tier)
        const ComplianceAudit   = 1 << 23;
        /// FIPS 140-2 mode (Government tier)
        const FipsMode          = 1 << 24;
        /// FedRAMP compliance (Government tier)
        const Fedramp           = 1 << 25;
        /// Classified data handling (Government tier)
        const ClassifiedData    = 1 << 26;

        // Advanced Security
        /// Threat intelligence (Pro tier+)
        const ThreatIntel       = 1 << 27;
        /// Sandbox analysis (Engineering tier+)
        const Sandbox           = 1 << 28;
        /// CASB integration (Engineering tier+)
        const Casb              = 1 << 29;
        /// RBI (Remote Browser Isolation) (Engineering tier+)
        const Rbi               = 1 << 30;

        // Hardware/TPM
        /// Hardware binding support (Pro tier+)
        const HwBinding         = 1 << 31;
        /// TPM sealing (Engineering tier+)
        const TpmSealing        = 1 << 32;
        /// HSM integration (Sovereign tier+)
        const HsmIntegration    = 1 << 33;
        /// Secure enclave (Government tier)
        const SecureEnclave     = 1 << 34;

        // Reserved for future use (bits 35-63)
        const Reserved35        = 1 << 35;
        const Reserved36        = 1 << 36;
        const Reserved37        = 1 << 37;
        const Reserved38        = 1 << 38;
        const Reserved39        = 1 << 39;
        const Reserved40        = 1 << 40;
        const Reserved41        = 1 << 41;
        const Reserved42        = 1 << 42;
        const Reserved43        = 1 << 43;
        const Reserved44        = 1 << 44;
        const Reserved45        = 1 << 45;
        const Reserved46        = 1 << 46;
        const Reserved47        = 1 << 47;
        const Reserved48        = 1 << 48;
        const Reserved49        = 1 << 49;
        const Reserved50        = 1 << 50;
        const Reserved51        = 1 << 51;
        const Reserved52        = 1 << 52;
        const Reserved53        = 1 << 53;
        const Reserved54        = 1 << 54;
        const Reserved55        = 1 << 55;
        const Reserved56        = 1 << 56;
        const Reserved57        = 1 << 57;
        const Reserved58        = 1 << 58;
        const Reserved59        = 1 << 59;
        const Reserved60        = 1 << 60;
        const Reserved61        = 1 << 61;
        const Reserved62        = 1 << 62;
        const Reserved63        = 1 << 63;
    }
}

/// Individual feature enum for iteration and matching
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(any(feature = "std", feature = "alloc"), derive(Serialize, Deserialize))]
#[repr(u8)]
pub enum Feature {
    SaseBasic         = 0,
    SaseAdvanced      = 1,
    SaseEnterprise    = 2,
    VpnBasic          = 3,
    VpnAdvanced       = 4,
    SdWan             = 5,
    MultiCloud        = 6,
    ZeroTrust         = 7,
    IdentityAccess    = 8,
    DevicePosture     = 9,
    ContinuousVerify  = 10,
    DlpBasic          = 11,
    DlpAdvanced       = 12,
    DlpCustom         = 13,
    DlpMl             = 14,
    ApiAccess         = 15,
    CustomPolicies    = 16,
    Webhooks          = 17,
    Scim              = 18,
    DataSovereignty   = 19,
    GeoFencing        = 20,
    DataResidency     = 21,
    CrossBorder       = 22,
    ComplianceAudit   = 23,
    FipsMode          = 24,
    Fedramp           = 25,
    ClassifiedData    = 26,
    ThreatIntel       = 27,
    Sandbox           = 28,
    Casb              = 29,
    Rbi               = 30,
    HwBinding         = 31,
    TpmSealing        = 32,
    HsmIntegration    = 33,
    SecureEnclave     = 34,
}

impl Feature {
    /// Get the FeatureSet bit for this feature
    #[inline(always)]
    pub const fn bit(self) -> FeatureSet {
        FeatureSet::from_bits_retain(1u64 << (self as u64))
    }

    /// Get the feature name as a static string
    #[inline(always)]
    pub const fn name(self) -> &'static str {
        match self {
            Feature::SaseBasic => "sase_basic",
            Feature::SaseAdvanced => "sase_advanced",
            Feature::SaseEnterprise => "sase_enterprise",
            Feature::VpnBasic => "vpn_basic",
            Feature::VpnAdvanced => "vpn_advanced",
            Feature::SdWan => "sd_wan",
            Feature::MultiCloud => "multi_cloud",
            Feature::ZeroTrust => "zero_trust",
            Feature::IdentityAccess => "identity_access",
            Feature::DevicePosture => "device_posture",
            Feature::ContinuousVerify => "continuous_verify",
            Feature::DlpBasic => "dlp_basic",
            Feature::DlpAdvanced => "dlp_advanced",
            Feature::DlpCustom => "dlp_custom",
            Feature::DlpMl => "dlp_ml",
            Feature::ApiAccess => "api_access",
            Feature::CustomPolicies => "custom_policies",
            Feature::Webhooks => "webhooks",
            Feature::Scim => "scim",
            Feature::DataSovereignty => "data_sovereignty",
            Feature::GeoFencing => "geo_fencing",
            Feature::DataResidency => "data_residency",
            Feature::CrossBorder => "cross_border",
            Feature::ComplianceAudit => "compliance_audit",
            Feature::FipsMode => "fips_mode",
            Feature::Fedramp => "fedramp",
            Feature::ClassifiedData => "classified_data",
            Feature::ThreatIntel => "threat_intel",
            Feature::Sandbox => "sandbox",
            Feature::Casb => "casb",
            Feature::Rbi => "rbi",
            Feature::HwBinding => "hw_binding",
            Feature::TpmSealing => "tpm_sealing",
            Feature::HsmIntegration => "hsm_integration",
            Feature::SecureEnclave => "secure_enclave",
        }
    }

    /// Get the display name
    #[inline(always)]
    pub const fn display_name(self) -> &'static str {
        match self {
            Feature::SaseBasic => "SASE Basic",
            Feature::SaseAdvanced => "SASE Advanced",
            Feature::SaseEnterprise => "SASE Enterprise",
            Feature::VpnBasic => "VPN Basic",
            Feature::VpnAdvanced => "VPN Advanced",
            Feature::SdWan => "SD-WAN",
            Feature::MultiCloud => "Multi-Cloud",
            Feature::ZeroTrust => "Zero Trust",
            Feature::IdentityAccess => "Identity Access",
            Feature::DevicePosture => "Device Posture",
            Feature::ContinuousVerify => "Continuous Verification",
            Feature::DlpBasic => "DLP Basic",
            Feature::DlpAdvanced => "DLP Advanced",
            Feature::DlpCustom => "DLP Custom",
            Feature::DlpMl => "DLP ML",
            Feature::ApiAccess => "API Access",
            Feature::CustomPolicies => "Custom Policies",
            Feature::Webhooks => "Webhooks",
            Feature::Scim => "SCIM",
            Feature::DataSovereignty => "Data Sovereignty",
            Feature::GeoFencing => "Geo-Fencing",
            Feature::DataResidency => "Data Residency",
            Feature::CrossBorder => "Cross-Border",
            Feature::ComplianceAudit => "Compliance Audit",
            Feature::FipsMode => "FIPS Mode",
            Feature::Fedramp => "FedRAMP",
            Feature::ClassifiedData => "Classified Data",
            Feature::ThreatIntel => "Threat Intelligence",
            Feature::Sandbox => "Sandbox",
            Feature::Casb => "CASB",
            Feature::Rbi => "RBI",
            Feature::HwBinding => "Hardware Binding",
            Feature::TpmSealing => "TPM Sealing",
            Feature::HsmIntegration => "HSM Integration",
            Feature::SecureEnclave => "Secure Enclave",
        }
    }

    /// Get the minimum tier required for this feature
    #[inline(always)]
    pub const fn min_tier(self) -> crate::tier::LicenseTier {
        use crate::tier::LicenseTier;
        match self {
            Feature::SaseBasic => LicenseTier::Free,
            Feature::SaseAdvanced | Feature::VpnBasic => LicenseTier::Base,
            Feature::SaseEnterprise | Feature::VpnAdvanced | Feature::ZeroTrust | Feature::IdentityAccess | Feature::DlpBasic | Feature::ThreatIntel => LicenseTier::Pro,
            Feature::SdWan | Feature::MultiCloud | Feature::DevicePosture | Feature::ContinuousVerify | Feature::DlpAdvanced | Feature::DlpCustom | Feature::ApiAccess | Feature::CustomPolicies | Feature::Webhooks | Feature::Sandbox | Feature::Casb | Feature::Rbi | Feature::HwBinding | Feature::TpmSealing => LicenseTier::Engineering,
            Feature::Scim | Feature::DataSovereignty | Feature::GeoFencing | Feature::DataResidency | Feature::CrossBorder | Feature::HsmIntegration => LicenseTier::Sovereign,
            Feature::ComplianceAudit | Feature::FipsMode | Feature::Fedramp | Feature::ClassifiedData | Feature::SecureEnclave => LicenseTier::Government,
            Feature::DlpMl => LicenseTier::Government,
        }
    }

    /// Check if this feature is available in a given tier
    #[inline(always)]
    pub const fn available_in(self, tier: crate::tier::LicenseTier) -> bool {
        tier.value() >= self.min_tier().value()
    }

    /// Get all features as an array
    #[inline(always)]
    pub const fn all_features() -> [Feature; 35] {
        [
            Feature::SaseBasic, Feature::SaseAdvanced, Feature::SaseEnterprise,
            Feature::VpnBasic, Feature::VpnAdvanced, Feature::SdWan, Feature::MultiCloud,
            Feature::ZeroTrust, Feature::IdentityAccess, Feature::DevicePosture, Feature::ContinuousVerify,
            Feature::DlpBasic, Feature::DlpAdvanced, Feature::DlpCustom, Feature::DlpMl,
            Feature::ApiAccess, Feature::CustomPolicies, Feature::Webhooks, Feature::Scim,
            Feature::DataSovereignty, Feature::GeoFencing, Feature::DataResidency, Feature::CrossBorder,
            Feature::ComplianceAudit, Feature::FipsMode, Feature::Fedramp, Feature::ClassifiedData,
            Feature::ThreatIntel, Feature::Sandbox, Feature::Casb, Feature::Rbi,
            Feature::HwBinding, Feature::TpmSealing, Feature::HsmIntegration, Feature::SecureEnclave,
        ]
    }
}

impl fmt::Display for Feature {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name())
    }
}

/// Feature set errors
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "std", derive(thiserror::Error))]
pub enum FeatureError {
    /// Invalid feature combination
    #[cfg_attr(feature = "std", error("Invalid feature combination"))]
    InvalidCombination,
    /// Feature not available in tier
    #[cfg_attr(feature = "std", error("Feature {0} not available in tier"))]
    NotInTier(&'static str),
    /// Feature requires higher tier
    #[cfg_attr(feature = "std", error("Feature requires tier {0:?} or higher"))]
    RequiresTier(crate::tier::LicenseTier),
    /// Unknown feature
    #[cfg_attr(feature = "std", error("Unknown feature: {0}"))]
    UnknownFeature(&'static str),
}

impl FeatureSet {
    /// Create a feature set from a tier's default features
    #[inline(always)]
    pub const fn from_tier(tier: crate::tier::LicenseTier) -> Self {
        tier.default_features()
    }

    /// Check if a specific feature is present
    #[inline(always)]
    pub const fn has(self, feature: Feature) -> bool {
        (self.bits() & feature.bit().bits()) != 0
    }

    /// Get the number of features in the set
    #[inline(always)]
    pub const fn count(self) -> u32 {
        self.bits().count_ones()
    }

    /// Check if this feature set is a subset of another
    #[inline(always)]
    pub const fn is_subset_of(self, other: FeatureSet) -> bool {
        (self.bits() & !other.bits()) == 0
    }

    /// Check if this feature set is a superset of another
    #[inline(always)]
    pub const fn is_superset_of(self, other: FeatureSet) -> bool {
        other.is_subset_of(self)
    }

    /// Get features missing from this set compared to required
    #[inline(always)]
    pub const fn missing(self, required: FeatureSet) -> FeatureSet {
        FeatureSet::from_bits_retain(required.bits() & !self.bits())
    }

    /// Get features in this set that are not in other
    #[inline(always)]
    pub const fn extra(self, other: FeatureSet) -> FeatureSet {
        FeatureSet::from_bits_retain(self.bits() & !other.bits())
    }
}

impl Default for FeatureSet {
    #[inline(always)]
    fn default() -> Self {
        Self::empty()
    }
}

impl From<Feature> for FeatureSet {
    #[inline(always)]
    fn from(feature: Feature) -> Self {
        feature.bit()
    }
}

#[cfg(any(feature = "std", feature = "alloc"))]
impl Serialize for FeatureSet {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_u64(self.bits())
    }
}

#[cfg(any(feature = "std", feature = "alloc"))]
impl<'de> Deserialize<'de> for FeatureSet {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let bits = u64::deserialize(deserializer)?;
        Ok(FeatureSet::from_bits_retain(bits))
    }
}

#[cfg(feature = "const_fn")]
impl FeatureSet {
    /// Const version of has
    #[inline(always)]
    pub const fn const_has(self, feature: Feature) -> bool {
        (self.bits() & feature.bit().bits()) != 0
    }

    /// Const version of is_subset_of
    #[inline(always)]
    pub const fn const_is_subset_of(self, other: FeatureSet) -> bool {
        (self.bits() & !other.bits()) == 0
    }
}

#[cfg(test)]
#[cfg(feature = "std")]
mod tests {
    use super::*;
    use crate::tier::LicenseTier;

    #[test]
    fn test_feature_bits() {
        assert_eq!(Feature::SaseBasic.bit().bits(), 1 << 0);
        assert_eq!(Feature::VpnBasic.bit().bits(), 1 << 3);
        assert_eq!(Feature::ZeroTrust.bit().bits(), 1 << 7);
    }

    #[test]
    fn test_feature_set_operations() {
        let mut set = FeatureSet::empty();
        set.insert(FeatureSet::from(Feature::SaseBasic));
        set.insert(FeatureSet::from(Feature::VpnBasic));
        assert!(set.has(Feature::SaseBasic));
        assert!(set.has(Feature::VpnBasic));
        assert!(!set.has(Feature::ZeroTrust));
    }

    #[test]
    fn test_feature_set_subset() {
        let basic = FeatureSet::from(Feature::SaseBasic);
        let advanced = FeatureSet::from(Feature::SaseAdvanced);
        let both = basic | advanced;

        assert!(basic.is_subset_of(both));
        assert!(both.is_superset_of(basic));
        assert!(!advanced.is_subset_of(basic));
    }

    #[test]
    fn test_feature_tier_requirements() {
        assert!(Feature::SaseBasic.available_in(LicenseTier::Free));
        assert!(Feature::ZeroTrust.available_in(LicenseTier::Pro));
        assert!(!Feature::ZeroTrust.available_in(LicenseTier::Base));
        assert!(Feature::TpmSealing.available_in(LicenseTier::Engineering));
        assert!(!Feature::TpmSealing.available_in(LicenseTier::Pro));
        assert!(Feature::FipsMode.available_in(LicenseTier::Government));
        assert!(!Feature::FipsMode.available_in(LicenseTier::Sovereign));
    }

    #[test]
    fn test_feature_min_tier() {
        assert_eq!(Feature::SaseBasic.min_tier(), LicenseTier::Free);
        assert_eq!(Feature::VpnBasic.min_tier(), LicenseTier::Base);
        assert_eq!(Feature::ZeroTrust.min_tier(), LicenseTier::Pro);
        assert_eq!(Feature::TpmSealing.min_tier(), LicenseTier::Engineering);
        assert_eq!(Feature::DataSovereignty.min_tier(), LicenseTier::Sovereign);
        assert_eq!(Feature::FipsMode.min_tier(), LicenseTier::Government);
    }

    #[test]
    fn test_feature_names() {
        assert_eq!(Feature::ZeroTrust.name(), "zero_trust");
        assert_eq!(Feature::DataSovereignty.name(), "data_sovereignty");
        assert_eq!(Feature::TpmSealing.name(), "tpm_sealing");
    }

    #[test]
    fn test_feature_display_names() {
        assert_eq!(Feature::ZeroTrust.display_name(), "Zero Trust");
        assert_eq!(Feature::DataSovereignty.display_name(), "Data Sovereignty");
        assert_eq!(Feature::TpmSealing.display_name(), "TPM Sealing");
    }

    #[test]
    fn test_tier_default_features() {
        let free = LicenseTier::Free.default_features();
        assert!(free.has(Feature::SaseBasic));
        assert!(!free.has(Feature::SaseAdvanced));

        let pro = LicenseTier::Pro.default_features();
        assert!(pro.has(Feature::SaseBasic));
        assert!(pro.has(Feature::SaseAdvanced));
        assert!(pro.has(Feature::ZeroTrust));
        assert!(pro.has(Feature::DlpBasic));

        let gov = LicenseTier::Government.default_features();
        assert!(gov.has(Feature::ComplianceAudit));
        assert!(gov.has(Feature::FipsMode));
        assert!(gov.has(Feature::SecureEnclave));
    }

    #[test]
    fn test_feature_set_from_tier() {
        let free_set = FeatureSet::from_tier(LicenseTier::Free);
        assert!(free_set.has(Feature::SaseBasic));

        let gov_set = FeatureSet::from_tier(LicenseTier::Government);
        assert_eq!(gov_set.bits(), u64::MAX);
    }

    #[test]
    fn test_feature_count() {
        let set = FeatureSet::from(Feature::SaseBasic) | FeatureSet::from(Feature::ZeroTrust);
        assert_eq!(set.count(), 2);
    }
}