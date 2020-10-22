use crate::EfiGuid;

pub const EFI_GLOBAL_VARIABLE: EfiGuid = EfiGuid::from_tuple((
    0x8BE4_DF61,
    0x93CA,
    0x11d2,
    [0xAA, 0x0D, 0x00, 0xE0, 0x98, 0x03, 0x2B, 0x8C],
));

pub const EFI_SIMPLE_TEXT_INPUT_PROTOCOL: EfiGuid = EfiGuid::from_tuple((
    0x3874_77C1,
    0x69C7,
    0x11D2,
    [0x8E, 0x39, 0x00, 0xA0, 0xC9, 0x69, 0x72, 0x3B],
));

pub const EFI_SIMPLE_TEXT_OUTPUT_PROTOCOL: EfiGuid = EfiGuid::from_tuple((
    0x3874_77C2,
    0x69C7,
    0x11D2,
    [0x8E, 0x39, 0x00, 0xA0, 0xC9, 0x69, 0x72, 0x3B],
));

pub const EFI_DEVICE_PATH_PROTOCOL: EfiGuid = EfiGuid::from_tuple((
    0x0957_6E91,
    0x6D3F,
    0x11D2,
    [0x8E, 0x39, 0x00, 0xA0, 0xC9, 0x69, 0x72, 0x3B],
));

pub const EFI_BLOCK_IO_PROTOCOL: EfiGuid = EfiGuid::from_tuple((
    0x964E_5B21,
    0x6459,
    0x11D2,
    [0x8E, 0x39, 0x00, 0xA0, 0xC9, 0x69, 0x72, 0x3B],
));

pub const EFI_DISK_IO_PROTOCOL: EfiGuid = EfiGuid::from_tuple((
    0xCE34_5171,
    0xBA0B,
    0x11D2,
    [0x8E, 0x4F, 0x00, 0xA0, 0xC9, 0x69, 0x72, 0x3B],
));

pub const EFI_RT_PROPERTIES_TABLE: EfiGuid = EfiGuid::from_tuple((
    0xEB66_918A,
    0x7EEF,
    0x402A,
    [0x84, 0x2E, 0x93, 0x1D, 0x21, 0xC3, 0x8A, 0xE9],
));
