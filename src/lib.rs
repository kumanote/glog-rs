mod glog_format;
mod kv_categorizer;
mod collector_serializer;
pub mod prelude {
    pub use slog_scope::{trace, debug, info, warn, error, crit};
}

use slog::{o, Logger, Drain, Never, FilterLevel};
use slog_async::Async;
use slog_envlogger::LogBuilder;
use slog_scope::{GlobalLoggerGuard, set_global_logger};
use std::sync::Mutex;
use crate::glog_format::GlogFormat;
use slog_term::PlainDecorator;
use crate::kv_categorizer::ErrorCategorizer;

/// Creates and sets default global logger.
/// Caller must keep the returned guard alive.
/// You can use "RUST_LOG" env var to set log level for each module. (see https://github.com/slog-rs/envlogger).
pub fn set_default_global_logger(async_drain: bool, chan_size: Option<usize>) -> GlobalLoggerGuard {
    let logger = create_default_root_logger(async_drain, chan_size);
    set_global_logger(logger)
}

/// Creates a root logger with default settings.
fn create_default_root_logger(async_drain: bool, chan_size: Option<usize>) -> Logger {
    let drain = GlogFormat::new(PlainDecorator::new(::std::io::stdout()), ErrorCategorizer).fuse();

    let mut builder = LogBuilder::new(drain);
    builder = builder.filter(None, FilterLevel::Info);
    if let Ok(s) = ::std::env::var("RUST_LOG") {
        builder = builder.parse(&s);
    }
    let logger = builder.build();
    get_logger(async_drain, chan_size, logger)
}

fn get_logger<D>(is_async: bool, chan_size: Option<usize>, drain: D) -> Logger
    where
        D: Drain<Err = Never, Ok = ()> + Send + 'static,
{
    if is_async {
        let async_builder = match chan_size {
            Some(chan_size_inner) => Async::new(drain).chan_size(chan_size_inner),
            None => Async::new(drain),
        };
        Logger::root(async_builder.build().fuse(), o!())
    } else {
        Logger::root(Mutex::new(drain).fuse(), o!())
    }
}

#[cfg(test)]
mod tests {
    use crate::set_default_global_logger;
    use crate::prelude::*;
    #[test]
    fn test_just_log() {
        // please run with "--nocapture" option.
        // you will have the following console output
        // I1021 13:32:13.346775 123145517920256 logger/src/lib.rs:63] Test log 1, tau: 6.28
        let _logger = set_default_global_logger(false, None);
        info!("Test log {}", 1; "tau" => 6.28);
    }
}
