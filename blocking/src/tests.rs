use crate::{Client, Fallible};

static API_KEY: &str = include_str!("../../api.key");

#[test]
fn get_system_connections() -> Fallible<()> {
    let client = Client::new(API_KEY);
    dbg!(client.get_system_connections()?);
    Ok(())
}

#[test]
fn get_system_debug() -> Fallible<()> {
    let client = Client::new(API_KEY);
    dbg!(client.get_system_debug()?);
    Ok(())
}

#[test]
fn get_system_discovery() -> Fallible<()> {
    let client = Client::new(API_KEY);
    dbg!(client.get_system_discovery()?);
    Ok(())
}

#[test]
fn get_system_log() -> Fallible<()> {
    let client = Client::new(API_KEY);
    dbg!(client.get_system_log()?);
    Ok(())
}

#[test]
fn get_system_ping() -> Fallible<()> {
    let client = Client::new(API_KEY);
    dbg!(client.get_system_ping()?);
    Ok(())
}

#[test]
fn get_system_error() -> Fallible<()> {
    let client = Client::new(API_KEY);
    dbg!(client.get_system_error()?);
    Ok(())
}

#[test]
#[ignore]
fn get_system_upgrade() -> Fallible<()> {
    let client = Client::new(API_KEY);
    dbg!(client.get_system_upgrade()?);
    Ok(())
}

#[test]
fn get_system_version() -> Fallible<()> {
    let client = Client::new(API_KEY);
    dbg!(client.get_system_version()?);
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
