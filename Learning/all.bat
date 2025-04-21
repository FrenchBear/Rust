for /d %%d in (*.*) do (
pushd %%d
if exist Cargo.toml call cargo %*
popd
)