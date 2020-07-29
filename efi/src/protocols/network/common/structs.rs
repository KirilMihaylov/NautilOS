use core::ops::{Deref, DerefMut};

#[repr(transparent)]
#[derive(Clone, Copy)]
pub struct EfiIPv4AddressRaw {
    address: [u8; 4],
}

impl Deref for EfiIPv4AddressRaw {
    type Target = [u8; 4];

    fn deref(&self) -> &<Self as Deref>::Target {
        &self.address
    }
}

impl DerefMut for EfiIPv4AddressRaw {
    fn deref_mut(&mut self) -> &mut <Self as Deref>::Target {
        &mut self.address
    }
}

#[repr(transparent)]
#[derive(Clone, Copy)]
pub struct EfiIPv6AddressRaw {
    address: [u8; 16],
}

impl Deref for EfiIPv6AddressRaw {
    type Target = [u8; 16];

    fn deref(&self) -> &<Self as Deref>::Target {
        &self.address
    }
}

impl DerefMut for EfiIPv6AddressRaw {
    fn deref_mut(&mut self) -> &mut <Self as Deref>::Target {
        &mut self.address
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub(crate) union EfiIPAddressRaw {
    ip_v4: EfiIPv4AddressRaw,
    ip_v6: EfiIPv6AddressRaw,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub enum EfiIPAddress {
    IPv4(EfiIPv4AddressRaw),
    IPv6(EfiIPv6AddressRaw),
}
