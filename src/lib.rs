#![feature(rust_2018_preview, use_extern_macros)]
#![warn(rust_2018_idioms)]

//! # Slog -  Structured, extensible, composable logging for Rust
//!
//! `slog-rs` is an ecosystem of reusable components for structured, extensible,
//! composable logging for Rust.
//!
//! `slog` is `slog-rs`'s main crate providing core components shared between
//! all other parts of `slog-rs` ecosystem.
//!
//! This is auto-generated technical documentation of `slog`. For information
//! about project organization, development, help, etc. please see
//! [slog github page](https://github.com/slog-rs/slog)
//!
//! ## Core advantages over `log` crate
//!
//! * **extensible** - `slog` crate provides core functionality: very basic
//!   and portable standard feature-set based on open `trait`s. This allows
//!   implementing new features that can be independently published.
//! * **composable** - `trait`s that `slog` exposes to provide extensibility
//!   are designed to be easy to efficiently reuse and combine. By combining
//!   different functionalities every application can specify when, where and
//!   how exactly process logging data from the application and it's
//!   dependencies.
//! * **flexible** - `slog` does not constrain logging to just one globally
//!   registered backend. Parts of your application can handle logging
//!   in a customized way, or completely independently.
//! * **structured** and both **human and machine readable** - By keeping the
//!   key-value data format and retaining its type information, meaning of logging
//!   data is preserved.  Data can be serialized to machine readable formats like
//!   JSON and send it to data-mining system for further analysis etc. On the
//!   other hand, when presenting on screen, logging information can be presented
//!   in aesthetically pleasing and easy to understand way.
//! * **contextual** - `slog`'s `Logger` objects carry set of key-value data
//!   pairs that contains the context of logging - information that otherwise
//!   would have to be repeated in every logging statement.
//!
//! ## `slog` features
//!
//! * performance oriented; read [what makes slog
//!   fast](https://github.com/slog-rs/slog/wiki/What-makes-slog-fast) and see:
//!   [slog bench log](https://github.com/dpc/slog-rs/wiki/Bench-log)
//!   * lazily evaluation through closure values
//!   * async IO support included: see [`slog-async`
//!     crate](https://docs.rs/slog-async)
//! * `#![no_std]` support (with opt-out `std` cargo feature flag)
//! * support for named format arguments (eg. `info!(logger, "printed {line_count} lines", line_count = 2);`)
//!   for easy bridging the human readable and machine-readable output
//! * tree-structured loggers
//! * modular, lightweight and very extensible
//!   * tiny core crate that does not pull any dependencies
//!   * feature-crates for specific functionality
//!   * using `slog` in library does not force users of the library to use slog
//!     (but provides additional functionality); see [example how to use
//!     `slog` in library](https://github.com/slog-rs/example-lib)
//! * backward and forward compatibility with `log` crate:
//!   see [`slog-stdlog` crate](https://docs.rs/slog-stdlog)
//! * convenience crates:
//!   * logging-scopes for implicit `Logger` passing: see
//!     [slog-scope crate](https://docs.rs/slog-scope)
//! * many existing core&community provided features:
//!   * multiple outputs
//!   * filtering control
//!       * compile-time log level filter using cargo features (same as in `log`
//!         crate)
//!       * by level, msg, and any other meta-data
//!       * [`slog-envlogger`](https://github.com/slog-rs/envlogger) - port of
//!         `env_logger`
//!       * terminal output, with color support: see [`slog-term`
//!         crate](https://docs.rs/slog-term)
//!  * [json](https://docs.rs/slog-json)
//!      * [bunyan](https://docs.rs/slog-bunyan)
//!  * [syslog](https://docs.rs/slog-syslog)
//!    and [journald](https://docs.rs/slog-journald) support
//!  * run-time configuration:
//!      * run-time behavior change;
//!        see [slog-atomic](https://docs.rs/slog-atomic)
//!      * run-time configuration; see
//!        [slog-config crate](https://docs.rs/slog-config)
//!
//!
//! [env_logger]: https://crates.io/crates/env_logger
//!
//! ## Notable details
//!
//! **Note:** At compile time `slog` by default removes trace and debug level
//! statements in release builds, and trace level records in debug builds. This
//! makes `trace` and `debug` level logging records practically free, which
//! should encourage using them freely. If you want to enable trace/debug
//! messages or raise the compile time logging level limit, use the following in
//! your `Cargo.toml`:
//!
//! ```norust
//! slog = { version = ... ,
//!          features = ["max_level_trace", "release_max_level_warn"] }
//! ```
//!
//! Root drain (passed to `Logger::root`) must be one that does not ever return
//! errors. This forces user to pick error handing strategy.
//! `Drain::fuse()` or `Drain::ignore_res()`.
//!
//! [env_logger]: https://crates.io/crates/env_logger
//! [fn-overv]: https://github.com/dpc/slog-rs/wiki/Functional-overview
//! [atomic-switch]: https://docs.rs/slog-atomic/
//!
//! ## Where to start
//!
//! [`Drain`](trait.Drain.html), [`Logger`](struct.Logger.html) and
//! [`log` macro](macro.log.html) are the most important elements of
//! slog. Make sure to read their respective documentation
//!
//! Typically the biggest problem is creating a `Drain`
//!
//!
//! ### Logging to the terminal
//!
//! ```
//! #[macro_use]
//! extern crate slog;
//! extern crate slog_term;
//! extern crate slog_async;
//!
//! use slog::Drain;
//!
//! fn main() {
//!     let decorator = slog_term::TermDecorator::new().build();
//!     let drain = slog_term::FullFormat::new(decorator).build().fuse();
//!     let drain = slog_async::Async::new(drain).build().fuse();
//!
//!     let _log = slog::Logger::root(drain, o!());
//! }
//! ```
//!
//! ### Logging to a file
//!
//! ```
//! #[macro_use]
//! extern crate slog;
//! extern crate slog_term;
//! extern crate slog_async;
//!
//! use std::fs::OpenOptions;
//! use slog::Drain;
//!
//! fn main() {
//!    let log_path = "target/your_log_file_path.log";
//!    let file = OpenOptions::new()
//!       .create(true)
//!       .write(true)
//!       .truncate(true)
//!       .open(log_path)
//!       .unwrap();
//!
//!     let decorator = slog_term::PlainDecorator::new(file);
//!     let drain = slog_term::FullFormat::new(decorator).build().fuse();
//!     let drain = slog_async::Async::new(drain).build().fuse();
//!
//!     let _log = slog::Logger::root(drain, o!());
//! }
//! ```
//!
//! You can consider using `slog-json` instead of `slog-term`.
//! `slog-term` only coincidently fits the role of a file output format. A
//! proper `slog-file` crate with suitable format, log file rotation and other
//! file-logging related features would be awesome. Contributions are welcome!
//!
//! ### Change logging level at runtime
//!
//! ```
//! #[macro_use]
//! extern crate slog;
//! extern crate slog_term;
//! extern crate slog_async;
//!
//! use slog::Drain;
//!
//! use std::sync::{Arc, atomic};
//! use std::sync::atomic::Ordering;
//! use std::result;
//!
//! /// Custom Drain logic
//! struct RuntimeLevelFilter<D>{
//!    drain: D,
//!    on: Arc<atomic::AtomicBool>,
//! }
//!
//! impl<D> Drain for RuntimeLevelFilter<D>
//!     where D : Drain {
//!     type Ok = Option<D::Ok>;
//!     type Err = Option<D::Err>;
//!
//!     fn log(&self,
//!           record: &slog::Record,
//!           values: &slog::OwnedKVList)
//!           -> result::Result<Self::Ok, Self::Err> {
//!           let current_level = if self.on.load(Ordering::Relaxed) {
//!               slog::Level::Trace
//!           } else {
//!               slog::Level::Info
//!           };
//!
//!           if record.level().is_at_least(current_level) {
//!               self.drain.log(
//!                   record,
//!                   values
//!               )
//!               .map(Some)
//!               .map_err(Some)
//!           } else {
//!               Ok(None)
//!           }
//!       }
//!   }
//!
//! fn main() {
//!     // atomic variable controlling logging level
//!     let on = Arc::new(atomic::AtomicBool::new(false));
//!
//!     let decorator = slog_term::TermDecorator::new().build();
//!     let drain = slog_term::FullFormat::new(decorator).build();
//!     let drain = RuntimeLevelFilter {
//!         drain: drain,
//!         on: on.clone(),
//!     }.fuse();
//!     let drain = slog_async::Async::new(drain).build().fuse();
//!
//!     let _log = slog::Logger::root(drain, o!());
//!
//!     // switch level in your code
//!     on.store(true, Ordering::Relaxed);
//! }
//! ```
//!
//! Why is this not an existing crate? Because there are multiple ways to
//! achieve the same result, and each application might come with it's own
//! variation. Supporting a more general solution is a maintenance effort.
//! There is also nothing stopping anyone from publishing their own crate
//! implementing it.
//!
//! Alternative to the above aproach is `slog-atomic` crate. It implements
//! swapping whole parts of `Drain` logging hierarchy.
//!
//! ## Examples & help
//!
//! Basic examples that are kept up-to-date are typically stored in
//! respective git repository, under `examples/` subdirectory. Eg.
//! [slog-term examples](https://github.com/slog-rs/term/tree/master/examples).
//!
//! [slog-rs wiki pages](https://github.com/slog-rs/slog/wiki) contain
//! some pages about `slog-rs` technical details.
//!
//! Source code of other [software using
//! slog-rs](https://crates.io/crates/slog/reverse_dependencies) can
//! be an useful reference.
//!
//! Visit [slog-rs gitter channel](https://gitter.im/slog-rs/slog) for immediate
//! help.
//!
//! ## Migrating from slog v1 to slog v2
//!
//! ### Key-value pairs come now after format string
//!
//! ```
//! #[macro_use]
//! extern crate slog;
//!
//! fn main() {
//!     let drain = slog::Discard;
//!     let root = slog::Logger::root(drain, o!());
//!     info!(root, "formatted: {}", 1; "log-key" => true);
//! }
//! ```
//!
//! See more information about format at [`log`](macro.log.html).
//!
//! ### `slog-streamer` is gone
//!
//! Create simple terminal logger like this:
//!
//! ```
//! #[macro_use]
//! extern crate slog;
//! extern crate slog_term;
//! extern crate slog_async;
//!
//! use slog::Drain;
//!
//! fn main() {
//!     let decorator = slog_term::TermDecorator::new().build();
//!     let drain = slog_term::FullFormat::new(decorator).build().fuse();
//!     let drain = slog_async::Async::new(drain).build().fuse();
//!
//!     let _log = slog::Logger::root(drain, o!());
//! }
//! ```
//!
//!
//! ### Logging macros now takes ownership of values.
//!
//! Pass them by reference: `&x`.
//!
// }}}

// {{{ Imports & meta
#![cfg_attr(not(feature = "std"), feature(alloc))]
#![cfg_attr(not(feature = "std"), feature(collections))]
#![warn(missing_docs)]
#![no_std]

#[cfg(not(feature = "std"))]
extern crate alloc;
#[cfg(not(feature = "std"))]
extern crate collections;
#[macro_use]
#[cfg(feature = "std")]
extern crate std;

mod key;
pub use self::key::Key;
#[cfg(not(feature = "std"))]
use alloc::arc::Arc;
#[cfg(not(feature = "std"))]
use alloc::boxed::Box;
#[cfg(not(feature = "std"))]
use alloc::rc::Rc;
#[cfg(not(feature = "std"))]
use collections::string::String;

#[cfg(feature = "nested-values")]
extern crate erased_serde;

use core::{convert, fmt, result};
use core::str::FromStr;
#[cfg(feature = "std")]
use std::boxed::Box;
#[cfg(feature = "std")]
use std::panic::{RefUnwindSafe, UnwindSafe};
#[cfg(feature = "std")]
use std::rc::Rc;
#[cfg(feature = "std")]
use std::string::String;
#[cfg(feature = "std")]
use std::sync::Arc;
// }}}

macro_rules! macro_alias {
    (@ ($dol:tt) $orig:ident, $alias:ident) => {
        #[macro_export]
        /// Alias to avoid name collision
        macro_rules! $alias {
            ($dol ( $dol args:tt )* ) => { $crate::$orig!($dol ( $dol args )*) }
        }
    };
    ($orig:ident, $alias:ident) => {
        macro_alias!(@ ($) $orig, $alias);
    }
}

// {{{ Macros
/// Macro for building group of key-value pairs:
/// [`OwnedKV`](struct.OwnedKV.html)
///
/// ```
/// #[macro_use]
/// extern crate slog;
///
/// fn main() {
///     let drain = slog::Discard;
///     let _root = slog::Logger::root(
///         drain,
///         o!("key1" => "value1", "key2" => "value2")
///     );
/// }
/// ```
#[macro_export]
macro_rules! o(
    ($($args:tt)*) => {
        $crate::OwnedKV($crate::kv!($($args)*))
    };
);

