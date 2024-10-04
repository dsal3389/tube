use futures::StreamExt;
use tube_inotify;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let mut inotify = tube_inotify::Inotify::new()
        .expect("couldn't create inotify")
        .watch("test.file".into(), tube_inotify::InotifyEvent::IN_ACCESS)?;
    println!("Hello, world!");

    while let Some(event) = inotify.next().await {
        println!("{:?}", event);
    }
    Ok(())
}
