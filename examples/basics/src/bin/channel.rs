use notify::{RecommendedWatcher, RecursiveMode, Watcher};
use std::path::Path;

fn main() {
    // can also be a crossbeam-channel
    let (tx, rx) = std::sync::mpsc::channel();

    let mut watcher = RecommendedWatcher::new(tx).unwrap();

    // watch some stuff
    watcher
        .watch(Path::new("."), RecursiveMode::Recursive)
        .unwrap();

    for e in rx {
        match e {
            Ok(event) => println!("{:?}", event),
            Err(e) => println!("watch error: {:?}", e),
        }
    }
}
