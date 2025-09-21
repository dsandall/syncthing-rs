use syncthing_types::Timestamp;

use crate::{Client, Fallible};

static API_KEY: &str = include_str!("../../api.key");

#[test]
fn browse() -> Fallible<()> {
    let client = Client::new(API_KEY);
    dbg!(client.browse(None)?);
    Ok(())
}

#[test]
fn get_connections() -> Fallible<()> {
    let client = Client::new(API_KEY);
    dbg!(client.get_connections()?);
    Ok(())
}

#[test]
fn get_discovery_cache() -> Fallible<()> {
    let client = Client::new(API_KEY);
    dbg!(client.get_discovery_cache()?);
    Ok(())
}

#[test]
fn get_log() -> Fallible<()> {
    let client = Client::new(API_KEY);
    dbg!(client.get_log(None)?);
    Ok(())
}

#[test]
fn get_log_since() -> Fallible<()> {
    let client = Client::new(API_KEY);
    dbg!(client.get_log(Some(Timestamp::parse_from_rfc3339(
        "2014-09-18T12:59:26.549953186+02:00"
    )?))?);
    Ok(())
}

#[test]
fn ping() -> Fallible<()> {
    let client = Client::new(API_KEY);
    dbg!(client.ping()?);
    Ok(())
}

#[test]
fn get_errors() -> Fallible<()> {
    let client = Client::new(API_KEY);
    dbg!(client.get_errors()?);
    Ok(())
}

#[test]
fn get_loglevels_info() -> Fallible<()> {
    let client = Client::new(API_KEY);
    dbg!(client.get_loglevels_info()?);
    Ok(())
}

#[test]
#[ignore]
fn get_upgrade_info() -> Fallible<()> {
    let client = Client::new(API_KEY);
    dbg!(client.get_upgrade_info()?);
    Ok(())
}

#[test]
fn get_version_info() -> Fallible<()> {
    let client = Client::new(API_KEY);
    dbg!(client.get_version_info()?);
    Ok(())
}

#[test]
fn get_events() -> Fallible<()> {
    let client = Client::new(API_KEY);
    dbg!(client.get_all_events(None, None)?);
    Ok(())
}

#[test]
fn event_stream() -> Fallible<()> {
    let client = Client::new(API_KEY);
    let stream = client.subscribe_to_all();
    let mut last = 0;
    let mut i = 0;
    for event in stream {
        if i > 3 {
            return Ok(());
        }
        let event = event?;
        if last == 0 {
            last = event.id;
        } else {
            i += 1;
            assert_eq!(last + 1, event.id);
            last = event.id;
        }
    }
    Ok(())
}
