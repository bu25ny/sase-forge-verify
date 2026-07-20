/*!
 * PyO3 Python Bindings
 *
 * Python bindings for SASE License Core.
 * Requires the `python` feature.
 */

#[cfg(feature = "python")]
mod python_impl {
    use crate::{
        LicenseTier, LicensePolicy, LicenseKey, FeatureSet, Feature,
        KeyId, PublicKey, Signature, VerificationContext,
        Ed25519Verifier, VerifyError,
        MAX_POLICY_SIZE, MAX_KEY_SIZE,
    };
    use pyo3::prelude::*;
    use pyo3::types::{PyBytes, PyDict, PyList, PyTuple};
    use std::convert::TryFrom;

    /// Python LicenseTier enum
    #[pyclass(eq, eq_int)]
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum LicenseTierPy {
        Free = 0,
        Base = 1,
        Pro = 2,
        Engineering = 3,
        Sovereign = 4,
        Government = 5,
    }

    impl From<LicenseTierPy> for LicenseTier {
        fn from(tier: LicenseTierPy) -> Self {
            match tier {
                LicenseTierPy::Free => LicenseTier::Free,
                LicenseTierPy::Base => LicenseTier::Base,
                LicenseTierPy::Pro => LicenseTier::Pro,
                LicenseTierPy::Engineering => LicenseTier::Engineering,
                LicenseTierPy::Sovereign => LicenseTier::Sovereign,
                LicenseTierPy::Government => LicenseTier::Government,
            }
        }
    }

    impl From<LicenseTier> for LicenseTierPy {
        fn from(tier: LicenseTier) -> Self {
            match tier {
                LicenseTier::Free => LicenseTierPy::Free,
                LicenseTier::Base => LicenseTierPy::Base,
                LicenseTier::Pro => LicenseTierPy::Pro,
                LicenseTier::Engineering => LicenseTierPy::Engineering,
                LicenseTier::Sovereign => LicenseTierPy::Sovereign,
                LicenseTier::Government => LicenseTierPy::Government,
            }
        }
    }

    #[pymethods]
    impl LicenseTierPy {
        /// Get the tier name
        #[getter]
        fn name(&self) -> &'static str {
            match self {
                LicenseTierPy::Free => "free",
                LicenseTierPy::Base => "base",
                LicenseTierPy::Pro => "pro",
                LicenseTierPy::Engineering => "engineering",
                LicenseTierPy::Sovereign => "sovereign",
                LicenseTierPy::Government => "government",
            }
        }

