@echo off
set "distribution_dir=%cd%"
cd ..\..
cargo build --release

cd %distribution_dir%
mkdir program
copy /y ..\..\target\release\alloy.exe program\alloy.exe
