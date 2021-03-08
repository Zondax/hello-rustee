//! This module contains wrappers around the secure storage api described by the OPTEE spec

use core::convert::TryInto;

use enumflags2::{bitflags, make_bitflags, BitFlags};

use crate::wrapper::raw::{self, TEE_ObjectHandle};

const TEE_HANDLE_NULL: TEE_ObjectHandle = raw::TEE_HANDLE_NULL as _;

mod objid;
use objid::ObjectID;

#[derive(Debug)]
///Describes the possible errors when dealing with objects
//TODO: describe
pub enum ObjectError {
    AccessConflict,
    OutOfMemory,
    NoSpace,
    Corrupt,
    NotSupported,
    BadParameters,
    Unavailable,
    UnknownError,
}

impl ObjectError {
    pub fn from_os(code: raw::TEE_Result) -> Result<(), Self> {
        match code {
            raw::TEE_SUCCESS => Ok(()),
            raw::TEE_ERROR_OUT_OF_MEMORY => Err(Self::OutOfMemory),
            raw::TEE_ERROR_ACCESS_CONFLICT => Err(Self::AccessConflict),
            raw::TEE_ERROR_STORAGE_NO_SPACE => Err(Self::NoSpace),
            raw::TEE_ERROR_NOT_SUPPORTED => Err(Self::NotSupported),
            raw::TEE_ERROR_BAD_PARAMETERS => Err(Self::BadParameters),
            raw::TEE_ERROR_CORRUPT_OBJECT => Err(Self::Corrupt),
            raw::TEE_ERROR_STORAGE_NOT_AVAILABLE => Err(Self::Unavailable),
            _ => Err(Self::UnknownError),
        }
    }
}

#[bitflags]
#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq)]
/// When creating or opening a persistent object, some access flags must be passed
///
/// This enum represents therefore the possible access flags to the object
pub enum PersistentObjectAccessFlags {
    ///Allow the TA to read the associated data stream
    Read = raw::TEE_DATA_FLAG_ACCESS_READ,
    ///Allow the TA to modify the associated data stream
    Write = raw::TEE_DATA_FLAG_ACCESS_WRITE,
    ///Allow the TA to delete or rename the object
    WriteMeta = raw::TEE_DATA_FLAG_ACCESS_WRITE_META,
    ///Allow the TA to obtain multiple read handles to the object
    ShareRead = raw::TEE_DATA_FLAG_SHARE_READ,
    ///Allow the TA to obtain multiple write handles to the object
    ShareWrite = raw::TEE_DATA_FLAG_SHARE_WRITE,
    ///Allow the TA to overwrite an existing object during creation
    Overwrite = raw::TEE_DATA_FLAG_OVERWRITE,
}

impl PersistentObjectAccessFlags {
    fn iter_to_flags(slice: impl IntoIterator<Item = Self>) -> BitFlags<Self> {
        slice
            .into_iter()
            .map(|f| BitFlags::from_flag(f))
            .fold(BitFlags::empty(), |acc, f| acc | f)
    }
}

/// A handle to a persistent object
pub struct PersistentObject<ID> {
    handle: TEE_ObjectHandle,
    id: ID,
    flags: BitFlags<PersistentObjectAccessFlags>,
}

/// A handle to a transient object
pub struct TransientObject<ID> {
    handle: TEE_ObjectHandle,
    id: ID,
}

impl<ID: ObjectID> PersistentObject<ID> {
    pub fn create_data_only(id: ID, data: &[u8], editable: bool) -> Result<Self, ObjectError> {
        let mut flags = make_bitflags!(PersistentObjectAccessFlags::{Read | Write | WriteMeta});
        if !editable {
            flags.remove(PersistentObjectAccessFlags::Write);
            flags.remove(PersistentObjectAccessFlags::WriteMeta);
        }

        Self::create(id, flags.iter(), None, Some(data))
    }

    pub fn create<'obj, 'd>(
        id: ID,
        flags: impl IntoIterator<Item = PersistentObjectAccessFlags>,
        attrs: impl Into<Option<&'obj TEE_ObjectHandle>>,
        data: impl Into<Option<&'d [u8]>>,
    ) -> Result<Self, ObjectError> {
        let attrs = attrs.into().unwrap_or(&TEE_HANDLE_NULL);
        let data = data.into().unwrap_or(&[]);
        let flags = PersistentObjectAccessFlags::iter_to_flags(flags);

        let mut object: TEE_ObjectHandle = TEE_HANDLE_NULL;

        let code = unsafe {
            raw::TEE_CreatePersistentObject(
                raw::TEE_STORAGE_PRIVATE,
                id.byte_slice().as_ptr() as _,
                id.byte_len() as _,
                flags.bits(),
                *attrs,
                data.as_ptr() as _,
                data.len() as _,
                &mut object as *mut _,
            )
        };

        ObjectError::from_os(code).map(|_| Self {
            handle: object,
            id,
            flags,
        })
    }
}

impl<ID> PersistentObject<ID> {
    pub fn handle(&self) -> &TEE_ObjectHandle {
        &self.handle
    }
}

impl<ID: ObjectID> TransientObject<ID> {
    pub fn try_into_persistent(
        self,
        flags: impl IntoIterator<Item = PersistentObjectAccessFlags>,
    ) -> Result<PersistentObject<ID>, ObjectError> {
        PersistentObject::create(self.id, flags, Some(&self.handle), None)
    }
}

impl<ID> Drop for TransientObject<ID> {
    fn drop(&mut self) {
        let handle = core::mem::replace(&mut self.handle, TEE_HANDLE_NULL);

        unsafe { raw::TEE_CloseObject(handle) };
    }
}

impl<ID> Drop for PersistentObject<ID> {
    fn drop(&mut self) {
        let handle = core::mem::replace(&mut self.handle, TEE_HANDLE_NULL);

        unsafe { raw::TEE_CloseObject(handle) };
    }
}
