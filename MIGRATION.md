# Migration Guide

## v4 to v5

The event receiver changed to a trait that can be re-implemented on top of other things.

If you've used a sender then you can still do so in v5.  

```rust
use notify::{RecommendedWatcher, RecursiveMode, Watcher};
use std::path::Path;

fn main() {
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
```

You may now also supply a function instead that is used as callback.

```rust
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
```