//! Get network adapters information for windows.
//!
//!
//! # Examples
//!
//! ```rust
//! # fn foo() -> ipconfig::error::Result<()> {
//! // Print the ip addresses and dns servers of all adapters:
//! for adapter in ipconfig::get_adapters()? {
//!     println!("Ip addresses: {:#?}", adapter.ip_addresses());
//!     println!("Dns servers: {:#?}", adapter.dns_servers());
//! }
//! # Ok(())
//! # }
//! # fn main() {
//!     # foo().unwrap();
//! # }
//! ```

// #![cfg(windows)]
// #![doc(html_root_url = "https://docs.rs/ipconfig/0.1.9/x86_64-pc-windows-msvc/ipconfig/")]

#[macro_use]
extern crate error_chain;

#[cfg(windows)]
extern crate socket2;
#[cfg(windows)]
extern crate winapi;
#[cfg(windows)]
extern crate widestring;
#[cfg(windows)]
extern crate winreg;

#[cfg(any(target_os = "linux", target_os = "macos"))]
extern crate nix;

pub mod error;
#[cfg(windows)]
pub mod computer;

mod adapter;
#[cfg(windows)]
mod bindings;

pub use adapter::{get_adapters, Adapter, OperStatus, IfType};
