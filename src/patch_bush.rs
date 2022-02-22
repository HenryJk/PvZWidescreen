use core::intrinsics::transmute;

use std::error::Error;

use iced_x86::code_asm::*;
use winapi::um::winnt::PAGE_READWRITE;

use crate::{
    memory::{alloc_mem, inject, patch},
    POLE_NIGHT_PTR, POLE_PTR,
};

const STRING_ASSIGN_FN_PTR: u64 = 0x404330;
const MEMORY_IMAGE_FN_PTR: u64 = 0x59A980;
const SHAREDIMAGEREF_DESTRUCTOR_FN_PTR: u64 = 0x59A8C0;
const FREE_FN_PTR: u64 = 0x61C19A;

const BUSH_DAY_NAMES: [&str; 5] = [
    "IMAGE_BD0",
    "IMAGE_BD1",
    "IMAGE_BD2",
    "IMAGE_BD3",
    "IMAGE_BD4",
];

const BUSH_NIGHT_NAMES: [&str; 5] = [
    "IMAGE_BN0",
    "IMAGE_BN1",
    "IMAGE_BN2",
    "IMAGE_BN3",
    "IMAGE_BN4",
];

const BUSH_POOL_NAMES: [&str; 6] = [
    "IMAGE_BP0",
    "IMAGE_BP1",
    "IMAGE_BP2",
    "IMAGE_BP3",
    "IMAGE_BP4",
    "IMAGE_BP5",
];

const BUSH_FOG_NAMES: [&str; 6] = [
    "IMAGE_BF0",
    "IMAGE_BF1",
    "IMAGE_BF2",
    "IMAGE_BF3",
    "IMAGE_BF4",
    "IMAGE_BF5",
];

const BUSH_ROOF_NAMES: [&str; 1] = ["IMAGE_POLE"];

const BUSH_MOON_NAMES: [&str; 1] = ["IMAGE_POLE_NIGHT"];

const BUSH_BASE_NAMES: [&str; 2] = ["IMAGE_B_BASE", "IMAGE_BN_BASE"];

const BASE_BUSH_RENDER_LAYER: u32 = 300_001;

unsafe fn inject_delay_load(
    names: &[&str],
    container_ptr: u32,
    injection_site: u32,
) -> Result<(), Box<dyn Error>> {
    let mut code = CodeAssembler::new(32)?;
    for i in 0..names.len() {
        let name = names[i];
        let storage_ptr = alloc_mem(0x100, PAGE_READWRITE) as u32;
        patch(storage_ptr + 0x20, name.as_bytes());
        patch(
            container_ptr + 8 * i as u32 + 4,
            &transmute::<i32, [u8; 4]>(642),
        );
        code.push(name.len() as u32)?;
        code.push(storage_ptr + 0x20)?;
        code.mov(ecx, storage_ptr)?;
        code.mov(dword_ptr(storage_ptr + 0x18), 0xf)?;
        code.mov(dword_ptr(storage_ptr + 0x14), 0)?;
        code.mov(byte_ptr(storage_ptr + 0x4), 0)?;
        code.call(STRING_ASSIGN_FN_PTR)?;
        code.mov(edx, dword_ptr(edi))?;
        code.mov(edx, dword_ptr(edx + 0x40))?;
        code.push(storage_ptr)?;
        code.push(storage_ptr + 0x80)?;
        code.mov(ecx, edi)?;
        code.call(edx)?;
        code.mov(ecx, eax)?;
        code.call(MEMORY_IMAGE_FN_PTR)?;
        code.mov(esi, storage_ptr + 0x80)?;
        code.mov(dword_ptr(container_ptr + 8 * i as u32), eax)?;
        code.call(SHAREDIMAGEREF_DESTRUCTOR_FN_PTR)?;
    }
    code.mov(al, 0x1)?;
    code.mov(ecx, dword_ptr(ebp - 0xC))?;
    code.jmp(injection_site as u64 + 5)?;
    inject(injection_site, code);

    Ok(())
}

