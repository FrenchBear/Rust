#/bin/sh
cargo build --release
[ ! -d ~/bin ] && mkdir ~/bin
cp target/release/rtree ~/bin
