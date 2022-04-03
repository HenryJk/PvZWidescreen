// Swinging gargantuar: 510
// Biting dancer: 429

use core::cmp::min;

use std::cmp::max;
use std::collections::HashSet;
use std::error::Error;
use std::intrinsics::transmute;
use std::time::SystemTime;

use iced_x86::code_asm::*;
use rand::{seq::SliceRandom, thread_rng};
use winapi::um::winnt::DLL_PROCESS_ATTACH;

mod calculation;
mod memory;

use memory::{inject, patch};

const DEBUG: bool = false;
const SPEEDUP: bool = true;

use crate::calculation::calculate_zombie_movement;

// use crate::calculation::calculate_zombie_movement;

static mut SKIP_FRAME: i32 = 1;

// static mut PREDICTION: f32 = 0.0;
static mut LAST_LAUNCH_TICK: i32 = 0;
static mut LAST_SPAWN_TICK: i32 = i32::MAX;

static mut CURRENT_SICK_COB: Option<(i32, i32)> = None;

// const EXTRA_X: i32 = 114;

#[inline]
fn seed_bank_x(index: i32) -> i32 {
    114 + 51 * index
}

static mut ROUND_TICK: i32 = 0;
static mut ROUND_START_TIME: Option<SystemTime> = None;

static mut MODIFIED: bool = false;

static mut FRAME_COUNT: i32 = 0;
static mut CLOSEST_GARG: i32 = i32::MAX;
static mut OLDEST_GARG: i32 = 0;
static mut CLOSEST_DANCER: i32 = i32::MAX;

#[derive(Debug)]
enum Event {
    WaveSpawn(i32),
    CobLaunch(i32),
    WaveRecharging(i32),
}

static mut EVENT_LOG: Vec<Event> = Vec::new();

const ZOMBIE_SET_1: [pvz::ZombieType; 12] = [
    pvz::ZombieType::Normal,
    pvz::ZombieType::Flag,
    pvz::ZombieType::TrafficCone,
    pvz::ZombieType::Pail,
    pvz::ZombieType::Newspaper,
    pvz::ZombieType::Door,
    pvz::ZombieType::Football,
    pvz::ZombieType::Dancer,
    pvz::ZombieType::Snorkel,
    pvz::ZombieType::Zamboni,
    pvz::ZombieType::Digger,
    pvz::ZombieType::RedeyeGargantuar,
];

const ZOMBIE_SET_2: [pvz::ZombieType; 12] = [
    pvz::ZombieType::Normal,
    pvz::ZombieType::Flag,
    pvz::ZombieType::TrafficCone,
    pvz::ZombieType::Pail,
    pvz::ZombieType::Dancer,
    pvz::ZombieType::Snorkel,
    pvz::ZombieType::Zamboni,
    pvz::ZombieType::DolphinRider,
    pvz::ZombieType::Balloon,
    pvz::ZombieType::Ladder,
    pvz::ZombieType::Gargantuar,
    pvz::ZombieType::RedeyeGargantuar,
];

const ZOMBIE_SET_5: [pvz::ZombieType; 12] = [
    pvz::ZombieType::Normal,
    pvz::ZombieType::Flag,
    pvz::ZombieType::TrafficCone,
    pvz::ZombieType::Football,
    pvz::ZombieType::Dancer,
    pvz::ZombieType::Digger,
    pvz::ZombieType::Pogo,
    pvz::ZombieType::Yeti,
    pvz::ZombieType::Bungee,
    pvz::ZombieType::Ladder,
    pvz::ZombieType::Catapult,
    pvz::ZombieType::RedeyeGargantuar,
];

const ZOMBIE_SET_6: [pvz::ZombieType; 10] = [
    pvz::ZombieType::Normal,
    pvz::ZombieType::Flag,
    pvz::ZombieType::Polevaulter,
    pvz::ZombieType::Door,
    pvz::ZombieType::Dancer,
    pvz::ZombieType::DolphinRider,
    pvz::ZombieType::Digger,
    pvz::ZombieType::Pogo,
    pvz::ZombieType::Catapult,
    pvz::ZombieType::RedeyeGargantuar,
];

