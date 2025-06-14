#[cfg(any(feature = "google-rest-bigquery-v2"))]
pub mod bigquery_v2;

#[cfg(any(feature = "google-rest-cloudresourcemanager-v3"))]
pub mod cloudresourcemanager_v3;

#[cfg(any(feature = "google-rest-compute-v1"))]
pub mod compute_v1;

#[cfg(any(feature = "google-rest-dns-v1"))]
pub mod dns_v1;

#[cfg(any(feature = "google-rest-fcm-v1"))]
pub mod fcm_v1;

#[cfg(any(feature = "google-rest-identitytoolkit-v3"))]
pub mod identitytoolkit_v3;

#[cfg(any(feature = "google-rest-lustre-v1"))]
pub mod lustre_v1;

#[cfg(any(feature = "google-rest-servicecontrol-v1"))]
pub mod servicecontrol_v1;

#[cfg(any(feature = "google-rest-servicecontrol-v2"))]
pub mod servicecontrol_v2;

#[cfg(any(feature = "google-rest-sqladmin-v1"))]
pub mod sqladmin_v1;

#[cfg(any(feature = "google-rest-storage-v1"))]
pub mod storage_v1;
