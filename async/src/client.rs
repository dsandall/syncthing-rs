use crate::Fallible;
use crate::event_stream::EventStream;
use http::Method;
use http::header::HeaderValue;
use http::uri::{Authority, Parts as UriParts, PathAndQuery, Scheme, Uri};
use reqwest::Client as HttpClient;
use serde::de::DeserializeOwned as Deserialize;
use std::collections::HashMap;
use syncthing_types::events::{Event, EventType};
use syncthing_types::utils;
use syncthing_types::{API_DEFAULT_AUTHORITY, Timestamp};
use syncthing_types::{API_HEADER_KEY, routes::*};
use syncthing_types::{EMPTY_EVENT_SUBSCRIPTION, system};

pub struct Client {
    client: HttpClient,
    authority: Authority,
    api_key: String,
}

impl Client {
    pub fn new(api_key: impl Into<String>) -> Self {
        Self {
            client: HttpClient::new(),
            authority: Authority::from_static(API_DEFAULT_AUTHORITY),
            api_key: api_key.into(),
        }
    }

    pub fn with_authority(api_key: impl Into<String>, authority: Authority) -> Self {
        Self {
            client: HttpClient::new(),
            authority,
            api_key: api_key.into(),
        }
    }

    pub(crate) async fn request<D: Deserialize, T: AsRef<[u8]> + 'static>(
        &self,
        method: Method,
        path_and_query: T,
    ) -> Fallible<D> {
        let mut uri_parts = UriParts::default();
        uri_parts.authority = Some(self.authority.clone());
        uri_parts.scheme = Some(Scheme::HTTP);
        uri_parts.path_and_query = Some(PathAndQuery::from_maybe_shared(path_and_query)?);
        let uri = Uri::from_parts(uri_parts)?;
        let resp = self
            .client
            .request(method, uri.to_string())
            .header(API_HEADER_KEY, HeaderValue::from_str(&self.api_key)?)
            .send()
            .await?
            .error_for_status()?;
        Ok(resp.json().await?)
    }

    pub async fn get_all_events(
        &self,
        since: Option<u64>,
        limit: Option<u64>,
    ) -> Fallible<Vec<Event>> {
        self.get_events(since, limit, &EMPTY_EVENT_SUBSCRIPTION)
            .await
    }

    pub async fn get_events(
        &self,
        since: Option<u64>,
        limit: Option<u64>,
        events: impl AsRef<[EventType]>,
    ) -> Fallible<Vec<Event>> {
        let path_and_query = utils::construct_event_url(since, limit, events)?;
        self.request(Method::GET, path_and_query).await
    }

    pub fn subscribe_to(self, events: impl Into<Vec<EventType>>) -> EventStream {
        EventStream::new(self, events.into())
    }

    pub fn subscribe_to_all(self) -> EventStream {
        EventStream::new(self, EMPTY_EVENT_SUBSCRIPTION.clone())
    }

    pub async fn browse(&self, pattern: Option<String>) -> Fallible<Vec<String>> {
        if let Some(pattern) = pattern {
            self.request(
                Method::GET,
                format!("{}?current={}", SYSTEM_BROWSE, pattern),
            )
            .await
        } else {
            self.request(Method::GET, SYSTEM_BROWSE).await
        }
    }

    pub async fn get_connections(&self) -> Fallible<system::Connections> {
        self.request(Method::GET, SYSTEM_CONNECTIONS).await
    }

    pub async fn get_discovery_cache(&self) -> Fallible<system::Discovery> {
        self.request(Method::GET, SYSTEM_DISCOVERY).await
    }

    pub async fn get_log(&self, since: Option<Timestamp>) -> Fallible<system::Log> {
        if let Some(since) = since {
            self.request(
                Method::GET,
                format!("{}?since={}", SYSTEM_LOG, since.to_rfc3339()),
            )
            .await
        } else {
            self.request(Method::GET, SYSTEM_LOG).await
        }
    }

    pub async fn get_errors(&self) -> Fallible<system::Error> {
        self.request(Method::GET, SYSTEM_ERROR).await
    }

    pub async fn get_loglevels_info(&self) -> Fallible<system::LogLevelsInfo> {
        self.request(Method::GET, SYSTEM_LOGLEVELS).await
    }

    pub async fn get_paths(&self) -> Fallible<HashMap<String, String>> {
        self.request(Method::GET, SYSTEM_PATHS).await
    }

    pub async fn ping(&self) -> Fallible<system::Ping> {
        self.request(Method::GET, SYSTEM_PING).await
    }

    pub async fn get_upgrade_info(&self) -> Fallible<system::UpgradeInfo> {
        self.request(Method::GET, SYSTEM_UPGRADE).await
    }

    pub async fn get_version_info(&self) -> Fallible<system::VersionInfo> {
        self.request(Method::GET, SYSTEM_VERSION).await
    }
}