const ZOMBIE_SET_7: [pvz::ZombieType; 10] = [
    pvz::ZombieType::Normal,
    pvz::ZombieType::Flag,
    pvz::ZombieType::Polevaulter,
    pvz::ZombieType::Newspaper,
    pvz::ZombieType::Football,
    pvz::ZombieType::Dancer,
    pvz::ZombieType::Snorkel,
    pvz::ZombieType::DolphinRider,
    pvz::ZombieType::JackInTheBox,
    pvz::ZombieType::Digger,
];

static mut REMAINING_TOTAL_HEALTH: i32 = 0;
static mut FULL_TOTAL_HEALTH: i32 = 0;

static mut GARG_MAY_EXIST: bool = false;
static mut PUFF_MAY_EXIST: bool = false;

#[no_mangle]
unsafe extern "cdecl" fn should_draw() -> bool {
    FRAME_COUNT += 1;
    FRAME_COUNT % SKIP_FRAME == 0
}

static mut HISTORICAL_CLOSEST_GARG: i32 = i32::MAX;
static mut HISTORICAL_OLDEST_GARG: i32 = 0;

fn garg_heuristic(garg: &pvz::Zombie) -> f32 {
    if garg.mPosX < 540.0 {
        garg.mPosX - 540.0 - garg.mBodyHealth as f32
    } else {
        garg.mPosX
            - (0.2 * garg.mVelX * 4800.0 * garg.mBodyHealth as f32 / garg.mBodyMaxHealth as f32)
    }
}

