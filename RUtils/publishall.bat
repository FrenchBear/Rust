@echo off
for /d %%d in (*) do ( 
    @echo off
	if exist "%%d\publish.ps1" (
        echo --- Publishing %%d using PowerShell ---
        pushd "%%d"
        powershell -ExecutionPolicy Bypass -File "publish.ps1"
        popd
    ) else if exist "%%d\publish.bat" (
        echo --- Publishing %%d using Batch ---
        pushd "%%d"
        call publish.bat
        popd
    )
)
echo Done.
echo on
