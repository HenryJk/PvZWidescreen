#![allow(non_snake_case)]

use libc::{c_void, FILE};
use windows::Win32::Foundation::FILETIME;

use crate::{
    stub::{StdBasicString, StdList, StdMap, StdSet, StdVector},
    SexyAppBase, Undefined1,
};

#[repr(C)]
pub struct PakCollection {
    pub mFileHandle: *mut c_void,
    pub mMappingHandle: *mut c_void,
    pub mDataPtr: *mut c_void,
}

#[repr(C)]
pub struct PakRecord {
    pub mCollection: *mut PakCollection,
    pub mFileName: StdBasicString,
    pub mFileTime: FILETIME,
    pub mStartPos: i32,
    pub mSize: i32,
}

#[repr(C)]
pub struct PFILE {
    pub mRecord: *mut PakRecord,
    pub mPos: i32,
    pub mFP: *mut FILE,
}

#[repr(C)]
pub struct EncodingParser {
    unknown0: u32,
    pub mFile: *mut PFILE,
    pub mBufferedText: *mut StdVector,
    pub mGetCharFunc:
        unsafe extern "thiscall" fn(this: *mut Self, theChar: *mut u16, error: *mut bool) -> bool,
    pub mForcedEncodingType: bool,
    pub mFirstChar: bool,
    pub mByteSwap: bool,
    unknown1: [u8; 1],
}

#[repr(C)]
pub struct XMLParser {
    pub base: EncodingParser,
    pub mFileName: StdBasicString,
    pub mErrorText: StdBasicString,
    pub mLineNum: i32,
    pub mHasFailed: bool,
    pub mAllowComments: bool,
    pub mSection: StdBasicString,
}

#[repr(C)]
pub struct ResourceManager {
    unknown: u32,
    pub mLoadedGroups: StdSet,
    pub mImageMap: StdMap,
    pub mSoundMap: StdMap,
    pub mFontMap: StdMap,
    pub mXMLParser: *mut XMLParser,
    pub mError: StdBasicString,
    pub mHasFailed: bool,
    pub mApp: *mut SexyAppBase,
    pub mCurResGroup: StdBasicString,
    pub mDefaultPath: StdBasicString,
    pub mDefaultIdPrefix: StdBasicString,
    pub mAllowMissingProgramResources: bool,
    pub mAllowAlreadyDefinedResources: bool,
    pub mHadAlreadyDefinedError: bool,
    pub mResGroupMap: StdMap,
    pub mCurResGroupList: *mut StdList,
    pub mCurResGroupListItr: [Undefined1; 8],
}
