pub mod task_priority {
    pub type EfiTaskPriorityLevel = usize;
}

pub mod memory {
    use {
        crate::{EfiPhysicalAddress, EfiVirtualAddress},
        core::{
            mem::{size_of, transmute},
            ops::Index,
            ops::IndexMut,
        },
    };
    #[repr(C)]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
    #[non_exhaustive]
    pub enum EfiAllocateType {
        AllocateAnyPages,
        AllocateMaxAddress,
        AllocateAddress,
        MaxAllocateType,
    }

    #[repr(C)]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
    #[non_exhaustive]
    pub enum EfiMemoryType {
        EfiReservedMemoryType,
        EfiLoaderCode,
        EfiLoaderData,
        EfiBootServicesCode,
        EfiBootServicesData,
        EfiRuntimeServicesCode,
        EfiRuntimeServicesData,
        EfiConventionalMemory,
        EfiUnusableMemory,
        EfiACPIReclaimMemory,
        EfiACPIMemoryNVS,
        EfiMemoryMappedIO,
        EfiMemoryMappedIOPortSpace,
        EfiPalCode,
        EfiPersistentMemory,
        EfiMaxMemoryType,
    }

    impl EfiMemoryType {
        pub const fn custom(mut memory_type: [u8; EFI_MEMORY_TYPE_SIZE]) -> EfiMemoryType {
            memory_type[EFI_MEMORY_TYPE_SIZE - 1] |= 0x80;

            unsafe { transmute(memory_type) }
        }

        pub fn is_custom(&self) -> bool {
            ((*self as usize) & 1usize.reverse_bits()) != 0
        }
    }

    pub const EFI_MEMORY_TYPE_SIZE: usize = size_of::<EfiMemoryType>();

