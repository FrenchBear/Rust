@echo off
for /d %%d in (*.*) do (
	echo ---------------------
	echo %%d
	pushd %%d
REM	cargo update --verbose
    cargo clippy
	popd
)