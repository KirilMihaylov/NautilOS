use core::mem::size_of;

use crate::{
	*,
	protocols::EfiProtocol,
	protocols::network::common::structs::EfiIPAddressRaw,
};

pub mod acpi;
pub mod bios_specification;
pub mod hardware;
pub mod media;
pub mod messaging;

#[non_exhaustive]
pub enum EfiDevicePathType<'a> {
	Undefined,

	HardwarePath(EfiHardwareDevicePathSubtype<'a>),
	AcpiPath(EfiAcpiDevicePathSubtype<'a>),
	Messaging(EfiMessagingDevicePathSubtype<'a>),
	Media(EfiMediaDevicePathSubtype<'a>),
	BiosSpecification(EfiBiosSpecificationDevicePathSubtype<'a>),
	
	EndOfDevicePathInstance,
	EndOfDevicePath,
}

impl<'a> From<&'a EfiDevicePathProcotol> for EfiDevicePathType<'a> {
	fn from(path: &'a EfiDevicePathProcotol) -> Self {
		use EfiDevicePathType::*;
	
		match path.path_type {
			1 => HardwarePath(EfiHardwareDevicePathSubtype::from(path)),
			2 => AcpiPath(EfiAcpiDevicePathSubtype::from(path)),
			3 => Messaging(EfiMessagingDevicePathSubtype::from(path)),
			4 => Media(EfiMediaDevicePathSubtype::from(path)),
			5 => BiosSpecification(EfiBiosSpecificationDevicePathSubtype::from(path)),
			
			0x7F => {
				match path.path_subtype {
					1 => EndOfDevicePathInstance,
					0xFF => EndOfDevicePath,
					_ => unreachable!("Undefined state!"),
				}
			},
			_ => Undefined,
		}
	}
}

#[non_exhaustive]
pub enum EfiHardwareDevicePathSubtype<'a> {
	Undefined,

	Pci(&'a hardware::EfiPciDevicePath),
	PcCard(&'a hardware::EfiPcCardDevicePath),
	MemoryMapped(&'a hardware::EfiMemoryMappedDevicePath),
	VendorDefined(&'a hardware::EfiVendorDefinedDevicePath),
	Controller(&'a hardware::EfiControllerDevicePath),
	BaseboardManagementController(&'a hardware::EfiBaseboardManagementControllerDevicePath),
}

impl<'a> From<&'a EfiDevicePathProcotol> for EfiHardwareDevicePathSubtype<'a> {
	fn from(path: &'a EfiDevicePathProcotol) -> Self {
		use EfiHardwareDevicePathSubtype::*;
		use hardware::*;
	
		match path.path_subtype {
			1 => match path.len() {
				6 => Pci(EfiPciDevicePath::new(path)),
				_ => Undefined,
			},
			2 => match path.len() {
				5 => PcCard(EfiPcCardDevicePath::new(path)),
				_ => Undefined,
			},
			3 => match path.len() {
				24 => MemoryMapped(EfiMemoryMappedDevicePath::new(path)),
				_ => Undefined,
			},
			4 => match path.len()  {
				x if x >= 20 => VendorDefined(EfiVendorDefinedDevicePath::new(path)),
				_ => Undefined,
			},
			5 => match path.len() {
				8 => Controller(EfiControllerDevicePath::new(path)),
				_ => Undefined,
			},
			6 => match path.len() {
				13 => BaseboardManagementController(EfiBaseboardManagementControllerDevicePath::new(path)),
				_ => Undefined,
			},
			_ => Undefined,
		}
	}
}

#[non_exhaustive]
pub enum EfiAcpiDevicePathSubtype<'a> {
	Undefined,

	Acpi(&'a acpi::EfiAcpiDevicePath),
	ExtendedAcpi(&'a acpi::EfiExtendedAcpiDevicePath),
	Address(&'a acpi::EfiAddressDevicePath),
	NVDIMM(&'a acpi::EfiNVDIMMDevicePath),
}

impl<'a> From<&'a EfiDevicePathProcotol> for EfiAcpiDevicePathSubtype<'a> {
	fn from(path: &'a EfiDevicePathProcotol) -> Self {
		use EfiAcpiDevicePathSubtype::*;
		use self::acpi::*;
	
		match path.path_subtype {
			1 => match path.len() {
				12 => Acpi(EfiAcpiDevicePath::new(path)),
				_ => Undefined,
			},
			2 => match path.len() {
				x if x >= 19 => ExtendedAcpi(EfiExtendedAcpiDevicePath::new(path)),
				_ => Undefined,
			},
			3 => match path.len() {
				x if x >= 8 && x % 4 == 0 => Address(EfiAddressDevicePath::new(path)),
				_ => Undefined,
			},
			4 => match path.len() {
				8 => NVDIMM(EfiNVDIMMDevicePath::new(path)),
				_ => Undefined,
			},
			_ => Undefined,
		}
	}
}

#[non_exhaustive]
#[allow(non_camel_case_types)]
pub enum EfiMessagingDevicePathSubtype<'a> {
	Undefined,

	ATAPacketInterface(&'a messaging::EfiAtapiDevicePath),
	SCSI(&'a messaging::EfiScsiDevicePath),
	FibreChannel(&'a messaging::EfiFibreChannelDevicePath),
	Firewire(&'a messaging::EfiFirewireDevicePath),
	UniversalSerialBus(&'a messaging::EfiUsbDevicePath),
	I2O(&'a messaging::EfiI2ODevicePath),
	Sata(&'a messaging::EfiSataDevicePath),
	InfiniBand(&'a messaging::EfiInfiniBandDevicePath),
	UARTFlowControl(&'a messaging::EfiUartFlowControlDevicePath),
	SerialAttachedSCSI(&'a messaging::EfiSerialAttachedScsiDevicePath),
	VendorDefined(&'a messaging::EfiVendorDefinedDevicePath),
	MAC_Address(&'a messaging::EfiMacAddressDevicePath),
	IPv4(&'a messaging::EfiIPv4DevicePath),
	IPv6(&'a messaging::EfiIPv6DevicePath),
	UART(&'a messaging::EfiUartDevicePath),
	UniversalSerialBusClass(&'a messaging::EfiUsbClassDevicePath),
	UniversalSerialBusWWID(&'a messaging::EfiUsbWwidDevicePath),
	LogicalUnit(&'a messaging::EfiLogicalUnitDevicePath),
	iSCSI(&'a messaging::EfiiScsiDevicePath),
	VLAN(&'a messaging::EfiVlanDevicePath),
	FibreChannel_Ex(&'a messaging::EfiFibreChannelExDevicePath),
	SerialAttachedSCSI_Ex(&'a messaging::EfiSerialAttachedScsiExDevicePath),
	NVMExpressNamespace(&'a messaging::EfiNvmExpressDevicePath),
	UniversalResourceIdentifier(&'a messaging::EfiUniversalResourceIdentifierDevicePath),
	UniversalFlashStorage(&'a messaging::EfiUniversalResourceIdentifierDevicePath),
	SecureDigital(&'a messaging::EfiSecureDigitalDevicePath),
	Bluetooth(&'a messaging::EfiBluetoothDevicePath),
	Wireless(&'a messaging::EfiWirelessDevicePath),
	EmbeddedMultiMediaCard(&'a messaging::EfiEmbeddedMultiMediaCardDevicePath),
	BluetoothLE(&'a messaging::EfiBluetoothLEDevicePath),
	DomainNameService(&'a messaging::EfiDomainNameServiceDevicePath),
	NVDIMM_Namespace(&'a messaging::EfiNvdimmNamespaceDevicePath),
	REST_Service(&'a messaging::EfiRestServiceDevicePath),
}

impl<'a> From<&'a EfiDevicePathProcotol> for EfiMessagingDevicePathSubtype<'a> {
	fn from(path: &'a EfiDevicePathProcotol) -> Self {
		use EfiMessagingDevicePathSubtype::*;
		use messaging::*;
	
		match path.path_subtype {
			1 => match path.len() {
				8 => ATAPacketInterface(EfiAtapiDevicePath::new(path)),
				_ => Undefined,
			},
			2 => match path.len() {
				8 => SCSI(EfiScsiDevicePath::new(path)),
				_ => Undefined,
			},
			3 => match path.len() {
				24 => FibreChannel(EfiFibreChannelDevicePath::new(path)),
				_ => Undefined,
			},
			4 => match path.len() {
				16 => Firewire(EfiFirewireDevicePath::new(path)),
				_ => Undefined,
			},
			5 => match path.len(){
				6 => UniversalSerialBus(EfiUsbDevicePath::new(path)),
				_ => Undefined,
			},
			6 => match path.len() {
				8 => I2O(EfiI2ODevicePath::new(path)),
				10 => Sata(EfiSataDevicePath::new(path)),
				_ => Undefined,
			},
			9 => match path.len() {
				48 => InfiniBand(EfiInfiniBandDevicePath::new(path)),
				_ => Undefined,
			},
			10 => match path.len() {
				x if x >= 20 => {
					let guid: EfiGuid = unsafe {
						EfiGuid::from_raw(&path.path_data as *const () as *const u8)
					};

					match guid.as_tuple() {
						(0x37499a9d, 0x542f, 0x4c89, [0xa0, 0x26, 0x35, 0xda, 0x14, 0x20, 0x94, 0xe4]) => UARTFlowControl(EfiUartFlowControlDevicePath::new(path)),
						(0xd487ddb4, 0x008b, 0x11d9, [0xaf, 0xdc, 0x00, 0x10, 0x83, 0xff, 0xca, 0x4d]) => SerialAttachedSCSI(EfiSerialAttachedScsiDevicePath::new(path)),
						_ => VendorDefined(EfiVendorDefinedDevicePath::new(path)), /* Generic Vendor Defined */
					}
				},
				_ => Undefined,
			},
			11 => match path.len() {
				37 => MAC_Address(EfiMacAddressDevicePath::new(path)),
				_ => Undefined,
			},
			12 => match path.len() {
				x if x == 19 || x == 27 => IPv4(EfiIPv4DevicePath::new(path)),
				_ => Undefined,
			},
			13 => match path.len() {
				60 => IPv6(EfiIPv6DevicePath::new(path)),
				_ => Undefined,
			},
			14 => match path.len() {
				19 => UART(EfiUartDevicePath::new(path)),
				_ => Undefined,
			},
			15 => match path.len() {
				11 => UniversalSerialBusClass(EfiUsbClassDevicePath::new(path)),
				_ => Undefined,
			},
			16 => match path.len() {
				x if x >= 10 => UniversalSerialBusWWID(EfiUsbWwidDevicePath::new(path)),
				_ => Undefined,
			},
			17 => match path.len() {
				5 => LogicalUnit(EfiLogicalUnitDevicePath::new(path)),
				_ => Undefined,
			},
			19 => match path.len() {
				x if x >= 18 => iSCSI(EfiiScsiDevicePath::new(path)),
				_ => Undefined,
			},
			20 => match path.len() {
				6 => VLAN(EfiVlanDevicePath::new(path)),
				_ => Undefined,
			},
			21 => match path.len() {
				20 => FibreChannel_Ex(EfiFibreChannelExDevicePath::new(path)),
				_ => Undefined,
			},
			22 => match path.len() {
				32 => SerialAttachedSCSI_Ex(EfiSerialAttachedScsiExDevicePath::new(path)),
				_ => Undefined,
			},
			23 => match path.len() {
				16 => NVMExpressNamespace(EfiNvmExpressDevicePath::new(path)),
				_ => Undefined,
			},
			24 => match path.len() {
				x if x >= 4 => UniversalResourceIdentifier(EfiUniversalResourceIdentifierDevicePath::new(path)),
				_ => Undefined,
			},
			25 => match path.len() {
				6 => UniversalFlashStorage(EfiUniversalResourceIdentifierDevicePath::new(path)),
				_ => Undefined,
			},
			26 => match path.len() {
				5 => SecureDigital(EfiSecureDigitalDevicePath::new(path)),
				_ => Undefined,
			},
			27 => match path.len() {
				10 => Bluetooth(EfiBluetoothDevicePath::new(path)),
				_ => Undefined,
			},
			28 => match path.len() {
				36 => Wireless(EfiWirelessDevicePath::new(path)),
				_ => Undefined,
			},
			29 => match path.len() {
				5 => EmbeddedMultiMediaCard(EfiEmbeddedMultiMediaCardDevicePath::new(path)),
				_ => Undefined,
			},
			30 => match path.len() {
				11 => BluetoothLE(EfiBluetoothLEDevicePath::new(path)),
				_ => Undefined,
			},
			31 => match path.len() {
				x if (x as usize - 5) % size_of::<EfiIPAddressRaw>() == 0 => DomainNameService(EfiDomainNameServiceDevicePath::new(path)),
				_ => Undefined,
			},
			32 => match path.len() {
				20 => NVDIMM_Namespace(EfiNvdimmNamespaceDevicePath::new(path)),
				x if x == 6 || x >= 21 => REST_Service(EfiRestServiceDevicePath::new(path)),
				_ => Undefined,
			},
			_ => Undefined,
		}
	}
}

#[non_exhaustive]
pub enum EfiMediaDevicePathSubtype<'a> {
	Undefined,

	HardDrive(&'a media::EfiHardDriveDevicePath),
	CDROM(&'a media::EfiCDROMDevicePath),
	VendorDefined(&'a media::EfiVendorDefinedDevicePath),
	FilePath(&'a media::EfiFilePathDevicePath),
	MediaProtocol(&'a media::EfiMediaProtocolDevicePath),
	PIWGFirmwareFile(&'a media::EfiPIWGFirmwareFileDevicePath),
	PIWGFirmwareVolume(&'a media::EfiPIWGFirmwareVolumeDevicePath),
	RelativeOffsetRange(&'a media::EfiRelativeOffsetRangeDevicePath),
	RAMDisk(&'a media::EfiRAMDiskDevicePath),
}

impl<'a> From<&'a EfiDevicePathProcotol> for EfiMediaDevicePathSubtype<'a> {
	fn from(path: &'a EfiDevicePathProcotol) -> Self {
		use EfiMediaDevicePathSubtype::*;
		use media::*;
	
		match path.path_subtype {
			1 if path.len() == 42 => HardDrive(EfiHardDriveDevicePath::new(path)),
			2 if path.len() == 24 => CDROM(EfiCDROMDevicePath::new(path)),
			3 if path.len() >= 20 => VendorDefined(EfiVendorDefinedDevicePath::new(path)),
			4 if path.len() >= 4 => FilePath(EfiFilePathDevicePath::new(path)),
			5 if path.len() == 20 => MediaProtocol(EfiMediaProtocolDevicePath::new(path)),
			6 if path.len() >= 4 => PIWGFirmwareFile(EfiPIWGFirmwareFileDevicePath::new(path)),
			7 if path.len() >= 4 => PIWGFirmwareVolume(EfiPIWGFirmwareVolumeDevicePath::new(path)),
			8 if path.len() == 24 => RelativeOffsetRange(EfiRelativeOffsetRangeDevicePath::new(path)),
			9 if path.len() == 38 => RAMDisk(EfiRAMDiskDevicePath::new(path)),
			_ => Undefined,
		}
	}
}

#[non_exhaustive]
pub enum EfiBiosSpecificationDevicePathSubtype<'a> {
	Undefined,

	V1_01(&'a bios_specification::EfiBiosBootSpecification_1_01_DevicePath),
}

impl<'a> From<&'a EfiDevicePathProcotol> for EfiBiosSpecificationDevicePathSubtype<'a> {
	fn from(path: &'a EfiDevicePathProcotol) -> Self {
		use EfiBiosSpecificationDevicePathSubtype::*;
		use bios_specification::*;
	
		match path.path_subtype {
			1 if path.len() >= 8 => V1_01(EfiBiosBootSpecification_1_01_DevicePath::new(path)),
			_ => Undefined,
		}
	}
}

#[repr(C)]
pub struct EfiDevicePathProcotol {
	path_type: u8,
	path_subtype: u8,
	length: [u8; 2],
	path_data: (),
}

impl EfiDevicePathProcotol {
	pub fn parse_object<'a>(&'a self) -> EfiDevicePathType<'a> {
		EfiDevicePathType::<'a>::from(self)
	}

	fn is_end_of_device_path(&self) -> bool {
		if self.path_type == 0x7F {
			if self.path_subtype == 0xFF {
				true
			} else {
				false
			}
		} else {
			false
		}
	}

	pub(crate) fn len(&self) -> u16 {
		unsafe {
			(
				self.length.as_ptr() as *const u16
			).read_unaligned()
		}
	}

	pub(crate) fn data(&self) -> *const u8 {
		&self.path_data as *const () as *const u8
	}
}

impl EfiProtocol for EfiDevicePathProcotol {
	fn guid() -> EfiGuid {
		EfiGuid::from_tuple((0x09576e91, 0x6d3f, 0x11d2, [0x8e, 0x39, 0x00, 0xa0, 0xc9, 0x69, 0x72, 0x3b]))
	}
}

pub struct EfiDevicePathProcotolIterator<'a> {
	current: &'a EfiDevicePathProcotol,
}

impl<'a> Iterator for EfiDevicePathProcotolIterator<'a> {
	type Item = &'a EfiDevicePathProcotol;

	fn next(&mut self) -> Option<<Self as Iterator>::Item> {
		if self.current.is_end_of_device_path() {
			None
		} else {
			let return_item: &'a EfiDevicePathProcotol = self.current;
			self.current = unsafe {
				&*(
					(
						self.current as *const EfiDevicePathProcotol as *const u8
					).offset(self.current.len() as usize as isize) as *const EfiDevicePathProcotol
				)
			};
			Some(return_item)
		}
	}
}

pub(crate) trait EfiDevicePathRepr: Sized {
	fn new<'a>(path: &'a EfiDevicePathProcotol) -> &'a Self {
		unsafe {
			&*(path as *const EfiDevicePathProcotol as *const Self)
		}
	}
}
