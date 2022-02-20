use core::{f32::consts::PI, intrinsics::transmute};

use std::error::Error;

use iced_x86::code_asm::*;
use winapi::um::winnt::PAGE_READWRITE;

use crate::{
    memory::{alloc_mem, inject, patch},
    POLE_NIGHT_PTR, POLE_PTR,
};

macro_rules! concat_array {
    () => {[]};
    ($a:expr) => {$a};
    ($a:expr, $b:expr) => {{
        let a = $a;
        let b = $b;
        let c: [_; $a.len() + $b.len()] = unsafe { $crate::concat(a, b) };
        // Constrain the element types to be the same to guide inference.
        let _: [*const _; 3] = [a.as_ptr(), b.as_ptr(), c.as_ptr()];
        c
    }};
    ($a:expr, $($rest:expr),*) => {concat_array!($a, concat_array!($($rest),*))};
    ($a:expr, $($rest:expr),*,) => {concat_array!($a, $($rest),*)};
}

const B0: u32 = 0;
const B3_1: u32 = 1;
const B3_2: u32 = 2;
const B3_3: u32 = 3;
const B3_4: u32 = 4;
const B3_5: u32 = 5;
const B3_6: u32 = 6;
const B3_7: u32 = 7;
const B4_1: u32 = 8;
const B4_2: u32 = 9;
const B4_3: u32 = 10;
const B4_4: u32 = 11;
const BUSH_IMAGE_NAMES: [&str; 12] = [
    "IMAGE_B0",
    "IMAGE_B3_1",
    "IMAGE_B3_2",
    "IMAGE_B3_3",
    "IMAGE_B3_4",
    "IMAGE_B3_5",
    "IMAGE_B3_6",
    "IMAGE_B3_7",
    "IMAGE_B4_1",
    "IMAGE_B4_2",
    "IMAGE_B4_3",
    "IMAGE_B4_4",
];

const BUSH_SIZES: [(f32, f32); 12] = [
    (329.0, 210.0),
    (86.0, 66.0),
    (137.0, 75.0),
    (110.0, 85.0),
    (198.0, 125.0),
    (74.0, 78.0),
    (118.0, 77.0),
    (84.0, 75.0),
    (203.0, 129.0),
    (228.0, 182.0),
    (194.0, 123.0),
    (193.0, 112.0),
];

#[rustfmt::skip]
fn affine_matrix(x: f32, y: f32, kx: f32, ky: f32, sx: f32, sy: f32, w: f32, h: f32) -> [f32; 9] {
    let tx = kx * PI / 180.0;
    let ty = ky * PI / 180.0;
    [ 
        sx * tx.cos(), -sy * ty.sin(), (w * sx * tx.cos() + h * -sy * ty.sin()) / 2.0 + x,
        sx * tx.sin(),  sy * ty.cos(), (w * sx * tx.sin() + h *  sy * ty.cos()) / 2.0 + y,
                  0.0,            0.0, 1.0,
    ]
}

static mut DAY_BUSHES_PTR: u32 = 0;
static mut BUSH_IMAGES_PTR: u32 = 0;

#[rustfmt::skip]
struct Bush {x: f32, y: f32, kx: f32, ky: f32, sx: f32, sy: f32, v: u32, z5: u32, z6: u32}

