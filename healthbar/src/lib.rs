use core::arch::asm;

use std::error::Error;

use config::Config;
use iced_x86::code_asm::*;
use winapi::um::winnt::DLL_PROCESS_ATTACH;

mod config;
mod memory;

use memory::inject;

#[derive(PartialEq, Clone, Copy)]
enum HealthBarVisibility {
    Never,
    Damaged,
    Always,
}

#[derive(PartialEq, Clone, Copy)]
enum HealthBarColor {
    Default,
    Teamcolor,
}

static mut HB_VISIBILITY: HealthBarVisibility = HealthBarVisibility::Damaged;
static mut HB_COLOR: HealthBarColor = HealthBarColor::Default;
static mut PLANT_HB_VISIBLE: bool = true;
static mut ZOMBIE_HB_VISIBLE: bool = false;

#[inline]
fn fill_rect(this: &mut pvz::Graphics, color: pvz::Color, x: i32, y: i32, width: i32, height: i32) {
    let old_color = this.state.mColor;
    this.state.mColor = color;
    unsafe {
        asm!(
            "pushad",
            "push {4}",
            "push {3}",
            "push {2}",
            "push {1}",
            "mov eax, {0}",
            "mov edx, 0x586d50",
            "call edx",
            "popad",
            in(reg) this,
            in(reg) x,
            in(reg) y,
            in(reg) width,
            in(reg) height,
        )
    }
    this.state.mColor = old_color;
}

#[inline]
fn draw_health_bar(
    graphics: &mut pvz::Graphics,
    hb_color: pvz::Color,
    x: i32,
    y: i32,
    width: i32,
    max_width: i32,
) {
    let black = pvz::Color::new(0x000000FF);
    let dark_gray = pvz::Color::new(0x303030FF);
    fill_rect(graphics, black, x - 1, y - 1, max_width + 2, 5);
    fill_rect(graphics, hb_color, x, y, width, 3);
    fill_rect(graphics, dark_gray, x + width, y, max_width - width, 3);
}

fn draw_plant_health_bar(graphics: &mut pvz::Graphics, plant: &pvz::Plant) {
    if plant.mDead || plant.mSquished {
        return;
    }

    if let HealthBarVisibility::Damaged = unsafe { HB_VISIBILITY } {
        if plant.mPlantHealth == plant.mPlantMaxHealth {
            return;
        }
    }

    let hb_color = if let HealthBarColor::Default = unsafe { HB_COLOR } {
        calc_health_color(plant.mPlantHealth as f32 / plant.mPlantMaxHealth as f32)
    } else {
        pvz::Color::new(0x008FC7FF)
    };

    let max_hb = if let pvz::SeedType::Cobcannon = plant.mSeedType {
        140
    } else {
        60
    };
    let hb_length = max_hb * plant.mPlantHealth / plant.mPlantMaxHealth;

    let extra_y = match plant.mSeedType {
        pvz::SeedType::Spikeweed
        | pvz::SeedType::Spikerock
        | pvz::SeedType::Flowerpot
        | pvz::SeedType::Lilypad => 50,
        pvz::SeedType::Tanglekelp | pvz::SeedType::Pumpkinshell => 35,
        _ => 0,
    };

    draw_health_bar(
        graphics,
        hb_color,
        plant.base.mX + 10,
        plant.base.mY + extra_y,
        hb_length,
        max_hb,
    );
}

fn calc_health_color(hp_fraction: f32) -> pvz::Color {
    pvz::Color {
        mRed: (255.0 * (1.0 - hp_fraction).sqrt()) as i32,
        mGreen: (255.0 * hp_fraction.sqrt()) as i32,
        mBlue: 0,
        mAlpha: 255,
    }
}

