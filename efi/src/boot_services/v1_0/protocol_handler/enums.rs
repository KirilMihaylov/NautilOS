#[repr(C)]
pub enum EfiInterfaceType {
	NativeInterface,
}

#[repr(C)]
pub enum EfiLocateSearchType {
	AllHandles,
	ByRegisterNotify,
	ByProtocol,
}
