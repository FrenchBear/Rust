#/bin/sh
cargo build --release
[ ! -d ~/bin ] && mkdir ~/bin
cp target/release/rcat ~/bin