fn draw_zombie_health_bar(graphics: &mut pvz::Graphics, zombie: &pvz::Zombie) {
    if zombie.mDead || zombie.mBlowingAway || zombie.mFlatTires || zombie.mBodyHealth == 0 {
        return;
    }
    if let pvz::ZombieType::Boss = zombie.mZombieType {
        return;
    }
    if let HealthBarVisibility::Damaged = unsafe { HB_VISIBILITY } {
        if zombie.mBodyHealth == zombie.mBodyMaxHealth
            && zombie.mHelmHealth == zombie.mHelmMaxHealth
            && zombie.mShieldHealth == zombie.mShieldMaxHealth
        {
            return;
        }
    }

    let max_body_health = zombie.mBodyMaxHealth + zombie.mHelmMaxHealth;
    let body_health = zombie.mBodyHealth + zombie.mHelmHealth;
    let max_health = max_body_health + zombie.mShieldMaxHealth;
    let hb_length = zombie.mZombieRect.mWidth + 20;
    let shield_hb_length: i32 = hb_length * zombie.mShieldMaxHealth / max_health - 1;
    let body_hb_length: i32 = hb_length - shield_hb_length - 1;

    let extra_y = match zombie.mZombieType {
        pvz::ZombieType::Imp => 35,
        _ => 0,
    };

    let hb_color = if let HealthBarColor::Default = unsafe { HB_COLOR } {
        calc_health_color(body_health as f32 / max_body_health as f32)
    } else {
        pvz::Color::new(0xA10B0BFF)
    };

    let body_hb_current = body_health * body_hb_length / max_body_health;
    draw_health_bar(
        graphics,
        hb_color,
        zombie.base.mX + zombie.mZombieRect.mX - 10,
        zombie.base.mY + zombie.mZombieRect.mY + extra_y - zombie.mAltitude as i32,
        body_hb_current,
        body_hb_length,
    );
    if zombie.mShieldMaxHealth == 0 {
        return;
    }
    let shield_hb_current = zombie.mShieldHealth * shield_hb_length / zombie.mShieldMaxHealth;
    draw_health_bar(
        graphics,
        pvz::Color::new(0xFFFFFFFF),
        zombie.base.mX + zombie.mZombieRect.mX - 9 + body_hb_length,
        zombie.base.mY + zombie.mZombieRect.mY + extra_y - zombie.mAltitude as i32,
        shield_hb_current,
        shield_hb_length,
    );
}

#[no_mangle]
unsafe extern "cdecl" fn draw_all_health_bar(board: &pvz::Board, graphics: &mut pvz::Graphics) {
    if let HealthBarVisibility::Never = HB_VISIBILITY {
        return;
    }

    if PLANT_HB_VISIBLE {
        for plant in &board.mPlants {
            draw_plant_health_bar(graphics, plant);
        }
    }

    if ZOMBIE_HB_VISIBLE {
        for zombie in &board.mZombies {
            draw_zombie_health_bar(graphics, zombie);
        }
    }
}

fn onboarddraw() -> Result<(), Box<dyn Error>> {
    let mut code = CodeAssembler::new(32)?;
    code.pushad()?;
    code.push(dword_ptr(esp + 40))?;
    code.push(dword_ptr(esp + 40))?;
    code.call(draw_all_health_bar as *const () as u64)?;
    code.add(esp, 8)?;
    code.popad()?;
    code.ret_1(8)?;

    unsafe { inject(0x417353, code) };
    Ok(())
}

#[no_mangle] // call it "DllMain" in the compiled DLL
#[allow(unused_variables)]
pub extern "stdcall" fn DllMain(
    hinst_dll: winapi::shared::minwindef::HINSTANCE,
    fdw_reason: winapi::shared::minwindef::DWORD,
    lpv_reserved: winapi::shared::minwindef::LPVOID,
) -> i32 {
    match fdw_reason {
        DLL_PROCESS_ATTACH => {
            let config = Config::get_config();
            if let Some(visibility) = config.visibility {
                unsafe {
                    if visibility == "Never" {
                        HB_VISIBILITY = HealthBarVisibility::Never;
                    } else if visibility == "Damaged" {
                        HB_VISIBILITY = HealthBarVisibility::Damaged;
                    } else if visibility == "Always" {
                        HB_VISIBILITY = HealthBarVisibility::Always;
                    }
                }
            }
            if let Some(color) = config.color {
                unsafe {
                    if color == "Default" {
                        HB_COLOR = HealthBarColor::Default;
                    } else if color == "Teamcolor" {
                        HB_COLOR = HealthBarColor::Teamcolor;
                    }
                }
            }
            if let Some(plant_hb_visible) = config.plant_hb_visible {
                unsafe {
                    PLANT_HB_VISIBLE = plant_hb_visible;
                }
            }

            if let Some(zombie_hb_visible) = config.zombie_hb_visible {
                unsafe {
                    ZOMBIE_HB_VISIBLE = zombie_hb_visible;
                }
            }

            onboarddraw().unwrap();
        }
        _ => {}
    }
    true as i32
}
