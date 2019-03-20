use super::*;
use blkid_sys::*;
use libc;
use std::ffi::CStr;
use std::marker::PhantomData;

pub struct Partition<'a> {
    pub(crate) partition: blkid_partition,
    pub(crate) _marker: PhantomData<&'a blkid_partition>,
}

impl<'a> Partition<'a> {
    // TODO: get_flags();

    /// Returns the optional PartLabel of the partition.
    pub fn get_name(&self) -> Option<&str> {
        unsafe { cstr_to_str(blkid_partition_get_name(self.partition)) }
    }

    pub fn get_partno(&self) -> Result<u32, BlkIdError> {
        unsafe { cvt(blkid_partition_get_partno(self.partition)).map(|v| v as u32) }
    }

    pub fn get_size(&self) -> i64 {
        unsafe { blkid_partition_get_size(self.partition) }
    }

    pub fn get_start(&self) -> i64 {
        unsafe { blkid_partition_get_start(self.partition) }
    }

    // pub fn get_table();

    pub fn get_type(&self) -> i32 {
        unsafe { blkid_partition_get_type(self.partition) }
    }

    pub fn get_type_string(&self) -> Option<&str> {
        unsafe { cstr_to_str(blkid_partition_get_type_string(self.partition)) }
    }

    /// Returns the optional PartUUID of the partition.
    pub fn get_uuid(&self) -> Option<&str> {
        unsafe { cstr_to_str(blkid_partition_get_uuid(self.partition)) }
    }

    pub fn is_extended(&self) -> bool {
        unsafe { blkid_partition_is_extended(self.partition) != 0 }
    }

    pub fn is_logical(&self) -> bool {
        unsafe { blkid_partition_is_logical(self.partition) != 0 }
    }

    pub fn is_primary(&self) -> bool {
        unsafe { blkid_partition_is_primary(self.partition) != 0 }
    }
}
