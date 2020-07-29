use crate::result::{Error, Result};

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
        Self { port }
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
                    asm!(
                        "in al, dx",
                        lateout("al") result,
                        in("dx") self.port,
                        options(nomem, nostack)
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
                    asm!(
                        "out dx, al",
                        in("al") value,
                        in("dx") self.port,
                        options(nomem, nostack)
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
                    asm!(
                        "in ax, dx",
                        lateout("ax") result,
                        in("dx") self.port,
                        options(nomem, nostack)
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
                    asm!(
                        "out dx, ax",
                        in("ax") value,
                        in("dx") self.port,
                        options(nomem, nostack)
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
                    asm!(
                        "in eax, dx",
                        lateout("eax") result,
                        in("dx") self.port,
                        options(nomem, nostack)
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
                    asm!(
                        "out dx, eax",
                        in("eax") value,
                        in("dx") self.port,
                        options(nomem, nostack)
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