macro_alias!(o, slog_o);

/// Macro for building group of key-value pairs in
/// [`BorrowedKV`](struct.BorrowedKV.html)
///
/// In most circumstances using this macro directly is unnecessary and `info!`
/// and other wrappers over `log!` should be used instead.
#[macro_export]
macro_rules! b(
    ($($args:tt)*) => {
        $crate::BorrowedKV(&$crate::kv!($($args)*))
    };
);

macro_alias!(b, slog_b);

/// Macro for build `KV` implementing type
///
/// You probably want to use `o!` or `b!` instead.
#[macro_export]
macro_rules! kv(
    (@ $args_ready:expr; $k:expr => %$v:expr) => {
        kv!(@ ($crate::SingleKV::from(($k, $crate::FmtDisplay($v))), $args_ready); )
    };
    (@ $args_ready:expr; $k:expr => %$v:expr, $($args:tt)* ) => {
        kv!(@ ($crate::SingleKV::from(($k, $crate::FmtDisplay($v))), $args_ready); $($args)* )
    };
    (@ $args_ready:expr; $k:expr => ?$v:expr) => {
        kv!(@ ($crate::SingleKV::from(($k, $crate::FmtDebug($v))), $args_ready); )
    };
    (@ $args_ready:expr; $k:expr => ?$v:expr, $($args:tt)* ) => {
        kv!(@ ($crate::SingleKV::from(($k, $crate::FmtDebug($v))), $args_ready); $($args)* )
    };
    (@ $args_ready:expr; $k:expr => $v:expr) => {
        kv!(@ ($crate::SingleKV::from(($k, $v)), $args_ready); )
    };
    (@ $args_ready:expr; $k:expr => $v:expr, $($args:tt)* ) => {
        kv!(@ ($crate::SingleKV::from(($k, $v)), $args_ready); $($args)* )
    };
    (@ $args_ready:expr; $kv:expr) => {
        kv!(@ ($kv, $args_ready); )
    };
    (@ $args_ready:expr; $kv:expr, $($args:tt)* ) => {
        kv!(@ ($kv, $args_ready); $($args)* )
    };
    (@ $args_ready:expr; ) => {
        $args_ready
    };
    (@ $args_ready:expr;, ) => {
        $args_ready
    };
    ($($args:tt)*) => {
        kv!(@ (); $($args)*)
    };
);

macro_alias!(kv, slog_kv);

#[macro_export]
/// Create `RecordStatic` at the given code location
macro_rules! record_static(
    ($lvl:expr, $tag:expr,) => { $crate::record_static!($lvl, $tag) };
    ($lvl:expr, $tag:expr) => {{
        static LOC : $crate::RecordLocation = $crate::RecordLocation {
            file: file!(),
            line: line!(),
            column: column!(),
            function: "",
            module: module_path!(),
        };
        $crate::RecordStatic {
            location : &LOC,
            level: $lvl,
            tag : $tag,
        }
    }};
);

macro_alias!(record_static, slog_record_static);

#[macro_export]
/// Create `Record` at the given code location
macro_rules! record(
    ($lvl:expr, $tag:expr, $args:expr, $b:expr,) => {
        $crate::record!($lvl, $tag, $args, $b)
    };
    ($lvl:expr, $tag:expr, $args:expr, $b:expr) => {{
        #[allow(dead_code)]
        static RS : $crate::RecordStatic<'static> = $crate::record_static!($lvl, $tag);
        $crate::Record::new(&RS, $args, $b)
    }};
);

macro_alias!(record, slog_record);

/// Log message a logging record
///
/// Use wrappers `error!`, `warn!` etc. instead
///
/// The `max_level_*` and `release_max_level*` cargo features can be used to
/// statically disable logging at various levels. See [slog notable
/// details](index.html#notable-details)
///
/// Use [version with longer name](macro.slog_log.html) if you want to prevent
/// clash with legacy `log` crate macro names.
///
/// ## Supported invocations
///
/// ### Simple
///
/// ```
/// #[macro_use]
/// extern crate slog;
///
/// fn main() {
///     let drain = slog::Discard;
///     let root = slog::Logger::root(
///         drain,
///         o!("key1" => "value1", "key2" => "value2")
///     );
///     info!(root, "test info log"; "log-key" => true);
/// }
/// ```
///
/// Note that `"key" => value` part is optional:
///
/// ```
/// #[macro_use]
/// extern crate slog;
///
/// fn main() {
///     let drain = slog::Discard;
///     let root = slog::Logger::root(
///         drain, o!("key1" => "value1", "key2" => "value2")
///     );
///     info!(root, "test info log");
/// }
/// ```
///
/// ### Formatting support:
///
/// ```
/// #[macro_use]
/// extern crate slog;
///
/// fn main() {
///     let drain = slog::Discard;
///     let root = slog::Logger::root(drain,
///         o!("key1" => "value1", "key2" => "value2")
///     );
///     info!(root, "formatted {num_entries} entries of {}", "something", num_entries = 2; "log-key" => true);
/// }
/// ```
///
/// Note:
///
/// * `;` is used to separate message arguments and key value pairs.
/// * message behaves like `format!`/`format_args!`
/// * Named arguments to messages will be added to key-value pairs as well!
///
/// `"key" => value` part is optional:
///
/// ```
/// #[macro_use]
/// extern crate slog;
///
/// fn main() {
///     let drain = slog::Discard;
///     let root = slog::Logger::root(
///         drain, o!("key1" => "value1", "key2" => "value2")
///     );
///     info!(root, "formatted: {}", 1);
/// }
/// ```
///
/// Use formatting support wisely. Prefer named arguments, so the associated
/// data is not "lost" by becoming an untyped string in the message.
///
/// ### Tags
///
/// All above versions can be supplemented with a tag - string literal prefixed
/// with `#`.
///
/// ```
/// #[macro_use]
/// extern crate slog;
///
/// fn main() {
///     let drain = slog::Discard;
///     let root = slog::Logger::root(drain,
///         o!("key1" => "value1", "key2" => "value2")
///     );
///     let ops = 3;
///     info!(
///         root,
///         #"performance-metric", "thread speed"; "ops_per_sec" => ops
///     );
/// }
/// ```
///
/// See `Record::tag()` for more information about tags.
///
/// ### Own implementations of `KV` and `Value`
///
/// List of key value pairs is a comma separated list of key-values. Typically,
/// a designed syntax is used in form of `k => v` where `k` can be any type
/// that implements `Value` type.
///
/// It's possible to directly specify type that implements `KV` trait without
/// `=>` syntax.
///
/// ```
/// #[macro_use]
/// extern crate slog;
///
/// use slog::*;
///
/// fn main() {
///     struct MyKV;
///     struct MyV;
///
///     impl KV for MyKV {
///        fn serialize(&self,
///                     _record: &Record,
///                     serializer: &mut Serializer)
///                    -> Result {
///            serializer.emit_u32("MyK", 16)
///        }
///     }
///
///     impl Value for MyV {
///        fn serialize(&self,
///                     _record: &Record,
///                     key : Key,
///                     serializer: &mut Serializer)
///                    -> Result {
///            serializer.emit_u32("MyKV", 16)
///        }
///     }
///
///     let drain = slog::Discard;
///
///     let root = slog::Logger::root(drain, o!(MyKV));
///
///     info!(
///         root,
///         "testing MyV"; "MyV" => MyV
///     );
/// }
/// ```
///
/// ### `fmt::Display` and `fmt::Debug` values
///
/// Value of any type that implements `std::fmt::Display` can be prefixed with
/// `%` in `k => v` expression to use it's text representation returned by
/// `format_args!("{}", v)`. This is especially useful for errors. Not that
/// this does not allocate any `String` since it operates on `fmt::Arguments`.
///
/// Similarly to use `std::fmt::Debug` value can be prefixed with `?`.
///
/// ```
/// #[macro_use]
/// extern crate slog;
/// use std::fmt::Write;
///
/// fn main() {
///     let drain = slog::Discard;
///     let log  = slog::Logger::root(drain, o!());
///
///     let mut output = String::new();
///
///     if let Err(e) = write!(&mut output, "write to string") {
///         error!(log, "write failed"; "err" => %e);
///     }
/// }
/// ```
#[macro_export]
macro_rules! log(
    // `2` means that `;` was already found
   (2 @ { $($fmt:tt)* }, { $($kv:tt)* },  $l:expr, $lvl:expr, $tag:expr, $msg_fmt:expr) => {
      $l.log(&record!($lvl, $tag, &format_args!($msg_fmt, $($fmt)*), b!($($kv)*)))
   };
   (2 @ { $($fmt:tt)* }, { $($kv:tt)* }, $l:expr, $lvl:expr, $tag:expr, $msg_fmt:expr,) => {
       $crate::log!(2 @ { $($fmt)* }, { $($kv)* }, $l, $lvl, $tag, $msg_fmt)
   };
   (2 @ { $($fmt:tt)* }, { $($kv:tt)* }, $l:expr, $lvl:expr, $tag:expr, $msg_fmt:expr;) => {
       $crate::log!(2 @ { $($fmt)* }, { $($kv)* }, $l, $lvl, $tag, $msg_fmt)
   };
   (2 @ { $($fmt:tt)* }, { $($kv:tt)* }, $l:expr, $lvl:expr, $tag:expr, $msg_fmt:expr, $($args:tt)*) => {
       $crate::log!(2 @ { $($fmt)* }, { $($kv)* $($args)*}, $l, $lvl, $tag, $msg_fmt)
   };
    // `1` means that we are still looking for `;`
    // -- handle named arguments to format string
   (1 @ { $($fmt:tt)* }, { $($kv:tt)* }, $l:expr, $lvl:expr, $tag:expr, $msg_fmt:expr, $k:ident = $v:expr) => {
       $crate::log!(2 @ { $($fmt)* $k = $v }, { $($kv)* stringify!($k) => $v, }, $l, $lvl, $tag, $msg_fmt)
   };
   (1 @ { $($fmt:tt)* }, { $($kv:tt)* }, $l:expr, $lvl:expr, $tag:expr, $msg_fmt:expr, $k:ident = $v:expr;) => {
       $crate::log!(2 @ { $($fmt)* $k = $v }, { $($kv)* stringify!($k) => $v, }, $l, $lvl, $tag, $msg_fmt)
   };
   (1 @ { $($fmt:tt)* }, { $($kv:tt)* }, $l:expr, $lvl:expr, $tag:expr, $msg_fmt:expr, $k:ident = $v:expr,) => {
       $crate::log!(2 @ { $($fmt)* $k = $v }, { $($kv)* stringify!($k) => $v, }, $l, $lvl, $tag, $msg_fmt)
   };
   (1 @ { $($fmt:tt)* }, { $($kv:tt)* }, $l:expr, $lvl:expr, $tag:expr, $msg_fmt:expr, $k:ident = $v:expr; $($args:tt)*) => {
       $crate::log!(2 @ { $($fmt)* $k = $v }, { $($kv)* stringify!($k) => $v, }, $l, $lvl, $tag, $msg_fmt, $($args)*)
   };
   (1 @ { $($fmt:tt)* }, { $($kv:tt)* }, $l:expr, $lvl:expr, $tag:expr, $msg_fmt:expr, $k:ident = $v:expr, $($args:tt)*) => {
       $crate::log!(1 @ { $($fmt)* $k = $v, }, { $($kv)* stringify!($k) => $v, }, $l, $lvl, $tag, $msg_fmt, $($args)*)
   };
    // -- look for `;` termination
   (1 @ { $($fmt:tt)* }, { $($kv:tt)* }, $l:expr, $lvl:expr, $tag:expr, $msg_fmt:expr,) => {
       $crate::log!(2 @ { $($fmt)* }, { $($kv)* }, $l, $lvl, $tag, $msg_fmt)
   };
   (1 @ { $($fmt:tt)* }, { $($kv:tt)* }, $l:expr, $lvl:expr, $tag:expr, $msg_fmt:expr) => {
       $crate::log!(2 @ { $($fmt)* }, { $($kv)* }, $l, $lvl, $tag, $msg_fmt)
   };
   (1 @ { $($fmt:tt)* }, { $($kv:tt)* }, $l:expr, $lvl:expr, $tag:expr, $msg_fmt:expr, ; $($args:tt)*) => {
       $crate::log!(1 @ { $($fmt)* }, { $($kv)* }, $l, $lvl, $tag, $msg_fmt; $($args)*)
   };
   (1 @ { $($fmt:tt)* }, { $($kv:tt)* }, $l:expr, $lvl:expr, $tag:expr, $msg_fmt:expr; $($args:tt)*) => {
       $crate::log!(2 @ { $($fmt)* }, { $($kv)* }, $l, $lvl, $tag, $msg_fmt, $($args)*)
   };
    // -- must be normal argument to format string
   (1 @ { $($fmt:tt)* }, { $($kv:tt)* }, $l:expr, $lvl:expr, $tag:expr, $msg_fmt:expr, $f:tt $($args:tt)*) => {
       $crate::log!(1 @ { $($fmt)* $f }, { $($kv)* }, $l, $lvl, $tag, $msg_fmt, $($args)*)
   };
   ($l:expr, $lvl:expr, $tag:expr, $($args:tt)*) => {
       if $lvl.as_usize() <= $crate::__slog_static_max_level().as_usize() {
           $crate::log!(1 @ { }, { }, $l, $lvl, $tag, $($args)*)
       }
   };
);

