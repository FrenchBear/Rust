@echo off
for /d %%d in (*.*) do (
	echo ---------------------
	echo %%d
	pushd %%d
	cargo update --verbose
	popd
)