/*!
 * License Tier Definitions
 *
 * Defines the SASE license tiers and their ordering.
 */

use core::fmt;
use core::str::FromStr;
#[cfg(any(feature = "std", feature = "alloc"))]
use serde::{Deserialize, Serialize};
use crate::features::{FeatureSet, Feature};

/// SASE License Tiers in ascending order of capabilities
///
/// Tiers form a hierarchy: Free < Base < Pro < Engineering < Sovereign < Government
/// Each tier includes all features of lower tiers plus additional capabilities.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(any(feature = "std", feature = "alloc"), derive(Serialize, Deserialize))]
#[cfg_attr(any(feature = "std", feature = "alloc"), serde(rename_all = "lowercase"))]
#[repr(u8)]
pub enum LicenseTier {
    /// Free tier - Basic SASE access, limited features
    Free = 0,
    /// Base tier - Standard SASE features
    Base = 1,
    /// Pro tier - Advanced SASE features
    Pro = 2,
    /// Engineering tier - Engineering/Development features
    Engineering = 3,
    /// Sovereign tier - Data sovereignty features
    Sovereign = 4,
    /// Government tier - Government/compliance features
    Government = 5,
}

/// Errors related to license tiers
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "std", derive(thiserror::Error))]
pub enum TierError {
    /// Invalid tier value
    #[cfg_attr(feature = "std", error("Invalid tier value: {0}"))]
    InvalidTier(u8),
    /// Tier not found
    #[cfg_attr(feature = "std", error("Tier not found: {0}"))]
    NotFound(&'static str),
    /// Tier parse error
    #[cfg_attr(feature = "std", error("Failed to parse tier: {0}"))]
    ParseError(&'static str),
}

impl LicenseTier {
    /// Get all license tiers in ascending order
    #[inline(always)]
    pub const fn all() -> [LicenseTier; 6] {
        [
            LicenseTier::Free,
            LicenseTier::Base,
            LicenseTier::Pro,
            LicenseTier::Engineering,
            LicenseTier::Sovereign,
            LicenseTier::Government,
        ]
    }

    /// Get the numeric value of the tier
    #[inline(always)]
    pub const fn value(self) -> u8 {
        self as u8
    }

    /// Get the tier name as a static string
    #[inline(always)]
    pub const fn name(self) -> &'static str {
        match self {
            LicenseTier::Free => "free",
            LicenseTier::Base => "base",
            LicenseTier::Pro => "pro",
            LicenseTier::Engineering => "engineering",
            LicenseTier::Sovereign => "sovereign",
            LicenseTier::Government => "government",
        }
    }

    /// Get the tier display name
    #[inline(always)]
    pub const fn display_name(self) -> &'static str {
        match self {
            LicenseTier::Free => "Free",
            LicenseTier::Base => "Base",
            LicenseTier::Pro => "Pro",
            LicenseTier::Engineering => "Engineering",
            LicenseTier::Sovereign => "Sovereign",
            LicenseTier::Government => "Government",
        }
    }

    /// Get the default FeatureSet for this tier
    #[inline(always)]
    pub const fn default_features(self) -> FeatureSet {
        use crate::features::{Feature, FeatureSet};

        // Use raw bitwise operations on u64 to avoid non-const bitflags operations
        let bits: u64 = match self {
            LicenseTier::Free => {
                Feature::SaseBasic.bit().bits()
            }
            LicenseTier::Base => {
                Feature::SaseBasic.bit().bits()
                | Feature::SaseAdvanced.bit().bits()
                | Feature::VpnBasic.bit().bits()
            }
            LicenseTier::Pro => {
                Feature::SaseBasic.bit().bits()
                | Feature::SaseAdvanced.bit().bits()
                | Feature::VpnBasic.bit().bits()
                | Feature::VpnAdvanced.bit().bits()
                | Feature::ZeroTrust.bit().bits()
                | Feature::DlpBasic.bit().bits()
            }
            LicenseTier::Engineering => {
                Feature::SaseBasic.bit().bits()
                | Feature::SaseAdvanced.bit().bits()
                | Feature::VpnBasic.bit().bits()
                | Feature::VpnAdvanced.bit().bits()
                | Feature::ZeroTrust.bit().bits()
                | Feature::DlpBasic.bit().bits()
                | Feature::DlpAdvanced.bit().bits()
                | Feature::ApiAccess.bit().bits()
                | Feature::CustomPolicies.bit().bits()
            }
            LicenseTier::Sovereign => {
                Feature::SaseBasic.bit().bits()
                | Feature::SaseAdvanced.bit().bits()
                | Feature::VpnBasic.bit().bits()
                | Feature::VpnAdvanced.bit().bits()
                | Feature::ZeroTrust.bit().bits()
                | Feature::DlpBasic.bit().bits()
                | Feature::DlpAdvanced.bit().bits()
                | Feature::ApiAccess.bit().bits()
                | Feature::CustomPolicies.bit().bits()
                | Feature::DataSovereignty.bit().bits()
                | Feature::GeoFencing.bit().bits()
            }
            LicenseTier::Government => {
                u64::MAX
            }
        };
        FeatureSet::from_bits_retain(bits)
    }

