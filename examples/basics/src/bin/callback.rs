use notify::{RecommendedWatcher, RecursiveMode, Watcher};
use std::path::Path;

fn main() {
    // start watcher with callback
    let mut watcher = RecommendedWatcher::new(|e| match e {
        Ok(event) => println!("{:?}", event),
        Err(e) => println!("watch error: {:?}", e),
    })
    .unwrap();

    // watch some stuff
    watcher
        .watch(Path::new("."), RecursiveMode::Recursive)
        .unwrap();

    loop {
        // unblocked main thread, for this demo we need to loop or the program would exit instantly
        std::thread::sleep(std::time::Duration::from_millis(250));
    }
}
