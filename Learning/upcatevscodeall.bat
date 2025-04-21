for /d %%d in (*.*) do (
pushd %%d
if exist .vscode\ copy /Y C:\Development\GitHub\Rust\Learning\.vscode .vscode
popd
)