        /// Get the tier display name
        #[getter]
        fn display_name(&self) -> &'static str {
            match self {
                LicenseTierPy::Free => "Free",
                LicenseTierPy::Base => "Base",
                LicenseTierPy::Pro => "Pro",
                LicenseTierPy::Engineering => "Engineering",
                LicenseTierPy::Sovereign => "Sovereign",
                LicenseTierPy::Government => "Government",
            }
        }

        /// Get the tier value as integer
        #[getter]
        fn value(&self) -> u8 {
            *self as u8
        }

        /// Check if this tier includes another
        fn includes(&self, other: LicenseTierPy) -> bool {
            (*self as u8) >= (other as u8)
        }

        /// Get the next higher tier
        fn next(&self) -> Option<LicenseTierPy> {
            match self {
                LicenseTierPy::Free => Some(LicenseTierPy::Base),
                LicenseTierPy::Base => Some(LicenseTierPy::Pro),
                LicenseTierPy::Pro => Some(LicenseTierPy::Engineering),
                LicenseTierPy::Engineering => Some(LicenseTierPy::Sovereign),
                LicenseTierPy::Sovereign => Some(LicenseTierPy::Government),
                LicenseTierPy::Government => None,
            }
        }

        /// Get the previous lower tier
        fn prev(&self) -> Option<LicenseTierPy> {
            match self {
                LicenseTierPy::Free => None,
                LicenseTierPy::Base => Some(LicenseTierPy::Free),
                LicenseTierPy::Pro => Some(LicenseTierPy::Base),
                LicenseTierPy::Engineering => Some(LicenseTierPy::Pro),
                LicenseTierPy::Sovereign => Some(LicenseTierPy::Engineering),
                LicenseTierPy::Government => Some(LicenseTierPy::Sovereign),
            }
        }

        /// Check if hardware binding is supported
        fn supports_hw_binding(&self) -> bool {
            matches!(self, LicenseTierPy::Pro | LicenseTierPy::Engineering | LicenseTierPy::Sovereign | LicenseTierPy::Government)
        }

        /// Check if TPM is supported
        fn supports_tpm(&self) -> bool {
            matches!(self, LicenseTierPy::Engineering | LicenseTierPy::Sovereign | LicenseTierPy::Government)
        }

        /// Check if data sovereignty is supported
        fn supports_sovereignty(&self) -> bool {
            matches!(self, LicenseTierPy::Sovereign | LicenseTierPy::Government)
        }

        /// Check if government features are supported
        fn supports_government(&self) -> bool {
            matches!(self, LicenseTierPy::Government)
        }
    }

    /// Python Feature enum
    #[pyclass(eq, eq_int)]
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum FeaturePy {
        SaseBasic = 0,
        SaseAdvanced = 1,
        SaseEnterprise = 2,
        VpnBasic = 3,
        VpnAdvanced = 4,
        SdWan = 5,
        MultiCloud = 6,
        ZeroTrust = 7,
        IdentityAccess = 8,
        DevicePosture = 9,
        ContinuousVerify = 10,
        DlpBasic = 11,
        DlpAdvanced = 12,
        DlpCustom = 13,
        DlpMl = 14,
        ApiAccess = 15,
        CustomPolicies = 16,
        Webhooks = 17,
        Scim = 18,
        DataSovereignty = 19,
        GeoFencing = 20,
        DataResidency = 21,
        CrossBorder = 22,
        ComplianceAudit = 23,
        FipsMode = 24,
        Fedramp = 25,
        ClassifiedData = 26,
        ThreatIntel = 27,
        Sandbox = 28,
        Casb = 29,
        Rbi = 30,
        HwBinding = 31,
        TpmSealing = 32,
        HsmIntegration = 33,
        SecureEnclave = 34,
    }

    impl From<FeaturePy> for Feature {
        fn from(f: FeaturePy) -> Self {
            match f {
                FeaturePy::SaseBasic => Feature::SaseBasic,
                FeaturePy::SaseAdvanced => Feature::SaseAdvanced,
                FeaturePy::SaseEnterprise => Feature::SaseEnterprise,
                FeaturePy::VpnBasic => Feature::VpnBasic,
                FeaturePy::VpnAdvanced => Feature::VpnAdvanced,
                FeaturePy::SdWan => Feature::SdWan,
                FeaturePy::MultiCloud => Feature::MultiCloud,
                FeaturePy::ZeroTrust => Feature::ZeroTrust,
                FeaturePy::IdentityAccess => Feature::IdentityAccess,
                FeaturePy::DevicePosture => Feature::DevicePosture,
                FeaturePy::ContinuousVerify => Feature::ContinuousVerify,
                FeaturePy::DlpBasic => Feature::DlpBasic,
                FeaturePy::DlpAdvanced => Feature::DlpAdvanced,
                FeaturePy::DlpCustom => Feature::DlpCustom,
                FeaturePy::DlpMl => Feature::DlpMl,
                FeaturePy::ApiAccess => Feature::ApiAccess,
                FeaturePy::CustomPolicies => Feature::CustomPolicies,
                FeaturePy::Webhooks => FeaturePy::Webhooks,
                FeaturePy::Scim => Feature::Scim,
                FeaturePy::DataSovereignty => Feature::DataSovereignty,
                FeaturePy::GeoFencing => Feature::GeoFencing,
                FeaturePy::DataResidency => Feature::DataResidency,
                FeaturePy::CrossBorder => Feature::CrossBorder,
                FeaturePy::ComplianceAudit => Feature::ComplianceAudit,
                FeaturePy::FipsMode => Feature::FipsMode,
                FeaturePy::Fedramp => Feature::Fedramp,
                FeaturePy::ClassifiedData => Feature::ClassifiedData,
                FeaturePy::ThreatIntel => Feature::ThreatIntel,
                FeaturePy::Sandbox => Feature::Sandbox,
                FeaturePy::Casb => Feature::Casb,
                FeaturePy::Rbi => Feature::Rbi,
                FeaturePy::HwBinding => Feature::HwBinding,
                FeaturePy::TpmSealing => Feature::TpmSealing,
                FeaturePy::HsmIntegration => Feature::HsmIntegration,
                FeaturePy::SecureEnclave => Feature::SecureEnclave,
            }
        }
    }

    impl From<Feature> for FeaturePy {
        fn from(f: Feature) -> Self {
            match f {
                Feature::SaseBasic => FeaturePy::SaseBasic,
                Feature::SaseAdvanced => FeaturePy::SaseAdvanced,
                Feature::SaseEnterprise => FeaturePy::SaseEnterprise,
                Feature::VpnBasic => FeaturePy::VpnBasic,
                Feature::VpnAdvanced => FeaturePy::VpnAdvanced,
                Feature::SdWan => FeaturePy::SdWan,
                Feature::MultiCloud => FeaturePy::MultiCloud,
                Feature::ZeroTrust => FeaturePy::ZeroTrust,
                Feature::IdentityAccess => FeaturePy::IdentityAccess,
                Feature::DevicePosture => FeaturePy::DevicePosture,
                Feature::ContinuousVerify => FeaturePy::ContinuousVerify,
                Feature::DlpBasic => FeaturePy::DlpBasic,
                Feature::DlpAdvanced => FeaturePy::DlpAdvanced,
                Feature::DlpCustom => FeaturePy::DlpCustom,
                Feature::DlpMl => FeaturePy::DlpMl,
                Feature::ApiAccess => FeaturePy::ApiAccess,
                Feature::CustomPolicies => FeaturePy::CustomPolicies,
                Feature::Webhooks => FeaturePy::Webhooks,
                Feature::Scim => FeaturePy::Scim,
                Feature::DataSovereignty => FeaturePy::DataSovereignty,
                Feature::GeoFencing => FeaturePy::GeoFencing,
                Feature::DataResidency => FeaturePy::DataResidency,
                Feature::CrossBorder => FeaturePy::CrossBorder,
                Feature::ComplianceAudit => FeaturePy::ComplianceAudit,
                Feature::FipsMode => FeaturePy::FipsMode,
                Feature::Fedramp => FeaturePy::Fedramp,
                Feature::ClassifiedData => FeaturePy::ClassifiedData,
                Feature::ThreatIntel => FeaturePy::ThreatIntel,
                Feature::Sandbox => FeaturePy::Sandbox,
                Feature::Casb => FeaturePy::Casb,
                Feature::Rbi => FeaturePy::Rbi,
                Feature::HwBinding => FeaturePy::HwBinding,
                Feature::TpmSealing => FeaturePy::TpmSealing,
                Feature::HsmIntegration => FeaturePy::HsmIntegration,
                Feature::SecureEnclave => FeaturePy::SecureEnclave,
            }
        }
    }

    #[pymethods]
    impl FeaturePy {
        /// Get the feature name
        #[getter]
        fn name(&self) -> &'static str {
            match self {
                FeaturePy::SaseBasic => "sase_basic",
                FeaturePy::SaseAdvanced => "sase_advanced",
                FeaturePy::SaseEnterprise => "sase_enterprise",
                FeaturePy::VpnBasic => "vpn_basic",
                FeaturePy::VpnAdvanced => "vpn_advanced",
                FeaturePy::SdWan => "sd_wan",
                FeaturePy::MultiCloud => "multi_cloud",
                FeaturePy::ZeroTrust => "zero_trust",
                FeaturePy::IdentityAccess => "identity_access",
                FeaturePy::DevicePosture => "device_posture",
                FeaturePy::ContinuousVerify => "continuous_verify",
                FeaturePy::DlpBasic => "dlp_basic",
                FeaturePy::DlpAdvanced => "dlp_advanced",
                FeaturePy::DlpCustom => "dlp_custom",
                FeaturePy::DlpMl => "dlp_ml",
                FeaturePy::ApiAccess => "api_access",
                FeaturePy::CustomPolicies => "custom_policies",
                FeaturePy::Webhooks => "webhooks",
                FeaturePy::Scim => "scim",
                FeaturePy::DataSovereignty => "data_sovereignty",
                FeaturePy::GeoFencing => "geo_fencing",
                FeaturePy::DataResidency => "data_residency",
                FeaturePy::CrossBorder => "cross_border",
                FeaturePy::ComplianceAudit => "compliance_audit",
                FeaturePy::FipsMode => "fips_mode",
                FeaturePy::Fedramp => "fedramp",
                FeaturePy::ClassifiedData => "classified_data",
                FeaturePy::ThreatIntel => "threat_intel",
                FeaturePy::Sandbox => "sandbox",
                FeaturePy::Casb => "casb",
                FeaturePy::Rbi => "rbi",
                FeaturePy::HwBinding => "hw_binding",
                FeaturePy::TpmSealing => "tpm_sealing",
                FeaturePy::HsmIntegration => "hsm_integration",
                FeaturePy::SecureEnclave => "secure_enclave",
            }
        }

        /// Get the feature display name
        #[getter]
        fn display_name(&self) -> &'static str {
            match self {
                FeaturePy::SaseBasic => "SASE Basic",
                FeaturePy::SaseAdvanced => "SASE Advanced",
                FeaturePy::SaseEnterprise => "SASE Enterprise",
                FeaturePy::VpnBasic => "VPN Basic",
                FeaturePy::VpnAdvanced => "VPN Advanced",
                FeaturePy::SdWan => "SD-WAN",
                FeaturePy::MultiCloud => "Multi-Cloud",
                FeaturePy::ZeroTrust => "Zero Trust",
                FeaturePy::IdentityAccess => "Identity Access",
                FeaturePy::DevicePosture => "Device Posture",
                FeaturePy::ContinuousVerify => "Continuous Verification",
                FeaturePy::DlpBasic => "DLP Basic",
                FeaturePy::DlpAdvanced => "DLP Advanced",
                FeaturePy::DlpCustom => "DLP Custom",
                FeaturePy::DlpMl => "DLP ML",
                FeaturePy::ApiAccess => "API Access",
                FeaturePy::CustomPolicies => "Custom Policies",
                FeaturePy::Webhooks => "Webhooks",
                FeaturePy::Scim => "SCIM",
                FeaturePy::DataSovereignty => "Data Sovereignty",
                FeaturePy::GeoFencing => "Geo-Fencing",
                FeaturePy::DataResidency => "Data Residency",
                FeaturePy::CrossBorder => "Cross-Border",
                FeaturePy::ComplianceAudit => "Compliance Audit",
                FeaturePy::FipsMode => "FIPS Mode",
                FeaturePy::Fedramp => "FedRAMP",
                FeaturePy::ClassifiedData => "Classified Data",
                FeaturePy::ThreatIntel => "Threat Intelligence",
                FeaturePy::Sandbox => "Sandbox",
                FeaturePy::Casb => "CASB",
                FeaturePy::Rbi => "RBI",
                FeaturePy::HwBinding => "Hardware Binding",
                FeaturePy::TpmSealing => "TPM Sealing",
                FeaturePy::HsmIntegration => "HSM Integration",
                FeaturePy::SecureEnclave => "Secure Enclave",
            }
        }

        /// Get the feature bit position
        #[getter]
        fn bit(&self) -> u8 {
            *self as u8
        }

        /// Check if feature is available in a tier
        fn available_in(&self, tier: LicenseTierPy) -> bool {
            let feature: Feature = (*self).into();
            let tier: LicenseTier = tier.into();
            feature.available_in(tier)
        }

        /// Get minimum tier required for this feature
        fn min_tier(&self) -> LicenseTierPy {
            let feature: Feature = (*self).into();
            feature.min_tier().into()
        }
    }

    /// Python FeatureSet class
    #[pyclass]
    #[derive(Clone, Debug)]
    pub struct FeatureSetPy {
        inner: FeatureSet,
    }

    #[pymethods]
    impl FeatureSetPy {
        #[new]
        fn new() -> Self {
            Self { inner: FeatureSet::empty() }
        }

        #[staticmethod]
        fn all() -> Self {
            Self { inner: FeatureSet::all() }
        }

        #[staticmethod]
        fn from_tier(tier: LicenseTierPy) -> Self {
            let tier: LicenseTier = tier.into();
            Self { inner: FeatureSet::from_tier(tier) }
        }

        fn insert(&mut self, feature: FeaturePy) {
            self.inner.insert(feature.into());
        }

        fn remove(&mut self, feature: FeaturePy) {
            self.inner.remove(feature.into());
        }

        fn has(&self, feature: FeaturePy) -> bool {
            self.inner.has(feature.into())
        }

        fn count(&self) -> u32 {
            self.inner.count()
        }

        fn is_subset_of(&self, other: &FeatureSetPy) -> bool {
            self.inner.is_subset_of(other.inner)
        }

        fn is_superset_of(&self, other: &FeatureSetPy) -> bool {
            self.inner.is_superset_of(other.inner)
        }

        fn missing(&self, required: &FeatureSetPy) -> FeatureSetPy {
            FeatureSetPy { inner: self.inner.missing(required.inner) }
        }

        fn extra(&self, other: &FeatureSetPy) -> FeatureSetPy {
            FeatureSetPy { inner: self.inner.extra(other.inner) }
        }

        fn __contains__(&self, feature: FeaturePy) -> bool {
            self.has(feature)
        }

        fn __or__(&self, other: &FeatureSetPy) -> FeatureSetPy {
            FeatureSetPy { inner: self.inner | other.inner }
        }

        fn __and__(&self, other: &FeatureSetPy) -> FeatureSetPy {
            FeatureSetPy { inner: self.inner & other.inner }
        }

        fn __xor__(&self, other: &FeatureSetPy) -> FeatureSetPy {
            FeatureSetPy { inner: self.inner ^ other.inner }
        }

        fn __sub__(&self, other: &FeatureSetPy) -> FeatureSetPy {
            FeatureSetPy { inner: self.inner - other.inner }
        }

        fn __repr__(&self) -> String {
            let features: Vec<String> = self.inner.iter().map(|f| f.name().to_string()).collect();
            format!("FeatureSet({{{}}})", features.join(", "))
        }

        fn __iter__(&self) -> PyResult<PyObject> {
            Python::with_gil(|py| {
                let features: Vec<FeaturePy> = self.inner.iter().map(|f| f.into()).collect();
                PyList::new(py, features).into_py(py)
            })
        }

        fn __len__(&self) -> usize {
            self.inner.count() as usize
        }
    }

    /// Python VerificationContext
    #[pyclass]
    #[derive(Clone, Debug)]
    pub struct VerificationContextPy {
        inner: VerificationContext,
    }

    #[pymethods]
    impl VerificationContextPy {
        #[new]
        #[pyo3(signature = (current_time, hw_fingerprint=None, pcr_values=None))]
        fn new(current_time: u64, hw_fingerprint: Option<&[u8]>, pcr_values: Option<&[u8]>) -> PyResult<Self> {
            let mut ctx = VerificationContext::new(current_time);

            if let Some(hw) = hw_fingerprint {
                if hw.len() != 32 {
                    return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(
                        "Hardware fingerprint must be 32 bytes"
                    ));
                }
                let mut arr = [0u8; 32];
                arr.copy_from_slice(hw);
                ctx = ctx.with_hw_fingerprint(arr);
            }

            if let Some(pcr) = pcr_values {
                if pcr.len() != 32 {
                    return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(
                        "PCR values must be 32 bytes"
                    ));
                }
                let mut arr = [0u8; 32];
                arr.copy_from_slice(pcr);
                ctx = ctx.with_pcr_values(arr);
            }

            Ok(Self { inner: ctx })
        }

        #[getter]
        fn current_time(&self) -> u64 {
            self.inner.current_time
        }

        #[getter]
        fn hw_fingerprint(&self) -> Option<Vec<u8>> {
            self.inner.hw_fingerprint.map(|h| h.to_vec())
        }

        #[getter]
        fn pcr_values(&self) -> Option<Vec<u8>> {
            self.inner.pcr_values.map(|p| p.to_vec())
        }
    }

    /// Python LicensePolicy
    #[pyclass]
    #[derive(Clone, Debug)]
    pub struct LicensePolicyPy {
        inner: LicensePolicy,
    }

    #[pymethods]
    impl LicensePolicyPy {
        #[new]
        #[pyo3(signature = (tier, max_tokens=0, max_requests=0, max_compute=0, hw_bound=false, expiry=0, features=None))]
        fn new(
            tier: LicenseTierPy,
            max_tokens: u64,
            max_requests: u64,
            max_compute: u32,
            hw_bound: bool,
            expiry: u64,
            features: Option<FeatureSetPy>,
        ) -> Self {
            let mut policy = LicensePolicy::new(tier.into())
                .max_tokens(max_tokens)
                .max_requests(max_requests)
                .max_compute(max_compute)
                .hw_bound(hw_bound)
                .expiry(expiry);

            if let Some(f) = features {
                policy = policy.features(f.inner);
            }

            Self { inner: policy }
        }

        #[getter]
        fn max_tokens(&self) -> u64 {
            self.inner.max_tokens
        }

        #[setter]
        fn set_max_tokens(&mut self, value: u64) {
            self.inner.max_tokens = value;
        }

        #[getter]
        fn max_requests(&self) -> u64 {
            self.inner.max_requests
        }

        #[setter]
        fn set_max_requests(&mut self, value: u64) {
            self.inner.max_requests = value;
        }

        #[getter]
        fn max_compute(&self) -> u32 {
            self.inner.max_compute
        }

        #[setter]
        fn set_max_compute(&mut self, value: u32) {
            self.inner.max_compute = value;
        }

        #[getter]
        fn hw_bound(&self) -> bool {
            self.inner.hw_bound
        }

        #[setter]
        fn set_hw_bound(&mut self, value: bool) {
            self.inner.hw_bound = value;
        }

        #[getter]
        fn expiry(&self) -> u64 {
            self.inner.expiry
        }

        #[setter]
        fn set_expiry(&mut self, value: u64) {
            self.inner.expiry = value;
        }

        #[getter]
        fn tier(&self) -> LicenseTierPy {
            self.inner.tier.into()
        }

        #[setter]
        fn set_tier(&mut self, value: LicenseTierPy) {
            self.inner.tier = value.into();
        }

        #[getter]
        fn features(&self) -> FeatureSetPy {
            FeatureSetPy { inner: self.inner.features }
        }

        #[setter]
        fn set_features(&mut self, value: FeatureSetPy) {
            self.inner.features = value.inner;
        }

        fn is_expired(&self, current_time: u64) -> bool {
            self.inner.is_expired(current_time)
        }

        fn unlimited_tokens(&self) -> bool {
            self.inner.unlimited_tokens()
        }

        fn unlimited_requests(&self) -> bool {
            self.inner.unlimited_requests()
        }

        fn unlimited_compute(&self) -> bool {
            self.inner.unlimited_compute()
        }

        fn time_until_expiry(&self, current_time: u64) -> u64 {
            self.inner.time_until_expiry(current_time)
        }

        fn validate(&self) -> PyResult<()> {
            self.inner.validate().map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("{}", e)))
        }

        fn to_postcard(&self) -> PyResult<Vec<u8>> {
            let mut buf = heapless::Vec::new();
            postcard::to_slice(&self.inner, &mut buf)
                .map_err(|_| PyErr::new::<pyo3::exceptions::PyValueError, _>("Serialization failed"))?;
            Ok(buf.to_vec())
        }

        #[staticmethod]
        fn from_postcard(data: &[u8]) -> PyResult<Self> {
            let policy = LicensePolicy::from_postcard(data)
                .map_err(|_| PyErr::new::<pyo3::exceptions::PyValueError, _>("Deserialization failed"))?;
            Ok(Self { inner: policy })
        }

        fn to_json(&self) -> PyResult<String> {
            serde_json::to_string(&self.inner)
                .map_err(|_| PyErr::new::<pyo3::exceptions::PyValueError, _>("JSON serialization failed"))
        }

        #[staticmethod]
        fn from_json(json: &str) -> PyResult<Self> {
            let policy = serde_json::from_str(json)
                .map_err(|_| PyErr::new::<pyo3::exceptions::PyValueError, _>("JSON deserialization failed"))?;
            Ok(Self { inner: policy })
        }

        fn __repr__(&self) -> String {
            format!("LicensePolicy(tier={}, max_tokens={}, max_requests={}, hw_bound={}, expiry={})",
                self.inner.tier, self.inner.max_tokens, self.inner.max_requests, self.inner.hw_bound, self.inner.expiry)
        }
    }

    /// Python LicenseKey
    #[pyclass]
    #[derive(Clone, Debug)]
    pub struct LicenseKeyPy {
        inner: LicenseKey,
    }

    #[pymethods]
    impl LicenseKeyPy {
        #[new]
        #[pyo3(signature = (policy, signature, public_key, key_id, issued_at))]
        fn new(
            policy: LicensePolicyPy,
            signature: &[u8],
            public_key: &[u8],
            key_id: &[u8],
            issued_at: u64,
        ) -> PyResult<Self> {
            if signature.len() != 64 {
                return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>("Signature must be 64 bytes"));
            }
            if public_key.len() != 32 {
                return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>("Public key must be 32 bytes"));
            }
            if key_id.len() != 16 {
                return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>("Key ID must be 16 bytes"));
            }

            let mut sig_arr = [0u8; 64];
            sig_arr.copy_from_slice(signature);

            let mut pk_arr = [0u8; 32];
            pk_arr.copy_from_slice(public_key);

            let mut kid_arr = [0u8; 16];
            kid_arr.copy_from_slice(key_id);

            let key = LicenseKey::new(
                policy.inner,
                Signature::new(sig_arr),
                PublicKey::new(pk_arr),
                KeyId::new(kid_arr),
                issued_at,
            );

            Ok(Self { inner: key })
        }

        #[getter]
        fn policy(&self) -> LicensePolicyPy {
            LicensePolicyPy { inner: self.inner.policy.clone() }
        }

        #[getter]
        fn signature(&self) -> Vec<u8> {
            self.inner.signature.as_bytes().to_vec()
        }

        #[getter]
        fn public_key(&self) -> Vec<u8> {
            self.inner.public_key.as_bytes().to_vec()
        }

        #[getter]
        fn key_id(&self) -> Vec<u8> {
            self.inner.key_id.as_bytes().to_vec()
        }

        #[getter]
        fn issued_at(&self) -> u64 {
            self.inner.issued_at
        }

        #[getter]
        fn tier(&self) -> LicenseTierPy {
            self.inner.tier().into()
        }

        fn features(&self) -> FeatureSetPy {
            FeatureSetPy { inner: self.inner.features() }
        }

        fn has_feature(&self, feature: FeaturePy) -> bool {
            self.inner.has_feature(feature.into())
        }

        fn verify(&self, current_time: u64, hw_fingerprint: Option<&[u8]>, pcr_values: Option<&[u8]>) -> PyResult<bool> {
            let mut ctx = VerificationContext::new(current_time);
            if let Some(hw) = hw_fingerprint {
                if hw.len() != 32 {
                    return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>("Hardware fingerprint must be 32 bytes"));
                }
                let mut arr = [0u8; 32];
                arr.copy_from_slice(hw);
                ctx = ctx.with_hw_fingerprint(arr);
            }
            if let Some(pcr) = pcr_values {
                if pcr.len() != 32 {
                    return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>("PCR values must be 32 bytes"));
                }
                let mut arr = [0u8; 32];
                arr.copy_from_slice(pcr);
                ctx = ctx.with_pcr_values(arr);
            }

            match self.inner.verify_with_context(&ctx) {
                Ok(()) => Ok(true),
                Err(_) => Ok(false),
            }
        }

        fn validate(&self, current_time: u64, hw_fingerprint: Option<&[u8]>, pcr_values: Option<&[u8]>) -> PyResult<()> {
            let mut ctx = VerificationContext::new(current_time);
            if let Some(hw) = hw_fingerprint {
                if hw.len() != 32 {
                    return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>("Hardware fingerprint must be 32 bytes"));
                }
                let mut arr = [0u8; 32];
                arr.copy_from_slice(hw);
                ctx = ctx.with_hw_fingerprint(arr);
            }
            if let Some(pcr) = pcr_values {
                if pcr.len() != 32 {
                    return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>("PCR values must be 32 bytes"));
                }
                let mut arr = [0u8; 32];
                arr.copy_from_slice(pcr);
                ctx = ctx.with_pcr_values(arr);
            }

            self.inner.validate(&ctx)
                .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("{}", e)))
        }

        fn is_expired(&self, current_time: u64) -> bool {
            self.inner.is_expired(current_time)
        }

        fn to_postcard(&self) -> PyResult<Vec<u8>> {
            let mut buf = heapless::Vec::new();
            postcard::to_slice(&self.inner, &mut buf)
                .map_err(|_| PyErr::new::<pyo3::exceptions::PyValueError, _>("Serialization failed"))?;
            Ok(buf.to_vec())
        }

        #[staticmethod]
        fn from_postcard(data: &[u8]) -> PyResult<Self> {
            let key = LicenseKey::from_postcard(data)
                .map_err(|_| PyErr::new::<pyo3::exceptions::PyValueError, _>("Deserialization failed"))?;
            Ok(Self { inner: key })
        }

        fn to_json(&self) -> PyResult<String> {
            serde_json::to_string(&self.inner)
                .map_err(|_| PyErr::new::<pyo3::exceptions::PyValueError, _>("JSON serialization failed"))
        }

        #[staticmethod]
        fn from_json(json: &str) -> PyResult<Self> {
            let key = serde_json::from_str(json)
                .map_err(|_| PyErr::new::<pyo3::exceptions::PyValueError, _>("JSON deserialization failed"))?;
            Ok(Self { inner: key })
        }

        fn __repr__(&self) -> String {
            format!("LicenseKey(key_id={:?}, tier={}, issued_at={})",
                hex::encode(self.inner.key_id.as_bytes()),
                self.inner.tier(),
                self.inner.issued_at)
        }
    }

    /// Python Ed25519 Verifier
    #[pyclass]
    pub struct Ed25519VerifierPy;

    #[pymethods]
    impl Ed25519VerifierPy {
        #[new]
        fn new() -> Self {
            Self
        }

        fn verify(&self, message: &[u8], signature: &[u8], public_key: &[u8]) -> PyResult<bool> {
            if signature.len() != 64 {
                return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>("Signature must be 64 bytes"));
            }
            if public_key.len() != 32 {
                return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>("Public key must be 32 bytes"));
            }

            let mut sig_arr = [0u8; 64];
            sig_arr.copy_from_slice(signature);

            let mut pk_arr = [0u8; 32];
            pk_arr.copy_from_slice(public_key);

            let sig = Signature::new(sig_arr);
            let pk = PublicKey::new(pk_arr);

            match Ed25519Verifier::verify(message, &sig, &pk) {
                Ok(()) => Ok(true),
                Err(_) => Ok(false),
            }
        }

        fn verify_prehashed(&self, prehash: &[u8], signature: &[u8], public_key: &[u8]) -> PyResult<bool> {
            if prehash.len() != 64 {
                return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>("Prehash must be 64 bytes"));
            }
            if signature.len() != 64 {
                return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>("Signature must be 64 bytes"));
            }
            if public_key.len() != 32 {
                return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>("Public key must be 32 bytes"));
            }

            let mut prehash_arr = [0u8; 64];
            prehash_arr.copy_from_slice(prehash);

            let mut sig_arr = [0u8; 64];
            sig_arr.copy_from_slice(signature);

            let mut pk_arr = [0u8; 32];
            pk_arr.copy_from_slice(public_key);

            let sig = Signature::new(sig_arr);
            let pk = PublicKey::new(pk_arr);

            match Ed25519Verifier::verify_prehashed(&prehash_arr, &sig, &pk) {
                Ok(()) => Ok(true),
                Err(_) => Ok(false),
            }
        }
    }

    /// Verify a license key (standalone function)
    #[pyfunction]
    #[pyo3(signature = (key_bytes, policy_json, current_time=None, hw_fingerprint=None, pcr_values=None))]
    fn verify_license(
        key_bytes: &[u8],
        policy_json: &str,
        current_time: Option<u64>,
        hw_fingerprint: Option<&[u8]>,
        pcr_values: Option<&[u8]>,
    ) -> PyResult<bool> {
        // Parse key from postcard
        let key = LicenseKey::from_postcard(key_bytes)
            .map_err(|_| PyErr::new::<pyo3::exceptions::PyValueError, _>("Failed to deserialize key"))?;

        // Parse policy from JSON
        let policy: LicensePolicy = serde_json::from_str(policy_json)
            .map_err(|_| PyErr::new::<pyo3::exceptions::PyValueError, _>("Failed to parse policy JSON"))?;

        // Verify policy matches
        if key.policy != policy {
            return Ok(false);
        }

        // Create verification context
        let mut ctx = VerificationContext::new(current_time.unwrap_or(0));
        if let Some(hw) = hw_fingerprint {
            if hw.len() != 32 {
                return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>("Hardware fingerprint must be 32 bytes"));
            }
            let mut arr = [0u8; 32];
            arr.copy_from_slice(hw);
            ctx = ctx.with_hw_fingerprint(arr);
        }
        if let Some(pcr) = pcr_values {
            if pcr.len() != 32 {
                return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>("PCR values must be 32 bytes"));
            }
            let mut arr = [0u8; 32];
            arr.copy_from_slice(pcr);
            ctx = ctx.with_pcr_values(arr);
        }

        match key.validate(&ctx) {
            Ok(()) => Ok(true),
            Err(_) => Ok(false),
        }
    }

    /// Generate a key ID from a public key
    #[pyfunction]
    fn key_id_from_public_key(public_key: &[u8]) -> PyResult<Vec<u8>> {
        if public_key.len() != 32 {
            return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>("Public key must be 32 bytes"));
        }
        let mut pk_arr = [0u8; 32];
        pk_arr.copy_from_slice(public_key);
        let pk = PublicKey::new(pk_arr);
        let kid = KeyId::from_public_key(&pk);
        Ok(kid.as_bytes().to_vec())
    }

    /// Get version info
    #[pyfunction]
    fn version() -> String {
        env!("CARGO_PKG_VERSION").to_string()
    }

    /// Get version as integer
    #[pyfunction]
    fn version_num() -> u32 {
        let major = env!("CARGO_PKG_VERSION_MAJOR").parse::<u32>().unwrap_or(0);
        let minor = env!("CARGO_PKG_VERSION_MINOR").parse::<u32>().unwrap_or(0);
        let patch = env!("CARGO_PKG_VERSION_PATCH").parse::<u32>().unwrap_or(0);
        (major << 22) | (minor << 12) | patch
    }

    /// Get license tier from string
    #[pyfunction]
    fn tier_from_str(tier_str: &str) -> PyResult<LicenseTierPy> {
        tier_str.parse::<LicenseTier>()
            .map(|t| t.into())
            .map_err(|_| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("Unknown tier: {}", tier_str)))
    }

    /// Get all license tiers
    #[pyfunction]
    fn all_tiers() -> Vec<LicenseTierPy> {
        LicenseTier::all().iter().map(|t| (*t).into()).collect()
    }

    /// Get all features
    #[pyfunction]
    fn all_features() -> Vec<FeaturePy> {
        Feature::all().iter().map(|f| (*f).into()).collect()
    }

    /// Create a policy with default features for a tier
    #[pyfunction]
    fn default_policy(tier: LicenseTierPy, max_tokens: u64, max_requests: u64, hw_bound: bool, expiry: u64) -> LicensePolicyPy {
        LicensePolicyPy::new(tier, max_tokens, max_requests, 0, hw_bound, expiry, None)
    }

    /// Python module definition
    #[pymodule]
    fn sase_license_core(_py: Python, m: &PyModule) -> PyResult<()> {
        m.add_class::<LicenseTierPy>()?;
        m.add_class::<FeaturePy>()?;
        m.add_class::<FeatureSetPy>()?;
        m.add_class::<VerificationContextPy>()?;
        m.add_class::<LicensePolicyPy>()?;
        m.add_class::<LicenseKeyPy>()?;
        m.add_class::<Ed25519VerifierPy>()?;

        m.add_function(wrap_pyfunction!(verify_license, m)?)?;
        m.add_function(wrap_pyfunction!(key_id_from_public_key, m)?)?;
        m.add_function(wrap_pyfunction!(version, m)?)?;
        m.add_function(wrap_pyfunction!(version_num, m)?)?;
        m.add_function(wrap_pyfunction!(tier_from_str, m)?)?;
        m.add_function(wrap_pyfunction!(all_tiers, m)?)?;
        m.add_function(wrap_pyfunction!(all_features, m)?)?;
        m.add_function(wrap_pyfunction!(default_policy, m)?)?;

        // Constants
        m.add("KEY_ID_SIZE", 16)?;
        m.add("SIGNATURE_SIZE", 64)?;
        m.add("PUBLIC_KEY_SIZE", 32)?;
        m.add("SECRET_KEY_SIZE", 32)?;
        m.add("MAX_POLICY_SIZE", MAX_POLICY_SIZE)?;
        m.add("MAX_KEY_SIZE", MAX_KEY_SIZE)?;
        m.add("MAX_MERKLE_NODE_SIZE", 256)?;
        m.add("MAX_MERKLE_PROOF_DEPTH", 32)?;
        m.add("MAX_PCR_COUNT", 9)?;
        m.add("PCR_MASK", 0x0001_00FFu32)?;

        Ok(())
    }
}

#[cfg(feature = "python")]
pub use python_impl::*;