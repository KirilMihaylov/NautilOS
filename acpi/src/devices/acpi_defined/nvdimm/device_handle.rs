#[repr(transparent)]
#[derive(Clone,Copy)]
pub struct NVDIMMDeviceHandle {
	handle: u32,
}

impl NVDIMMDeviceHandle {
	pub fn dimm_number(&self) -> u8 {
		(self.handle & 15) as u8
	}

	pub fn channel_number(&self) -> u8 {
		((self.handle >> 4) & 15) as u8
	}

	pub fn controller_id(&self) -> u8 {
		((self.handle >> 8) & 15) as u8
	}

	pub fn socket_id(&self) -> u8 {
		((self.handle >> 12) & 15) as u8
	}

	pub fn node_controller_id(&self) -> u16 {
		((self.handle >> 16) & 4095) as u16
	}
}
