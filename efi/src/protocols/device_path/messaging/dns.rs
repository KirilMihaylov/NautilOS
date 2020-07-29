use core::{marker::PhantomData, mem::size_of};

use crate::protocols::{
    device_path::{EfiDevicePathProcotol, EfiDevicePathRepr},
    network::common::structs::{EfiIPAddressRaw, EfiIPv4AddressRaw, EfiIPv6AddressRaw},
};

#[repr(C)]
pub struct EfiDomainNameServiceDevicePath {
    base: EfiDevicePathProcotol,
    address_type: u8,
    addresses: (),
}

impl EfiDomainNameServiceDevicePath {
    pub fn address_type(&self) -> EfiDomainNameServiceDevicePathAddressType {
        use EfiDomainNameServiceDevicePathAddressType::*;

        match self.address_type {
            0 => IPv4,
            1 => IPv6,

            _ => Undefined,
        }
    }

    pub fn addresses(&self) -> EfiDomainNameServiceDevicePathAddresses {
        use EfiDomainNameServiceDevicePathAddressType as AddressType;
        use EfiDomainNameServiceDevicePathAddresses::*;

        match self.address_type() {
            AddressType::IPv4 => IPv4(EfiIPv4AddressIterator {
                ptr: &self.addresses as *const () as *const EfiIPAddressRaw,
                count: (self.base.len() as usize - 5) / size_of::<EfiIPAddressRaw>(),
                _phantom_data: PhantomData,
            }),
            AddressType::IPv6 => IPv6(EfiIPv6AddressIterator {
                ptr: &self.addresses as *const () as *const EfiIPAddressRaw,
                count: (self.base.len() as usize - 5) / size_of::<EfiIPAddressRaw>(),
                _phantom_data: PhantomData,
            }),

            _ => Undefined,
        }
    }
}

impl EfiDevicePathRepr for EfiDomainNameServiceDevicePath {}

#[non_exhaustive]
#[derive(Clone, Copy)]
pub enum EfiDomainNameServiceDevicePathAddressType {
    IPv4,
    IPv6,

    Undefined,
}

#[non_exhaustive]
#[derive(Clone, Copy)]
pub enum EfiDomainNameServiceDevicePathAddresses<'a> {
    IPv4(EfiIPv4AddressIterator<'a>),
    IPv6(EfiIPv6AddressIterator<'a>),

    Undefined,
}

#[derive(Clone, Copy)]
pub struct EfiIPv4AddressIterator<'a> {
    ptr: *const EfiIPAddressRaw,
    count: usize,
    _phantom_data: PhantomData<&'a EfiIPv4AddressRaw>,
}

impl<'a> Iterator for EfiIPv4AddressIterator<'a> {
    type Item = EfiIPv4AddressRaw;

    fn next(&mut self) -> Option<<Self as Iterator>::Item> {
        if self.count != 0 {
            self.count -= 1;
            let ptr: *const EfiIPAddressRaw = self.ptr;
            unsafe {
                self.ptr = self.ptr.offset(1);

                Some((ptr as *const EfiIPv4AddressRaw).read_unaligned())
            }
        } else {
            None
        }
    }
}

#[derive(Clone, Copy)]
pub struct EfiIPv6AddressIterator<'a> {
    ptr: *const EfiIPAddressRaw,
    count: usize,
    _phantom_data: PhantomData<&'a EfiIPv4AddressRaw>,
}

impl<'a> Iterator for EfiIPv6AddressIterator<'a> {
    type Item = EfiIPv6AddressRaw;

    fn next(&mut self) -> Option<<Self as Iterator>::Item> {
        if self.count != 0 {
            self.count -= 1;
            let ptr: *const EfiIPAddressRaw = self.ptr;
            unsafe {
                self.ptr = self.ptr.offset(1);

                Some((ptr as *const EfiIPv6AddressRaw).read_unaligned())
            }
        } else {
            None
        }
    }
}
