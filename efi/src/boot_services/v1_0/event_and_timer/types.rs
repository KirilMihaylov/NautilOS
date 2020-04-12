use crate::types::{
	EfiEvent,
	VoidPtr,
};

pub type EfiEventNotifyCallback = extern "efiapi" fn(event: EfiEvent, context: VoidPtr);