    /// Check if this tier includes another tier
    #[inline(always)]
    pub const fn includes(self, other: LicenseTier) -> bool {
        self.value() >= other.value()
    }

    /// Get the next higher tier, if tier up
    #[inline(always)]
    pub const fn next(self) -> Option<LicenseTier> {
        match self {
            LicenseTier::Free => Some(LicenseTier::Base),
            LicenseTier::Base => Some(LicenseTier::Pro),
            LicenseTier::Pro => Some(LicenseTier::Engineering),
            LicenseTier::Engineering => Some(LicenseTier::Sovereign),
            LicenseTier::Sovereign => Some(LicenseTier::Government),
            LicenseTier::Government => None,
        }
    }

    /// Get the previous tier down
    #[inline(always)]
    pub const fn prev(self) -> Option<LicenseTier> {
        match self {
            LicenseTier::Free => None,
            LicenseTier::Base => Some(LicenseTier::Free),
            LicenseTier::Pro => Some(LicenseTier::Base),
            LicenseTier::Engineering => Some(LicenseTier::Pro),
            LicenseTier::Sovereign => Some(LicenseTier::Engineering),
            LicenseTier::Government => Some(LicenseTier::Sovereign),
        }
    }

    /// Check if this tier has hardware binding support
    #[inline(always)]
    pub const fn supports_hw_binding(self) -> bool {
        matches!(self, LicenseTier::Pro | LicenseTier::Engineering | LicenseTier::Sovereign | LicenseTier::Government)
    }

    /// Check if this tier has TPM support
    #[inline(always)]
    pub const fn supports_tpm(self) -> bool {
        matches!(self, LicenseTier::Engineering | LicenseTier::Sovereign | LicenseTier::Government)
    }

    /// Check if this tier has data sovereignty features
    #[inline(always)]
    pub const fn supports_sovereignty(self) -> bool {
        matches!(self, LicenseTier::Sovereign | LicenseTier::Government)
    }

    /// Check if this tier has government/compliance features
    #[inline(always)]
    pub const fn supports_government(self) -> bool {
        matches!(self, LicenseTier::Government)
    }
}

impl fmt::Display for LicenseTier {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name())
    }
}

impl FromStr for LicenseTier {
    type Err = TierError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "free" => Ok(LicenseTier::Free),
            "base" => Ok(LicenseTier::Base),
            "pro" => Ok(LicenseTier::Pro),
            "engineering" => Ok(LicenseTier::Engineering),
            "sovereign" => Ok(LicenseTier::Sovereign),
            "government" => Ok(LicenseTier::Government),
            _ => Err(TierError::ParseError("Unknown tier")),
        }
    }
}

impl TryFrom<u8> for LicenseTier {
    type Error = TierError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(LicenseTier::Free),
            1 => Ok(LicenseTier::Base),
            2 => Ok(LicenseTier::Pro),
            3 => Ok(LicenseTier::Engineering),
            4 => Ok(LicenseTier::Sovereign),
            5 => Ok(LicenseTier::Government),
            _ => Err(TierError::InvalidTier(value)),
        }
    }
}

impl From<LicenseTier> for u8 {
    fn from(tier: LicenseTier) -> Self {
        tier as u8
    }
}

#[cfg(feature = "const_fn")]
impl LicenseTier {
    /// Const version of default_features
    #[inline(always)]
    pub const fn const_default_features(self) -> FeatureSet {
        // Use raw bitwise operations on u64 to avoid non-const bitflags operations
        let bits: u64 = match self {
            LicenseTier::Free => {
                Feature::SaseBasic.bit().bits()
            }
            LicenseTier::Base => {
                Feature::SaseBasic.bit().bits()
                | Feature::SaseAdvanced.bit().bits()
                | Feature::VpnBasic.bit().bits()
            }
            LicenseTier::Pro => {
                Feature::SaseBasic.bit().bits()
                | Feature::SaseAdvanced.bit().bits()
                | Feature::VpnBasic.bit().bits()
                | Feature::VpnAdvanced.bit().bits()
                | Feature::ZeroTrust.bit().bits()
                | Feature::DlpBasic.bit().bits()
            }
            LicenseTier::Engineering => {
                Feature::SaseBasic.bit().bits()
                | Feature::SaseAdvanced.bit().bits()
                | Feature::VpnBasic.bit().bits()
                | Feature::VpnAdvanced.bit().bits()
                | Feature::ZeroTrust.bit().bits()
                | Feature::DlpBasic.bit().bits()
                | Feature::DlpAdvanced.bit().bits()
                | Feature::ApiAccess.bit().bits()
                | Feature::CustomPolicies.bit().bits()
            }
            LicenseTier::Sovereign => {
                Feature::SaseBasic.bit().bits()
                | Feature::SaseAdvanced.bit().bits()
                | Feature::VpnBasic.bit().bits()
                | Feature::VpnAdvanced.bit().bits()
                | Feature::ZeroTrust.bit().bits()
                | Feature::DlpBasic.bit().bits()
                | Feature::DlpAdvanced.bit().bits()
                | Feature::ApiAccess.bit().bits()
                | Feature::CustomPolicies.bit().bits()
                | Feature::DataSovereignty.bit().bits()
                | Feature::GeoFencing.bit().bits()
            }
            LicenseTier::Government => {
                u64::MAX
            }
        };
        FeatureSet::from_bits_retain(bits)
    }
}

