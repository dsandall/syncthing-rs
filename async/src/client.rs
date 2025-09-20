use crate::Fallible;
use crate::events::EventStream;
use http::Method;
use http::header::HeaderValue;
use http::uri::{Authority, Parts as UriParts, PathAndQuery, Scheme, Uri};
use reqwest::Client as HttpClient;
use serde::de::DeserializeOwned as Deserialize;
use syncthing_types::events::{Event, EventType};
use syncthing_types::routes::*;
use syncthing_types::system;
use syncthing_types::utils;

static API_HEADER_KEY: &str = "X-API-Key";
static API_DEFAULT_AUTHORITY: &str = "127.0.0.1:8384";
static EMPTY_EVENT_SUBSCRIPTION: Vec<EventType> = Vec::new();

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

    pub async fn get_system_browse(&self, pattern: Option<String>) -> Fallible<Vec<String>> {
        if let Some(pattern) = pattern {
            self.request(
                Method::GET,
                format!("{}?current={}", SYSTEM_BROWSE_PATH, pattern),
            )
            .await
        } else {
            self.request(Method::GET, SYSTEM_BROWSE_PATH).await
        }
    }

    pub async fn get_system_connections(&self) -> Fallible<system::Connections> {
        self.request(Method::GET, SYSTEM_CONNECTIONS_PATH).await
    }

    pub async fn get_system_discovery(&self) -> Fallible<system::Discovery> {
        self.request(Method::GET, SYSTEM_DISCOVERY_PATH).await
    }

    pub async fn get_system_log(&self) -> Fallible<system::Log> {
        self.request(Method::GET, SYSTEM_LOG_PATH).await
    }

    pub async fn get_system_error(&self) -> Fallible<system::Error> {
        self.request(Method::GET, SYSTEM_ERROR_PATH).await
    }

    pub async fn get_system_ping(&self) -> Fallible<system::Ping> {
        self.request(Method::GET, SYSTEM_PING_PATH).await
    }

    pub async fn get_system_upgrade(&self) -> Fallible<system::UpgradeInfo> {
        self.request(Method::GET, SYSTEM_UPGRADE_PATH).await
    }

    pub async fn get_system_version(&self) -> Fallible<system::Version> {
        self.request(Method::GET, SYSTEM_VERSION_PATH).await
    }
}