#[rustfmt::skip]
const BUSHES3_DATA: [Bush; 15] = [
    Bush {x: 8.1, y: -1.0, kx: 0.0, ky: 0.0, sx: 0.996, sy: 1.035, 
        v: B0, z5: 300_002, z6: 300_002},
    Bush {x: 27.5, y: -1.0, kx: 350.5, ky: 350.5, sx: 0.784, sy: 0.732, 
        v: B3_4, z5: 399_940, z6: 399_940},
    Bush {x: 156.8, y: -23.5, kx: 19.8, ky: 21.8, sx: 1.086, sy: 1.126, 
        v: B3_6, z5: 399_941, z6: 399_941},
    Bush {x: 213.9, y: 34.4, kx: 22.2, ky: 24.4, sx: 0.919, sy: 0.792, 
        v: B3_2, z5: 399_942, z6: 399_942},
    Bush {x: 148.1, y: 36.4, kx: 0.0, ky: 0.0, sx: 1.081, sy: 1.006, 
        v: B3_7, z5: 399_943, z6: 399_943},
    Bush {x: 66.0, y: 14.0, kx: 345.7, ky: 344.2, sx: 1.260, sy: 1.145, 
        v: B3_5, z5: 399_944, z6: 399_944},
    Bush {x: 7.3, y: 66.5, kx: 338.2, ky: 336.1, sx: 1.164, sy: 1.108, 
        v: B3_1, z5: 399_945, z6: 399_945},
    Bush {x: 67.5, y: 55.9, kx: 0.0, ky: 0.0, sx: 1.109, sy: 1.030, 
        v: B3_3, z5: 399_946, z6: 399_946},
    Bush {x: 177.8, y: 64.7, kx: 7.1, ky: 7.9, sx: 1.223, sy: 1.141, 
        v: B3_3, z5: 399_947, z6: 399_947},
    Bush {x: 325.6, y: 91.6, kx: 79.4, ky: 82.2, sx: 1.393, sy: 1.370, 
        v: B3_5, z5: 399_948, z6: 399_948},
    Bush {x: 10.6, y: 99.8, kx: 345.7, ky: 344.2, sx: 1.055, sy: 0.958, 
        v: B3_2, z5: 399_949, z6: 399_949},
    Bush {x: 148.1, y: 85.5, kx: 0.0, ky: 0.0, sx: 0.912, sy: 0.881, 
        v: B3_7, z5: 399_950, z6: 399_950},
    Bush {x: 135.5, y: 121.3, kx: 0.0, ky: 0.0, sx: 1.087, sy: 1.043, 
        v: B3_2, z5: 399_951, z6: 399_951},
    Bush {x: 186.0, y: 139.0, kx: 0.0, ky: 0.0, sx: 0.951, sy: 0.845, 
        v: B3_7, z5: 399_952, z6: 399_952},
    Bush {x: 37.9, y: 105.4, kx: 356.7, ky: 356.3, sx: 1.417, sy: 1.346, 
        v: B3_5, z5: 399_953, z6: 399_953},
];

