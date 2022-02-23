@echo off
cargo +nightly build -Z build-std=std,panic_abort -Z build-std-features=panic_immediate_abort --target x86_64-pc-windows-msvc --release
copy target\x86_64-pc-windows-msvc\release\pvz_widescreen.exe dist\pvz_widescreen.exe
