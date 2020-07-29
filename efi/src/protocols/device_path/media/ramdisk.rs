use crate::{
    protocols::device_path::{EfiDevicePathProcotol, EfiDevicePathRepr},
    *,
};

#[repr(C)]
pub struct EfiRAMDiskDevicePath {
    base: EfiDevicePathProcotol,
    starting_address: [u8; 8],
    ending_address: [u8; 8],
    disk_type_guid: [u8; 16],
    disk_instance: [u8; 2],
}

impl EfiRAMDiskDevicePath {
    pub fn starting_address(&self) -> u64 {
        unsafe { (self.starting_address.as_ptr() as *const u64).read_unaligned() }
    }

    pub fn ending_address(&self) -> u64 {
        unsafe { (self.ending_address.as_ptr() as *const u64).read_unaligned() }
    }

    pub fn disk_type_guid(&self) -> EfiGuid {
        EfiGuid::from_array(&self.disk_type_guid)
    }

    pub fn disk_type(&self) -> EfiRAMDiskDevicePathDeviceType {
        const RAW_VOLATILE_GUID: EfiGuid = EfiGuid::from_tuple((
            0x77AB535A,
            0x45FC,
            0x624B,
            [0x55, 0x60, 0xF7, 0xB2, 0x81, 0xD1, 0xF9, 0x6E],
        ));
        const ISO_VOLATILE_GUID: EfiGuid = EfiGuid::from_tuple((
            0x3D5ABD30,
            0x4175,
            0x87CE,
            [0x6D, 0x64, 0xD2, 0xAD, 0xE5, 0x23, 0xC4, 0xBB],
        ));
        const RAW_PERSISTANT_GUID: EfiGuid = EfiGuid::from_tuple((
            0x5CEA02C9,
            0x4D07,
            0x69D3,
            [0x26, 0x9F, 0x44, 0x96, 0xFB, 0xE0, 0x96, 0xF9],
        ));
        const ISO_PERSISTANT_GUID: EfiGuid = EfiGuid::from_tuple((
            0x08018188,
            0x42CD,
            0xBB48,
            [0x10, 0x0F, 0x53, 0x87, 0xD5, 0x3D, 0xED, 0x3D],
        ));

        match self.disk_type_guid() {
            RAW_VOLATILE_GUID => EfiRAMDiskDevicePathDeviceType::RawVolatile,
            ISO_VOLATILE_GUID => EfiRAMDiskDevicePathDeviceType::IsoVolatile,
            RAW_PERSISTANT_GUID => EfiRAMDiskDevicePathDeviceType::RawPersistant,
            ISO_PERSISTANT_GUID => EfiRAMDiskDevicePathDeviceType::IsoPersistant,

            guid => EfiRAMDiskDevicePathDeviceType::VendorDefined(guid),
        }
    }

    pub fn disk_instance(&self) -> u16 {
        unsafe { (self.disk_instance.as_ptr() as *const u16).read_unaligned() }
    }
}

impl EfiDevicePathRepr for EfiRAMDiskDevicePath {}

#[non_exhaustive]
#[derive(Clone, Copy)]
pub enum EfiRAMDiskDevicePathDeviceType {
    VendorDefined(EfiGuid),

    RawVolatile,
    IsoVolatile,
    RawPersistant,
    IsoPersistant,
}
