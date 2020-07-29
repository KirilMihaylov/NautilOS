use crate::types::EfiGuid;

pub unsafe trait EfiConfigurationTable {
    fn guid() -> EfiGuid;
}
