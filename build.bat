@echo off
cargo +nightly build -Z build-std=std,panic_abort -Z build-std-features=panic_immediate_abort --target i686-pc-windows-msvc --release
copy target\i686-pc-windows-msvc\release\wide.dll dist\wide.dll