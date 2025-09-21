use crate::{Client, Fallible, Timestamp};
use futures_util::stream::StreamExt;

static API_KEY: &str = include_str!("../../api.key");

#[tokio::test]
async fn browse() -> Fallible<()> {
    let client = Client::new(API_KEY);
    dbg!(client.browse(None).await?);
    Ok(())
}

#[tokio::test]
async fn get_connections() -> Fallible<()> {
    let client = Client::new(API_KEY);
    dbg!(client.get_connections().await?);
    Ok(())
}

#[tokio::test]
async fn get_discovery_cache() -> Fallible<()> {
    let client = Client::new(API_KEY);
    dbg!(client.get_discovery_cache().await?);
    Ok(())
}

#[tokio::test]
async fn get_log() -> Fallible<()> {
    let client = Client::new(API_KEY);
    dbg!(client.get_log(None).await?);
    Ok(())
}

#[tokio::test]
async fn get_log_since() -> Fallible<()> {
    let client = Client::new(API_KEY);
    dbg!(
        client
            .get_log(Some(Timestamp::parse_from_rfc3339(
                "2014-09-18T12:59:26.549953186+02:00"
            )?))
            .await?
    );
    Ok(())
}

#[tokio::test]
async fn ping() -> Fallible<()> {
    let client = Client::new(API_KEY);
    dbg!(client.ping().await?);
    Ok(())
}

#[tokio::test]
async fn get_errors() -> Fallible<()> {
    let client = Client::new(API_KEY);
    dbg!(client.get_errors().await?);
    Ok(())
}

#[tokio::test]
async fn get_loglevels_info() -> Fallible<()> {
    let client = Client::new(API_KEY);
    dbg!(client.get_loglevels_info().await?);
    Ok(())
}

#[tokio::test]
async fn get_paths() -> Fallible<()> {
    let client = Client::new(API_KEY);
    dbg!(client.get_paths().await?);
    Ok(())
}

#[tokio::test]
#[ignore]
async fn get_upgrade_info() -> Fallible<()> {
    let client = Client::new(API_KEY);
    dbg!(client.get_upgrade_info().await?);
    Ok(())
}

#[tokio::test]
async fn get_version_info() -> Fallible<()> {
    let client = Client::new(API_KEY);
    dbg!(client.get_version_info().await?);
    Ok(())
}

#[tokio::test]
async fn status() -> Fallible<()> {
    let client = Client::new(API_KEY);
    dbg!(client.status().await?);
    Ok(())
}

#[tokio::test]
async fn get_events() -> Fallible<()> {
    let client = Client::new(API_KEY);
    dbg!(client.get_all_events(None, None).await?);
    Ok(())
}

#[tokio::test]
async fn event_stream() -> Fallible<()> {
    let client = Client::new(API_KEY);
    let mut stream = client.subscribe_to_all();
    let mut last = 0;
    let mut i = 0;
    while let Some(event) = stream.next().await {
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

#[tokio::test]
async fn get_cluster_pending_devices() -> Fallible<()> {
    let client = Client::new(API_KEY);
    dbg!(client.get_cluster_pending_devices().await?);
    Ok(())
}
