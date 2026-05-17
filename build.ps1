$ErrorActionPreference = "Stop"

Write-Host "==> Building 32-bit (i686-pc-windows-msvc)..."
cargo build --release --target i686-pc-windows-msvc

Write-Host "==> Building 64-bit (x86_64-pc-windows-msvc)..."
cargo build --release --target x86_64-pc-windows-msvc

$out32 = "target\i686-pc-windows-msvc\release\gmsv_dotenv.dll"
$out64 = "target\x86_64-pc-windows-msvc\release\gmsv_dotenv.dll"

Copy-Item $out32 "gmsv_dotenv_win32.dll" -Force
Copy-Item $out64 "gmsv_dotenv_win64.dll" -Force

Write-Host ""
Write-Host "Done. Output files:"
Write-Host "  gmsv_dotenv_win32.dll  ->  garrysmod/lua/bin/"
Write-Host "  gmsv_dotenv_win64.dll  ->  garrysmod/lua/bin/"