macro_alias!(log, slog_log);

/// Log critical level record
///
/// See `log` for documentation.
#[macro_export]
macro_rules! crit(
    ($l:expr, #$tag:expr, $($args:tt)+) => {
        $crate::log!($l, $crate::Level::Critical, $tag, $($args)+)
    };
    ($l:expr, $($args:tt)+) => {
        $crate::log!($l, $crate::Level::Critical, "", $($args)+)
    };
);

macro_alias!(crit, slog_crit);

/// Log error level record
///
/// See `log` for documentation.
#[macro_export]
macro_rules! error(
    ($l:expr, #$tag:expr, $($args:tt)+) => {
        $crate::log!($l, $crate::Level::Error, $tag, $($args)+)
    };
    ($l:expr, $($args:tt)+) => {
        $crate::log!($l, $crate::Level::Error, "", $($args)+)
    };
);

macro_alias!(error, slog_error);

/// Log warning level record
///
/// See `log` for documentation.
#[macro_export]
macro_rules! warn(
    ($l:expr, #$tag:expr, $($args:tt)+) => {
        $crate::log!($l, $crate::Level::Warning, $tag, $($args)+)
    };
    ($l:expr, $($args:tt)+) => {
        $crate::log!($l, $crate::Level::Warning, "", $($args)+)
    };
);

macro_alias!(warn, slog_warn);

/// Log info level record
///
/// See `slog_log` for documentation.
#[macro_export]
macro_rules! info(
    ($l:expr, #$tag:expr, $($args:tt)*) => {
        $crate::log!($l, $crate::Level::Info, $tag, $($args)*)
    };
    ($l:expr, $($args:tt)*) => {
        $crate::log!($l, $crate::Level::Info, "", $($args)*)
    };
);

macro_alias!(info, slog_info);

/// Log debug level record
///
/// See `log` for documentation.
#[macro_export]
macro_rules! debug(
    ($l:expr, #$tag:expr, $($args:tt)+) => {
        $crate::log!($l, $crate::Level::Debug, $tag, $($args)+)
    };
    ($l:expr, $($args:tt)+) => {
        $crate::log!($l, $crate::Level::Debug, "", $($args)+)
    };
);

macro_alias!(debug, slog_debug);

/// Log trace level record
///
/// See `log` for documentation.
#[macro_export]
macro_rules! trace(
    ($l:expr, #$tag:expr, $($args:tt)+) => {
        $crate::log!($l, $crate::Level::Trace, $tag, $($args)+)
    };
    ($l:expr, $($args:tt)+) => {
        $crate::log!($l, $crate::Level::Trace, "", $($args)+)
    };
);

macro_alias!(trace, slog_trace);

// }}}

// {{{ Logger
/// Logging handle used to execute logging statements
///
/// In an essence `Logger` instance holds two pieces of information:
///
/// * drain - destination where to forward logging `Record`s for
/// processing.
/// * context - list of key-value pairs associated with it.
///
/// Root `Logger` is created with a `Drain` that will be cloned to every
/// member of it's hierarchy.
///
/// Child `Logger` are built from existing ones, and inherit their key-value
/// pairs, which can be supplemented with additional ones.
///
/// Cloning existing loggers and creating new ones is cheap. Loggers can be
/// freely passed around the code and between threads.
///
/// `Logger`s are `Sync+Send` - there's no need to synchronize accesses to them,
/// as they can accept logging records from multiple threads at once. They can
/// be sent to any thread. Because of that they require the `Drain` to be
/// `Sync+Sync` as well. Not all `Drain`s are `Sync` or `Send` but they can
/// often be made so by wrapping in a `Mutex` and/or `Arc`.
///
/// `Logger` implements `Drain` trait. Any logging `Record` delivered to
/// a `Logger` functioning as a `Drain`, will be delivered to it's `Drain`
/// with existing key-value pairs appended to the `Logger`s key-value pairs.
/// By itself it's effectively very similar to `Logger` being an ancestor
/// of `Logger` that originated the logging `Record`. Combined with other
/// `Drain`s, allows custom processing logic for a sub-tree of a whole logging
/// tree.
///
/// Logger is parametrized over type of a `Drain` associated with it (`D`). It
/// default to type-erased version so `Logger` without any type annotation
/// means `Logger<Arc<SendSyncRefUnwindSafeDrain<Ok = (), Err = Never>>>`. See
/// `Logger::root_typed` and `Logger::to_erased` for more information.
#[derive(Clone)]
pub struct Logger<D = Arc<dyn SendSyncRefUnwindSafeDrain<Ok = (), Err = Never>>>
where
    D: SendSyncUnwindSafeDrain<Ok = (), Err = Never>,
{
    drain: D,
    list: OwnedKVList,
}

impl<D> Logger<D>
where
    D: SendSyncUnwindSafeDrain<Ok = (), Err = Never>,
{
    /// Build a root `Logger`
    ///
    /// Root logger starts a new tree associated with a given `Drain`. Root
    /// logger drain must return no errors. See `Drain::ignore_res()` and
    /// `Drain::fuse()`.
    ///
    /// All children and their children (and so on), form one logging tree
    /// sharing a common drain. See `Logger::new`.
    ///
    /// This version (as opposed to `Logger:root_typed`) will take `drain` and
    /// made it into `Arc<SendSyncRefUnwindSafeDrain<Ok = (), Err = Never>>`.
    /// This is typically the most convenient way to work with `Logger`s.
    ///
    /// Use `o!` macro to build `OwnedKV` object.
    ///
    /// ```
    /// #[macro_use]
    /// extern crate slog;
    ///
    /// fn main() {
    ///     let _root = slog::Logger::root(
    ///         slog::Discard,
    ///         o!("key1" => "value1", "key2" => "value2"),
    ///     );
    /// }
    /// ```
    pub fn root<T>(drain: D, values: OwnedKV<T>) -> Logger
    where
        D: 'static + SendSyncRefUnwindSafeDrain<Err = Never, Ok = ()>,
        T: SendSyncRefUnwindSafeKV + 'static,
    {
        Logger {
            drain: Arc::new(drain)
                as Arc<dyn SendSyncRefUnwindSafeDrain<Ok = (), Err = Never>>,
            list: OwnedKVList::root(values),
        }
    }

    /// Build a root `Logger` that retains `drain` type
    ///
    /// Unlike `Logger::root`, this constructor retains the type of a `drain`,
    /// which allows highest performance possible by eliminating indirect call
    /// on `Drain::log`, and allowing monomorphization of `Logger` and `Drain`
    /// objects.
    ///
    /// If you don't understand the implications, you should probably just
    /// ignore it.
    ///
    /// See `Logger:into_erased` and `Logger::to_erased` for conversion from
    /// type returned by this function to version that would be returned by
    /// `Logger::root`.
    pub fn root_typed<T>(drain: D, values: OwnedKV<T>) -> Logger<D>
    where
        D: 'static + SendSyncUnwindSafeDrain<Err = Never, Ok = ()> + Sized,
        T: SendSyncRefUnwindSafeKV + 'static,
    {
        Logger {
            drain: drain,
            list: OwnedKVList::root(values),
        }
    }

    /// Build a child logger
    ///
    /// Child logger inherits all existing key-value pairs from its parent and
    /// supplements them with additional ones.
    ///
    /// Use `o!` macro to build `OwnedKV` object.
    ///
    /// ### Drain cloning (`D : Clone` requirement)
    ///
    /// All children, their children and so on, form one tree sharing a
    /// common drain. This drain, will be `Clone`d when this method is called.
    /// That is why `Clone` must be implemented for `D` in `Logger<D>::new`.
    ///
    /// For some `Drain` types `Clone` is cheap or even free (a no-op). This is
    /// the case for any `Logger` returned by `Logger::root` and it's children.
    ///
    /// When using `Logger::root_typed`, it's possible that cloning might be
    /// expensive, or even impossible.
    ///
    /// The reason why wrapping in an `Arc` is not done internally, and exposed
    /// to the user is performance. Calling `Drain::log` through an `Arc` is
    /// tiny bit slower than doing it directly.
    ///
    /// ```
    /// #[macro_use]
    /// extern crate slog;
    ///
    /// fn main() {
    ///     let root = slog::Logger::root(slog::Discard,
    ///         o!("key1" => "value1", "key2" => "value2"));
    ///     let _log = root.new(o!("key" => "value"));
    /// }
    #[cfg_attr(feature = "cargo-clippy", allow(wrong_self_convention))]
    pub fn new<T>(&self, values: OwnedKV<T>) -> Logger<D>
    where
        T: SendSyncRefUnwindSafeKV + 'static,
        D: Clone,
    {
        Logger {
            drain: self.drain.clone(),
            list: OwnedKVList::new(values, self.list.node.clone()),
        }
    }

    /// Log one logging `Record`
    ///
    /// Use specific logging functions instead. See `log!` macro
    /// documentation.
    #[inline]
    pub fn log(&self, record: &Record) {
        let _ = self.drain.log(record, &self.list);
    }

    /// Get list of key-value pairs assigned to this `Logger`
    pub fn list(&self) -> &OwnedKVList {
        &self.list
    }

    /// Convert to default, "erased" type:
    /// `Logger<Arc<SendSyncUnwindSafeDrain>>`
    ///
    /// Useful to adapt `Logger<D : Clone>` to an interface expecting
    /// `Logger<Arc<...>>`.
    ///
    /// Note that calling on a `Logger<Arc<...>>` will convert it to
    /// `Logger<Arc<Arc<...>>>` which is not optimal. This might be fixed when
    /// Rust gains trait implementation specialization.
    pub fn into_erased(
        self,
    ) -> Logger<Arc<dyn SendSyncRefUnwindSafeDrain<Ok = (), Err = Never>>>
    where
        D: SendRefUnwindSafeDrain + 'static,
    {
        Logger {
            drain: Arc::new(self.drain)
                as Arc<dyn SendSyncRefUnwindSafeDrain<Ok = (), Err = Never>>,
            list: self.list,
        }
    }

    /// Create a copy with "erased" type
    ///
    /// See `into_erased`
    pub fn to_erased(
        &self,
    ) -> Logger<Arc<dyn SendSyncRefUnwindSafeDrain<Ok = (), Err = Never>>>
    where
        D: SendRefUnwindSafeDrain + 'static + Clone,
    {
        self.clone().into_erased()
    }
}

impl<D> fmt::Debug for Logger<D>
where
    D: SendSyncUnwindSafeDrain<Ok = (), Err = Never>,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        try!(write!(f, "Logger{:?}", self.list));
        Ok(())
    }
}

impl<D> Drain for Logger<D>
where
    D: SendSyncUnwindSafeDrain<Ok = (), Err = Never>,
{
    type Ok = ();
    type Err = Never;

    fn log(
        &self,
        record: &Record,
        values: &OwnedKVList,
    ) -> result::Result<Self::Ok, Self::Err> {
        let chained = OwnedKVList {
            node: Arc::new(MultiListNode {
                next_node: values.node.clone(),
                node: self.list.node.clone(),
            }),
        };
        self.drain.log(record, &chained)
    }

    #[inline]
    fn is_enabled(&self, level: Level) -> bool {
        self.drain.is_enabled(level)
    }
}

