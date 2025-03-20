REM Build library crate library.rlib first
rustc --crate-type=lib rary.rs

REM Build binary crate rsplit.exe
rustc split.rs --extern rary=library.rlib
