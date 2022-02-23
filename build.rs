use core::intrinsics::transmute;

use std::{
    error::Error,
    fs::{self, File},
    io::Write,
    path::Path,
    time::SystemTime,
};

use walkdir::WalkDir;

const CYPHER_BYTE: u8 = 0xF7;
const BUILD_DIR: &str = "dist";
const RESOURCES_DIR: &str = "resources";
const NOT_PACKED: [&str; 6] = [
    r#"bass.dll"#,
    r#"eula.txt"#,
    r#"PlantsVsZombies.exe"#,
    r#"properties\partner.xml"#,
    r#"properties\partner.xml.sig"#,
    r#"properties\partner_logo.jpg"#,
];

fn is_included(path: &Path) -> bool {
    NOT_PACKED.into_iter().all(|p| p != path.to_str().unwrap())
}

fn encrypt(buf: &[u8]) -> Vec<u8> {
    let mut result = buf.to_vec();
    result.iter_mut().for_each(|x| *x ^= CYPHER_BYTE);
    result
}

fn main() -> Result<(), Box<dyn Error>> {
    fs::create_dir_all(BUILD_DIR)?;
    let main_pak_path = Path::new(BUILD_DIR).join("main.pak");
    let mut main_pak_file = File::create(main_pak_path)?;
    let magic_header = [0xC0, 0x4A, 0xC0, 0xBA, 0x00, 0x00, 0x00, 0x00];
    main_pak_file.write(&encrypt(&magic_header))?;

    for entry in WalkDir::new(RESOURCES_DIR)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        let filepath = entry.path().strip_prefix(RESOURCES_DIR)?;
        if entry.metadata()?.is_dir() || !is_included(filepath) {
            continue;
        }
        let filesize = entry.metadata()?.len() as u32;
        let filetime = entry.metadata()?.modified()?;

        const ENTRY_FLAG: u8 = 0x00;
        let filepath_buf = filepath.to_str().unwrap().as_bytes();

        main_pak_file.write(&encrypt(&[ENTRY_FLAG]))?;
        main_pak_file.write(&encrypt(&[filepath_buf.len() as u8]))?;
        main_pak_file.write(&encrypt(filepath_buf))?;
        main_pak_file.write(&encrypt(&unsafe { transmute::<u32, [u8; 4]>(filesize) }))?;
        main_pak_file.write(&encrypt(&unsafe {
            transmute::<SystemTime, [u8; 8]>(filetime)
        }))?;
    }

    const END_FLAG: u8 = 0x80;
    main_pak_file.write(&encrypt(&[END_FLAG]))?;

    for entry in WalkDir::new(RESOURCES_DIR)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        let filepath = entry.path().strip_prefix(RESOURCES_DIR)?;
        if entry.metadata()?.is_dir() || !is_included(filepath) {
            continue;
        }
        let data = fs::read(entry.path())?;
        main_pak_file.write(&encrypt(&data))?;
    }

    for filepath_str in NOT_PACKED {
        let src_file = Path::new(RESOURCES_DIR).join(Path::new(filepath_str));
        let dest_file = Path::new(BUILD_DIR).join(Path::new(filepath_str));
        fs::create_dir_all(dest_file.parent().unwrap())?;
        fs::copy(src_file, dest_file)?;
    }

    Ok(())
}
