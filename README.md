# glog

## glog formatted logger

This package provides common slog with glog formatting.

**Log format**
```
[TDIWEC]mmdd hh:mm:ss.uuuuuu threadid file:line] msg
```

- `IWEF` — Log Levels, T for TRACE, D for DEBUG, I for INFO, W for WARNING, E for ERROR and C for Critical.
- `mmdd` — Month and Date.
- `hh:mm:ss.uuuuuu` — Hours, Minutes, Seconds and Microseconds.
- `threadid` — PID/TID of the process/thread.
- `file:line` — File name and line number.
- `msg` — Actual user-specified log message.

## Installation

#### Dependencies

- [Rust with Cargo](http://rust-lang.org)

#### Importing

**Cargo.toml**

```toml
[dependencies]
glog = { version = "0.1.0", git = "https://github.com/kumanote/glog-rs", branch = "main" }
```

**rust files**

```rust
use glog::set_default_global_logger;
use glog::prelude::*;
```

## Examples

Here's a basic example:

```rust
use glog::set_default_global_logger;
use glog::prelude::*;

fn main() {
    // you will have the following console output
    // I1021 13:32:13.346775 123145517920256 src/main.rs:8] Test log 1, tau: 6.28
    let _logger = set_default_global_logger(false, None); // set global logger. must live through whole process.
    info!("Test log {}", 1; "tau" => 6.28); // output info log.
}
```
