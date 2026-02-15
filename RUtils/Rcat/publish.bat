@echo off
echo You should use the Powershell version build.ps1
cargo build --release
COPY /Y target\release\rcat.exe C:\Utils