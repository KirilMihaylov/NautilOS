#[repr(C)]
#[derive(Clone,Copy)]
#[non_exhaustive]
pub enum EfiEventType {
	#[allow(overflowing_literals)]
	Timer = 0x80000000,
	Runtime = 0x40000000,
	NotifyWait = 0x100,
	NotifySignal = 0x200,
	SignalExitBootServices = 0x201,
	SignalVirtualAddressChange = 0x60000202,
}

#[repr(C)]
#[derive(Clone,Copy)]
#[non_exhaustive]
pub enum EfiTimerDelay {
	Cancel,
	Pediodic,
	Relative,
}
