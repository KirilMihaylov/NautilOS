use core::{mem::transmute, slice::from_raw_parts};

pub struct EfiLoadOption<'a> {
    attributes: u32,
    description: &'a [u16],
    file_path_list: &'a [u8],
    optional_data: &'a [u8],
}

impl<'a> EfiLoadOption<'a> {
    pub fn parse(data: &'a [u8]) -> Option<Self> {
        if data.len() < 7 {
            None
        } else {
            let attributes: u32 = unsafe { *(data.as_ptr() as *const u32) };
            let file_path_list_length: u16 = unsafe { *(&data[4] as *const u8 as *const u16) };
            let mut description: &[u16] =
                unsafe { from_raw_parts(data[6..].as_ptr() as *const u16, (data.len() - 6) / 2) };
            if description.iter().any(|&unit: &u16| unit == 0) {
                description = unsafe {
                    from_raw_parts(
                        description.as_ptr(),
                        description
                            .iter()
                            .take_while(|&&unit: &&u16| unit != 0)
                            .count()
                            + 1,
                    )
                };

                let mut file_path_list: &[u8] =
                    unsafe { transmute::<&[u8], &[u8]>(&data[(6 + description.len() * 2)..]) };

                if file_path_list_length as usize <= file_path_list.len() {
                    let optional_data: &[u8] = &file_path_list[file_path_list_length as usize..];
                    file_path_list = &file_path_list[..file_path_list_length as usize];
                    Some(Self {
                        attributes,
                        description,
                        file_path_list,
                        optional_data,
                    })
                } else {
                    None
                }
            } else {
                None
            }
        }
    }

    pub fn attributes(&self) -> u32 {
        self.attributes
    }

    pub fn description(&self) -> &[u16] {
        self.description
    }

    pub fn file_path_list(&self) -> &[u8] {
        self.file_path_list
    }

    pub fn optional_data(&self) -> &[u8] {
        self.optional_data
    }
}
