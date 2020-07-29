use crate::protocols::device_path::{EfiDevicePathProcotol, EfiDevicePathRepr};

#[repr(C)]
pub struct EfiSerialAttachedScsiDevicePath {
    base: EfiDevicePathProcotol,
    vendor_guid: [u8; 16],
    _reserved: [u8; 4],
    sas_address: [u8; 8],
    logical_unit_number: [u8; 8],
    device_and_topology_info: [u8; 2],
    relative_target_port: [u8; 2],
}

impl EfiSerialAttachedScsiDevicePath {
    pub fn sas_address(&self) -> u64 {
        unsafe { (self.sas_address.as_ptr() as *const u64).read_unaligned() }
    }

    pub fn logical_unit_number(&self) -> u64 {
        unsafe { (self.sas_address.as_ptr() as *const u64).read_unaligned() }
    }

    pub fn device_and_topology_info(&self) -> EfiSerialAttachedScsiDevicePathDeviceAndTopologyInfo {
        unsafe {
            (self.sas_address.as_ptr()
                as *const EfiSerialAttachedScsiDevicePathDeviceAndTopologyInfo)
                .read_unaligned()
        }
    }

    pub fn relative_target_port(&self) -> u16 {
        unsafe { (self.relative_target_port.as_ptr() as *const u16).read_unaligned() }
    }
}

impl EfiDevicePathRepr for EfiSerialAttachedScsiDevicePath {}

#[non_exhaustive]
#[derive(Clone, Copy)]
pub enum EfiSerialAttachedScsiDevicePathAdditionalInfoBytes {
    NoBytes,
    OneByte,
    TwoBytes,
}

#[non_exhaustive]
#[derive(Clone, Copy)]
pub enum EfiSerialAttachedScsiDevicePathDeviceType {
    SAS { internal: bool },
    SATA { internal: bool },
}

#[non_exhaustive]
#[derive(Clone, Copy)]
pub enum EfiSerialAttachedScsiDevicePathTopology {
    DirectConnect,
    ExpanderConnect,
}

#[repr(transparent)]
#[derive(Clone, Copy)]
pub struct EfiSerialAttachedScsiDevicePathDeviceAndTopologyInfo {
    device_and_topology_info: u16,
}

impl EfiSerialAttachedScsiDevicePathDeviceAndTopologyInfo {
    fn additional_info_bytes(
        &self,
    ) -> Result<EfiSerialAttachedScsiDevicePathAdditionalInfoBytes, u16> {
        use EfiSerialAttachedScsiDevicePathAdditionalInfoBytes::*;

        Ok(match self.device_and_topology_info & 0xF {
            0 => NoBytes,
            1 => OneByte,
            2 => TwoBytes,
            _ => return Err(self.device_and_topology_info), /* Overrides Ok */
        })
    }

    pub fn device_type(&self) -> Result<EfiSerialAttachedScsiDevicePathDeviceType, u16> {
        use EfiSerialAttachedScsiDevicePathAdditionalInfoBytes::*;
        use EfiSerialAttachedScsiDevicePathDeviceType::*;

        Ok(match self.additional_info_bytes()? {
            /* Specification defines field as valid when "additional_info_bytes" is non-zero */
            NoBytes => return Err(self.device_and_topology_info),
            _ => match (self.device_and_topology_info >> 4) & 3 {
                0 => SAS { internal: true },
                1 => SATA { internal: true },
                2 => SAS { internal: false },
                3 => SATA { internal: false },
                _ => unreachable!(),
            },
        })
    }

    pub fn topology(&self) -> Result<EfiSerialAttachedScsiDevicePathTopology, u16> {
        use EfiSerialAttachedScsiDevicePathAdditionalInfoBytes::*;
        use EfiSerialAttachedScsiDevicePathTopology::*;

        Ok(match self.additional_info_bytes()? {
            /* Specification defines field as valid when "additional_info_bytes" is non-zero */
            NoBytes => return Err(self.device_and_topology_info),
            _ => match (self.device_and_topology_info >> 6) & 3 {
                0 => DirectConnect,
                1 => ExpanderConnect,
                _ => return Err(self.device_and_topology_info),
            },
        })
    }

    pub fn internal_drive_id(&self) -> Result<u16, u16> {
        use EfiSerialAttachedScsiDevicePathAdditionalInfoBytes::*;
        use EfiSerialAttachedScsiDevicePathDeviceType::*;

        match self.additional_info_bytes()? {
            TwoBytes => match self.device_type()? {
                SAS { internal } | SATA { internal } if internal => (),
                _ => return Err(self.device_and_topology_info),
            },
            _ => return Err(self.device_and_topology_info),
        }

        Ok(self.device_and_topology_info >> 8)
    }
}