// {{{ Drain
/// Logging drain
///
/// `Drain`s typically mean destination for logs, but `slog` generalizes the
/// term.
///
/// `Drain`s are responsible for handling logging statements (`Record`s) from
/// `Logger`s associated with them: filtering, modifying, formatting
/// and writing the log records into given destination(s).
///
/// It's a typical pattern to parametrize `Drain`s over `Drain` traits to allow
/// composing `Drain`s.
///
/// Implementing this trait allows writing custom `Drain`s. Slog users should
/// not be afraid of implementing their own `Drain`s. Any custom log handling
/// logic should be implemented as a `Drain`.
pub trait Drain {
    /// Type returned by this drain
    ///
    /// It can be useful in some circumstances, but rarely. It will probably
    /// default to `()` once https://github.com/rust-lang/rust/issues/29661 is
    /// stable.
    type Ok;
    /// Type of potential errors that can be returned by this `Drain`
    type Err;
    /// Handle one logging statement (`Record`)
    ///
    /// Every logging `Record` built from a logging statement (eg.
    /// `info!(...)`), and key-value lists of a `Logger` it was executed on
    /// will be passed to the root drain registered during `Logger::root`.
    ///
    /// Typically `Drain`s:
    ///
    /// * pass this information (or not) to the sub-logger(s) (filters)
    /// * format and write the information the a destination (writers)
    /// * deal with the errors returned from the sub-logger(s)
    fn log(
        &self,
        record: &Record,
        values: &OwnedKVList,
    ) -> result::Result<Self::Ok, Self::Err>;

    /// **Avoid**: Check if messages at the specified log level are **maybe**
    /// enabled for this logger.
    ///
    /// The purpose of it so to allow **imprecise** detection if a given logging
    /// level has any chance of actually being logged. This might be used
    /// to explicitly skip needless computation.
    ///
    /// **It is best effort, can return false positives, but not false negatives.**
    ///
    /// The logger is still free to ignore records even if the level is enabled,
    /// so an enabled level doesn't necessarily guarantee that the record will
    /// actually be logged.
    ///
    /// This function is somewhat needless, and is better expressed by using
    /// lazy values (see `FnValue`).  A `FnValue` is more precise and does not
    /// require additional (potentially recursive) calls to do something that
    /// `log` will already do anyways (making decision if something should be
    /// logged or not).
    ///
    /// ```
    /// # #[macro_use]
    /// # extern crate slog;
    /// # use slog::*;
    /// # fn main() {
    /// let logger = Logger::root(Discard, o!());
    /// if logger.is_enabled(Level::Debug) {
    ///     let num = 5.0f64;
    ///     let sqrt = num.sqrt();
    ///     debug!(logger, "Sqrt"; "num" => num, "sqrt" => sqrt);
    /// }
    /// # }
    /// ```
    #[inline]
    fn is_enabled(&self, level: Level) -> bool {
        level.as_usize() <= ::__slog_static_max_level().as_usize()
    }

    /// **Avoid**: See `is_enabled`
    #[inline]
    fn is_critical_enabled(&self) -> bool {
        self.is_enabled(Level::Critical)
    }

    /// **Avoid**: See `is_enabled`
    #[inline]
    fn is_error_enabled(&self) -> bool {
        self.is_enabled(Level::Error)
    }

    /// **Avoid**: See `is_enabled`
    #[inline]
    fn is_warning_enabled(&self) -> bool {
        self.is_enabled(Level::Warning)
    }

    /// **Avoid**: See `is_enabled`
    #[inline]
    fn is_info_enabled(&self) -> bool {
        self.is_enabled(Level::Info)
    }

    /// **Avoid**: See `is_enabled`
    #[inline]
    fn is_debug_enabled(&self) -> bool {
        self.is_enabled(Level::Debug)
    }

    /// **Avoid**: See `is_enabled`
    #[inline]
    fn is_trace_enabled(&self) -> bool {
        self.is_enabled(Level::Trace)
    }

    /// Pass `Drain` through a closure, eg. to wrap
    /// into another `Drain`.
    ///
    /// ```
    /// #[macro_use]
    /// extern crate slog;
    /// use slog::*;
    ///
    /// fn main() {
    ///     let _drain = Discard.map(Fuse);
    /// }
    /// ```
    fn map<F, R>(self, f: F) -> R
    where
        Self: Sized,
        F: FnOnce(Self) -> R,
    {
        f(self)
    }

    /// Filter logging records passed to `Drain`
    ///
    /// Wrap `Self` in `Filter`
    ///
    /// This will convert `self` to a `Drain that ignores `Record`s
    /// for which `f` returns false.
    fn filter<F>(self, f: F) -> Filter<Self, F>
    where
        Self: Sized,
        F: FilterFn,
    {
        Filter::new(self, f)
    }

    /// Filter logging records passed to `Drain` (by level)
    ///
    /// Wrap `Self` in `LevelFilter`
    ///
    /// This will convert `self` to a `Drain that ignores `Record`s of
    /// logging lever smaller than `level`.
    fn filter_level(self, level: Level) -> LevelFilter<Self>
    where
        Self: Sized,
    {
        LevelFilter(self, level)
    }

    /// Map logging errors returned by this drain
    ///
    /// `f` is a closure that takes `Drain::Err` returned by a given
    /// drain, and returns new error of potentially different type
    fn map_err<F, E>(self, f: F) -> MapError<Self, E>
    where
        Self: Sized,
        F: MapErrFn<Self::Err, E>,
    {
        MapError::new(self, f)
    }

    /// Ignore results returned by this drain
    ///
    /// Wrap `Self` in `IgnoreResult`
    fn ignore_res(self) -> IgnoreResult<Self>
    where
        Self: Sized,
    {
        IgnoreResult::new(self)
    }

    /// Make `Self` panic when returning any errors
    ///
    /// Wrap `Self` in `Map`
    fn fuse(self) -> Fuse<Self>
    where
        Self::Err: fmt::Debug,
        Self: Sized,
    {
        self.map(Fuse)
    }
}

impl<'a, D: Drain + 'a> Drain for &'a D {
    type Ok = D::Ok;
    type Err = D::Err;
    #[inline]
    fn log(
        &self,
        record: &Record,
        values: &OwnedKVList,
    ) -> result::Result<Self::Ok, Self::Err> {
        (**self).log(record, values)
    }
    #[inline]
    fn is_enabled(&self, level: Level) -> bool {
        (**self).is_enabled(level)
    }
}

impl<'a, D: Drain + 'a> Drain for &'a mut D {
    type Ok = D::Ok;
    type Err = D::Err;
    #[inline]
    fn log(
        &self,
        record: &Record,
        values: &OwnedKVList,
    ) -> result::Result<Self::Ok, Self::Err> {
        (**self).log(record, values)
    }
    #[inline]
    fn is_enabled(&self, level: Level) -> bool {
        (**self).is_enabled(level)
    }
}

#[cfg(feature = "std")]
/// `Send + Sync + UnwindSafe` bound
///
/// This type is used to enforce `Drain`s associated with `Logger`s
/// are thread-safe.
pub trait SendSyncUnwindSafe: Send + Sync + UnwindSafe {}

#[cfg(feature = "std")]
impl<T> SendSyncUnwindSafe for T
where
    T: Send + Sync + UnwindSafe + ?Sized,
{
}

#[cfg(feature = "std")]
/// `Drain + Send + Sync + UnwindSafe` bound
///
/// This type is used to enforce `Drain`s associated with `Logger`s
/// are thread-safe.
pub trait SendSyncUnwindSafeDrain: Drain + Send + Sync + UnwindSafe {}

#[cfg(feature = "std")]
impl<T> SendSyncUnwindSafeDrain for T
where
    T: Drain + Send + Sync + UnwindSafe + ?Sized,
{
}

#[cfg(feature = "std")]
/// `Drain + Send + Sync + RefUnwindSafe` bound
///
/// This type is used to enforce `Drain`s associated with `Logger`s
/// are thread-safe.
pub trait SendSyncRefUnwindSafeDrain: Drain + Send + Sync + RefUnwindSafe {}

#[cfg(feature = "std")]
impl<T> SendSyncRefUnwindSafeDrain for T
where
    T: Drain + Send + Sync + RefUnwindSafe + ?Sized,
{
}

#[cfg(feature = "std")]
/// Function that can be used in `MapErr` drain
pub trait MapErrFn<EI, EO>
    : 'static + Sync + Send + UnwindSafe + RefUnwindSafe + Fn(EI) -> EO {
}

#[cfg(feature = "std")]
impl<T, EI, EO> MapErrFn<EI, EO> for T
where
    T: 'static
        + Sync
        + Send
        + ?Sized
        + UnwindSafe
        + RefUnwindSafe
        + Fn(EI) -> EO,
{
}

#[cfg(feature = "std")]
/// Function that can be used in `Filter` drain
pub trait FilterFn
    : 'static + Sync + Send + UnwindSafe + RefUnwindSafe + Fn(&Record) -> bool {
}

#[cfg(feature = "std")]
impl<T> FilterFn for T
where
    T: 'static
        + Sync
        + Send
        + ?Sized
        + UnwindSafe
        + RefUnwindSafe
        + Fn(&Record) -> bool,
{
}

#[cfg(not(feature = "std"))]
/// `Drain + Send + Sync + UnwindSafe` bound
///
/// This type is used to enforce `Drain`s associated with `Logger`s
/// are thread-safe.
pub trait SendSyncUnwindSafeDrain: Drain + Send + Sync {}

#[cfg(not(feature = "std"))]
impl<T> SendSyncUnwindSafeDrain for T
where
    T: Drain + Send + Sync + ?Sized,
{
}

#[cfg(not(feature = "std"))]
/// `Drain + Send + Sync + RefUnwindSafe` bound
///
/// This type is used to enforce `Drain`s associated with `Logger`s
/// are thread-safe.
pub trait SendSyncRefUnwindSafeDrain: Drain + Send + Sync {}

#[cfg(not(feature = "std"))]
impl<T> SendSyncRefUnwindSafeDrain for T
where
    T: Drain + Send + Sync + ?Sized,
{
}

#[cfg(feature = "std")]
/// `Drain + Send + RefUnwindSafe` bound
pub trait SendRefUnwindSafeDrain: Drain + Send + RefUnwindSafe {}

#[cfg(feature = "std")]
impl<T> SendRefUnwindSafeDrain for T
where
    T: Drain + Send + RefUnwindSafe + ?Sized,
{
}

#[cfg(not(feature = "std"))]
/// `Drain + Send + RefUnwindSafe` bound
pub trait SendRefUnwindSafeDrain: Drain + Send {}

#[cfg(not(feature = "std"))]
impl<T> SendRefUnwindSafeDrain for T
where
    T: Drain + Send + ?Sized,
{
}

#[cfg(not(feature = "std"))]
/// Function that can be used in `MapErr` drain
pub trait MapErrFn<EI, EO>: 'static + Sync + Send + Fn(EI) -> EO {}

#[cfg(not(feature = "std"))]
impl<T, EI, EO> MapErrFn<EI, EO> for T
where
    T: 'static + Sync + Send + ?Sized + Fn(EI) -> EO,
{
}

#[cfg(not(feature = "std"))]
/// Function that can be used in `Filter` drain
pub trait FilterFn: 'static + Sync + Send + Fn(&Record) -> bool {}

#[cfg(not(feature = "std"))]
impl<T> FilterFn for T
where
    T: 'static + Sync + Send + ?Sized + Fn(&Record) -> bool,
{
}

impl<D: Drain + ?Sized> Drain for Box<D> {
    type Ok = D::Ok;
    type Err = D::Err;
    fn log(
        &self,
        record: &Record,
        o: &OwnedKVList,
    ) -> result::Result<Self::Ok, D::Err> {
        (**self).log(record, o)
    }
    #[inline]
    fn is_enabled(&self, level: Level) -> bool {
        (**self).is_enabled(level)
    }
}

impl<D: Drain + ?Sized> Drain for Arc<D> {
    type Ok = D::Ok;
    type Err = D::Err;
    fn log(
        &self,
        record: &Record,
        o: &OwnedKVList,
    ) -> result::Result<Self::Ok, D::Err> {
        (**self).log(record, o)
    }
    #[inline]
    fn is_enabled(&self, level: Level) -> bool {
        (**self).is_enabled(level)
    }
}

/// `Drain` discarding everything
///
/// `/dev/null` of `Drain`s
#[derive(Debug, Copy, Clone)]
pub struct Discard;

impl Drain for Discard {
    type Ok = ();
    type Err = Never;
    fn log(&self, _: &Record, _: &OwnedKVList) -> result::Result<(), Never> {
        Ok(())
    }
    #[inline]
    fn is_enabled(&self, _1: Level) -> bool {
        false
    }
}

/// `Drain` filtering records
///
/// Wraps another `Drain` and passes `Record`s to it, only if they satisfy a
/// given condition.
#[derive(Debug, Clone)]
pub struct Filter<D: Drain, F>(pub D, pub F)
where
    F: Fn(&Record) -> bool + 'static + Send + Sync;

impl<D: Drain, F> Filter<D, F>
where
    F: FilterFn,
{
    /// Create `Filter` wrapping given `drain`
    pub fn new(drain: D, cond: F) -> Self {
        Filter(drain, cond)
    }
}

