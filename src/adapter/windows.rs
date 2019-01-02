use std;
use std::ffi::CStr;
use std::net::IpAddr;

use winapi::shared::winerror::{ERROR_SUCCESS, ERROR_BUFFER_OVERFLOW};
use winapi::shared::ws2def::AF_UNSPEC;
use widestring::WideCString;

use socket2;
use error::*;

use bindings::*;

use super::*;

/// Get all the network adapters on this machine.
pub fn get_adapters() -> Result<Vec<Adapter>> {
    unsafe {
        let mut buf_len: ULONG = 0;
        let result = GetAdaptersAddresses(AF_UNSPEC as u32, 0, std::ptr::null_mut(), std::ptr::null_mut(), &mut buf_len as *mut ULONG);

        assert!(result != ERROR_SUCCESS);

        if result != ERROR_BUFFER_OVERFLOW {
            bail!(ErrorKind::Os(result));
        }

        let mut adapters_addresses_buffer: Vec<u8> = vec![0; buf_len as usize];
        let mut adapter_addresses_ptr: PIP_ADAPTER_ADDRESSES = std::mem::transmute(adapters_addresses_buffer.as_mut_ptr());
        let result = GetAdaptersAddresses(AF_UNSPEC as u32, 0, std::ptr::null_mut(), adapter_addresses_ptr, &mut buf_len as *mut ULONG);

        if result != ERROR_SUCCESS {
            bail!(ErrorKind::Os(result));
        }

        let mut adapters = vec![];
        while adapter_addresses_ptr != std::ptr::null_mut() {
            adapters.push(get_adapter(adapter_addresses_ptr)?);
            adapter_addresses_ptr = (*adapter_addresses_ptr).Next;
        }

        Ok(adapters)
    }
}

unsafe fn get_adapter(adapter_addresses_ptr: PIP_ADAPTER_ADDRESSES) -> Result<Adapter> {
    let adapter_addresses = &*adapter_addresses_ptr;
    let adapter_name = CStr::from_ptr(adapter_addresses.AdapterName).to_str()?.to_owned();
    let dns_servers = get_dns_servers(adapter_addresses.FirstDnsServerAddress)?;
    let unicast_addresses = get_unicast_addresses(adapter_addresses.FirstUnicastAddress)?;
    let receive_link_speed: u64 = adapter_addresses.ReceiveLinkSpeed;
    let transmit_link_speed: u64 = adapter_addresses.TransmitLinkSpeed;
    let oper_status = match adapter_addresses.OperStatus {
            1 => OperStatus::IfOperStatusUp,
            2 => OperStatus::IfOperStatusDown,
            3 => OperStatus::IfOperStatusTesting,
            4 => OperStatus::IfOperStatusUnknown,
            5 => OperStatus::IfOperStatusDormant,
            6 => OperStatus::IfOperStatusNotPresent,
            7 => OperStatus::IfOperStatusLowerLayerDown,
            v => { panic!("unexpected OperStatus value: {}", v); }
        };
    let if_type = match adapter_addresses.IfType {
        1 => IfType::Other,
        6 => IfType::EthernetCsmacd,
        9 => IfType::Iso88025Tokenring,
        23 => IfType::Ppp,
        24 => IfType::SoftwareLoopback,
        37 => IfType::Atm,
        71 => IfType::Ieee80211,
        131 => IfType::Tunnel,
        144 => IfType::Ieee1394,
        _ => IfType::Unsupported,
    };

    let description = WideCString::from_ptr_str(adapter_addresses.Description).to_string()?;
    let friendly_name = WideCString::from_ptr_str(adapter_addresses.FriendlyName).to_string()?;
    let physical_address = if adapter_addresses.PhysicalAddressLength == 0 {
        None
    } else {
        Some(adapter_addresses.PhysicalAddress[..adapter_addresses.PhysicalAddressLength as usize].to_vec())
    };
    Ok(Adapter {
        adapter_name: adapter_name,
        ip_addresses: unicast_addresses,
        dns_servers: dns_servers,
        description: description,
        friendly_name: friendly_name,
        physical_address: physical_address,
        receive_link_speed: receive_link_speed,
        transmit_link_speed: transmit_link_speed,
        oper_status: oper_status,
        if_type: if_type,
    })
}

unsafe fn socket_address_to_ipaddr(socket_address: &SOCKET_ADDRESS) -> IpAddr {
    let sockaddr = socket2::SockAddr::from_raw_parts(std::mem::transmute(socket_address.lpSockaddr), socket_address.iSockaddrLength);

    // Could be either ipv4 or ipv6
    sockaddr.as_inet()
        .map(|s| IpAddr::V4(*s.ip()))
        .unwrap_or_else(|| IpAddr::V6(*sockaddr.as_inet6().unwrap().ip()))
}

unsafe fn get_dns_servers(mut dns_server_ptr: PIP_ADAPTER_DNS_SERVER_ADDRESS_XP) -> Result<Vec<IpAddr>> {
    let mut dns_servers = vec![];

    while dns_server_ptr != std::ptr::null_mut() {
        let dns_server = &*dns_server_ptr;
        let ipaddr = socket_address_to_ipaddr(&dns_server.Address);
        dns_servers.push(ipaddr);

        dns_server_ptr = dns_server.Next;
    }

    Ok(dns_servers)
}

unsafe fn get_unicast_addresses(mut unicast_addresses_ptr: PIP_ADAPTER_UNICAST_ADDRESS_LH) -> Result<Vec<IpAddr>> {
    let mut unicast_addresses = vec![];

    while unicast_addresses_ptr != std::ptr::null_mut() {
        let unicast_address = &*unicast_addresses_ptr;
        let ipaddr = socket_address_to_ipaddr(&unicast_address.Address);
        unicast_addresses.push(ipaddr);

        unicast_addresses_ptr = unicast_address.Next;
    }

    Ok(unicast_addresses)
}