pub unsafe fn patch_bush() -> Result<(), Box<dyn Error>> {
    let day_bushes_ptr = alloc_mem(8 * BUSH_DAY_NAMES.len(), PAGE_READWRITE) as u32;
    inject_delay_load(&BUSH_DAY_NAMES, day_bushes_ptr, 0x47530A)?;
    let night_bushes_ptr = alloc_mem(8 * BUSH_NIGHT_NAMES.len(), PAGE_READWRITE) as u32;
    inject_delay_load(&BUSH_NIGHT_NAMES, night_bushes_ptr, 0x47547A)?;
    let pool_bushes_ptr = alloc_mem(8 * BUSH_POOL_NAMES.len(), PAGE_READWRITE) as u32;
    inject_delay_load(&BUSH_POOL_NAMES, pool_bushes_ptr, 0x4755EA)?;
    let fog_bushes_ptr = alloc_mem(8 * BUSH_FOG_NAMES.len(), PAGE_READWRITE) as u32;
    inject_delay_load(&BUSH_FOG_NAMES, fog_bushes_ptr, 0x47580E)?;

    inject_delay_load(&BUSH_ROOF_NAMES, POLE_PTR, 0x475920)?;
    inject_delay_load(&BUSH_MOON_NAMES, POLE_NIGHT_PTR, 0x475A30)?;

    // Load bush base images (Sexy::ExtractLoadingImagesResources)
    let bush_base_ptr = alloc_mem(16, PAGE_READWRITE) as u32;
    let mut code = CodeAssembler::new(32)?;
    for i in 0..BUSH_BASE_NAMES.len() {
        let mut no_destructor = code.create_label();
        let name = BUSH_BASE_NAMES[i];
        let string_ptr = alloc_mem(name.len(), PAGE_READWRITE) as u32;
        patch(string_ptr, name.as_bytes());
        patch(
            bush_base_ptr + 8 * i as u32 + 4,
            &transmute::<i32, [u8; 4]>(642),
        );
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
        code.mov(dword_ptr(bush_base_ptr + 8 * i as u32), eax)?;
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

    // Put Obstruction pole on draw queue (Board::DrawGameObjects)
    let mut code = CodeAssembler::new(32)?;
    let mut endif = code.create_label();
    let mut case_bg_1 = code.create_label();
    let mut case_bg_2 = code.create_label();
    let mut case_bg_3 = code.create_label();
    let mut case_bg_4 = code.create_label();
    let mut case_bg_5 = code.create_label();
    let mut case_bg_6 = code.create_label();
    code.mov(esi, dword_ptr(esp + 0x14))?;
    code.mov(esi, dword_ptr(esi + 0x554C))?;
    code.cmp(esi, 0)?;
    code.je(case_bg_1)?;
    code.cmp(esi, 1)?;
    code.je(case_bg_2)?;
    code.cmp(esi, 2)?;
    code.je(case_bg_3)?;
    code.cmp(esi, 3)?;
    code.je(case_bg_4)?;
    code.cmp(esi, 4)?;
    code.je(case_bg_5)?;
    code.cmp(esi, 5)?;
    code.je(case_bg_6)?;
    code.mov(esi, dword_ptr(esp + 0x4))?;
    code.jmp(endif)?;
    code.set_label(&mut case_bg_1)?;
    code.mov(esi, dword_ptr(esp + 0x4))?;
    for i in 0..BUSH_DAY_NAMES.len() as u32 {
        code.mov(dword_ptr(esi + 12 * i), 0x19)?;
        code.mov(dword_ptr(esi + 12 * i + 0x4), 310_999 + 10_000 * i)?;
        code.mov(dword_ptr(esi + 12 * i + 0x8), day_bushes_ptr + 8 * i)?;
    }
    code.mov(dword_ptr(esi + 12 * BUSH_DAY_NAMES.len() as u32), 0x19)?;
    code.mov(
        dword_ptr(esi + 12 * BUSH_DAY_NAMES.len() as u32 + 0x4),
        BASE_BUSH_RENDER_LAYER,
    )?;
    code.mov(
        dword_ptr(esi + 12 * BUSH_DAY_NAMES.len() as u32 + 0x8),
        bush_base_ptr,
    )?;
    code.add(esi, 12 * (BUSH_DAY_NAMES.len() as u32 + 1))?;
    code.add(ecx, BUSH_DAY_NAMES.len() as u32 + 1)?;
    code.add(edi, BUSH_DAY_NAMES.len() as u32 + 1)?;
    code.mov(dword_ptr(esp + 0x4), esi)?;
    code.mov(dword_ptr(esp + 0x8), edi)?;
    code.jmp(endif)?;
    code.set_label(&mut case_bg_2)?;
    code.mov(esi, dword_ptr(esp + 0x4))?;
    for i in 0..BUSH_NIGHT_NAMES.len() as u32 {
        code.mov(dword_ptr(esi + 12 * i), 0x19)?;
        code.mov(dword_ptr(esi + 12 * i + 0x4), 310_999 + 10_000 * i)?;
        code.mov(dword_ptr(esi + 12 * i + 0x8), night_bushes_ptr + 8 * i)?;
    }
    code.mov(dword_ptr(esi + 12 * BUSH_NIGHT_NAMES.len() as u32), 0x19)?;
    code.mov(
        dword_ptr(esi + 12 * BUSH_NIGHT_NAMES.len() as u32 + 0x4),
        BASE_BUSH_RENDER_LAYER,
    )?;
    code.mov(
        dword_ptr(esi + 12 * BUSH_NIGHT_NAMES.len() as u32 + 0x8),
        bush_base_ptr + 8,
    )?;
    code.add(esi, 12 * (BUSH_NIGHT_NAMES.len() as u32 + 1))?;
    code.add(ecx, BUSH_NIGHT_NAMES.len() as u32 + 1)?;
    code.add(edi, BUSH_NIGHT_NAMES.len() as u32 + 1)?;
    code.mov(dword_ptr(esp + 0x4), esi)?;
    code.mov(dword_ptr(esp + 0x8), edi)?;
    code.jmp(endif)?;
    code.set_label(&mut case_bg_3)?;
    code.mov(esi, dword_ptr(esp + 0x4))?;
    for i in 0..BUSH_POOL_NAMES.len() as u32 {
        code.mov(dword_ptr(esi + 12 * i), 0x19)?;
        code.mov(dword_ptr(esi + 12 * i + 0x4), 310_999 + 10_000 * i)?;
        code.mov(dword_ptr(esi + 12 * i + 0x8), pool_bushes_ptr + 8 * i)?;
    }
    code.mov(dword_ptr(esi + 12 * BUSH_POOL_NAMES.len() as u32), 0x19)?;
    code.mov(
        dword_ptr(esi + 12 * BUSH_POOL_NAMES.len() as u32 + 0x4),
        BASE_BUSH_RENDER_LAYER,
    )?;
    code.mov(
        dword_ptr(esi + 12 * BUSH_POOL_NAMES.len() as u32 + 0x8),
        bush_base_ptr,
    )?;
    code.add(esi, 12 * (BUSH_POOL_NAMES.len() as u32 + 1))?;
    code.add(ecx, BUSH_POOL_NAMES.len() as u32 + 1)?;
    code.add(edi, BUSH_POOL_NAMES.len() as u32 + 1)?;
    code.mov(dword_ptr(esp + 0x4), esi)?;
    code.mov(dword_ptr(esp + 0x8), edi)?;
    code.jmp(endif)?;
    code.set_label(&mut case_bg_4)?;
    code.mov(esi, dword_ptr(esp + 0x4))?;
    for i in 0..BUSH_FOG_NAMES.len() as u32 {
        code.mov(dword_ptr(esi + 12 * i), 0x19)?;
        code.mov(dword_ptr(esi + 12 * i + 0x4), 310_999 + 10_000 * i)?;
        code.mov(dword_ptr(esi + 12 * i + 0x8), fog_bushes_ptr + 8 * i)?;
    }
    code.mov(dword_ptr(esi + 12 * BUSH_FOG_NAMES.len() as u32), 0x19)?;
    code.mov(
        dword_ptr(esi + 12 * BUSH_FOG_NAMES.len() as u32 + 0x4),
        BASE_BUSH_RENDER_LAYER,
    )?;
    code.mov(
        dword_ptr(esi + 12 * BUSH_FOG_NAMES.len() as u32 + 0x8),
        bush_base_ptr + 8,
    )?;
    code.add(esi, 12 * (BUSH_FOG_NAMES.len() as u32 + 1))?;
    code.add(ecx, BUSH_FOG_NAMES.len() as u32 + 1)?;
    code.add(edi, BUSH_FOG_NAMES.len() as u32 + 1)?;
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
    let mut code = CodeAssembler::new(32)?;
    let mut not_draw_pole = code.create_label();
    code.mov(eax, dword_ptr(ebx - 0x8))?;
    code.cmp(eax, 0x19)?;
    code.jne(not_draw_pole)?;
    code.pushad()?;
    code.mov(ebx, dword_ptr(ebx))?;
    code.push(0)?;
    code.push(dword_ptr(ebx + 0x4))?;
    code.mov(eax, dword_ptr(ebp + 0xC))?;
    code.mov(ebx, dword_ptr(ebx))?;
    code.call(0x587150)?;
    code.popad()?;
    code.set_label(&mut not_draw_pole)?;
    code.cmp(eax, 0x18)?;
    code.jmp(0x416FA6)?;
    inject(0x416FA0, code);

    Ok(())
}