impl<D: Drain, F> Drain for Filter<D, F>
where
    F: FilterFn,
{
    type Ok = Option<D::Ok>;
    type Err = D::Err;
    fn log(
        &self,
        record: &Record,
        logger_values: &OwnedKVList,
    ) -> result::Result<Self::Ok, Self::Err> {
        if (self.1)(record) {
            Ok(Some(self.0.log(record, logger_values)?))
        } else {
            Ok(None)
        }
    }
    #[inline]
    fn is_enabled(&self, level: Level) -> bool {
        /*
         * This is one of the reasons we can't guarantee the value is actually logged.
         * The filter function is given dynamic control over whether or not the record is logged
         * and could filter stuff out even if the log level is supposed to be enabled
         */
        self.0.is_enabled(level)
    }
}

/// `Drain` filtering records by `Record` logging level
///
/// Wraps a drain and passes records to it, only
/// if their level is at least given level.
///
/// TODO: Remove this type. This drain is a special case of `Filter`, but
/// because `Filter` can not use static dispatch ATM due to Rust limitations
/// that will be lifted in the future, it is a standalone type.
/// Reference: https://github.com/rust-lang/rust/issues/34511
#[derive(Debug, Clone)]
pub struct LevelFilter<D: Drain>(pub D, pub Level);

impl<D: Drain> LevelFilter<D> {
    /// Create `LevelFilter`
    pub fn new(drain: D, level: Level) -> Self {
        LevelFilter(drain, level)
    }
}

impl<D: Drain> Drain for LevelFilter<D> {
    type Ok = Option<D::Ok>;
    type Err = D::Err;
    fn log(
        &self,
        record: &Record,
        logger_values: &OwnedKVList,
    ) -> result::Result<Self::Ok, Self::Err> {
        if record.level().is_at_least(self.1) {
            Ok(Some(self.0.log(record, logger_values)?))
        } else {
            Ok(None)
        }
    }
    #[inline]
    fn is_enabled(&self, level: Level) -> bool {
        level.is_at_least(self.1) && self.0.is_enabled(level)
    }
}

/// `Drain` mapping error returned by another `Drain`
///
/// See `Drain::map_err` for convenience function.
pub struct MapError<D: Drain, E> {
    drain: D,
    // eliminated dynamic dispatch, after rust learns `-> impl Trait`
    map_fn: Box<dyn MapErrFn<D::Err, E, Output = E>>,
}

impl<D: Drain, E> MapError<D, E> {
    /// Create `Filter` wrapping given `drain`
    pub fn new<F>(drain: D, map_fn: F) -> Self
    where
        F: MapErrFn<<D as Drain>::Err, E>,
    {
        MapError {
            drain: drain,
            map_fn: Box::new(map_fn),
        }
    }
}

impl<D: Drain, E> Drain for MapError<D, E> {
    type Ok = D::Ok;
    type Err = E;
    fn log(
        &self,
        record: &Record,
        logger_values: &OwnedKVList,
    ) -> result::Result<Self::Ok, Self::Err> {
        self.drain
            .log(record, logger_values)
            .map_err(|e| (self.map_fn)(e))
    }
    #[inline]
    fn is_enabled(&self, level: Level) -> bool {
        self.drain.is_enabled(level)
    }
}

/// `Drain` duplicating records into two other `Drain`s
///
/// Can be nested for more than two outputs.
#[derive(Debug, Clone)]
pub struct Duplicate<D1: Drain, D2: Drain>(pub D1, pub D2);

impl<D1: Drain, D2: Drain> Duplicate<D1, D2> {
    /// Create `Duplicate`
    pub fn new(drain1: D1, drain2: D2) -> Self {
        Duplicate(drain1, drain2)
    }
}

impl<D1: Drain, D2: Drain> Drain for Duplicate<D1, D2> {
    type Ok = (D1::Ok, D2::Ok);
    type Err = (
        result::Result<D1::Ok, D1::Err>,
        result::Result<D2::Ok, D2::Err>,
    );
    fn log(
        &self,
        record: &Record,
        logger_values: &OwnedKVList,
    ) -> result::Result<Self::Ok, Self::Err> {
        let res1 = self.0.log(record, logger_values);
        let res2 = self.1.log(record, logger_values);

        match (res1, res2) {
            (Ok(o1), Ok(o2)) => Ok((o1, o2)),
            (r1, r2) => Err((r1, r2)),
        }
    }
    #[inline]
    fn is_enabled(&self, level: Level) -> bool {
        self.0.is_enabled(level) || self.1.is_enabled(level)
    }
}

/// `Drain` panicking on error
///
/// `Logger` requires a root drain to handle all errors (`Drain::Error == ()`),
/// `Fuse` will wrap a `Drain` and panic if it returns any errors.
///
/// Note: `Drain::Err` must implement `Display` (for displaying on panick). It's
/// easy to create own `Fuse` drain if this requirement can't be fulfilled.
#[derive(Debug, Clone)]
pub struct Fuse<D: Drain>(pub D)
where
    D::Err: fmt::Debug;

impl<D: Drain> Fuse<D>
where
    D::Err: fmt::Debug,
{
    /// Create `Fuse` wrapping given `drain`
    pub fn new(drain: D) -> Self {
        Fuse(drain)
    }
}

impl<D: Drain> Drain for Fuse<D>
where
    D::Err: fmt::Debug,
{
    type Ok = ();
    type Err = Never;
    fn log(
        &self,
        record: &Record,
        logger_values: &OwnedKVList,
    ) -> result::Result<Self::Ok, Never> {
        let _ = self.0
            .log(record, logger_values)
            .unwrap_or_else(|e| panic!("slog::Fuse Drain: {:?}", e));
        Ok(())
    }
    #[inline]
    fn is_enabled(&self, level: Level) -> bool {
        self.0.is_enabled(level)
    }
}

/// `Drain` ignoring result
///
/// `Logger` requires a root drain to handle all errors (`Drain::Err=()`), and
/// returns nothing (`Drain::Ok=()`) `IgnoreResult` will ignore any result
/// returned by the `Drain` it wraps.
#[derive(Clone)]
pub struct IgnoreResult<D: Drain> {
    drain: D,
}

impl<D: Drain> IgnoreResult<D> {
    /// Create `IgnoreResult` wrapping `drain`
    pub fn new(drain: D) -> Self {
        IgnoreResult { drain: drain }
    }
}

impl<D: Drain> Drain for IgnoreResult<D> {
    type Ok = ();
    type Err = Never;
    fn log(
        &self,
        record: &Record,
        logger_values: &OwnedKVList,
    ) -> result::Result<(), Never> {
        let _ = self.drain.log(record, logger_values);
        Ok(())
    }

    #[inline]
    fn is_enabled(&self, level: Level) -> bool {
        self.drain.is_enabled(level)
    }
}

/// Error returned by `Mutex<D : Drain>`
#[cfg(feature = "std")]
#[derive(Clone)]
pub enum MutexDrainError<D: Drain> {
    /// Error acquiring mutex
    Mutex,
    /// Error returned by drain
    Drain(D::Err),
}

#[cfg(feature = "std")]
impl<D> fmt::Debug for MutexDrainError<D>
where
    D: Drain,
    D::Err: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> result::Result<(), fmt::Error> {
        match *self {
            MutexDrainError::Mutex => write!(f, "MutexDrainError::Mutex"),
            MutexDrainError::Drain(ref e) => e.fmt(f),
        }
    }
}

#[cfg(feature = "std")]
impl<D> std::error::Error for MutexDrainError<D>
where
    D: Drain,
    D::Err: fmt::Debug + fmt::Display + std::error::Error,
{
    fn description(&self) -> &str {
        match *self {
            MutexDrainError::Mutex => "Mutex acquire failed",
            MutexDrainError::Drain(ref e) => e.description(),
        }
    }

    fn cause(&self) -> Option<&dyn std::error::Error> {
        match *self {
            MutexDrainError::Mutex => None,
            MutexDrainError::Drain(ref e) => Some(e),
        }
    }
}

#[cfg(feature = "std")]
impl<'a, D: Drain> From<std::sync::PoisonError<std::sync::MutexGuard<'a, D>>>
    for MutexDrainError<D> {
    fn from(
        _: std::sync::PoisonError<std::sync::MutexGuard<'a, D>>,
    ) -> MutexDrainError<D> {
        MutexDrainError::Mutex
    }
}

#[cfg(feature = "std")]
impl<D: Drain> fmt::Display for MutexDrainError<D>
where
    D::Err: fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> result::Result<(), fmt::Error> {
        match *self {
            MutexDrainError::Mutex => write!(f, "MutexError"),
            MutexDrainError::Drain(ref e) => write!(f, "{}", e),
        }
    }
}

#[cfg(feature = "std")]
impl<D: Drain> Drain for std::sync::Mutex<D> {
    type Ok = D::Ok;
    type Err = MutexDrainError<D>;
    fn log(
        &self,
        record: &Record,
        logger_values: &OwnedKVList,
    ) -> result::Result<Self::Ok, Self::Err> {
        let d = self.lock()?;
        d.log(record, logger_values).map_err(MutexDrainError::Drain)
    }
    #[inline]
    fn is_enabled(&self, level: Level) -> bool {
        self.lock().ok().map_or(true, |lock| lock.is_enabled(level))
    }
}
// }}}

// {{{ Level & FilterLevel
/// Official capitalized logging (and logging filtering) level names
///
/// In order of `as_usize()`.
pub static LOG_LEVEL_NAMES: [&'static str; 7] =
    ["OFF", "CRITICAL", "ERROR", "WARN", "INFO", "DEBUG", "TRACE"];

/// Official capitalized logging (and logging filtering) short level names
///
/// In order of `as_usize()`.
pub static LOG_LEVEL_SHORT_NAMES: [&'static str; 7] =
    ["OFF", "CRIT", "ERRO", "WARN", "INFO", "DEBG", "TRCE"];

/// Logging level associated with a logging `Record`
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum Level {
    /// Critical
    Critical,
    /// Error
    Error,
    /// Warning
    Warning,
    /// Info
    Info,
    /// Debug
    Debug,
    /// Trace
    Trace,
}

/// Logging filtering level
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum FilterLevel {
    /// Log nothing
    Off,
    /// Log critical level only
    Critical,
    /// Log only error level and above
    Error,
    /// Log only warning level and above
    Warning,
    /// Log only info level and above
    Info,
    /// Log only debug level and above
    Debug,
    /// Log everything
    Trace,
}

impl Level {
    /// Convert to `str` from `LOG_LEVEL_SHORT_NAMES`
    pub fn as_short_str(&self) -> &'static str {
        LOG_LEVEL_SHORT_NAMES[self.as_usize()]
    }

    /// Convert to `str` from `LOG_LEVEL_NAMES`
    pub fn as_str(&self) -> &'static str {
        LOG_LEVEL_NAMES[self.as_usize()]
    }

    /// Cast `Level` to ordering integer
    ///
    /// `Critical` is the smallest and `Trace` the biggest value
    #[inline]
    pub fn as_usize(&self) -> usize {
        match *self {
            Level::Critical => 1,
            Level::Error => 2,
            Level::Warning => 3,
            Level::Info => 4,
            Level::Debug => 5,
            Level::Trace => 6,
        }
    }

    /// Get a `Level` from an `usize`
    ///
    /// This complements `as_usize`
    #[inline]
    pub fn from_usize(u: usize) -> Option<Level> {
        match u {
            1 => Some(Level::Critical),
            2 => Some(Level::Error),
            3 => Some(Level::Warning),
            4 => Some(Level::Info),
            5 => Some(Level::Debug),
            6 => Some(Level::Trace),
            _ => None,
        }
    }
}

impl FilterLevel {
    /// Convert to `usize` value
    ///
    /// `Off` is 0, and `Trace` 6
    #[inline]
    pub fn as_usize(&self) -> usize {
        match *self {
            FilterLevel::Off => 0,
            FilterLevel::Critical => 1,
            FilterLevel::Error => 2,
            FilterLevel::Warning => 3,
            FilterLevel::Info => 4,
            FilterLevel::Debug => 5,
            FilterLevel::Trace => 6,
        }
    }

    /// Get a `FilterLevel` from an `usize`
    ///
    /// This complements `as_usize`
    #[inline]
    pub fn from_usize(u: usize) -> Option<FilterLevel> {
        match u {
            0 => Some(FilterLevel::Off),
            1 => Some(FilterLevel::Critical),
            2 => Some(FilterLevel::Error),
            3 => Some(FilterLevel::Warning),
            4 => Some(FilterLevel::Info),
            5 => Some(FilterLevel::Debug),
            6 => Some(FilterLevel::Trace),
            _ => None,
        }
    }

    /// Maximum logging level (log everything)
    #[inline]
    pub fn max() -> Self {
        FilterLevel::Trace
    }

    /// Minimum logging level (log nothing)
    #[inline]
    pub fn min() -> Self {
        FilterLevel::Off
    }
}

