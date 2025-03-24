for /d %%d in (*.*) do (
pushd %%d
call cargo build
popd
)