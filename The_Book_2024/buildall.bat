for /d %%d in (*.*) do (
pushd %%d
call cargo update
call cargo build
popd
)