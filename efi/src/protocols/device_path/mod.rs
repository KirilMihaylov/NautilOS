use crate::common::EfiGuid;
use crate::protocols::EfiProtocol;

pub mod hardware;

#[non_exhaustive]
pub enum EfiDevicePathType<'a> {
	Undefined,

	HardwarePath(EfiHardwareDevicePathSubtype<'a>),
	
	EndOfDevicePathInstance,
	EndOfDevicePath,
}

impl<'a> From<&'a EfiDevicePathProcotol> for EfiDevicePathType<'a> {
	fn from(path: &'a EfiDevicePathProcotol) -> Self {
		use EfiDevicePathType::*;
	
		match path.path_type {
			1 => HardwarePath(EfiHardwareDevicePathSubtype::from(path)),
			
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
			1 if path.len() == 6 => Pci(EfiPciDevicePath::new(path)),
			2 if path.len() == 5 => PcCard(EfiPcCardDevicePath::new(path)),
			3 if path.len() == 24 => MemoryMapped(EfiMemoryMappedDevicePath::new(path)),
			4 if path.len() >= 20 => VendorDefined(EfiVendorDefinedDevicePath::new(path)),
			5 if path.len() == 8 => Controller(EfiControllerDevicePath::new(path)),
			6 if path.len() == 13 => BaseboardManagementController(EfiBaseboardManagementControllerDevicePath::new(path)),
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
	type Interface = Self;

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

pub(crate) trait EfiDevicePathInto<T> {
	fn new<'a>(path: &'a EfiDevicePathProcotol) -> &'a T {
		unsafe {
			&*(path as *const EfiDevicePathProcotol as *const T)
		}
	}
}
