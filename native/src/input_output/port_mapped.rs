use crate::result::{
	Error,
	Result,
};

global_target_arch_else_error! {
	["x86","x86_64"]
	/// Represents the type required hold the port address.
	pub type IOPortType = u16;

	else "Error: Type \"IOPortType\" not defined for this platform!"
}

/// Abstract implementation over port-mapped I/O interface.
#[repr(transparent)]
pub struct IOPort {
	port: IOPortType,
}

impl IOPort {
	/// Creates new instance bound to the given port.
	pub fn new(port: IOPortType) -> Self {
		Self {
			port: port,
		}
	}

	/// Returns the port number the instance of the instance.
	pub fn port(&self) -> IOPortType {
		self.port
	}

	/// Receive 8-bit input from the I/O port.
	///
	/// # Notes
	/// On IA-32 (x86) and AMD64 (x86_64) it *should* always returns [`core::result::Result::Ok`].
	pub fn input_u8(&self) -> Result<u8> {
		target_arch_else! {
			["x86", "x86_64"] {
				let result: u8;

				unsafe {
					llvm_asm!(
						"in al, dx" :
						"={al}"(result) :
						"{dx}"(self.port) :
						:
						"intel"
					);
				}

				Ok(result)
			}
			else {
				Err(Error::Unavailable);
			}
		}
	}

	/// Send 8-bit input from the I/O port.
	///
	/// # Notes
	/// On IA-32 (x86) and AMD64 (x86_64) it *should* always returns [`core::result::Result::Ok`].
	pub fn output_u8(&self, value: u8) -> Result<()> {
		target_arch_else! {
			["x86", "x86_64"] {
				unsafe {
					llvm_asm!(
						"out dx, al" :
						:
						"{al}"(value), "{dx}"(self.port) :
						:
						"intel"
					);
				}

				Ok(())
			}
			else {
				Err(Error::Unavailable)
			}
		}
	}

	/// Recieve 16-bit input from the I/O port.
	pub fn input_u16(&self) -> Result<u16> {
		target_arch_else! {
			["x86", "x86_64"] {
				if self.port % 2 != 0 {
					return Err(Error::Unaligned);
				}
				
				let result: u16;

				unsafe {
					llvm_asm!(
						"in ax, dx" :
						"={ax}"(result) :
						"{dx}"(self.port) :
						:
						"intel"
					);
				}

				Ok(result)
			}
			else {
				Err(Error::Unavailable)
			}
		}
	}

	/// Send 16-bit input from the I/O port.
	pub fn output_u16(&self, value: u16) -> crate::result::Result<()> {
		target_arch_else! {
			["x86", "x86_64"] {
				if self.port % 2 != 0 {
					return Err(Error::Unaligned);
				}

				unsafe {
					llvm_asm!(
						"out dx, ax" :
						:
						"{ax}"(value), "{dx}"(self.port) :
						:
						"intel"
					);
				}

				Ok(())
			}
			else {
				Err(Error::Unavailable)
			}
		}
	}

	/// Recieve 32-bit input from the I/O port.
	pub fn input_u32(&self) -> Result<u32> {
		target_arch_else! {
			["x86", "x86_64"] {
				if self.port % 4 != 0 {
					return Err(Error::Unaligned);
				}

				let result: u32;

				unsafe {
					llvm_asm!(
						"in eax, dx" :
						"={eax}"(result) :
						"{dx}"(self.port) :
						:
						"intel"
					);
				}

				Ok(result)
			}
			else {
				Err(Error::Unavailable)
			}
		}
	}

	/// Send 32-bit input from the I/O port.
	pub fn output_u32(&self, value: u32) -> Result<()> {
		target_arch_else! {
			["x86", "x86_64"] {
				if self.port % 4 != 0 {
					return Err(Error::Unaligned);
				}

				unsafe {
					llvm_asm!(
						"out dx, eax" :
						:
						"{eax}"(value), "{dx}"(self.port) :
						:
						"intel"
					);
				}
				
				Ok(())
			}
			else {
				Err(Error::Unavailable)
			}
		}
	}
}
