use crate::protocols::device_path::{
	EfiDevicePathProcotol,
	EfiDevicePathInto,
};

#[repr(C)]
pub struct EfiUartFlowControlDevicePath {
	base: EfiDevicePathProcotol,
	vendor_guid: [u8; 16],
	flow_control_map: [u8; 4],
}

impl EfiUartFlowControlDevicePath {
	pub fn flow_control_map(&self) -> u32 {
		unsafe {
			(
				self.flow_control_map.as_ptr() as *const u32
			).read_unaligned()
		}
	}
}

impl EfiDevicePathInto<EfiUartFlowControlDevicePath> for EfiUartFlowControlDevicePath {}
