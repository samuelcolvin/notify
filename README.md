# Notify

[![» Crate](https://flat.badgen.net/crates/v/notify)][crate]
[![» Docs](https://flat.badgen.net/badge/api/docs.rs/df3600)][docs]
[![» CI](https://flat.badgen.net/github/checks/notify-rs/notify/main)][build]
[![» Downloads](https://flat.badgen.net/crates/d/notify)][crate]
[![» Conduct](https://flat.badgen.net/badge/contributor/covenant/5e0d73)][coc]
[![» Public Domain](https://flat.badgen.net/badge/license/CC0-1.0/purple)][cc0]

_Cross-platform filesystem notification library for Rust._

**Caution! This is unstable code!**

You likely want either [the latest 4.0 release] or [5.0.0-pre.15].

[the latest 4.0 release]: https://github.com/notify-rs/notify/tree/v4.0.16#notify
[5.0.0-pre.15]: https://github.com/notify-rs/notify/tree/5.0.0-pre.15#notify

(Looking for desktop notifications instead? Have a look at [notify-rust] or
[alert-after]!)

- **incomplete [Guides and in-depth docs][wiki]**
- [API Documentation][docs]
- [Crate page][crate]
- [Changelog][changelog]
- Earliest supported Rust version: **1.56.0**

As used by: [alacritty], [cargo watch], [cobalt], [docket], [mdBook], [pax],
[rdiff], [rust-analyzer], [timetrack], [watchexec], [xi-editor], and others.

## Installation

```toml
[dependencies]
crossbeam-channel = "0.4.0"
notify = "5.0.0-pre.15"
```

## Usage

Notify provides a raw and a debounced mode. The the raw mode gives you all events as received by your specific OS.

### Undebounced

Channel-Based example:

```rust
use notify::{RecommendedWatcher, RecursiveMode, Watcher};
use std::path::Path;

fn main() {
    let (tx, rx) = std::sync::mpsc::channel();
    // Automatically select the best implementation for your platform.
    // You can also access each implementation directly e.g. INotifyWatcher.
    let mut watcher = RecommendedWatcher::new(tx).unwrap();

    // Add a path to be watched. All files and directories at that path and
    // below will be monitored for changes.
    watcher
        .watch(Path::new("."), RecursiveMode::Recursive)
        .unwrap();

    // This is a simple loop, but you may want to use more complex logic here,
    // for example to handle I/O.
    for e in rx {
        match e {
            Ok(event) => println!("{:?}", event),
            Err(e) => println!("watch error: {:?}", e),
        }
    }
}
```

You may also use a callback function instead of a channel.

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

### Serde

Events can be serialisable via [serde]. To enable the feature:

```toml
notify = { version = "5.0.0-pre.15", features = ["serde"] }
```

## Platforms

- Linux / Android: inotify
- macOS: FSEvents (default) / kqueue
- Windows: ReadDirectoryChangesW
- FreeBSD / NetBSD / OpenBSD / DragonflyBSD: kqueue
- All platforms: polling, fallback for everything else

Polling also supports a mode for watching directories that cannot be monitored by watching the last change date per file (`/sys` on linux).

### FSEvents

Due to the inner security model of FSEvents (see [FileSystemEventSecurity]),
some event cannot be observed easily when trying to follow files that do not
belong to you. In this case, reverting to the pollwatcher can fix the issue,
with a slight performance cost.

## License

Notify was undergoing a transition to using the
[Artistic License 2.0][artistic] from [CC Zero 1.0][cc0]. A part of
the code is only under CC0, and another part, including _all new code_ since
commit [`3378ac5a`], is under _both_ CC0 and Artistic. When the project was to be
entirely free of CC0 code, the license would be formally changed (and that would
have incurred a major version bump). As part of this, contributions to Notify since
would agree to release under both.

[`3378ac5a`]: https://github.com/notify-rs/notify/commit/3378ac5ad5f174dfeacce6edadd7ded1a08d384e

## Origins

Inspired by Go's [fsnotify] and Node.js's [Chokidar], born out of need for
[cargo watch], and general frustration at the non-existence of C/Rust
cross-platform notify libraries.

Written by [Félix Saparelli] and awesome [contributors].

[Chokidar]: https://github.com/paulmillr/chokidar
[FileSystemEventSecurity]: https://developer.apple.com/library/mac/documentation/Darwin/Conceptual/FSEvents_ProgGuide/FileSystemEventSecurity/FileSystemEventSecurity.html
[Félix Saparelli]: https://passcod.name
[alacritty]: https://github.com/jwilm/alacritty
[alert-after]: https://github.com/frewsxcv/alert-after
[artistic]: ./LICENSE.ARTISTIC
[build]: https://github.com/notify-rs/notify/actions
[cargo watch]: https://github.com/passcod/cargo-watch
[cc0]: ./LICENSE
[changelog]: ./CHANGELOG.md
[cobalt]: https://github.com/cobalt-org/cobalt.rs
[coc]: http://contributor-covenant.org/version/1/4/
[contributors]: https://github.com/notify-rs/notify/graphs/contributors
[crate]: https://crates.io/crates/notify
[docket]: https://iwillspeak.github.io/docket/
[docs]: https://docs.rs/notify/5.0.0-pre.15/notify/
[fsnotify]: https://github.com/go-fsnotify/fsnotify
[handlebars-iron]: https://github.com/sunng87/handlebars-iron
[hotwatch]: https://github.com/francesca64/hotwatch
[mdBook]: https://github.com/rust-lang-nursery/mdBook
[notify-rust]: https://github.com/hoodie/notify-rust
[pax]: https://pax.js.org/
[rdiff]: https://github.com/dyule/rdiff
[rust-analyzer]: https://github.com/rust-analyzer/rust-analyzer
[serde]: https://serde.rs/
[timetrack]: https://github.com/joshmcguigan/timetrack
[watchexec]: https://github.com/mattgreen/watchexec
[wiki]: https://github.com/notify-rs/notify/wiki
[xi-editor]: https://xi-editor.io/
