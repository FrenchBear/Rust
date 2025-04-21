for /d %%d in (*.*) do (
pushd %%d
if exist .vscode\ copy /Y C:\Development\GitHub\Rust\The_Book_2024\.vscode .vscode
popd
)