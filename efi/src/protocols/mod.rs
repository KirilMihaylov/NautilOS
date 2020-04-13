use crate::guid::EfiGuid;

pub mod console;
pub mod device_path;
pub mod media;

pub trait EfiProtocol {
	type Interface;

	fn guid() -> EfiGuid;
}