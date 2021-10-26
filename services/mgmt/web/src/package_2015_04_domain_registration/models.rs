#![doc = "generated by AutoRust 0.1.0"]
#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Address {
    pub address1: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub address2: Option<String>,
    pub city: String,
    pub country: String,
    #[serde(rename = "postalCode")]
    pub postal_code: String,
    pub state: String,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Contact {
    #[serde(rename = "addressMailing", default, skip_serializing_if = "Option::is_none")]
    pub address_mailing: Option<Address>,
    pub email: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fax: Option<String>,
    #[serde(rename = "jobTitle", default, skip_serializing_if = "Option::is_none")]
    pub job_title: Option<String>,
    #[serde(rename = "nameFirst")]
    pub name_first: String,
    #[serde(rename = "nameLast")]
    pub name_last: String,
    #[serde(rename = "nameMiddle", default, skip_serializing_if = "Option::is_none")]
    pub name_middle: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub organization: Option<String>,
    pub phone: String,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Domain {
    #[serde(flatten)]
    pub resource: Resource,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<domain::Properties>,
}
pub mod domain {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub struct Properties {
        #[serde(rename = "contactAdmin")]
        pub contact_admin: Contact,
        #[serde(rename = "contactBilling")]
        pub contact_billing: Contact,
        #[serde(rename = "contactRegistrant")]
        pub contact_registrant: Contact,
        #[serde(rename = "contactTech")]
        pub contact_tech: Contact,
        #[serde(rename = "registrationStatus", default, skip_serializing_if = "Option::is_none")]
        pub registration_status: Option<properties::RegistrationStatus>,
        #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
        pub provisioning_state: Option<properties::ProvisioningState>,
        #[serde(rename = "nameServers", default, skip_serializing_if = "Vec::is_empty")]
        pub name_servers: Vec<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub privacy: Option<bool>,
        #[serde(rename = "createdTime", default, skip_serializing_if = "Option::is_none")]
        pub created_time: Option<String>,
        #[serde(rename = "expirationTime", default, skip_serializing_if = "Option::is_none")]
        pub expiration_time: Option<String>,
        #[serde(rename = "lastRenewedTime", default, skip_serializing_if = "Option::is_none")]
        pub last_renewed_time: Option<String>,
        #[serde(rename = "autoRenew", default, skip_serializing_if = "Option::is_none")]
        pub auto_renew: Option<bool>,
        #[serde(rename = "readyForDnsRecordManagement", default, skip_serializing_if = "Option::is_none")]
        pub ready_for_dns_record_management: Option<bool>,
        #[serde(rename = "managedHostNames", default, skip_serializing_if = "Vec::is_empty")]
        pub managed_host_names: Vec<HostName>,
        pub consent: DomainPurchaseConsent,
        #[serde(rename = "domainNotRenewableReasons", default, skip_serializing_if = "Vec::is_empty")]
        pub domain_not_renewable_reasons: Vec<String>,
        #[serde(rename = "dnsType", default, skip_serializing_if = "Option::is_none")]
        pub dns_type: Option<properties::DnsType>,
        #[serde(rename = "dnsZoneId", default, skip_serializing_if = "Option::is_none")]
        pub dns_zone_id: Option<String>,
        #[serde(rename = "targetDnsType", default, skip_serializing_if = "Option::is_none")]
        pub target_dns_type: Option<properties::TargetDnsType>,
        #[serde(rename = "authCode", default, skip_serializing_if = "Option::is_none")]
        pub auth_code: Option<String>,
    }
    pub mod properties {
        use super::*;
        #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
        pub enum RegistrationStatus {
            Active,
            Awaiting,
            Cancelled,
            Confiscated,
            Disabled,
            Excluded,
            Expired,
            Failed,
            Held,
            Locked,
            Parked,
            Pending,
            Reserved,
            Reverted,
            Suspended,
            Transferred,
            Unknown,
            Unlocked,
            Unparked,
            Updated,
            JsonConverterFailed,
        }
        #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
        pub enum ProvisioningState {
            Succeeded,
            Failed,
            Canceled,
            InProgress,
            Deleting,
        }
        #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
        pub enum DnsType {
            AzureDns,
            DefaultDomainRegistrarDns,
        }
        #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
        pub enum TargetDnsType {
            AzureDns,
            DefaultDomainRegistrarDns,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DomainAvailablilityCheckResult {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub available: Option<bool>,
    #[serde(rename = "domainType", default, skip_serializing_if = "Option::is_none")]
    pub domain_type: Option<domain_availablility_check_result::DomainType>,
}
pub mod domain_availablility_check_result {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum DomainType {
        Regular,
        SoftDeleted,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DomainCollection {
    pub value: Vec<Domain>,
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DomainControlCenterSsoRequest {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(rename = "postParameterKey", default, skip_serializing_if = "Option::is_none")]
    pub post_parameter_key: Option<String>,
    #[serde(rename = "postParameterValue", default, skip_serializing_if = "Option::is_none")]
    pub post_parameter_value: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DomainOwnershipIdentifier {
    #[serde(flatten)]
    pub proxy_only_resource: ProxyOnlyResource,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<domain_ownership_identifier::Properties>,
}
pub mod domain_ownership_identifier {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub struct Properties {
        #[serde(rename = "ownershipId", default, skip_serializing_if = "Option::is_none")]
        pub ownership_id: Option<String>,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DomainOwnershipIdentifierCollection {
    pub value: Vec<DomainOwnershipIdentifier>,
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DomainPatchResource {
    #[serde(flatten)]
    pub proxy_only_resource: ProxyOnlyResource,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<domain_patch_resource::Properties>,
}
pub mod domain_patch_resource {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub struct Properties {
        #[serde(rename = "contactAdmin")]
        pub contact_admin: Contact,
        #[serde(rename = "contactBilling")]
        pub contact_billing: Contact,
        #[serde(rename = "contactRegistrant")]
        pub contact_registrant: Contact,
        #[serde(rename = "contactTech")]
        pub contact_tech: Contact,
        #[serde(rename = "registrationStatus", default, skip_serializing_if = "Option::is_none")]
        pub registration_status: Option<properties::RegistrationStatus>,
        #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
        pub provisioning_state: Option<properties::ProvisioningState>,
        #[serde(rename = "nameServers", default, skip_serializing_if = "Vec::is_empty")]
        pub name_servers: Vec<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub privacy: Option<bool>,
        #[serde(rename = "createdTime", default, skip_serializing_if = "Option::is_none")]
        pub created_time: Option<String>,
        #[serde(rename = "expirationTime", default, skip_serializing_if = "Option::is_none")]
        pub expiration_time: Option<String>,
        #[serde(rename = "lastRenewedTime", default, skip_serializing_if = "Option::is_none")]
        pub last_renewed_time: Option<String>,
        #[serde(rename = "autoRenew", default, skip_serializing_if = "Option::is_none")]
        pub auto_renew: Option<bool>,
        #[serde(rename = "readyForDnsRecordManagement", default, skip_serializing_if = "Option::is_none")]
        pub ready_for_dns_record_management: Option<bool>,
        #[serde(rename = "managedHostNames", default, skip_serializing_if = "Vec::is_empty")]
        pub managed_host_names: Vec<HostName>,
        pub consent: DomainPurchaseConsent,
        #[serde(rename = "domainNotRenewableReasons", default, skip_serializing_if = "Vec::is_empty")]
        pub domain_not_renewable_reasons: Vec<String>,
        #[serde(rename = "dnsType", default, skip_serializing_if = "Option::is_none")]
        pub dns_type: Option<properties::DnsType>,
        #[serde(rename = "dnsZoneId", default, skip_serializing_if = "Option::is_none")]
        pub dns_zone_id: Option<String>,
        #[serde(rename = "targetDnsType", default, skip_serializing_if = "Option::is_none")]
        pub target_dns_type: Option<properties::TargetDnsType>,
        #[serde(rename = "authCode", default, skip_serializing_if = "Option::is_none")]
        pub auth_code: Option<String>,
    }
    pub mod properties {
        use super::*;
        #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
        pub enum RegistrationStatus {
            Active,
            Awaiting,
            Cancelled,
            Confiscated,
            Disabled,
            Excluded,
            Expired,
            Failed,
            Held,
            Locked,
            Parked,
            Pending,
            Reserved,
            Reverted,
            Suspended,
            Transferred,
            Unknown,
            Unlocked,
            Unparked,
            Updated,
            JsonConverterFailed,
        }
        #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
        pub enum ProvisioningState {
            Succeeded,
            Failed,
            Canceled,
            InProgress,
            Deleting,
        }
        #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
        pub enum DnsType {
            AzureDns,
            DefaultDomainRegistrarDns,
        }
        #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
        pub enum TargetDnsType {
            AzureDns,
            DefaultDomainRegistrarDns,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DomainPurchaseConsent {
    #[serde(rename = "agreementKeys", default, skip_serializing_if = "Vec::is_empty")]
    pub agreement_keys: Vec<String>,
    #[serde(rename = "agreedBy", default, skip_serializing_if = "Option::is_none")]
    pub agreed_by: Option<String>,
    #[serde(rename = "agreedAt", default, skip_serializing_if = "Option::is_none")]
    pub agreed_at: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DomainRecommendationSearchParameters {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub keywords: Option<String>,
    #[serde(rename = "maxDomainRecommendations", default, skip_serializing_if = "Option::is_none")]
    pub max_domain_recommendations: Option<i32>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct HostName {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "siteNames", default, skip_serializing_if = "Vec::is_empty")]
    pub site_names: Vec<String>,
    #[serde(rename = "azureResourceName", default, skip_serializing_if = "Option::is_none")]
    pub azure_resource_name: Option<String>,
    #[serde(rename = "azureResourceType", default, skip_serializing_if = "Option::is_none")]
    pub azure_resource_type: Option<host_name::AzureResourceType>,
    #[serde(rename = "customHostNameDnsRecordType", default, skip_serializing_if = "Option::is_none")]
    pub custom_host_name_dns_record_type: Option<host_name::CustomHostNameDnsRecordType>,
    #[serde(rename = "hostNameType", default, skip_serializing_if = "Option::is_none")]
    pub host_name_type: Option<host_name::HostNameType>,
}
pub mod host_name {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum AzureResourceType {
        Website,
        TrafficManager,
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum CustomHostNameDnsRecordType {
        CName,
        A,
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum HostNameType {
        Verified,
        Managed,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct NameIdentifierCollection {
    pub value: Vec<NameIdentifier>,
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ErrorResponse {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TldLegalAgreement {
    #[serde(rename = "agreementKey")]
    pub agreement_key: String,
    pub title: String,
    pub content: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TldLegalAgreementCollection {
    pub value: Vec<TldLegalAgreement>,
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TopLevelDomain {
    #[serde(flatten)]
    pub proxy_only_resource: ProxyOnlyResource,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<top_level_domain::Properties>,
}
pub mod top_level_domain {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub struct Properties {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub name: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub privacy: Option<bool>,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TopLevelDomainAgreementOption {
    #[serde(rename = "includePrivacy", default, skip_serializing_if = "Option::is_none")]
    pub include_privacy: Option<bool>,
    #[serde(rename = "forTransfer", default, skip_serializing_if = "Option::is_none")]
    pub for_transfer: Option<bool>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TopLevelDomainCollection {
    pub value: Vec<TopLevelDomain>,
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct NameIdentifier {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Resource {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
    pub location: String,
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ProxyOnlyResource {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CsmOperationCollection {
    pub value: Vec<CsmOperationDescription>,
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CsmOperationDescription {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub display: Option<CsmOperationDisplay>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub origin: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<CsmOperationDescriptionProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CsmOperationDisplay {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub provider: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resource: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub operation: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CsmOperationDescriptionProperties {
    #[serde(rename = "serviceSpecification", default, skip_serializing_if = "Option::is_none")]
    pub service_specification: Option<ServiceSpecification>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ServiceSpecification {
    #[serde(rename = "metricSpecifications", default, skip_serializing_if = "Vec::is_empty")]
    pub metric_specifications: Vec<MetricSpecification>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MetricSpecification {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[serde(rename = "displayDescription", default, skip_serializing_if = "Option::is_none")]
    pub display_description: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub unit: Option<String>,
    #[serde(rename = "aggregationType", default, skip_serializing_if = "Option::is_none")]
    pub aggregation_type: Option<String>,
    #[serde(rename = "supportsInstanceLevelAggregation", default, skip_serializing_if = "Option::is_none")]
    pub supports_instance_level_aggregation: Option<bool>,
    #[serde(rename = "enableRegionalMdmAccount", default, skip_serializing_if = "Option::is_none")]
    pub enable_regional_mdm_account: Option<bool>,
    #[serde(rename = "sourceMdmAccount", default, skip_serializing_if = "Option::is_none")]
    pub source_mdm_account: Option<String>,
    #[serde(rename = "sourceMdmNamespace", default, skip_serializing_if = "Option::is_none")]
    pub source_mdm_namespace: Option<String>,
    #[serde(rename = "metricFilterPattern", default, skip_serializing_if = "Option::is_none")]
    pub metric_filter_pattern: Option<String>,
    #[serde(rename = "fillGapWithZero", default, skip_serializing_if = "Option::is_none")]
    pub fill_gap_with_zero: Option<bool>,
    #[serde(rename = "isInternal", default, skip_serializing_if = "Option::is_none")]
    pub is_internal: Option<bool>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub dimensions: Vec<Dimension>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub availabilities: Vec<MetricAvailability>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Dimension {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[serde(rename = "internalName", default, skip_serializing_if = "Option::is_none")]
    pub internal_name: Option<String>,
    #[serde(rename = "toBeExportedForShoebox", default, skip_serializing_if = "Option::is_none")]
    pub to_be_exported_for_shoebox: Option<bool>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MetricAvailability {
    #[serde(rename = "timeGrain", default, skip_serializing_if = "Option::is_none")]
    pub time_grain: Option<String>,
    #[serde(rename = "blobDuration", default, skip_serializing_if = "Option::is_none")]
    pub blob_duration: Option<String>,
}