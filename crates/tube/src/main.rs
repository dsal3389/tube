use futures::StreamExt;
use tube_inotify::{Flag, Inotify, Mask};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let mut inotify = Inotify::with_flags(Flag::NONBLOCKING)
        .expect("couldn't create inotify")
        .watch("foo".into(), Mask::CREATE | Mask::DELETE)?;
    println!("Hello, world!");

    while let Some(events) = inotify.next().await {
        if events.is_err() {
            break;
        }

        let events = unsafe { events.unwrap_unchecked() };
        for event in events {
            println!("event {:?}", event);
        }
        println!("event end!");
    }
    Ok(())
}
