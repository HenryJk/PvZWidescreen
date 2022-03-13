#![allow(non_snake_case)]

use std::ffi::c_void;

use windows::core::{GUID, HRESULT};

use crate::stub::D3DDEVICEDESC7;

#[repr(C)]
pub struct _D3DVIEWPORT7 {
    pub dwX: u32,
    pub dwY: u32,
    pub dwWidth: u32,
    pub dwHeight: u32,
    pub dvMinZ: f32,
    pub dvMaxZ: f32,
}

#[repr(C)]
pub struct IDirect3DDevice7 {
    pub vtable: &'static IDirect3DDevice7VTable,
}

#[repr(C)]
pub struct D3DRECT {
    pub x1: i32,
    pub y1: i32,
    pub x2: i32,
    pub y2: i32,
}

#[repr(C)]
pub struct IDirect3DDevice7VTable {
    pub QueryInterface:
        unsafe extern "stdcall" fn(*mut IDirect3DDevice7, &GUID, *mut *mut c_void) -> HRESULT,
    pub AddRef: unsafe extern "stdcall" fn(*mut IDirect3DDevice7) -> u32,
    pub Release: unsafe extern "stdcall" fn(*mut IDirect3DDevice7) -> u32,
    pub GetCaps: unsafe extern "stdcall" fn(*mut IDirect3DDevice7, *mut D3DDEVICEDESC7) -> HRESULT,
    pub EnumTextureFormats: *const c_void,
    pub BeginScene: unsafe extern "stdcall" fn(*mut IDirect3DDevice7) -> HRESULT,
    pub EndScene: unsafe extern "stdcall" fn(*mut IDirect3DDevice7) -> HRESULT,
    pub GetDirect3D: *const c_void,
    pub SetRenderTarget: *const c_void,
    pub GetRenderTarget: *const c_void,
    pub Clear: unsafe extern "stdcall" fn(
        *mut IDirect3DDevice7,
        u32,
        *const D3DRECT,
        u32,
        u32,
        f32,
        u32,
    ) -> HRESULT,
    pub SetTransform: *const c_void,
    pub GetTransform: *const c_void,
    pub SetViewport: *const c_void,
    pub MultiplyTransform: *const c_void,
    pub GetViewport: *const c_void,
}
