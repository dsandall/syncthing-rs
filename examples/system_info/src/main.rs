use syncthing_async::{Client, Fallible};

#[tokio::main]
async fn main() -> Fallible<()> {
    let client = Client::new(include_str!("../../../api.key"));
    let system = client.get_version_info().await?;
    println!(
        "syncthing {} is running on {:?}!",
        system.version, system.os
    );
    Ok(())
}
