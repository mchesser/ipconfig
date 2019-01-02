use std::net::IpAddr;

#[cfg(windows)]
#[path = "windows.rs"]
mod inner;

#[cfg(any(target_os = "linux", target_os = "macos"))] // TODO: check whether this works on macos
#[path = "linux.rs"]
mod inner;

pub use self::inner::*;

/// Represent an operational status of the adapter
/// See IP_ADAPTER_ADDRESSES docs for more details
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum OperStatus {
    IfOperStatusUp = 1,
    IfOperStatusDown = 2,
    IfOperStatusTesting = 3,
    IfOperStatusUnknown = 4,
    IfOperStatusDormant = 5,
    IfOperStatusNotPresent = 6,
    IfOperStatusLowerLayerDown = 7,
}

/// Represent an interface type
/// See IANA docs on iftype for more details
/// https://www.iana.org/assignments/ianaiftype-mib/ianaiftype-mib
/// Note that we only support a subset of the IANA interface
/// types and in case the adapter has an unsupported type,
/// `IfType::Unsupported` is used. `IfType::Other`
/// is different from `IfType::Unsupported`, as the former
/// one is defined by the IANA itself.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum IfType {
    Other = 1,
    EthernetCsmacd = 6,
    Iso88025Tokenring = 9,
    Ppp = 23,
    SoftwareLoopback = 24,
    Atm = 37,
    Ieee80211 = 71,
    Tunnel = 131,
    Ieee1394 = 144,
    Unsupported,
    /// This enum may grow additional variants, so this makes sure clients
    /// don't count on exhaustive matching. (Otherwise, adding a new variant
    /// could break existing code.)
    #[doc(hidden)]
    __Nonexhaustive,
}

/// Represent an adapter.
#[derive(Debug)]
pub struct Adapter {
    adapter_name: String,
    ip_addresses: Vec<IpAddr>,
    dns_servers: Vec<IpAddr>,
    description: String,
    friendly_name: String,
    physical_address: Option<Vec<u8>>,
    receive_link_speed: u64,
    transmit_link_speed: u64,
    oper_status: OperStatus,
    if_type: IfType,
}

impl Adapter {
    /// Get the adapter's name
    pub fn adapter_name(&self) -> &String {
        &self.adapter_name
    }
    /// Get the adapter's ip addresses (unicast ip addresses)
    pub fn ip_addresses(&self) -> &Vec<IpAddr> {
        &self.ip_addresses
    }
    /// Get the adapter's dns servers (the preferred dns server is first)
    pub fn dns_servers(&self) -> &Vec<IpAddr> {
        &self.dns_servers
    }
    /// Get the adapter's description
    pub fn description(&self) -> &String {
        &self.description
    }
    /// Get the adapter's friendly name
    pub fn friendly_name(&self) -> &String {
        &self.friendly_name
    }
    /// Get the adapter's physical (MAC) address
    pub fn physical_address(&self) -> &Option<Vec<u8>> {
        &self.physical_address
    }

    /// Get the adapter Recieve Link Speed (bits per second)
    pub fn receive_link_speed(&self) -> u64 {
        self.receive_link_speed
    }

    /// Get the Trasnmit Link Speed (bits per second)
    pub fn transmit_link_speed(&self) -> u64 {
        self.transmit_link_speed
    }

    /// Check if the adapter is up (OperStatus is IfOperStatusUp)
    pub fn oper_status(&self) -> OperStatus {
        self.oper_status
    }

    /// Get the interface type
    pub fn if_type(&self) -> IfType {
        self.if_type
    }
}