#[cfg_attr(rustfmt, rustfmt_skip)]
static ASCII_LOWERCASE_MAP: [u8; 256] =
    [0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a, 0x0b,
     0x0c, 0x0d, 0x0e, 0x0f, 0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17,
     0x18, 0x19, 0x1a, 0x1b, 0x1c, 0x1d, 0x1e, 0x1f, b' ', b'!', b'"', b'#',
     b'$', b'%', b'&', b'\'', b'(', b')', b'*', b'+', b',', b'-', b'.', b'/',
     b'0', b'1', b'2', b'3', b'4', b'5', b'6', b'7', b'8', b'9', b':', b';',
     b'<', b'=', b'>', b'?', b'@', b'a', b'b', b'c', b'd', b'e', b'f', b'g',
     b'h', b'i', b'j', b'k', b'l', b'm', b'n', b'o', b'p', b'q', b'r', b's',
     b't', b'u', b'v', b'w', b'x', b'y', b'z', b'[', b'\\', b']', b'^', b'_',
     b'`', b'a', b'b', b'c', b'd', b'e', b'f', b'g', b'h', b'i', b'j', b'k',
     b'l', b'm', b'n', b'o', b'p', b'q', b'r', b's', b't', b'u', b'v', b'w',
     b'x', b'y', b'z', b'{', b'|', b'}', b'~', 0x7f, 0x80, 0x81, 0x82, 0x83,
     0x84, 0x85, 0x86, 0x87, 0x88, 0x89, 0x8a, 0x8b, 0x8c, 0x8d, 0x8e, 0x8f,
     0x90, 0x91, 0x92, 0x93, 0x94, 0x95, 0x96, 0x97, 0x98, 0x99, 0x9a, 0x9b,
     0x9c, 0x9d, 0x9e, 0x9f, 0xa0, 0xa1, 0xa2, 0xa3, 0xa4, 0xa5, 0xa6, 0xa7,
     0xa8, 0xa9, 0xaa, 0xab, 0xac, 0xad, 0xae, 0xaf, 0xb0, 0xb1, 0xb2, 0xb3,
     0xb4, 0xb5, 0xb6, 0xb7, 0xb8, 0xb9, 0xba, 0xbb, 0xbc, 0xbd, 0xbe, 0xbf,
     0xc0, 0xc1, 0xc2, 0xc3, 0xc4, 0xc5, 0xc6, 0xc7, 0xc8, 0xc9, 0xca, 0xcb,
     0xcc, 0xcd, 0xce, 0xcf, 0xd0, 0xd1, 0xd2, 0xd3, 0xd4, 0xd5, 0xd6, 0xd7,
     0xd8, 0xd9, 0xda, 0xdb, 0xdc, 0xdd, 0xde, 0xdf, 0xe0, 0xe1, 0xe2, 0xe3,
     0xe4, 0xe5, 0xe6, 0xe7, 0xe8, 0xe9, 0xea, 0xeb, 0xec, 0xed, 0xee, 0xef,
     0xf0, 0xf1, 0xf2, 0xf3, 0xf4, 0xf5, 0xf6, 0xf7, 0xf8, 0xf9, 0xfa, 0xfb,
     0xfc, 0xfd, 0xfe, 0xff];

impl FromStr for Level {
    type Err = ();
    fn from_str(level: &str) -> core::result::Result<Level, ()> {
        LOG_LEVEL_NAMES
            .iter()
            .position(|&name| {
                name.as_bytes().iter().zip(level.as_bytes().iter()).all(
                    |(a, b)| {
                        ASCII_LOWERCASE_MAP[*a as usize]
                            == ASCII_LOWERCASE_MAP[*b as usize]
                    },
                )
            })
            .map(|p| Level::from_usize(p).unwrap())
            .ok_or(())
    }
}

impl FromStr for FilterLevel {
    type Err = ();
    fn from_str(level: &str) -> core::result::Result<FilterLevel, ()> {
        LOG_LEVEL_NAMES
            .iter()
            .position(|&name| {
                name.as_bytes().iter().zip(level.as_bytes().iter()).all(
                    |(a, b)| {
                        ASCII_LOWERCASE_MAP[*a as usize]
                            == ASCII_LOWERCASE_MAP[*b as usize]
                    },
                )
            })
            .map(|p| FilterLevel::from_usize(p).unwrap())
            .ok_or(())
    }
}

impl fmt::Display for Level {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.as_short_str())
    }
}

impl Level {
    /// Returns true if `self` is at least `level` logging level
    #[inline]
    pub fn is_at_least(&self, level: Self) -> bool {
        self.as_usize() <= level.as_usize()
    }
}

#[test]
fn level_at_least() {
    assert!(Level::Debug.is_at_least(Level::Debug));
    assert!(Level::Debug.is_at_least(Level::Trace));
    assert!(!Level::Debug.is_at_least(Level::Info));
}

#[test]
fn filterlevel_sanity() {
    assert!(Level::Critical.as_usize() > FilterLevel::Off.as_usize());
    assert!(Level::Critical.as_usize() == FilterLevel::Critical.as_usize());
    assert!(Level::Trace.as_usize() == FilterLevel::Trace.as_usize());
}

#[test]
fn level_from_str() {
    assert_eq!("info".parse::<FilterLevel>().unwrap(), FilterLevel::Info);
}
// }}}

// {{{ Record
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct RecordLocation {
    /// File
    pub file: &'static str,
    /// Line
    pub line: u32,
    /// Column (currently not implemented)
    pub column: u32,
    /// Function (currently not implemented)
    pub function: &'static str,
    /// Module
    pub module: &'static str,
}
#[doc(hidden)]
/// Information that can be static in the given record thus allowing to optimize
/// record creation to be done mostly at compile-time.
///
/// This is not cosidered a part of stable API, and macros should be used
/// instead.
pub struct RecordStatic<'a> {
    /// Code location
    pub location: &'a RecordLocation,
    /// Tag
    pub tag: &'a str,
    /// Logging level
    pub level: Level,
}

/// One logging record
///
/// Corresponds to one logging statement like `info!(...)` and carries all it's
/// data: eg. message, immediate key-value pairs and key-value pairs of `Logger`
/// used to execute it.
///
/// Record is passed to a `Logger`, which delivers it to it's own `Drain`,
/// where actual logging processing is implemented.
pub struct Record<'a> {
    rstatic: &'a RecordStatic<'a>,
    msg: &'a fmt::Arguments<'a>,
    kv: BorrowedKV<'a>,
}

impl<'a> Record<'a> {
    /// Create a new `Record`
    ///
    /// This function is not considered a part of stable API
    #[inline]
    #[doc(hidden)]
    pub fn new(
        s: &'a RecordStatic<'a>,
        msg: &'a fmt::Arguments<'a>,
        kv: BorrowedKV<'a>,
    ) -> Self {
        Record {
            rstatic: s,
            msg: msg,
            kv: kv,
        }
    }

    /// Get a log record message
    pub fn msg(&self) -> &fmt::Arguments {
        self.msg
    }

    /// Get record logging level
    pub fn level(&self) -> Level {
        self.rstatic.level
    }

    /// Get line number
    pub fn line(&self) -> u32 {
        self.rstatic.location.line
    }

    /// Get line number
    pub fn location(&self) -> &RecordLocation {
        self.rstatic.location
    }

    /// Get error column
    pub fn column(&self) -> u32 {
        self.rstatic.location.column
    }

    /// Get file path
    pub fn file(&self) -> &'static str {
        self.rstatic.location.file
    }

    /// Get tag
    ///
    /// Tag is information that can be attached to `Record` that is not meant
    /// to be part of the norma key-value pairs, but only as an ad-hoc control
    /// flag for quick lookup in the `Drain`s. As such should be used carefully
    /// and mostly in application code (as opposed to libraries) - where tag
    /// meaning across the system can be coordinated. When used in libraries,
    /// make sure to prefix is with something reasonably distinct, like create
    /// name.
    pub fn tag(&self) -> &str {
        self.rstatic.tag
    }

    /// Get module
    pub fn module(&self) -> &'static str {
        self.rstatic.location.module
    }

    /// Get function (placeholder)
    ///
    /// There's currently no way to obtain that information
    /// in Rust at compile time, so it is not implemented.
    ///
    /// It will be implemented at first opportunity, and
    /// it will not be considered a breaking change.
    pub fn function(&self) -> &'static str {
        self.rstatic.location.function
    }

    /// Get key-value pairs
    pub fn kv(&self) -> BorrowedKV {
        BorrowedKV(self.kv.0)
    }
}
// }}}

// {{{ Serializer
macro_rules! impl_default_as_fmt{
    ($t:ty, $f:ident) => {
        /// Emit $t
        fn $f(&mut self, key : Key, val : $t)
            -> Result {
                self.emit_arguments(key, &format_args!("{}", val))
            }
    };
}

/// This is a workaround to be able to pass &mut Serializer, from
/// `Serializer::emit_serde` default implementation. `&Self` can't be casted to
/// `&Serializer` (without : Sized, which break object safety), but it can be
/// used as <T: Serializer>.
#[cfg(feature = "nested-values")]
struct SerializerForward<'a, T: 'a + ?Sized>(&'a mut T);

#[cfg(feature = "nested-values")]
impl<'a, T: Serializer + 'a + ?Sized> Serializer for SerializerForward<'a, T> {
    fn emit_arguments(&mut self, key: Key, val: &fmt::Arguments) -> Result {
        self.0.emit_arguments(key, val)
    }

    #[cfg(feature = "nested-values")]
    fn emit_serde(&mut self, _key: Key, _value: &SerdeValue) -> Result {
        panic!();
    }
}

/// Serializer
///
/// Drains using `Format` will internally use
/// types implementing this trait.
pub trait Serializer {
    /// Emit usize
    impl_default_as_fmt!(usize, emit_usize);
    /// Emit isize
    impl_default_as_fmt!(isize, emit_isize);
    /// Emit bool
    impl_default_as_fmt!(bool, emit_bool);
    /// Emit char
    impl_default_as_fmt!(char, emit_char);
    /// Emit u8
    impl_default_as_fmt!(u8, emit_u8);
    /// Emit i8
    impl_default_as_fmt!(i8, emit_i8);
    /// Emit u16
    impl_default_as_fmt!(u16, emit_u16);
    /// Emit i16
    impl_default_as_fmt!(i16, emit_i16);
    /// Emit u32
    impl_default_as_fmt!(u32, emit_u32);
    /// Emit i32
    impl_default_as_fmt!(i32, emit_i32);
    /// Emit f32
    impl_default_as_fmt!(f32, emit_f32);
    /// Emit u64
    impl_default_as_fmt!(u64, emit_u64);
    /// Emit i64
    impl_default_as_fmt!(i64, emit_i64);
    /// Emit f64
    impl_default_as_fmt!(f64, emit_f64);
    /// Emit str
    impl_default_as_fmt!(&str, emit_str);

    /// Emit `()`
    fn emit_unit(&mut self, key: Key) -> Result {
        self.emit_arguments(key, &format_args!("()"))
    }

    /// Emit `None`
    fn emit_none(&mut self, key: Key) -> Result {
        self.emit_arguments(key, &format_args!(""))
    }

    /// Emit `fmt::Arguments`
    ///
    /// This is the only method that has to implemented, but for performance and
    /// to retain type information most serious `Serializer`s will want to
    /// implement all other methods as well.
    fn emit_arguments(&mut self, key: Key, val: &fmt::Arguments) -> Result;

    /// Emit a value implementing
    /// [`serde::Serialize`](https://docs.rs/serde/1/serde/trait.Serialize.html)
    ///
    /// This is especially useful for composite values, eg. structs as Json values, or sequences.
    ///
    /// To prevent pulling-in `serde` dependency, this is an extension behind a
    /// `serde` feature flag.
    ///
    /// The value needs to implement `SerdeValue`.
    #[cfg(feature = "nested-values")]
    fn emit_serde(&mut self, key: Key, value: &SerdeValue) -> Result {
        value.serialize_fallback(key, &mut SerializerForward(self))
    }
}

/// Serializer to closure adapter.
///
/// Formats all arguments as `fmt::Arguments` and passes them to a given closure.
struct AsFmtSerializer<F>(crate F)
where
    F: for<'a> FnMut(Key, fmt::Arguments<'a>) -> Result;

impl<F> Serializer for AsFmtSerializer<F>
where
    F: for<'a> FnMut(Key, fmt::Arguments<'a>) -> Result,
{
    fn emit_arguments(&mut self, key: Key, val: &fmt::Arguments) -> Result {
        (self.0)(key, *val)
    }
}
// }}}

