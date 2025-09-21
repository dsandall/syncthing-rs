use crate::Fallible;
use crate::event_stream::EventStream;
use anyhow::bail;
use http::header::HeaderValue;
use http::uri::{Authority, Parts as UriParts, PathAndQuery, Scheme, Uri};
use http::{Method, Request};
use serde::de::DeserializeOwned as Deserialize;
use std::collections::HashMap;
use syncthing_types::events::{Event, EventType};
use syncthing_types::utils;
use syncthing_types::{API_DEFAULT_AUTHORITY, Timestamp};
use syncthing_types::{API_HEADER_KEY, routes::*};
use syncthing_types::{EMPTY_EVENT_SUBSCRIPTION, system};
use ureq::Agent;

pub struct Client {
    agent: Agent,
    authority: Authority,
    api_key: String,
}

impl Client {
    pub fn new(api_key: impl Into<String>) -> Self {
        Self {
            agent: Agent::new_with_defaults(),
            authority: Authority::from_static(API_DEFAULT_AUTHORITY),
            api_key: api_key.into(),
        }
    }

    pub fn with_authority(api_key: impl Into<String>, authority: Authority) -> Fallible<Self> {
        Ok(Self {
            agent: Agent::new_with_defaults(),
            authority,
            api_key: api_key.into(),
        })
    }

    pub(crate) fn request<D: Deserialize, T: AsRef<[u8]> + 'static>(
        &self,
        method: Method,
        path_and_query: T,
    ) -> Fallible<D> {
        let mut uri_parts = UriParts::default();
        uri_parts.authority = Some(self.authority.clone());
        uri_parts.scheme = Some(Scheme::HTTP);
        uri_parts.path_and_query = Some(PathAndQuery::from_maybe_shared(path_and_query)?);
        let uri = Uri::from_parts(uri_parts)?;
        let mut request = Request::new(());
        *request.uri_mut() = uri;
        *request.method_mut() = method;
        request
            .headers_mut()
            .insert(API_HEADER_KEY, HeaderValue::from_str(&self.api_key)?);
        let mut resp = self.agent.run(request)?;
        let status_code = resp.status().as_u16();
        if status_code < 200 || status_code > 299 {
            bail!(
                "got http status code '{}' with following response body:\n {}",
                status_code,
                resp.body_mut().read_to_string()?
            )
        } else {
            Ok(serde_json::from_reader(resp.body_mut().as_reader())?)
        }
    }

    pub fn get_all_events(&self, since: Option<u64>, limit: Option<u64>) -> Fallible<Vec<Event>> {
        self.get_events(since, limit, &EMPTY_EVENT_SUBSCRIPTION)
    }

    pub fn get_events(
        &self,
        since: Option<u64>,
        limit: Option<u64>,
        events: impl AsRef<[EventType]>,
    ) -> Fallible<Vec<Event>> {
        let path_and_query = utils::construct_event_url(since, limit, events)?;
        self.request(Method::GET, path_and_query)
    }

    pub fn subscribe_to(self, events: impl Into<Vec<EventType>>) -> EventStream {
        EventStream::new(self, events.into())
    }

    pub fn subscribe_to_all(self) -> EventStream {
        EventStream::new(self, EMPTY_EVENT_SUBSCRIPTION.clone())
    }

    pub fn browse(&self, pattern: Option<String>) -> Fallible<Vec<String>> {
        if let Some(pattern) = pattern {
            self.request(
                Method::GET,
                format!("{}?current={}", SYSTEM_BROWSE, pattern),
            )
        } else {
            self.request(Method::GET, SYSTEM_BROWSE)
        }
    }

    pub fn get_connections(&self) -> Fallible<system::Connections> {
        self.request(Method::GET, SYSTEM_CONNECTIONS)
    }

    pub fn get_discovery_cache(&self) -> Fallible<system::Discovery> {
        self.request(Method::GET, SYSTEM_DISCOVERY)
    }

    pub fn get_log(&self, since: Option<Timestamp>) -> Fallible<system::Log> {
        if let Some(since) = since {
            self.request(
                Method::GET,
                format!("{}?since={}", SYSTEM_LOG, since.to_rfc3339()),
            )
        } else {
            self.request(Method::GET, SYSTEM_LOG)
        }
    }

    pub fn get_errors(&self) -> Fallible<system::Error> {
        self.request(Method::GET, SYSTEM_ERROR)
    }

    pub fn get_loglevels_info(&self) -> Fallible<system::LogLevelsInfo> {
        self.request(Method::GET, SYSTEM_LOGLEVELS)
    }

    pub fn get_paths(&self) -> Fallible<HashMap<String, String>> {
        self.request(Method::GET, SYSTEM_PATHS)
    }

    pub fn ping(&self) -> Fallible<system::Ping> {
        self.request(Method::GET, SYSTEM_PING)
    }

    pub fn get_upgrade_info(&self) -> Fallible<system::UpgradeInfo> {
        self.request(Method::GET, SYSTEM_UPGRADE)
    }

    pub fn get_version_info(&self) -> Fallible<system::VersionInfo> {
        self.request(Method::GET, SYSTEM_VERSION)
    }
}