    pub struct EfiGetMemoryMapResult<'a> {
        key: usize,
        descriptor_version: u32,
        descriptors: EfiMemoryDescriptorsMut<'a>,
    }

    impl<'a> EfiGetMemoryMapResult<'a> {
        #[must_use]
        pub(crate) const fn new(
            key: usize,
            descriptor_version: u32,
            descriptors: EfiMemoryDescriptorsMut<'a>,
        ) -> Self {
            Self {
                key,
                descriptor_version,
                descriptors,
            }
        }

        #[must_use]
        pub const fn key(&self) -> usize {
            self.key
        }

        #[must_use]
        pub const fn descriptor_version(&self) -> u32 {
            self.descriptor_version
        }

        #[must_use]
        pub const fn take_descriptors(self) -> EfiMemoryDescriptorsMut<'a> {
            self.descriptors
        }

        #[must_use]
        pub const fn take(self) -> (usize, u32, EfiMemoryDescriptorsMut<'a>) {
            (self.key, self.descriptor_version, self.descriptors)
        }
    }

    #[repr(C)]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct EfiMemoryDescriptor {
        memory_type: u32,
        physical_start: EfiPhysicalAddress,
        virtual_start: EfiVirtualAddress,
        number_of_pages: u64,
        attributes: u64,
    }

    impl EfiMemoryDescriptor {
        #[must_use]
        pub fn memory_type(&self) -> u32 {
            self.memory_type
        }

        pub fn set_memory_type(&mut self, memory_type: u32) {
            self.memory_type = memory_type;
        }

        #[must_use]
        pub fn physical_start(&self) -> EfiPhysicalAddress {
            self.physical_start
        }

        pub fn set_physical_start(&mut self, physical_start: EfiPhysicalAddress) {
            self.physical_start = physical_start;
        }

        #[must_use]
        pub fn virtual_start(&self) -> EfiVirtualAddress {
            self.virtual_start
        }

        pub fn set_virtual_start(&mut self, virtual_start: EfiVirtualAddress) {
            self.virtual_start = virtual_start;
        }

        #[must_use]
        pub fn number_of_pages(&self) -> u64 {
            self.number_of_pages
        }

        pub fn set_number_of_pages(&mut self, number_of_pages: u64) {
            self.number_of_pages = number_of_pages;
        }

        #[must_use]
        pub fn attributes(&self) -> u64 {
            self.attributes
        }

        pub fn set_attributes(&mut self, attributes: u64) {
            self.attributes = attributes;
        }
    }

    #[derive(Clone)]
    pub struct EfiMemoryDescriptors<'a> {
        buffer: &'a [u8],
        descriptor_size: usize,
    }

    impl<'a> EfiMemoryDescriptors<'a> {
        #[must_use]
        pub fn new(buffer: &'a [u8], mut descriptor_size: usize) -> Self {
            descriptor_size = descriptor_size.max(size_of::<EfiMemoryDescriptor>());

            Self {
                buffer: &buffer[..buffer.len() - (buffer.len() % descriptor_size)],
                descriptor_size,
            }
        }

        #[must_use]
        pub const fn is_empty(&self) -> bool {
            self.buffer.is_empty()
        }

        #[must_use]
        pub const fn len(&self) -> usize {
            self.buffer.len() / self.descriptor_size
        }

        #[must_use]
        pub const fn descriptor_size(&self) -> usize {
            self.descriptor_size
        }

        #[must_use]
        pub const fn iter(&'a self) -> EfiMemoryDescriptorIterator<'a> {
            EfiMemoryDescriptorIterator::new(self)
        }
    }

    impl Index<usize> for EfiMemoryDescriptors<'_> {
        type Output = EfiMemoryDescriptor;

        fn index(&self, index: usize) -> &<Self as Index<usize>>::Output {
            assert!(index < self.len());

            unsafe {
                &*(&self.buffer[index * self.descriptor_size] as *const u8
                    as *const EfiMemoryDescriptor)
            }
        }
    }

    pub struct EfiMemoryDescriptorsMut<'a> {
        buffer: &'a mut [u8],
        descriptor_size: usize,
    }

    impl<'a> EfiMemoryDescriptorsMut<'a> {
        #[must_use]
        pub fn new(buffer: &'a mut [u8], mut descriptor_size: usize) -> Self {
            descriptor_size = descriptor_size.max(size_of::<EfiMemoryDescriptor>());

            let length: usize = buffer.len() - (buffer.len() % descriptor_size);

            Self {
                buffer: &mut buffer[..length],
                descriptor_size,
            }
        }

        #[must_use]
        pub const fn is_empty(&self) -> bool {
            self.buffer.is_empty()
        }

        #[must_use]
        pub const fn len(&self) -> usize {
            self.buffer.len() / self.descriptor_size
        }

        #[must_use]
        pub const fn descriptor_size(&self) -> usize {
            self.descriptor_size
        }

        #[must_use]
        pub fn iter(&'a self) -> EfiMemoryDescriptorIterator<'a> {
            EfiMemoryDescriptorIterator::new(self.as_ref())
        }
    }

    impl Index<usize> for EfiMemoryDescriptorsMut<'_> {
        type Output = EfiMemoryDescriptor;

        fn index(&self, index: usize) -> &<Self as Index<usize>>::Output {
            assert!(index < self.len());

            unsafe {
                &*(&self.buffer[index * self.descriptor_size] as *const u8
                    as *const EfiMemoryDescriptor)
            }
        }
    }

    impl IndexMut<usize> for EfiMemoryDescriptorsMut<'_> {
        fn index_mut(&mut self, index: usize) -> &mut <Self as Index<usize>>::Output {
            assert!(index < self.len());

            unsafe {
                &mut *(&mut self.buffer[index * self.descriptor_size] as *mut u8
                    as *mut EfiMemoryDescriptor)
            }
        }
    }

    impl<'a> AsRef<EfiMemoryDescriptors<'a>> for EfiMemoryDescriptorsMut<'a> {
        fn as_ref(&self) -> &EfiMemoryDescriptors<'a> {
            unsafe { &*(self as *const EfiMemoryDescriptorsMut as *const EfiMemoryDescriptors) }
        }
    }

    #[derive(Clone)]
    pub struct EfiMemoryDescriptorIterator<'a> {
        descriptors: &'a EfiMemoryDescriptors<'a>,
        index: usize,
    }

    impl<'a> EfiMemoryDescriptorIterator<'a> {
        #[must_use]
        const fn new(descriptors: &'a EfiMemoryDescriptors<'a>) -> Self {
            Self {
                descriptors,
                index: 0,
            }
        }

        pub fn reset_position(&mut self) {
            self.index = 0;
        }
    }

    impl Iterator for EfiMemoryDescriptorIterator<'_> {
        type Item = EfiMemoryDescriptor;

        fn next(&mut self) -> Option<<Self as Iterator>::Item> {
            if self.index < self.descriptors.len() {
                self.index += 1;

                Some(self.descriptors[self.index - 1])
            } else {
                None
            }
        }
    }
}

pub mod event_and_timer {
    use crate::{EfiEvent, VoidPtr};

    #[repr(u32)]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
    #[non_exhaustive]
    pub enum EfiEventType {
        // #[allow(overflowing_literals)]
        Timer = 0x80000000,
        Runtime = 0x40000000,
        NotifyWait = 0x100,
        NotifySignal = 0x200,
        SignalExitBootServices = 0x201,
        SignalVirtualAddressChange = 0x60000202,
    }

    #[repr(C)]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
    #[non_exhaustive]
    pub enum EfiTimerDelay {
        Cancel,
        Pediodic,
        Relative,
    }

    pub type EfiEventNotifyCallback = extern "efiapi" fn(EfiEvent, VoidPtr);
}

pub mod protocol_handler {
    #[repr(C)]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
    #[non_exhaustive]
    pub enum EfiInterfaceType {
        NativeInterface,
    }

    #[repr(C)]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
    #[non_exhaustive]
    pub enum EfiLocateSearchType {
        AllHandles,
        ByRegisterNotify,
        ByProtocol,
    }
}