// {{{ serde
/// A value that can be serialized via serde
///
/// This is useful for implementing nested values, like sequences or structures.
#[cfg(feature = "nested-values")]
pub trait SerdeValue: erased_serde::Serialize + Value {
    /// Serialize the value in a way that is compatible with `slog::Serializer`s
    /// that do not support serde.
    ///
    /// The implementation should *not* call `slog::Serialize::serialize`
    /// on itself, as it will lead to infinite recursion.
    ///
    /// Default implementation is provided, but it returns error, so use it
    /// only for internal types in systems and libraries where `serde` is always
    /// enabled.
    fn serialize_fallback(
        &self,
        _key: Key,
        _serializer: &mut Serializer,
    ) -> Result<()> {
        Err(Error::Other)
    }

    /// Convert to `erased_serialize::Serialize` of the underlying value,
    /// so `slog::Serializer`s can use it to serialize via `serde`.
    fn as_serde(&self) -> &erased_serde::Serialize;

    /// Convert to a boxed value that can be sent across threads
    ///
    /// This enables functionality like `slog-async` and similar.
    fn to_sendable(&self) -> Box<SerdeValue + Send + 'static>;
}

// }}}

// {{{ Value
/// # Value that can be serialized
///
/// Types that implement this type implement custome serialization in the
/// structured part of the log macros. Without an implementation of `Value` for
/// your type you must emit using either the `?` "debug", `%` "display" or
/// [`SerdeValue`](trait.SerdeValue.html) (if you have the `nested-values`
/// feature enabled) formatters.
///
/// # Example
///
/// ```
/// use slog::{Key, Value, Record, Result, Serializer};
/// struct MyNewType(i64);
///
/// impl Value for MyNewType {
///     fn serialize(&self, _rec: &Record, key: Key, serializer: &mut Serializer) -> Result {
///         serializer.emit_i64(key, self.0)
///     }
/// }
/// ```
///
/// See also [`KV`](trait.KV.html) for formatting both the key and value.
pub trait Value {
    /// Serialize self into `Serializer`
    ///
    /// Structs implementing this trait should generally
    /// only call respective methods of `serializer`.
    fn serialize(
        &self,
        record: &Record,
        key: Key,
        serializer: &mut dyn Serializer,
    ) -> Result;
}

impl<'a, V> Value for &'a V
where
    V: Value + ?Sized,
{
    fn serialize(
        &self,
        record: &Record,
        key: Key,
        serializer: &mut dyn Serializer,
    ) -> Result {
        (*self).serialize(record, key, serializer)
    }
}

macro_rules! impl_value_for{
    ($t:ty, $f:ident) => {
        impl Value for $t {
            fn serialize(&self,
                         _record : &Record,
                         key : Key,
                         serializer : &mut dyn Serializer
                         ) -> Result {
                serializer.$f(key, *self)
            }
        }
    };
}

impl_value_for!(usize, emit_usize);
impl_value_for!(isize, emit_isize);
impl_value_for!(bool, emit_bool);
impl_value_for!(char, emit_char);
impl_value_for!(u8, emit_u8);
impl_value_for!(i8, emit_i8);
impl_value_for!(u16, emit_u16);
impl_value_for!(i16, emit_i16);
impl_value_for!(u32, emit_u32);
impl_value_for!(i32, emit_i32);
impl_value_for!(f32, emit_f32);
impl_value_for!(u64, emit_u64);
impl_value_for!(i64, emit_i64);
impl_value_for!(f64, emit_f64);

impl Value for () {
    fn serialize(
        &self,
        _record: &Record,
        key: Key,
        serializer: &mut dyn Serializer,
    ) -> Result {
        serializer.emit_unit(key)
    }
}

impl Value for str {
    fn serialize(
        &self,
        _record: &Record,
        key: Key,
        serializer: &mut dyn Serializer,
    ) -> Result {
        serializer.emit_str(key, self)
    }
}

impl<'a> Value for fmt::Arguments<'a> {
    fn serialize(
        &self,
        _record: &Record,
        key: Key,
        serializer: &mut dyn Serializer,
    ) -> Result {
        serializer.emit_arguments(key, self)
    }
}

impl Value for String {
    fn serialize(
        &self,
        _record: &Record,
        key: Key,
        serializer: &mut dyn Serializer,
    ) -> Result {
        serializer.emit_str(key, self.as_str())
    }
}

impl<T: Value> Value for Option<T> {
    fn serialize(
        &self,
        record: &Record,
        key: Key,
        serializer: &mut dyn Serializer,
    ) -> Result {
        match *self {
            Some(ref s) => s.serialize(record, key, serializer),
            None => serializer.emit_none(key),
        }
    }
}

impl<T> Value for Box<T>
where
    T: Value + ?Sized,
{
    fn serialize(
        &self,
        record: &Record,
        key: Key,
        serializer: &mut dyn Serializer,
    ) -> Result {
        (**self).serialize(record, key, serializer)
    }
}
impl<T> Value for Arc<T>
where
    T: Value + ?Sized,
{
    fn serialize(
        &self,
        record: &Record,
        key: Key,
        serializer: &mut dyn Serializer,
    ) -> Result {
        (**self).serialize(record, key, serializer)
    }
}

impl<T> Value for Rc<T>
where
    T: Value,
{
    fn serialize(
        &self,
        record: &Record,
        key: Key,
        serializer: &mut dyn Serializer,
    ) -> Result {
        (**self).serialize(record, key, serializer)
    }
}

impl<T> Value for core::num::Wrapping<T>
where
    T: Value,
{
    fn serialize(
        &self,
        record: &Record,
        key: Key,
        serializer: &mut dyn Serializer,
    ) -> Result {
        self.0.serialize(record, key, serializer)
    }
}

impl<'a> Value for std::path::Display<'a> {
    fn serialize(
        &self,
        _record: &Record,
        key: Key,
        serializer: &mut dyn Serializer,
    ) -> Result {
        serializer.emit_arguments(key, &format_args!("{}", *self))
    }
}

/// Explicit lazy-closure `Value`
pub struct FnValue<V: Value, F>(pub F)
where
    F: for<'c, 'd> Fn(&'c Record<'d>) -> V;

impl<'a, V: 'a + Value, F> Value for FnValue<V, F>
where
    F: 'a + for<'c, 'd> Fn(&'c Record<'d>) -> V,
{
    fn serialize(
        &self,
        record: &Record,
        key: Key,
        serializer: &mut dyn Serializer,
    ) -> Result {
        (self.0)(record).serialize(record, key, serializer)
    }
}

#[deprecated(note = "Renamed to `PushFnValueSerializer`")]
/// Old name of `PushFnValueSerializer`
pub type PushFnSerializer<'a> = PushFnValueSerializer<'a>;

/// Handle passed to `PushFnValue` closure
///
/// It makes sure only one value is serialized, and will automatically emit
/// `()` if nothing else was serialized.
pub struct PushFnValueSerializer<'a> {
    record: &'a Record<'a>,
    key: Key,
    serializer: &'a mut dyn Serializer,
    done: bool,
}

impl<'a> PushFnValueSerializer<'a> {
    #[deprecated(note = "Renamed to `emit`")]
    /// Emit a value
    pub fn serialize<'b, S: 'b + Value>(self, s: S) -> Result {
        self.emit(s)
    }

    /// Emit a value
    ///
    /// This consumes `self` to prevent serializing one value multiple times
    pub fn emit<'b, S: 'b + Value>(mut self, s: S) -> Result {
        self.done = true;
        s.serialize(self.record, self.key.clone(), self.serializer)
    }
}

impl<'a> Drop for PushFnValueSerializer<'a> {
    fn drop(&mut self) {
        if !self.done {
            // unfortunately this gives no change to return serialization errors
            let _ = self.serializer.emit_unit(self.key.clone());
        }
    }
}

