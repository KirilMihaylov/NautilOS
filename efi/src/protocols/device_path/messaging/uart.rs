use crate::protocols::device_path::{EfiDevicePathProcotol, EfiDevicePathRepr};

#[repr(C)]
pub struct EfiUartDevicePath {
    base: EfiDevicePathProcotol,
    _reserved: [u8; 4],
    baud_rate: [u8; 8],
    data_bits: u8,
    parity: u8,
    stop_bits: u8,
}

impl EfiUartDevicePath {
    pub fn baud_rate(&self) -> u64 {
        unsafe { (self.baud_rate.as_ptr() as *const u64).read_unaligned() }
    }

    pub fn data_bits(&self) -> u8 {
        self.data_bits
    }

    pub fn parity(&self) -> EfiUartDevicePathParity {
        use EfiUartDevicePathParity::*;

        match self.parity {
            0 => Default,
            1 => NoParity,
            2 => Even,
            3 => Odd,
            4 => Mark,
            5 => Space,
            x => Other(x),
        }
    }

    pub fn stop_bits(&self) -> EfiUartDevicePathStopBits {
        use EfiUartDevicePathStopBits::*;

        match self.parity {
            0 => Default,
            1 => _1Bit,
            2 => _1_5Bits,
            3 => _2Bits,
            x => Other(x),
        }
    }
}

impl EfiDevicePathRepr for EfiUartDevicePath {}

#[non_exhaustive]
#[derive(Clone, Copy)]
pub enum EfiUartDevicePathParity {
    Default,
    NoParity,
    Even,
    Odd,
    Mark,
    Space,

    Other(u8),
}

#[non_exhaustive]
#[derive(Clone, Copy)]
pub enum EfiUartDevicePathStopBits {
    Default,
    _1Bit,
    _1_5Bits,
    _2Bits,

    Other(u8),
}
