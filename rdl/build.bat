SET ARCH=x86_64-pc-windows-msvc
SET CARGO_TARGET_DIR=F:\rust\rust-tools\rdl
SET RELEASE_DIR=%CARGO_TARGET_DIR%\%ARCH%
cargo build --release --target=%ARCH%
copy %RELEASE_DIR%\release\rdl.dll F:\rust\rust-tools\rdl\rdl-test\rdl-test\bin\x64\Debug\rdl.dll
rmdir /S /Q %RELEASE_DIR%
rmdir /S /Q %CARGO_TARGET_DIR%\release