/// Lazy `Value` that writes to Serializer
///
/// It's more ergonomic for closures used as lazy values to return type
/// implementing `Serialize`, but sometimes that forces an allocation (eg.
/// `String`s)
///
/// In some cases it might make sense for another closure form to be used - one
/// taking a serializer as an argument, which avoids lifetimes / allocation
/// issues.
///
/// Generally this method should be used if it avoids a big allocation of
/// `Serialize`-implementing type in performance-critical logging statement.
///
/// ```
/// #[macro_use]
/// extern crate slog;
/// use slog::{PushFnValue, Logger, Discard};
///
/// fn main() {
///     // Create a logger with a key-value printing
///     // `file:line` string value for every logging statement.
///     // `Discard` `Drain` used for brevity.
///     let root = Logger::root(Discard, o!(
///         "source_location" => PushFnValue(|record , s| {
///              s.serialize(
///                   format_args!(
///                        "{}:{}",
///                        record.file(),
///                        record.line(),
///                   )
///              )
///         })
///     ));
/// }
/// ```
pub struct PushFnValue<F>(pub F)
where
    F: 'static
        + for<'c, 'd> Fn(&'c Record<'d>, PushFnValueSerializer<'c>) -> Result;

impl<F> Value for PushFnValue<F>
where
    F: 'static
        + for<'c, 'd> Fn(&'c Record<'d>, PushFnValueSerializer<'c>) -> Result,
{
    fn serialize(
        &self,
        record: &Record,
        key: Key,
        serializer: &mut dyn Serializer,
    ) -> Result {
        let ser = PushFnValueSerializer {
            record: record,
            key: key,
            serializer: serializer,
            done: false,
        };
        (self.0)(record, ser)
    }
}
// }}}

// {{{ KV
/// # Key-value pair(s) for log events
///
/// Zero, one or more key value pairs chained together
///
/// Any logging data must implement this trait for slog to be able to use it,
/// although slog comes with default implementations within its macros (the
/// `=>` and `kv!` portions of the log macros).
///
/// If you don't use this trait, you must emit your structured data by
/// specifying both key and value in each log event:
///
/// ```ignore
/// info!(logger, "my event"; "type_key" => %my_val);
/// ```
///
/// If you implement this trait, that can become:
///
/// ```ignore
/// info!(logger, "my event"; my_val);
/// ```
///
/// Types implementing this trait can emit multiple key-value pairs, and can
/// customize their structured representation. The order of emitting them
/// should be consistent with the way key-value pair hierarchy is traversed:
/// from data most specific to the logging context to the most general one. Or
/// in other words: from newest to oldest.
///
/// Implementers are are responsible for calling the `emit_*` methods on the
/// `Serializer` passed in, the `Record` can be used to make display decisions
/// based on context, but for most plain-value structs you will just call
/// `emit_*`.
///
/// # Example
///
/// ```
/// use slog::{KV, Record, Result, Serializer};
///
/// struct MyNewType(i64);
///
/// impl KV for MyNewType {
///    fn serialize(&self, _rec: &Record, serializer: &mut Serializer) -> Result {
///        serializer.emit_i64("my_new_type", self.0)
///    }
/// }
/// ```
///
/// See also [`Value`](trait.Value.html), which allows you to customize just
/// the right hand side of the `=>` structure macro, and (if you have the
/// `nested-values` feature enabled) [`SerdeValue`](trait.SerdeValue.html)
/// which allows emitting anything serde can emit.
pub trait KV {
    /// Serialize self into `Serializer`
    ///
    /// `KV` should call respective `Serializer` methods
    /// for each key-value pair it contains.
    fn serialize(&self, record: &Record, serializer: &mut dyn Serializer)
        -> Result;
}

impl<'a, T> KV for &'a T
where
    T: KV,
{
    fn serialize(
        &self,
        record: &Record,
        serializer: &mut dyn Serializer,
    ) -> Result {
        (**self).serialize(record, serializer)
    }
}

#[cfg(feature = "std")]
/// Thread-local safety bound for `KV`
///
/// This type is used to enforce `KV`s stored in `Logger`s are thread-safe.
pub trait SendSyncRefUnwindSafeKV: KV + Send + Sync + RefUnwindSafe {}

#[cfg(feature = "std")]
impl<T> SendSyncRefUnwindSafeKV for T
where
    T: KV + Send + Sync + RefUnwindSafe + ?Sized,
{
}

#[cfg(not(feature = "std"))]
/// This type is used to enforce `KV`s stored in `Logger`s are thread-safe.
pub trait SendSyncRefUnwindSafeKV: KV + Send + Sync {}

#[cfg(not(feature = "std"))]
impl<T> SendSyncRefUnwindSafeKV for T
where
    T: KV + Send + Sync + ?Sized,
{
}

/// Single pair `Key` and `Value`
pub struct SingleKV<V>(pub Key, pub V)
where
    V: Value;

#[cfg(feature = "dynamic-keys")]
impl<V: Value> From<(String, V)> for SingleKV<V> {
    fn from(x: (String, V)) -> SingleKV<V> {
        SingleKV(Key::from(x.0), x.1)
    }
}
#[cfg(feature = "dynamic-keys")]
impl<V: Value> From<(&'static str, V)> for SingleKV<V> {
    fn from(x: (&'static str, V)) -> SingleKV<V> {
        SingleKV(Key::from(x.0), x.1)
    }
}
#[cfg(not(feature = "dynamic-keys"))]
impl<V: Value> From<(&'static str, V)> for SingleKV<V> {
    fn from(x: (&'static str, V)) -> SingleKV<V> {
        SingleKV(x.0, x.1)
    }
}

impl<V> KV for SingleKV<V>
where
    V: Value,
{
    fn serialize(
        &self,
        record: &Record,
        serializer: &mut dyn Serializer,
    ) -> Result {
        self.1.serialize(record, self.0.clone(), serializer)
    }
}

impl KV for () {
    fn serialize(
        &self,
        _record: &Record,
        _serializer: &mut dyn Serializer,
    ) -> Result {
        Ok(())
    }
}

impl<T: KV, R: KV> KV for (T, R) {
    fn serialize(
        &self,
        record: &Record,
        serializer: &mut dyn Serializer,
    ) -> Result {
        try!(self.0.serialize(record, serializer));
        self.1.serialize(record, serializer)
    }
}

impl<T> KV for Box<T>
where
    T: KV + ?Sized,
{
    fn serialize(
        &self,
        record: &Record,
        serializer: &mut dyn Serializer,
    ) -> Result {
        (**self).serialize(record, serializer)
    }
}

impl<T> KV for Arc<T>
where
    T: KV + ?Sized,
{
    fn serialize(
        &self,
        record: &Record,
        serializer: &mut dyn Serializer,
    ) -> Result {
        (**self).serialize(record, serializer)
    }
}

impl<T> KV for OwnedKV<T>
where
    T: SendSyncRefUnwindSafeKV + ?Sized,
{
    fn serialize(
        &self,
        record: &Record,
        serializer: &mut dyn Serializer,
    ) -> Result {
        self.0.serialize(record, serializer)
    }
}

impl<'a> KV for BorrowedKV<'a> {
    fn serialize(
        &self,
        record: &Record,
        serializer: &mut dyn Serializer,
    ) -> Result {
        self.0.serialize(record, serializer)
    }
}
// }}}

// {{{ OwnedKV
/// Owned KV
///
/// "Owned" means that the contained data (key-value pairs) can belong
/// to a `Logger` and thus must be thread-safe (`'static`, `Send`, `Sync`)
///
/// Zero, one or more owned key-value pairs.
///
/// Can be constructed with [`o!` macro](macro.o.html).
pub struct OwnedKV<T>(
    #[doc(hidden)]
    /// The exact details of that it are not considered public
    /// and stable API. `slog_o` or `o` macro should be used
    /// instead to create `OwnedKV` instances.
    pub T,
)
where
    T: SendSyncRefUnwindSafeKV + ?Sized;
// }}}

// {{{ BorrowedKV
/// Borrowed `KV`
///
/// "Borrowed" means that the data is only a temporary
/// referenced (`&T`) and can't be stored directly.
///
/// Zero, one or more borrowed key-value pairs.
///
/// Can be constructed with [`b!` macro](macro.b.html).
pub struct BorrowedKV<'a>(
    /// The exact details of it function are not
    /// considered public and stable API. `log` and other
    /// macros should be used instead to create
    /// `BorrowedKV` instances.
    #[doc(hidden)]
    pub &'a dyn KV,
);

// }}}

// {{{ OwnedKVList
struct OwnedKVListNode<T>
where
    T: SendSyncRefUnwindSafeKV + 'static,
{
    next_node: Arc<dyn SendSyncRefUnwindSafeKV + 'static>,
    kv: T,
}

struct MultiListNode {
    next_node: Arc<dyn SendSyncRefUnwindSafeKV + 'static>,
    node: Arc<dyn SendSyncRefUnwindSafeKV + 'static>,
}

/// Chain of `SyncMultiSerialize`-s of a `Logger` and its ancestors
#[derive(Clone)]
pub struct OwnedKVList {
    node: Arc<dyn SendSyncRefUnwindSafeKV + 'static>,
}

impl<T> KV for OwnedKVListNode<T>
where
    T: SendSyncRefUnwindSafeKV + 'static,
{
    fn serialize(
        &self,
        record: &Record,
        serializer: &mut dyn Serializer,
    ) -> Result {
        try!(self.kv.serialize(record, serializer));
        try!(self.next_node.serialize(record, serializer));

        Ok(())
    }
}

impl KV for MultiListNode {
    fn serialize(
        &self,
        record: &Record,
        serializer: &mut dyn Serializer,
    ) -> Result {
        try!(self.next_node.serialize(record, serializer));
        try!(self.node.serialize(record, serializer));

        Ok(())
    }
}

impl KV for OwnedKVList {
    fn serialize(
        &self,
        record: &Record,
        serializer: &mut dyn Serializer,
    ) -> Result {
        try!(self.node.serialize(record, serializer));

        Ok(())
    }
}

impl fmt::Debug for OwnedKVList {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        try!(write!(f, "("));
        let mut i = 0;

        {
            let mut as_str_ser = AsFmtSerializer(|key, _val| {
                if i != 0 {
                    try!(write!(f, ", "));
                }

                try!(write!(f, "{}", key));
                i += 1;
                Ok(())
            });
            let record_static = record_static!(Level::Trace, "");

            try!(
                self.node
                    .serialize(
                        &Record::new(
                            &record_static,
                            &format_args!(""),
                            BorrowedKV(&STATIC_TERMINATOR_UNIT)
                        ),
                        &mut as_str_ser
                    )
                    .map_err(|_| fmt::Error)
            );
        }

        try!(write!(f, ")"));
        Ok(())
    }
}

impl OwnedKVList {
    /// New `OwnedKVList` node without a parent (root)
    fn root<T>(values: OwnedKV<T>) -> Self
    where
        T: SendSyncRefUnwindSafeKV + 'static,
    {
        OwnedKVList {
            node: Arc::new(OwnedKVListNode {
                next_node: Arc::new(()),
                kv: values.0,
            }),
        }
    }

    /// New `OwnedKVList` node with an existing parent
    fn new<T>(
        values: OwnedKV<T>,
        next_node: Arc<dyn SendSyncRefUnwindSafeKV + 'static>,
    ) -> Self
    where
        T: SendSyncRefUnwindSafeKV + 'static,
    {
        OwnedKVList {
            node: Arc::new(OwnedKVListNode {
                next_node: next_node,
                kv: values.0,
            }),
        }
    }
}

impl<T> convert::From<OwnedKV<T>> for OwnedKVList
where
    T: SendSyncRefUnwindSafeKV + 'static,
{
    fn from(from: OwnedKV<T>) -> Self {
        OwnedKVList::root(from)
    }
}
// }}}

// {{{ Error
#[derive(Debug)]
#[cfg(feature = "std")]
/// Serialization Error
pub enum Error {
    /// `io::Error` (not available in ![no_std] mode)
    Io(std::io::Error),
    /// `fmt::Error`
    Fmt(std::fmt::Error),
    /// Other error
    Other,
}

#[derive(Debug)]
#[cfg(not(feature = "std"))]
/// Serialization Error
pub enum Error {
    /// `fmt::Error`
    Fmt(core::fmt::Error),
    /// Other error
    Other,
}

/// Serialization `Result`
pub type Result<T = ()> = result::Result<T, Error>;

#[cfg(feature = "std")]
impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Error {
        Error::Io(err)
    }
}

impl From<core::fmt::Error> for Error {
    fn from(_: core::fmt::Error) -> Error {
        Error::Other
    }
}

#[cfg(feature = "std")]
impl From<Error> for std::io::Error {
    fn from(e: Error) -> std::io::Error {
        match e {
            Error::Io(e) => e,
            Error::Fmt(_) => std::io::Error::new(
                std::io::ErrorKind::Other,
                "formatting error",
            ),
            Error::Other => {
                std::io::Error::new(std::io::ErrorKind::Other, "other error")
            }
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for Error {
    fn description(&self) -> &str {
        match *self {
            Error::Io(ref e) => e.description(),
            Error::Fmt(_) => "formatting error",
            Error::Other => "serialization error",
        }
    }

    fn cause(&self) -> Option<&dyn std::error::Error> {
        match *self {
            Error::Io(ref e) => Some(e),
            Error::Fmt(ref e) => Some(e),
            Error::Other => None,
        }
    }
}

#[cfg(feature = "std")]
impl core::fmt::Display for Error {
    fn fmt(&self, fmt: &mut core::fmt::Formatter) -> std::fmt::Result {
        match *self {
            Error::Io(ref e) => e.fmt(fmt),
            Error::Fmt(ref e) => e.fmt(fmt),
            Error::Other => fmt.write_str("Other serialization error"),
        }
    }
}
// }}}

// {{{ Misc
/// This type is here just to abstract away lack of `!` type support in stable
/// rust during time of the release. It will be switched to `!` at some point
/// and `Never` should not be considered "stable" API.
#[doc(hidden)]
pub type Never = private::NeverStruct;

mod private {
    #[doc(hidden)]
    #[derive(Debug)]
    pub struct NeverStruct(());
}

/// This is not part of "stable" API
#[doc(hidden)]
pub static STATIC_TERMINATOR_UNIT: () = ();

#[allow(unknown_lints)]
#[allow(inline_always)]
#[inline(always)]
#[doc(hidden)]
/// Not an API
///
/// Generally it's a bad idea to depend on static logging level
/// in your code. Use closures to perform operations lazily
/// only when logging actually takes place.
pub fn __slog_static_max_level() -> FilterLevel {
    if !cfg!(debug_assertions) {
        if cfg!(feature = "release_max_level_off") {
            return FilterLevel::Off;
        } else if cfg!(feature = "release_max_level_error") {
            return FilterLevel::Error;
        } else if cfg!(feature = "release_max_level_warn") {
            return FilterLevel::Warning;
        } else if cfg!(feature = "release_max_level_info") {
            return FilterLevel::Info;
        } else if cfg!(feature = "release_max_level_debug") {
            return FilterLevel::Debug;
        } else if cfg!(feature = "release_max_level_trace") {
            return FilterLevel::Trace;
        }
    }
    if cfg!(feature = "max_level_off") {
        FilterLevel::Off
    } else if cfg!(feature = "max_level_error") {
        FilterLevel::Error
    } else if cfg!(feature = "max_level_warn") {
        FilterLevel::Warning
    } else if cfg!(feature = "max_level_info") {
        FilterLevel::Info
    } else if cfg!(feature = "max_level_debug") {
        FilterLevel::Debug
    } else if cfg!(feature = "max_level_trace") {
        FilterLevel::Trace
    } else {
        if !cfg!(debug_assertions) {
            FilterLevel::Info
        } else {
            FilterLevel::Debug
        }
    }
}

/// Helper for `?` syntax in `kv!` to work around `format_args!` issues
#[doc(hidden)]
pub struct FmtDebug<T: fmt::Debug>(pub T);

impl<T: fmt::Debug> Value for FmtDebug<T> {
    fn serialize(
        &self,
        _record: &Record,
        key: Key,
        serializer: &mut dyn Serializer,
    ) -> Result {
        serializer.emit_arguments(key, &format_args!("{:?}", self.0))
    }
}

/// Helper for `%` syntax in `kv!` to work around `format_args!` issues
#[doc(hidden)]
pub struct FmtDisplay<T: fmt::Display>(pub T);

impl<T: fmt::Display> Value for FmtDisplay<T> {
    fn serialize(
        &self,
        _record: &Record,
        key: Key,
        serializer: &mut dyn Serializer,
    ) -> Result {
        serializer.emit_arguments(key, &format_args!("{}", self.0))
    }
}
// }}}

// {{{ Slog v1 Compat
#[deprecated(note = "Renamed to `Value`")]
/// Compatibility name to ease upgrading from `slog v1`
pub type Serialize = dyn Value;

#[deprecated(note = "Renamed to `PushFnValue`")]
/// Compatibility name to ease upgrading from `slog v1`
pub type PushLazy<T> = PushFnValue<T>;

#[deprecated(note = "Renamed to `PushFnValueSerializer`")]
/// Compatibility name to ease upgrading from `slog v1`
pub type ValueSerializer<'a> = PushFnValueSerializer<'a>;

#[deprecated(note = "Renamed to `OwnedKVList`")]
/// Compatibility name to ease upgrading from `slog v1`
pub type OwnedKeyValueList = OwnedKVList;

#[deprecated(note = "Content of ser module moved to main namespace")]
/// Compatibility name to ease upgrading from `slog v1`
pub mod ser {
    #[allow(deprecated)]
    pub use super::{OwnedKeyValueList, PushLazy, Serialize, Serializer,
                    ValueSerializer};
}
// }}}

// {{{ Test
#[cfg(test)]
mod tests;

// }}}

// vim: foldmethod=marker foldmarker={{{,}}}
