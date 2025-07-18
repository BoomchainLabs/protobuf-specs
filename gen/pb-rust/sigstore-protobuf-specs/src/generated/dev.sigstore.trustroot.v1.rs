// This file is @generated by prost-build.
/// TransparencyLogInstance describes the immutable parameters from a
/// transparency log.
/// See <https://www.rfc-editor.org/rfc/rfc9162.html#name-log-parameters>
/// for more details.
/// The included parameters are the minimal set required to identify a log,
/// and verify an inclusion proof/promise.
#[derive(
    sigstore_protobuf_specs_derive::Deserialize_proto,
    sigstore_protobuf_specs_derive::Serialize_proto
)]
#[derive(::prost_reflect::ReflectMessage)]
#[prost_reflect(message_name = "dev.sigstore.trustroot.v1.TransparencyLogInstance")]
#[prost_reflect(file_descriptor_set_bytes = "crate::FILE_DESCRIPTOR_SET_BYTES")]
#[derive(Clone, PartialEq, Eq, Hash, ::prost::Message)]
pub struct TransparencyLogInstance {
    /// The base URL at which can be used to URLs for the client.
    /// SHOULD match the origin on the log checkpoint:
    /// <https://github.com/C2SP/C2SP/blob/main/tlog-checkpoint.md#note-text.>
    #[prost(string, tag = "1")]
    pub base_url: ::prost::alloc::string::String,
    /// The hash algorithm used for the Merkle Tree.
    #[prost(enumeration = "super::super::common::v1::HashAlgorithm", tag = "2")]
    pub hash_algorithm: i32,
    /// The public key used to verify signatures generated by the log.
    /// This attribute contains the signature algorithm used by the log.
    #[prost(message, optional, tag = "3")]
    pub public_key: ::core::option::Option<super::super::common::v1::PublicKey>,
    /// The identifier for this transparency log.
    /// Represented as the SHA-256 hash of the log's public key,
    /// calculated over the DER encoding of the key represented as
    /// SubjectPublicKeyInfo.
    /// See <https://www.rfc-editor.org/rfc/rfc6962#section-3.2>
    /// For Rekor v2 instances, log_id and checkpoint_key_id will be set
    /// to the same value.
    /// It is recommended to use checkpoint_key_id instead, since log_id is not
    /// guaranteed to be unique across multiple deployments. Clients
    /// must use the key name and key ID, as defined by the signed-note spec
    /// linked below, from a checkpoint to determine the correct
    /// TransparencyLogInstance to verify a proof.
    /// log_id will eventually be deprecated in favor of checkpoint_id.
    #[prost(message, optional, tag = "4")]
    pub log_id: ::core::option::Option<super::super::common::v1::LogId>,
    /// The unique identifier for the log, used in the checkpoint.
    /// Only supported for TrustedRoot media types matching or greater than
    /// application/vnd.dev.sigstore.trustedroot.v0.2+json
    /// Its calculation is described in
    /// <https://github.com/C2SP/C2SP/blob/main/signed-note.md#signatures>
    /// SHOULD be set for all logs. When not set, clients MUST use log_id.
    ///
    /// For Ed25519 signatures, the key ID is computed per the C2SP spec:
    /// key ID = SHA-256(key name || 0x0A || 0x01 || 32-byte Ed25519 public key)\[:4\]
    /// For ECDSA signatures, the key ID is computed per the C2SP spec:
    /// key ID = SHA-256(PKIX ASN.1 DER-encoded public key, in SubjectPublicKeyInfo format)\[:4\]
    /// For RSA signatures, the signature type will be 0xff with an appended identifier for the format,
    /// "PKIX-RSA-PKCS#1v1.5":
    /// key ID = SHA-256(key name || 0x0A || 0xff || PKIX-RSA-PKCS#1v1.5 || PKIX ASN.1 DER-encoded public key)\[:4\]
    ///
    /// This is provided for convenience. Clients can also calculate the
    /// checkpoint key ID given the log's public key.
    /// SHOULD be 4 bytes long, as a truncated hash.
    ///
    /// To find a matching TransparencyLogInstance in the TrustedRoot,
    /// clients will parse the checkpoint, and for each signature line,
    /// use the key name (i.e. log origin, base_url from TrustedRoot)
    /// and checkpoint key ID (i.e. checkpoint_key_id from TrustedRoot)
    /// which can then be compared against the TrustedRoot log instances.
    #[prost(message, optional, tag = "5")]
    pub checkpoint_key_id: ::core::option::Option<super::super::common::v1::LogId>,
    /// The name of the operator of this log deployment. Operator MUST be
    /// formatted as a scheme-less URI, e.g. sigstore.dev
    /// Only supported for TrustedRoot media types matching or greater than
    /// application/vnd.dev.sigstore.trustedroot.v0.2+json
    /// This MUST be used when there are multiple transparency log instances
    /// to determine if log proof verification meets a specified threshold,
    /// e.g. two proofs from log deployments operated by the same operator
    /// should count as only one valid proof.
    #[prost(string, tag = "6")]
    pub operator: ::prost::alloc::string::String,
}
/// CertificateAuthority enlists the information required to identify which
/// CA to use and perform signature verification.
#[derive(
    sigstore_protobuf_specs_derive::Deserialize_proto,
    sigstore_protobuf_specs_derive::Serialize_proto
)]
#[derive(::prost_reflect::ReflectMessage)]
#[prost_reflect(message_name = "dev.sigstore.trustroot.v1.CertificateAuthority")]
#[prost_reflect(file_descriptor_set_bytes = "crate::FILE_DESCRIPTOR_SET_BYTES")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CertificateAuthority {
    /// The root certificate MUST be self-signed, and so the subject and
    /// issuer are the same.
    #[prost(message, optional, tag = "1")]
    pub subject: ::core::option::Option<super::super::common::v1::DistinguishedName>,
    /// The URI identifies the certificate authority.
    ///
    /// It is RECOMMENDED that the URI is the base URL for the certificate
    /// authority, that can be provided to any SDK/client provided
    /// by the certificate authority to interact with the certificate
    /// authority.
    #[prost(string, tag = "2")]
    pub uri: ::prost::alloc::string::String,
    /// The certificate chain for this CA. The last certificate in the chain
    /// MUST be the trust anchor. The trust anchor MAY be a self-signed root
    /// CA certificate or MAY be an intermediate CA certificate.
    #[prost(message, optional, tag = "3")]
    pub cert_chain: ::core::option::Option<
        super::super::common::v1::X509CertificateChain,
    >,
    /// The time the *entire* chain was valid. This is at max the
    /// longest interval when *all* certificates in the chain were valid,
    /// but it MAY be shorter. Clients MUST check timestamps against *both*
    /// the `valid_for` time range *and* the entire certificate chain.
    ///
    /// The TimeRange should be considered valid *inclusive* of the
    /// endpoints.
    #[prost(message, optional, tag = "4")]
    pub valid_for: ::core::option::Option<super::super::common::v1::TimeRange>,
    /// The name of the operator of this certificate or timestamp authority.
    /// Operator MUST be formatted as a scheme-less URI, e.g. sigstore.dev
    /// This MUST be used when there are multiple timestamp authorities to
    /// determine if the signed timestamp verification meets a specified
    /// threshold, e.g. two signed timestamps from timestamp authorities
    /// operated by the same operator should count as only one valid
    /// timestamp.
    #[prost(string, tag = "5")]
    pub operator: ::prost::alloc::string::String,
}
/// TrustedRoot describes the client's complete set of trusted entities.
/// How the TrustedRoot is populated is not specified, but can be a
/// combination of many sources such as TUF repositories, files on disk etc.
///
/// The TrustedRoot is not meant to be used for any artifact verification, only
/// to capture the complete/global set of trusted verification materials.
/// When verifying an artifact, based on the artifact and policies, a selection
/// of keys/authorities are expected to be extracted and provided to the
/// verification function. This way the set of keys/authorities can be kept to
/// a minimal set by the policy to gain better control over what signatures
/// that are allowed.
///
/// The embedded transparency logs, CT logs, CAs and TSAs MUST include any
/// previously used instance -- otherwise signatures made in the past cannot
/// be verified.
///
/// All the listed instances SHOULD be sorted by the 'valid_for' in ascending
/// order, that is, the oldest instance first. Only the last instance is
/// allowed to have their 'end' timestamp unset. All previous instances MUST
/// have a closed interval of validity. The last instance MAY have a closed
/// interval. Clients MUST accept instances that overlaps in time, if not
/// clients may experience problems during rotations of verification
/// materials.
///
/// To be able to manage planned rotations of either transparency logs or
/// certificate authorities, clienst MUST accept lists of instances where
/// the last instance have a 'valid_for' that belongs to the future.
/// This should not be a problem as clients SHOULD first seek the trust root
/// for a suitable instance before creating a per artifact trust root (that
/// is, a sub-set of the complete trust root) that is used for verification.
#[derive(
    sigstore_protobuf_specs_derive::Deserialize_proto,
    sigstore_protobuf_specs_derive::Serialize_proto
)]
#[derive(::prost_reflect::ReflectMessage)]
#[prost_reflect(message_name = "dev.sigstore.trustroot.v1.TrustedRoot")]
#[prost_reflect(file_descriptor_set_bytes = "crate::FILE_DESCRIPTOR_SET_BYTES")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TrustedRoot {
    /// MUST be application/vnd.dev.sigstore.trustedroot.v0.2+json
    /// when encoded as JSON.
    /// Clients MAY choose to also support
    /// application/vnd.dev.sigstore.trustedroot.v0.1+json
    /// Clients MAY process and parse content with the media type defined
    /// in the old format:
    /// application/vnd.dev.sigstore.trustedroot+json;version=0.1
    #[prost(string, tag = "1")]
    pub media_type: ::prost::alloc::string::String,
    /// A set of trusted Rekor servers.
    #[prost(message, repeated, tag = "2")]
    pub tlogs: ::prost::alloc::vec::Vec<TransparencyLogInstance>,
    /// A set of trusted certificate authorities (e.g Fulcio), and any
    /// intermediate certificates they provide.
    /// If a CA is issuing multiple intermediate certificate, each
    /// combination shall be represented as separate chain. I.e, a single
    /// root cert may appear in multiple chains but with different
    /// intermediate and/or leaf certificates.
    /// The certificates are intended to be used for verifying artifact
    /// signatures.
    #[prost(message, repeated, tag = "3")]
    pub certificate_authorities: ::prost::alloc::vec::Vec<CertificateAuthority>,
    /// A set of trusted certificate transparency logs.
    #[prost(message, repeated, tag = "4")]
    pub ctlogs: ::prost::alloc::vec::Vec<TransparencyLogInstance>,
    /// A set of trusted timestamping authorities.
    #[prost(message, repeated, tag = "5")]
    pub timestamp_authorities: ::prost::alloc::vec::Vec<CertificateAuthority>,
}
/// SigningConfig represents the trusted entities/state needed by Sigstore
/// signing. In particular, it primarily contains service URLs that a Sigstore
/// signer may need to connect to for the online aspects of signing.
#[derive(
    sigstore_protobuf_specs_derive::Deserialize_proto,
    sigstore_protobuf_specs_derive::Serialize_proto
)]
#[derive(::prost_reflect::ReflectMessage)]
#[prost_reflect(message_name = "dev.sigstore.trustroot.v1.SigningConfig")]
#[prost_reflect(file_descriptor_set_bytes = "crate::FILE_DESCRIPTOR_SET_BYTES")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SigningConfig {
    /// MUST be application/vnd.dev.sigstore.signingconfig.v0.2+json
    /// Clients MAY choose to also support
    /// application/vnd.dev.sigstore.signingconfig.v0.1+json
    #[prost(string, tag = "5")]
    pub media_type: ::prost::alloc::string::String,
    /// URLs to Fulcio-compatible CAs, capable of receiving
    /// Certificate Signing Requests (CSRs) and responding with
    /// issued certificates.
    ///
    /// These URLs MUST be the "base" URL for the CAs, which clients
    /// should construct an appropriate CSR endpoint on top of.
    /// For example, if a CA URL is `<https://example.com/ca`,> then
    /// the client MAY construct the CSR endpoint as
    /// `<https://example.com/ca/api/v2/signingCert`.>
    ///
    /// Clients MUST select only one Service with the highest API version
    /// that the client is compatible with, that is within its
    /// validity period, and has the newest validity start date.
    /// Client SHOULD select the first Service that meets this requirement.
    /// All listed Services SHOULD be sorted by the `valid_for` window in
    /// descending order, with the newest instance first.
    #[prost(message, repeated, tag = "6")]
    pub ca_urls: ::prost::alloc::vec::Vec<Service>,
    /// URLs to OpenID Connect identity providers.
    ///
    /// These URLs MUST be the "base" URLs for the OIDC IdPs, which clients
    /// should perform well-known OpenID Connect discovery against.
    ///
    /// Clients MUST select only one Service with the highest API version
    /// that the client is compatible with, that is within its
    /// validity period, and has the newest validity start date.
    /// Client SHOULD select the first Service that meets this requirement.
    /// All listed Services SHOULD be sorted by the `valid_for` window in
    /// descending order, with the newest instance first.
    #[prost(message, repeated, tag = "7")]
    pub oidc_urls: ::prost::alloc::vec::Vec<Service>,
    /// URLs to Rekor transparency logs.
    ///
    /// These URL MUST be the "base" URLs for the transparency logs,
    /// which clients should construct appropriate API endpoints on top of.
    ///
    /// Clients MUST group Services by `operator` and select at most one
    /// Service from each operator. Clients MUST select Services with the
    /// highest API version that the client is compatible with, that are
    /// within its validity period, and have the newest validity start dates.
    /// All listed Services SHOULD be sorted by the `valid_for` window in
    /// descending order, with the newest instance first.
    ///
    /// Clients MUST select Services based on the selector value of
    /// `rekor_tlog_config`.
    #[prost(message, repeated, tag = "8")]
    pub rekor_tlog_urls: ::prost::alloc::vec::Vec<Service>,
    /// Specifies how a client should select the set of Rekor transparency
    /// logs to write to.
    #[prost(message, optional, tag = "9")]
    pub rekor_tlog_config: ::core::option::Option<ServiceConfiguration>,
    /// URLs to RFC 3161 Time Stamping Authorities (TSA).
    ///
    /// These URLs MUST be the *full* URL for the TSA, meaning that it
    /// should be suitable for submitting Time Stamp Requests (TSRs) to
    /// via HTTP, per RFC 3161.
    ///
    /// Clients MUST group Services by `operator` and select at most one
    /// Service from each operator. Clients MUST select Services with the
    /// highest API version that the client is compatible with, that are
    /// within its validity period, and have the newest validity start dates.
    /// All listed Services SHOULD be sorted by the `valid_for` window in
    /// descending order, with the newest instance first.
    ///
    /// Clients MUST select Services based on the selector value of
    /// `tsa_config`.
    #[prost(message, repeated, tag = "10")]
    pub tsa_urls: ::prost::alloc::vec::Vec<Service>,
    /// Specifies how a client should select the set of TSAs to request
    /// signed timestamps from.
    #[prost(message, optional, tag = "11")]
    pub tsa_config: ::core::option::Option<ServiceConfiguration>,
}
/// Service represents an instance of a service that is a part of Sigstore infrastructure.
/// When selecting one or multiple services from a list of services, clients MUST:
/// * Use the API version hint to determine the service with the highest API version
///    that the client is compatible with.
/// * Only select services within the specified validity period and that have the
///    newest validity start date.
/// When selecting multiple services, clients MUST:
/// * Use the ServiceConfiguration to determine how many services MUST be selected.
///    Clients MUST return an error if there are not enough services that meet the
///    selection criteria.
/// * Group services by `operator` and select at most one service from an operator.
///    During verification, clients MUST treat valid verification metadata from the
///    operator as valid only once towards a threshold.
/// * Select services from only the highest supported API version.
#[derive(
    sigstore_protobuf_specs_derive::Deserialize_proto,
    sigstore_protobuf_specs_derive::Serialize_proto
)]
#[derive(::prost_reflect::ReflectMessage)]
#[prost_reflect(message_name = "dev.sigstore.trustroot.v1.Service")]
#[prost_reflect(file_descriptor_set_bytes = "crate::FILE_DESCRIPTOR_SET_BYTES")]
#[derive(Clone, PartialEq, Eq, Hash, ::prost::Message)]
pub struct Service {
    /// URL of the service. MUST include scheme and authority. MAY include path.
    #[prost(string, tag = "1")]
    pub url: ::prost::alloc::string::String,
    /// Specifies the major API version. A value of 0 represents a service that
    /// has not yet been released.
    #[prost(uint32, tag = "2")]
    pub major_api_version: u32,
    /// Validity period of a service. A service that has only a start date
    /// SHOULD be considered the most recent instance of that service, but
    /// the client MUST NOT assume there is only one valid instance.
    /// The TimeRange MUST be considered valid *inclusive* of the
    /// endpoints.
    #[prost(message, optional, tag = "3")]
    pub valid_for: ::core::option::Option<super::super::common::v1::TimeRange>,
    /// Specifies the name of the service operator. When selecting multiple
    /// services, clients MUST use the operator to select services from
    /// distinct operators. Operator MUST be formatted as a scheme-less
    /// URI, e.g. sigstore.dev
    #[prost(string, tag = "4")]
    pub operator: ::prost::alloc::string::String,
}
/// ServiceConfiguration specifies how a client should select a set of
/// Services to connect to, along with a count when a specific number
/// of Services is requested.
#[derive(
    sigstore_protobuf_specs_derive::Deserialize_proto,
    sigstore_protobuf_specs_derive::Serialize_proto
)]
#[derive(::prost_reflect::ReflectMessage)]
#[prost_reflect(message_name = "dev.sigstore.trustroot.v1.ServiceConfiguration")]
#[prost_reflect(file_descriptor_set_bytes = "crate::FILE_DESCRIPTOR_SET_BYTES")]
#[derive(Clone, Copy, PartialEq, Eq, Hash, ::prost::Message)]
pub struct ServiceConfiguration {
    /// How a client should select a set of Services to connect to.
    /// Clients SHOULD NOT select services from multiple API versions.
    #[prost(enumeration = "ServiceSelector", tag = "1")]
    pub selector: i32,
    /// count specifies the number of Services the client should use.
    /// Only used when selector is set to EXACT, and count MUST be greater
    /// than 0. count MUST be less than or equal to the number of Services.
    /// Clients MUST return an error is there are not enough services
    /// that meet selection criteria.
    #[prost(uint32, tag = "2")]
    pub count: u32,
}
/// ClientTrustConfig describes the complete state needed by a client
/// to perform both signing and verification operations against a particular
/// instance of Sigstore.
#[derive(
    sigstore_protobuf_specs_derive::Deserialize_proto,
    sigstore_protobuf_specs_derive::Serialize_proto
)]
#[derive(::prost_reflect::ReflectMessage)]
#[prost_reflect(message_name = "dev.sigstore.trustroot.v1.ClientTrustConfig")]
#[prost_reflect(file_descriptor_set_bytes = "crate::FILE_DESCRIPTOR_SET_BYTES")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ClientTrustConfig {
    /// MUST be application/vnd.dev.sigstore.clienttrustconfig.v0.1+json
    #[prost(string, tag = "1")]
    pub media_type: ::prost::alloc::string::String,
    /// The root of trust, which MUST be present.
    #[prost(message, optional, tag = "2")]
    pub trusted_root: ::core::option::Option<TrustedRoot>,
    /// Configuration for signing clients, which MUST be present.
    #[prost(message, optional, tag = "3")]
    pub signing_config: ::core::option::Option<SigningConfig>,
}
/// ServiceSelector specifies how a client SHOULD select a set of
/// Services to connect to. A client SHOULD throw an error if
/// the value is SERVICE_SELECTOR_UNDEFINED.
#[derive(
    sigstore_protobuf_specs_derive::Deserialize_proto,
    sigstore_protobuf_specs_derive::Serialize_proto
)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ServiceSelector {
    Undefined = 0,
    /// Clients SHOULD select all Services based on supported API version
    /// and validity window.
    All = 1,
    /// Clients SHOULD select one Service based on supported API version
    /// and validity window. It is up to the client implementation to
    /// decide how to select the Service, e.g. random or round-robin.
    Any = 2,
    /// Clients SHOULD select a specific number of Services based on
    /// supported API version and validity window, using the provided
    /// `count`. It is up to the client implementation to decide how to
    /// select the Service, e.g. random or round-robin.
    Exact = 3,
}
impl ServiceSelector {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            Self::Undefined => "SERVICE_SELECTOR_UNDEFINED",
            Self::All => "ALL",
            Self::Any => "ANY",
            Self::Exact => "EXACT",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "SERVICE_SELECTOR_UNDEFINED" => Some(Self::Undefined),
            "ALL" => Some(Self::All),
            "ANY" => Some(Self::Any),
            "EXACT" => Some(Self::Exact),
            _ => None,
        }
    }
}
