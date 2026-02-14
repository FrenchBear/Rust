# 1. Build the app
cargo build --release

# 2. Define paths
$exePath = "target\release\rnormalizedates.exe"
$signtool = "C:\Program Files (x86)\Windows Kits\10\bin\10.0.26100.0\x64\signtool.exe"

# 3. Sign
& $signtool sign /f "C:\Development\MyDevCert.pfx" /p "Myrt1_ll3!" /fd SHA256 /t http://timestamp.digicert.com $exePath

# 4. Deploy
Copy-Item $exePath C:\Utils
