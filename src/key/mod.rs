#[cfg(feature = "dynamic-keys")]
mod dynamic;
#[cfg(feature = "dynamic-keys")]
pub use self::dynamic::Key;

#[cfg(not(feature = "dynamic-keys"))]
#[path = "static.rs"]
mod static_;
#[cfg(not(feature = "dynamic-keys"))]
#[allow(rust_2018_idioms)] // bug? This shouldn't be "unreachable" as far as I can tell
pub use self::static_::Key;