#[cfg(test)]
#[cfg(feature = "std")]
mod tests {
    use super::*;
    use crate::features::{Feature, FeatureSet};

    #[test]
    fn test_tier_ordering() {
        assert!(LicenseTier::Free < LicenseTier::Base);
        assert!(LicenseTier::Base < LicenseTier::Pro);
        assert!(LicenseTier::Pro < LicenseTier::Engineering);
        assert!(LicenseTier::Engineering < LicenseTier::Sovereign);
        assert!(LicenseTier::Sovereign < LicenseTier::Government);
    }

    #[test]
    fn test_tier_values() {
        assert_eq!(LicenseTier::Free.value(), 0);
        assert_eq!(LicenseTier::Base.value(), 1);
        assert_eq!(LicenseTier::Pro.value(), 2);
        assert_eq!(LicenseTier::Engineering.value(), 3);
        assert_eq!(LicenseTier::Sovereign.value(), 4);
        assert_eq!(LicenseTier::Government.value(), 5);
    }

    #[test]
    fn test_tier_names() {
        assert_eq!(LicenseTier::Free.name(), "free");
        assert_eq!(LicenseTier::Base.name(), "base");
        assert_eq!(LicenseTier::Pro.name(), "pro");
        assert_eq!(LicenseTier::Engineering.name(), "engineering");
        assert_eq!(LicenseTier::Sovereign.name(), "sovereign");
        assert_eq!(LicenseTier::Government.name(), "government");
    }

    #[test]
    fn test_tier_display_names() {
        assert_eq!(LicenseTier::Free.display_name(), "Free");
        assert_eq!(LicenseTier::Base.display_name(), "Base");
        assert_eq!(LicenseTier::Pro.display_name(), "Pro");
        assert_eq!(LicenseTier::Engineering.display_name(), "Engineering");
        assert_eq!(LicenseTier::Sovereign.display_name(), "Sovereign");
        assert_eq!(LicenseTier::Government.display_name(), "Government");
    }

    #[test]
    fn test_tier_includes() {
        assert!(LicenseTier::Pro.includes(LicenseTier::Base));
        assert!(LicenseTier::Government.includes(LicenseTier::Free));
        assert!(!LicenseTier::Base.includes(LicenseTier::Pro));
    }

    #[test]
    fn test_tier_next_prev() {
        assert_eq!(LicenseTier::Free.next(), Some(LicenseTier::Base));
        assert_eq!(LicenseTier::Government.next(), None);
        assert_eq!(LicenseTier::Base.prev(), Some(LicenseTier::Free));
        assert_eq!(LicenseTier::Free.prev(), None);
    }

    #[test]
    fn test_tier_features() {
        let free_features = LicenseTier::Free.default_features();
        assert!(free_features.contains(FeatureSet::from(Feature::SaseBasic)));
        assert!(!free_features.contains(FeatureSet::from(Feature::SaseAdvanced)));

        let pro_features = LicenseTier::Pro.default_features();
        assert!(pro_features.contains(FeatureSet::from(Feature::SaseBasic)));
        assert!(pro_features.contains(FeatureSet::from(Feature::SaseAdvanced)));
        assert!(pro_features.contains(FeatureSet::from(Feature::ZeroTrust)));
        assert!(pro_features.contains(FeatureSet::from(Feature::DlpBasic)));

        let gov_features = LicenseTier::Government.default_features();
        assert!(gov_features.contains(FeatureSet::from(Feature::DataSovereignty)));
        assert!(gov_features.contains(FeatureSet::from(Feature::GeoFencing)));
        assert!(gov_features.contains(FeatureSet::from(Feature::ComplianceAudit)));
    }

    #[test]
    fn test_tier_parse() {
        assert_eq!("free".parse::<LicenseTier>(), Ok(LicenseTier::Free));
        assert_eq!("PRO".parse::<LicenseTier>(), Ok(LicenseTier::Pro));
        assert_eq!("Sovereign".parse::<LicenseTier>(), Ok(LicenseTier::Sovereign));
        assert!("invalid".parse::<LicenseTier>().is_err());
    }

    #[test]
    fn test_tier_try_from() {
        assert_eq!(LicenseTier::try_from(0u8), Ok(LicenseTier::Free));
        assert_eq!(LicenseTier::try_from(5u8), Ok(LicenseTier::Government));
        assert!(LicenseTier::try_from(6u8).is_err());
    }

    #[test]
    fn test_tier_capabilities() {
        assert!(!LicenseTier::Free.supports_hw_binding());
        assert!(LicenseTier::Pro.supports_hw_binding());
        assert!(LicenseTier::Engineering.supports_tpm());
        assert!(LicenseTier::Sovereign.supports_sovereignty());
        assert!(LicenseTier::Government.supports_government());
    }
}