#[no_mangle]
unsafe extern "cdecl" fn run(board: *mut pvz::Board) {
    let board = &mut *board;

    const PUFFSHROOM_INDEX: usize = 0;
    const NUFFSHROOM_INDEX: usize = 1;

    // const SPIKEWEED_INDEX: usize = 0;
    // const SPIKEROCK_INDEX: usize = 1;

    const PUMPKINSHELL_INDEX: usize = 2;

    const KERNELPULT_INDEX: usize = 3;
    const COBCANNON_INDEX: usize = 4;

    const BLOVER_INDEX: usize = 5;

    const INSTANT_COFFEE_INDEX: usize = 6;
    const LILYPAD_INDEX: usize = 7;
    const FUMESHROOM_INDEX: usize = 8;
    const GLOOMSHROOM_INDEX: usize = 9;

    ROUND_TICK += 1;
    if (*board.mCutScene).mSeedChoosing {
        if !MODIFIED {
            if SPEEDUP {
                patch(0x551DBF, &[0x90, 0x90]);
                pvz::LawnApp::instance().base.base.mFrameTime = 0;
                *(0x6A66F4 as *mut bool) = false;
                SKIP_FRAME = 17;
            }
            MODIFIED = true;
        }

        LAST_LAUNCH_TICK = 0;
        LAST_SPAWN_TICK = i32::MAX;

        if DEBUG {
            board.mZombiesInWave = [[pvz::ZombieType::Invalid; 50]; 100];
            board.mZombieAllowed = [false; 100];
            for zombie_type in ZOMBIE_SET_6 {
                board.mZombieAllowed[zombie_type as usize] = true;
            }
            board.PickZombieWaves();

            println!("closest garg: {}", CLOSEST_GARG);
            println!("oldest garg: {}", OLDEST_GARG);
            println!("closest dancer: {}", CLOSEST_DANCER);
            HISTORICAL_CLOSEST_GARG = min(HISTORICAL_CLOSEST_GARG, CLOSEST_GARG);
            HISTORICAL_OLDEST_GARG = max(HISTORICAL_OLDEST_GARG, OLDEST_GARG);
            println!("historical closest garg: {}", HISTORICAL_CLOSEST_GARG);
            println!("historical oldest garg: {}", HISTORICAL_OLDEST_GARG);
            CLOSEST_GARG = i32::MAX;
            OLDEST_GARG = 0;
            CLOSEST_DANCER = i32::MAX;
        }

        // board.mSunMoney = 9990;

        let seed_chooser_screen = &mut *pvz::LawnApp::instance().mSeedChooserScreen;

        seed_chooser_screen.ChooseSeed(pvz::SeedType::Puffshroom, pvz::SeedType::None);
        seed_chooser_screen.ChooseSeed(pvz::SeedType::Imitater, pvz::SeedType::Puffshroom);
        // seed_chooser_screen.ChooseSeed(pvz::SeedType::Spikeweed, pvz::SeedType::None);
        // seed_chooser_screen.ChooseSeed(pvz::SeedType::Spikerock, pvz::SeedType::None);
        seed_chooser_screen.ChooseSeed(pvz::SeedType::Pumpkinshell, pvz::SeedType::None);
        seed_chooser_screen.ChooseSeed(pvz::SeedType::Kernelpult, pvz::SeedType::None);
        seed_chooser_screen.ChooseSeed(pvz::SeedType::Cobcannon, pvz::SeedType::None);
        seed_chooser_screen.ChooseSeed(pvz::SeedType::Blover, pvz::SeedType::None);
        seed_chooser_screen.ChooseSeed(pvz::SeedType::InstantCoffee, pvz::SeedType::None);
        seed_chooser_screen.ChooseSeed(pvz::SeedType::Lilypad, pvz::SeedType::None);
        seed_chooser_screen.ChooseSeed(pvz::SeedType::Fumeshroom, pvz::SeedType::None);
        seed_chooser_screen.ChooseSeed(pvz::SeedType::Gloomshroom, pvz::SeedType::None);

        seed_chooser_screen.CloseSeedChooser();

        if let Some(now) = ROUND_START_TIME {
            let speedup = ROUND_TICK as f64 / (now.elapsed().unwrap().as_secs_f64() * 100.0);
            println!("Game speed: {:.2}x", speedup);
        }

        ROUND_TICK = 0;
        ROUND_START_TIME = Some(SystemTime::now());

        let mut zombie_exist = [false; 34];
        GARG_MAY_EXIST = false;
        for i in 0..20 {
            let wave = &board.mZombiesInWave[i];
            for zombie_type in wave {
                if *zombie_type as usize >= zombie_exist.len() {
                    continue;
                }
                if matches!(zombie_type, pvz::ZombieType::RedeyeGargantuar) {
                    GARG_MAY_EXIST = true;
                }
                zombie_exist[*zombie_type as usize] = true;
            }
        }

        let mut zombies = Vec::new();
        for i in 0..zombie_exist.len() {
            if zombie_exist[i] {
                zombies.push(transmute::<i32, pvz::ZombieType>(i as i32));
            }
        }
        println!("");
        println!("flag {}: {:?}", (*board.mChallenge).mSurvivalStage, zombies);

        EVENT_LOG.clear();
    }

    if board.mZombieCountDown == board.mZombieCountDownStart && board.mHugeWaveCountDown == 0 {
        LAST_SPAWN_TICK = ROUND_TICK;
        if DEBUG && board.mCurrentWave != 0 {
            EVENT_LOG.push(Event::WaveSpawn(ROUND_TICK));
        }
    }

    if DEBUG {
        if board.mZombieCountDown == 200 {
            EVENT_LOG.push(Event::WaveRecharging(ROUND_TICK));
        }

        if ROUND_TICK - LAST_LAUNCH_TICK == 373
            && (board.mZombieCountDown < 200 || ROUND_TICK - LAST_SPAWN_TICK < 373)
            && board.mCurrentWave > 0
        {
            println!(
                "early summon: launched at {} hp left ({}%), tick: {}, wave: {} at impact",
                REMAINING_TOTAL_HEALTH,
                REMAINING_TOTAL_HEALTH as f32 / FULL_TOTAL_HEALTH as f32 * 100.0,
                if board.mZombieCountDown < 200 {
                    -board.mZombieCountDown
                } else {
                    ROUND_TICK - LAST_SPAWN_TICK
                },
                if board.mZombieCountDown < 200 {
                    board.mCurrentWave + 1
                } else {
                    board.mCurrentWave
                },
            );
        }

        for zombie in &board.mZombies {
            match zombie.mZombieType {
                pvz::ZombieType::RedeyeGargantuar => {
                    CLOSEST_GARG = min(CLOSEST_GARG, zombie.base.mX);
                    OLDEST_GARG = max(OLDEST_GARG, zombie.mZombieAge);
                }
                pvz::ZombieType::BackupDancer => {
                    CLOSEST_DANCER = min(CLOSEST_DANCER, zombie.base.mX);
                }
                _ => {}
            }
        }

        if board
            .mPlants
            .into_iter()
            .any(|plant| match plant.mSeedType {
                pvz::SeedType::Cobcannon => plant.mSquished || plant.mPlantHealth < 20,
                pvz::SeedType::Gloomshroom => plant.mDead || plant.mPlantHealth < 20,
                _ => false,
            })
        {
            board.mPaused = true;
            println!("closest garg: {}", CLOSEST_GARG);
            println!("oldest garg: {}", OLDEST_GARG);
            println!("closest dancer: {}", CLOSEST_DANCER);
            for e in &EVENT_LOG {
                println!("{:?}", e);
            }
            println!("Current tick: {}", ROUND_TICK);
            SKIP_FRAME = 1;
            pvz::LawnApp::instance().base.base.mFrameTime = 10;
        }
    }

    let mut garg_exist = false;
    let mut free_gargs = Vec::new();
    if GARG_MAY_EXIST {
        for zombie in &board.mZombies {
            if matches!(zombie.mZombieType, pvz::ZombieType::RedeyeGargantuar)
                && !zombie.mDead
                && zombie.mBodyHealth > 0
            {
                garg_exist = true;
                if matches!(zombie.mZombiePhase, pvz::ZombiePhase::ZombieNormal) {
                    free_gargs.push(zombie);
                }
            }
        }
    }

    let buffer_locations = if PUFF_MAY_EXIST {
        board
            .mPlants
            .into_iter()
            .filter_map(|plant| {
                if plant.mDead || plant.mSquished || plant.mPlantHealth <= 0 {
                    None
                } else {
                    let col = plant.mPlantCol;
                    let row = plant.base.mRow;
                    if col >= 6 && row != 2 && row != 3 {
                        Some((col, row))
                    } else {
                        None
                    }
                }
            })
            .collect()
    } else {
        Vec::new()
    };

    if garg_exist {
        let mut puffshroom_locations: HashSet<(i32, i32)> = buffer_locations.into_iter().collect();

        free_gargs.shuffle(&mut thread_rng());
        free_gargs.sort_unstable_by(|g1, g2| {
            garg_heuristic(g1).partial_cmp(&garg_heuristic(g2)).unwrap()
        });

        let free_garg_positions: Vec<(f32, i32)> = free_gargs
            .into_iter()
            .map(|garg| (garg.mPosX, garg.base.mRow))
            .collect();
        for (pos_x, row) in free_garg_positions {
            if pos_x < 511.0 || pos_x > 740.0 {
                continue;
            }
            let col = (pos_x as i32 - 31) / 80;
            if board.mIceMinX[row as usize] <= col * 80 + 110
                || puffshroom_locations.contains(&(col, row))
            {
                continue;
            }
            if !(*board.mSeedBank).mSeedPackets[PUFFSHROOM_INDEX].mRefreshing {
                board.MouseDown(0, 0, -1);
                board.MouseDown(seed_bank_x(PUFFSHROOM_INDEX as i32), 43, 1);
                board.MouseDown(80 * col + 80, 85 * row + 142, 1);
                board.MouseDown(0, 0, -1);
                puffshroom_locations.insert((col, row));
                PUFF_MAY_EXIST = true;
            } else if !(*board.mSeedBank).mSeedPackets[NUFFSHROOM_INDEX].mRefreshing {
                board.MouseDown(0, 0, -1);
                board.MouseDown(seed_bank_x(NUFFSHROOM_INDEX as i32), 43, 1);
                board.MouseDown(80 * col + 80, 85 * row + 142, 1);
                board.MouseDown(0, 0, -1);
                puffshroom_locations.insert((col, row));
                PUFF_MAY_EXIST = true;
            } else if pos_x < 540.0 && ROUND_TICK - LAST_LAUNCH_TICK > 373 {
                for backup_seed in [
                    FUMESHROOM_INDEX,
                    KERNELPULT_INDEX,
                    BLOVER_INDEX,
                    PUMPKINSHELL_INDEX,
                ] {
                    if !(*board.mSeedBank).mSeedPackets[backup_seed].mRefreshing {
                        println!(
                            "\x1b[93m[WARNING]: {:?} USED AS GARGANTUAR SACRIFICE at row {}\x1b[0m",
                            (*board.mSeedBank).mSeedPackets[backup_seed].mPacketType,
                            row
                        );
                        board.MouseDown(0, 0, -1);
                        board.MouseDown(seed_bank_x(backup_seed as i32), 43, 1);
                        board.MouseDown(80 * col + 80, 85 * row + 142, 1);
                        board.MouseDown(0, 0, -1);
                        puffshroom_locations.insert((col, row));
                        PUFF_MAY_EXIST = true;
                        break;
                    }
                }
            }
        }
    } else {
        board.MouseDown(0, 0, -1);
        for (col, row) in buffer_locations {
            board.MouseDown(seed_bank_x(10), 43, 1);
            board.MouseDown(80 * col + 80, 85 * row + 142, 1);
            board.MouseDown(0, 0, -1);
            PUFF_MAY_EXIST = false;
        }
    }

    if ROUND_TICK - LAST_LAUNCH_TICK >= 869
        && ROUND_TICK - LAST_SPAWN_TICK >= 869 - 200 - 373
        && board.mZombieCountDown > 574
        && (board.TotalZombiesHealthInWave(board.mCurrentWave - 1)
            > board.mZombieHealthToNextWave + 1800
            || board.mCurrentWave % 10 == 9)
    {
        let mut target_x = i32::MAX;
        let mut dolphin_exist = false;
        for zombie in &board.mZombies {
            if !zombie.mHasHead || zombie.mBodyHealth <= 0 {
                continue;
            }
            if zombie.base.mRow == 2 || zombie.base.mRow == 3 {
                if matches!(zombie.mZombieType, pvz::ZombieType::DolphinRider) {
                    dolphin_exist = true;
                } else {
                    continue;
                }
            }
            if zombie.base.mX + zombie.mZombieRect.mX + zombie.mZombieRect.mWidth < 626 {
                continue;
            }
            match zombie.mZombieType {
                pvz::ZombieType::Dancer => {
                    target_x = 621;
                    break;
                }

                pvz::ZombieType::Imp
                | pvz::ZombieType::Balloon
                | pvz::ZombieType::Digger
                | pvz::ZombieType::Bungee => {}

                _ => {
                    target_x = min(
                        target_x,
                        zombie.base.mX + zombie.mZombieRect.mX + zombie.mZombieRect.mWidth / 2,
                    )
                }
            }
        }
        if target_x != i32::MAX {
            if dolphin_exist {
                target_x = min(target_x, 670);
            }
            target_x = max(target_x, 621);
            let targets: [(i32, i32); 2] = [(target_x, 227), (target_x, 482)];
            let mut i = 0;
            for plant in &mut board.mPlants {
                if !matches!(plant.mSeedType, pvz::SeedType::Cobcannon) {
                    continue;
                }
                if !matches!(plant.mState, pvz::PlantState::CobcannonReady) {
                    continue;
                }
                plant.CobCannonFire(targets[i].0, targets[i].1);
                i += 1;
                if i >= targets.len() {
                    break;
                }
            }
            LAST_LAUNCH_TICK = ROUND_TICK;
            EVENT_LOG.push(Event::CobLaunch(ROUND_TICK));
            REMAINING_TOTAL_HEALTH = board.TotalZombiesHealthInWave(board.mCurrentWave - 1)
                - board.mZombieHealthToNextWave;
            FULL_TOTAL_HEALTH = board.mZombieHealthWaveStart - board.mZombieHealthToNextWave;
        }
    }

    if !(*board.mSeedBank).mSeedPackets[INSTANT_COFFEE_INDEX].mRefreshing
        && !(*board.mSeedBank).mSeedPackets[FUMESHROOM_INDEX].mRefreshing
        && !(*board.mSeedBank).mSeedPackets[GLOOMSHROOM_INDEX].mRefreshing
    {
        const GLOOM_LOCATIONS: [(i32, i32); 2] = [(6, 2), (6, 3)];
        let mut gloom_exist = [false; GLOOM_LOCATIONS.len()];
        for plant in &board.mPlants {
            if matches!(plant.mSeedType, pvz::SeedType::Gloomshroom) {
                if let Some(idx) = GLOOM_LOCATIONS
                    .iter()
                    .position(|&pos| (plant.mPlantCol, plant.base.mRow) == pos)
                {
                    gloom_exist[idx] = true;
                }
            }
        }
        let missing_gloom_index = gloom_exist.iter().position(|&x| !x);
        if let Some(idx) = missing_gloom_index {
            let (col, row) = GLOOM_LOCATIONS[idx];
            println!("\x1b[93m[WARNING]: Gloom at row {} is dead\x1b[0m", row);
            if row == 2 || row == 3 {
                board.MouseDown(0, 0, -1);
                board.MouseDown(seed_bank_x(LILYPAD_INDEX as i32), 43, -1);
                board.MouseDown(col * 80 + 80, row * 85 + 142, 1);
                board.MouseDown(0, 0, -1);
            }
            board.MouseDown(0, 0, -1);
            board.MouseDown(seed_bank_x(FUMESHROOM_INDEX as i32), 43, 1);
            board.MouseDown(col * 80 + 80, row * 85 + 142, 1);
            board.MouseDown(0, 0, -1);
            board.MouseDown(seed_bank_x(GLOOMSHROOM_INDEX as i32), 43, 1);
            board.MouseDown(col * 80 + 80, row * 85 + 142, 1);
            board.MouseDown(0, 0, -1);
            board.MouseDown(seed_bank_x(INSTANT_COFFEE_INDEX as i32), 43, 1);
            board.MouseDown(col * 80 + 80, row * 85 + 142, 1);
            board.MouseDown(0, 0, -1);
        }
    }

    if !(*board.mSeedBank).mSeedPackets[PUMPKINSHELL_INDEX].mRefreshing {
        const PUMPKIN_LOCATIONS: [(i32, i32); 6] = [(6, 2), (6, 3), (5, 2), (5, 3), (4, 2), (4, 3)];
        let mut pumpkin_exist = [false; PUMPKIN_LOCATIONS.len()];

        let mut target = (-1, -1);
        let mut lowest_health = 2000;
        for plant in &board.mPlants {
            if matches!(plant.mSeedType, pvz::SeedType::Pumpkinshell) {
                if let Some(idx) = PUMPKIN_LOCATIONS
                    .iter()
                    .position(|&pos| (plant.mPlantCol, plant.base.mRow) == pos)
                {
                    pumpkin_exist[idx] = true;
                }
                if plant.mPlantHealth < lowest_health {
                    lowest_health = plant.mPlantHealth;
                    target = (plant.mPlantCol, plant.base.mRow);
                }
            }
        }
        let missing_pumpkin_index = pumpkin_exist.iter().position(|&x| !x);
        if let Some(idx) = missing_pumpkin_index {
            let (col, row) = PUMPKIN_LOCATIONS[idx];
            println!(
                "\x1b[93m[WARNING]: Pumpkin at row {} col {} is dead\x1b[0m",
                row, col
            );
            if row == 2 || row == 3 {
                board.MouseDown(0, 0, -1);
                board.MouseDown(seed_bank_x(LILYPAD_INDEX as i32), 0, -1);
                board.MouseDown(col * 80 + 80, row * 85 + 142, 1);
                board.MouseDown(0, 0, -1);
            }
            board.MouseDown(0, 0, -1);
            board.MouseDown(seed_bank_x(PUMPKINSHELL_INDEX as i32), 43, 1);
            board.MouseDown(col * 80 + 80, row * 85 + 142, 1);
        } else if target != (-1, -1) {
            let (col, row) = target;
            println!(
                "\x1b[92m[INFO]: Pumpkin at row {} col {} is damaged at {} hp, replacing now\x1b[0m",
                row, col, lowest_health
            );
            board.MouseDown(0, 0, -1);
            board.MouseDown(seed_bank_x(PUMPKINSHELL_INDEX as i32), 43, 1);
            board.MouseDown(col * 80 + 80, row * 85 + 142, 1);
        }
    }

    if ROUND_TICK % 100 == 0 {
        let mut balloon_almost_get_in = false;
        for zombie in &board.mZombies {
            if matches!(zombie.mZombieType, pvz::ZombieType::Balloon) && zombie.base.mX < 80 {
                balloon_almost_get_in = true;
                break;
            }
        }
        if balloon_almost_get_in {
            board.MouseDown(0, 0, -1);
            board.MouseDown(seed_bank_x(BLOVER_INDEX as i32), 43, 1);
            board.MouseDown(80, 142, 1);
        }
    }

    let mut targets = Vec::new();
    for coin in &board.mCoins {
        if !coin.mIsBeingCollected && coin.mHitGround {
            targets.push((
                coin.mPosX as i32 + coin.base.mWidth / 2,
                coin.mPosY as i32 + coin.base.mHeight / 2,
            ));
        }
    }
    board.MouseDown(0, 0, -1);
    for (x, y) in targets {
        board.MouseDown(x, y, 1);
    }

    if matches!(CURRENT_SICK_COB, None)
        && !(*board.mSeedBank).mSeedPackets[KERNELPULT_INDEX].mRefreshing
        && (!(*board.mSeedBank).mSeedPackets[COBCANNON_INDEX].mRefreshing
            || (*board.mSeedBank).mSeedPackets[COBCANNON_INDEX].mRefreshCounter > 5000 - 750)
    {
        let mut cob_exist = [false; 4];
        let mut worst_cob: Option<&pvz::Plant> = None;
        for plant in &mut board.mPlants {
            if matches!(plant.mSeedType, pvz::SeedType::Cobcannon)
                && plant.base.mRow != 2
                && plant.base.mRow != 3
                && !plant.mSquished
                && !plant.mDead
            {
                let row = plant.base.mRow;
                let idx = if row >= 2 { row - 2 } else { row } as usize;
                cob_exist[idx] = true;
                if let Some(cob) = worst_cob {
                    if plant.mPlantHealth < cob.mPlantHealth {
                        worst_cob = Some(plant);
                    }
                } else if plant.mPlantHealth < 300 {
                    worst_cob = Some(plant);
                }
            }
        }
        let missing_cob_idx = cob_exist.iter().position(|x| !x);
        if let Some(idx) = missing_cob_idx {
            let row = if idx > 1 { idx + 2 } else { idx } as i32;
            CURRENT_SICK_COB = Some((4, row));
            println!(
                "\x1b[93m[WARNING]: Cob cannon at row {} is dead\x1b[0m",
                row
            );
        } else if let Some(cob) = worst_cob {
            if matches!(cob.mState, pvz::PlantState::CobcannonArming) && cob.mStateCountdown > 1500
            {
                CURRENT_SICK_COB = Some((cob.mPlantCol, cob.base.mRow));
                println!(
                    "\x1b[92m[INFO]: Cob cannon at row {} is damaged at {} hp, replacing now\x1b[0m",
                    cob.base.mRow, cob.mPlantHealth
                );
            }
        }

        if let Some((col, row)) = CURRENT_SICK_COB {
            board.MouseDown(0, 0, -1);
            board.MouseDown(seed_bank_x(10), 43, 1);
            board.MouseDown(col * 80 + 80, row * 85 + 142, 1);
        }
    }

    if let Some((col, row)) = CURRENT_SICK_COB {
        let mut kp1 = false;
        let mut kp2 = false;
        for plant in &board.mPlants {
            if matches!(plant.mSeedType, pvz::SeedType::Kernelpult) {
                if plant.mPlantCol == col && plant.base.mRow == row {
                    kp1 = true;
                }
                if plant.mPlantCol == col + 1 && plant.base.mRow == row {
                    kp2 = true;
                }
                if kp1 && kp2 {
                    break;
                }
            }
        }
        if kp1 && kp2 && !(*board.mSeedBank).mSeedPackets[COBCANNON_INDEX].mRefreshing {
            board.MouseDown(0, 0, -1);
            board.MouseDown(seed_bank_x(COBCANNON_INDEX as i32), 43, 1);
            board.MouseDown(col * 80 + 80, row * 85 + 142, 1);
            CURRENT_SICK_COB = None;
        } else if kp1 && !(*board.mSeedBank).mSeedPackets[KERNELPULT_INDEX].mRefreshing {
            board.MouseDown(0, 0, -1);
            board.MouseDown(seed_bank_x(KERNELPULT_INDEX as i32), 43, 1);
            board.MouseDown(col * 80 + 160, row * 85 + 142, 1);
        } else if !(*board.mSeedBank).mSeedPackets[KERNELPULT_INDEX].mRefreshing {
            board.MouseDown(0, 0, -1);
            board.MouseDown(seed_bank_x(KERNELPULT_INDEX as i32), 43, 1);
            board.MouseDown(col * 80 + 80, row * 85 + 142, 1);
        }
    }
}

