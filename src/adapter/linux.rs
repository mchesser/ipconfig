use std::collections::HashMap;

use nix::net::if_::InterfaceFlags;
use nix::sys::socket::SockAddr;

use error::*;

use super::*;

/// Get all the network adapters on this machine.
pub fn get_adapters() -> Result<Vec<Adapter>> {
    let addrs = nix::ifaddrs::getifaddrs()?;
    let mut adapters = HashMap::new();

    for ifaddr in addrs {
        let name = ifaddr.interface_name.clone();
        let adapter = adapters
            .entry(name.clone())
            .or_insert_with(|| base_adapter(name));

        match ifaddr.address {
            Some(SockAddr::Link(addr)) => {
                let flags = ifaddr.flags;

                adapter.oper_status = {
                    if flags.contains(InterfaceFlags::IFF_UP) || flags.contains(InterfaceFlags::IFF_RUNNING) { OperStatus::IfOperStatusUp }
                    else if flags.contains(InterfaceFlags::IFF_POINTOPOINT) { OperStatus::IfOperStatusTesting }
                    else if flags.contains(InterfaceFlags::IFF_DORMANT) { OperStatus::IfOperStatusDormant }
                    else { OperStatus::IfOperStatusDown }
                };

                adapter.if_type = {
                    if flags.contains(InterfaceFlags::IFF_LOOPBACK) { IfType::SoftwareLoopback }
                    else if flags.contains(InterfaceFlags::IFF_POINTOPOINT) { IfType::Ppp }
                    else if flags.contains(InterfaceFlags::IFF_TUN) { IfType::Tunnel }
                    else { IfType::Other }
                };

                adapter.physical_address = Some(addr.addr()[..].into());
            },

            Some(SockAddr::Inet(addr)) => {
                adapter.ip_addresses.push(addr.to_std().ip());
            },

            Some(_) => {}
            None => {}
        }
    }

    Ok(adapters.into_iter().map(|x| x.1).collect())
}

fn base_adapter(name: String) -> Adapter {
    Adapter {
        adapter_name: name,
        ip_addresses: vec![],
        dns_servers: vec![],
        description: "".into(),
        friendly_name: "".into(),
        physical_address: None,
        receive_link_speed: 0,
        transmit_link_speed: 0,
        oper_status: OperStatus::IfOperStatusUnknown,
        if_type: IfType::Other,
    }
}