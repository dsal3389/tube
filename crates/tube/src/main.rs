use tube_inotify;

fn main() -> anyhow::Result<()> {
    let mut inotify = tube_inotify::Inotify::new()
        .expect("couldn't create inotify")
        .watch("test.file".into())?;
    println!("Hello, world!");
    Ok(())
}