#[no_mangle]
unsafe extern "cdecl" fn handle_lose(zombie: *mut pvz::Zombie) {
    let zombie = &mut *zombie;
    println!("{:?} Zombie breached your defense", zombie.mZombieType);
}

fn onboardtick() -> Result<(), Box<dyn Error>> {
    unsafe {
        let mut code = CodeAssembler::new(32)?;
        code.pushad()?;
        code.push(ebx)?;
        code.call(run as *const () as u64)?;
        code.add(esp, 4)?;
        code.popad()?;
        code.sub(esp, 8)?;
        code.push(ebp)?;
        code.push(esi)?;
        code.jmp(0x4130D5)?;
        inject(0x4130D0, code);

        // patch(0x523509, &[0xB8, 0x38, 0x01, 0x00, 0x00]);
        // patch(0x522598, &[1]);
        // patch(0x551DBF, &[0x90, 0x90]);
        // patch(0x54412A, &[0x00]);

        let mut code = CodeAssembler::new(32)?;
        let mut draw = code.create_label();
        code.pushad()?;
        code.call(should_draw as *const () as u64)?;
        code.test(al, al)?;
        code.popad()?;
        code.jnz(draw)?;
        code.ret_1(4)?;
        code.set_label(&mut draw)?;
        code.mov(eax, fs)?;
        code.jmp(0x41ACF6)?;
        inject(0x41ACF0, code);

        patch(0x54EBA8, &[0xEB, 0x00]);
        patch(0x54B267, &[0x70]);
        patch(0x482149, &transmute::<i32, [u8; 4]>(0xC8B13 + 0x1B));

        let mut code = CodeAssembler::new(32)?;
        code.pushad()?;
        code.push(dword_ptr(esp + 0x28))?;
        code.call(handle_lose as *const () as u64)?;
        code.add(esp, 4)?;
        code.popad()?;
        code.push(ebp)?;
        code.mov(ebp, esp)?;
        code.and(esp, -8)?;
        code.jmp(0x413406)?;
        inject(0x413400, code);
    }
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
            onboardtick().unwrap();
        }
        _ => {}
    }
    true as i32
}