pub unsafe fn patch_bush() -> Result<(), Box<dyn Error>> {
    const STRING_ASSIGN_FN_PTR: u64 = 0x404330;
    const MEMORY_IMAGE_FN_PTR: u64 = 0x59A980;
    const SHAREDIMAGEREF_DESTRUCTOR_FN_PTR: u64 = 0x59A8C0;
    const FREE_FN_PTR: u64 = 0x61C19A;

    BUSH_IMAGES_PTR = alloc_mem(96, PAGE_READWRITE) as u32;
    // Load bush images (Sexy::ExtractLoadingImagesResources)
    let mut code = CodeAssembler::new(32)?;
    for i in 0..BUSH_IMAGE_NAMES.len() {
        let mut no_destructor = code.create_label();
        let name = BUSH_IMAGE_NAMES[i];
        let string_ptr = alloc_mem(name.len(), PAGE_READWRITE) as u32;
        patch(string_ptr, name.as_bytes());
        code.push(name.len() as u32)?;
        code.push(string_ptr)?;
        code.lea(ecx, ptr(ebp - 0x44))?;
        code.mov(dword_ptr(ebp - 0x2C), ebx)?;
        code.mov(dword_ptr(ebp - 0x30), 0x0)?;
        code.mov(byte_ptr(ebp - 0x40), 0x0)?;
        code.call(STRING_ASSIGN_FN_PTR)?;
        code.lea(eax, ptr(ebp - 0x44))?;
        code.mov(byte_ptr(ebp - 0x4), 0x2)?;
        code.mov(edx, dword_ptr(edi))?;
        code.mov(edx, dword_ptr(edx + 0x40))?;
        code.push(eax)?;
        code.lea(ecx, ptr(ebp - 0x1C))?;
        code.push(ecx)?;
        code.mov(ecx, edi)?;
        code.call(edx)?;
        code.mov(ecx, eax)?;
        code.call(MEMORY_IMAGE_FN_PTR)?;
        code.lea(esi, ptr(ebp - 0x1C))?;
        code.mov(dword_ptr(BUSH_IMAGES_PTR + 4 * i as u32), eax)?;
        code.call(SHAREDIMAGEREF_DESTRUCTOR_FN_PTR)?;
        code.mov(byte_ptr(ebp - 0x4), 0x0)?;
        code.cmp(dword_ptr(ebp - 0x2C), 0x10)?;
        code.jc(no_destructor)?;
        code.mov(eax, dword_ptr(ebp - 0x40))?;
        code.push(eax)?;
        code.call(FREE_FN_PTR)?;
        code.add(esp, 0x4)?;
        code.set_label(&mut no_destructor)?;
    }
    code.push(0x66A014)?;
    code.jmp(0x4780F4)?;
    inject(0x4780EF, code);

    let all_bush_data = concat_array!(BUSHES3_DATA);
    DAY_BUSHES_PTR = alloc_mem(40 * all_bush_data.len(), PAGE_READWRITE) as u32;
    {
        let mut i = 0;
        for bush in BUSHES3_DATA {
            patch(
                DAY_BUSHES_PTR + 40 * i,
                &transmute::<u32, [u8; 4]>(BUSH_IMAGES_PTR + 4 * bush.v),
            );
            patch(
                DAY_BUSHES_PTR + 40 * i + 4,
                &transmute::<[f32; 9], [u8; 36]>(affine_matrix(
                    bush.x + 200.0,
                    bush.y + 200.0,
                    bush.kx,
                    bush.ky,
                    bush.sx,
                    bush.sy,
                    BUSH_SIZES[bush.v as usize].0,
                    BUSH_SIZES[bush.v as usize].1,
                )),
            );
            i += 1;
        }
    }

    // Load IMAGE_POLE (Sexy::ExtractDelayLoad_Background5Resources)
    let storage_ptr = alloc_mem(0x100, PAGE_READWRITE) as u32;
    const POLE_NAME: &str = "IMAGE_POLE";
    patch(storage_ptr + 0x20, POLE_NAME.as_bytes());
    let mut code = CodeAssembler::new(32)?;
    code.push(POLE_NAME.len() as u32)?;
    code.push(storage_ptr + 0x20)?;
    code.mov(ecx, storage_ptr)?;
    code.mov(dword_ptr(storage_ptr + 0x18), 0xf)?;
    code.mov(dword_ptr(storage_ptr + 0x14), 0)?;
    code.mov(byte_ptr(storage_ptr + 0x4), 0)?;
    code.call(0x404330)?;
    code.mov(edx, dword_ptr(edi))?;
    code.mov(edx, dword_ptr(edx + 0x40))?;
    code.push(storage_ptr)?;
    code.push(storage_ptr + 0x80)?;
    code.mov(ecx, edi)?;
    code.call(edx)?;
    code.mov(ecx, eax)?;
    code.call(0x59A990)?;
    code.mov(esi, storage_ptr + 0x80)?;
    code.mov(dword_ptr(POLE_PTR), eax)?;
    code.call(0x59A8D0)?;
    code.mov(al, 1)?;
    code.mov(ecx, dword_ptr(ebp - 0xC))?;
    code.jmp(0x475925)?;
    inject(0x475920, code);

    // Load IMAGE_POLE_NIGHT (Sexy::ExtractDelayLoad_Background6Resources)
    let storage_ptr = alloc_mem(0x100, PAGE_READWRITE) as u32;
    const POLE_NIGHT_NAME: &str = "IMAGE_POLE_NIGHT";
    patch(storage_ptr + 0x20, POLE_NIGHT_NAME.as_bytes());
    let mut code = CodeAssembler::new(32)?;
    code.push(POLE_NIGHT_NAME.len() as u32)?;
    code.push(storage_ptr + 0x20)?;
    code.mov(ecx, storage_ptr)?;
    code.mov(dword_ptr(storage_ptr + 0x18), 0xf)?;
    code.mov(dword_ptr(storage_ptr + 0x14), 0)?;
    code.mov(byte_ptr(storage_ptr + 0x4), 0)?;
    code.call(0x404330)?;
    code.mov(edx, dword_ptr(edi))?;
    code.mov(edx, dword_ptr(edx + 0x40))?;
    code.push(storage_ptr)?;
    code.push(storage_ptr + 0x80)?;
    code.mov(ecx, edi)?;
    code.call(edx)?;
    code.mov(ecx, eax)?;
    code.call(0x59A990)?;
    code.mov(esi, storage_ptr + 0x80)?;
    code.mov(dword_ptr(POLE_NIGHT_PTR), eax)?;
    code.call(0x59A8D0)?;
    code.mov(al, 1)?;
    code.mov(ecx, dword_ptr(ebp - 0xC))?;
    code.jmp(0x475A35)?;
    inject(0x475A30, code);

    // Put Obstruction pole on draw queue (Board::DrawGameObjects)
    let mut code = CodeAssembler::new(32)?;
    let mut endif = code.create_label();
    let mut case_bg_1 = code.create_label();
    let mut case_bg_5 = code.create_label();
    let mut case_bg_6 = code.create_label();
    code.mov(esi, dword_ptr(esp + 0x14))?;
    code.mov(esi, dword_ptr(esi + 0x554C))?;
    code.cmp(esi, 0)?;
    code.je(case_bg_1)?;
    code.cmp(esi, 4)?;
    code.je(case_bg_5)?;
    code.cmp(esi, 5)?;
    code.je(case_bg_6)?;
    code.mov(esi, dword_ptr(esp + 0x4))?;
    code.jmp(endif)?;
    code.set_label(&mut case_bg_1)?;
    code.mov(esi, dword_ptr(esp + 0x4))?;
    for i in 0..all_bush_data.len() {
        code.mov(dword_ptr(esi + 12 * i), 0x19)?;
        code.mov(dword_ptr(esi + 12 * i + 0x4), all_bush_data[i].z5)?;
        code.mov(
            dword_ptr(esi + 12 * i + 0x8),
            DAY_BUSHES_PTR + 40 * i as u32,
        )?;
    }
    code.add(esi, (12 * all_bush_data.len()) as u32)?;
    code.add(ecx, all_bush_data.len() as u32)?;
    code.add(edi, all_bush_data.len() as u32)?;
    code.mov(dword_ptr(esp + 0x4), esi)?;
    code.mov(dword_ptr(esp + 0x8), edi)?;
    code.jmp(endif)?;
    code.set_label(&mut case_bg_5)?;
    code.mov(esi, dword_ptr(esp + 0x4))?;
    code.mov(dword_ptr(esi), 0x19)?;
    code.mov(dword_ptr(esi + 0x4), 400_001)?;
    code.mov(dword_ptr(esi + 0x8), POLE_PTR)?;
    code.add(dword_ptr(esp + 0x4), 12)?;
    code.add(dword_ptr(esp + 0x8), 1)?;
    code.add(esi, 12)?;
    code.inc(ecx)?;
    code.inc(edi)?;
    code.jmp(endif)?;
    code.set_label(&mut case_bg_6)?;
    code.mov(esi, dword_ptr(esp + 0x4))?;
    code.mov(dword_ptr(esi), 0x19)?;
    code.mov(dword_ptr(esi + 0x4), 400_001)?;
    code.mov(dword_ptr(esi + 0x8), POLE_NIGHT_PTR)?;
    code.add(dword_ptr(esp + 0x4), 0xC)?;
    code.add(dword_ptr(esp + 0x8), 1)?;
    code.add(esi, 0xC)?;
    code.inc(ecx)?;
    code.inc(edi)?;
    code.set_label(&mut endif)?;
    code.call(0x41E840)?;
    code.jmp(0x416F7F)?;
    inject(0x416F7A, code);

    // Inject drawing function for obstruction pole (Board::DrawGameObjects)
    let canvas_rect_ptr = alloc_mem(16, PAGE_READWRITE) as u32;
    patch(
        canvas_rect_ptr,
        &transmute::<[u32; 4], [u8; 16]>([0, 0, 800, 600]),
    );
    let mut code = CodeAssembler::new(32)?;
    let mut not_bush = code.create_label();
    code.mov(eax, dword_ptr(ebx - 0x8))?;
    code.cmp(eax, 0x19)?;
    code.jne(not_bush)?;
    code.pushad()?;

    code.mov(ebx, dword_ptr(ebx))?;
    code.mov(eax, dword_ptr(ebx))?;
    code.mov(eax, dword_ptr(eax))?;
    code.push(dword_ptr(eax + 0x28))?;
    code.push(dword_ptr(eax + 0x24))?;
    code.push(0)?;
    code.push(0)?;

    code.mov(edi, dword_ptr(ebp + 0xC))?;
    code.push(dword_ptr(edi + 0x44))?;
    code.push(0x72289C)?;
    code.add(edi, 0x20)?;
    code.push(edi)?;
    // code.push(canvas_rect_ptr)?;
    code.lea(edi, ptr(ebx + 4))?;
    code.push(edi)?;

    code.mov(esi, dword_ptr(ebp + 0xC))?;

    // code.mov(eax, dword_ptr(0x6a74e0))?;
    code.lea(ebx, ptr(esp + 0x10))?;
    code.call(0x512650)?;
    code.add(esp, 0x20)?;

    code.popad()?;
    code.set_label(&mut not_bush)?;
    code.cmp(eax, 0x18)?;
    code.jmp(0x416FA6)?;
    inject(0x416FA0, code);

    Ok(())
}
