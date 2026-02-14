echo Prefer publish.ps1 that signs the .exe
cargo build --release
COPY /Y target\release\rfind.exe C:\Utils