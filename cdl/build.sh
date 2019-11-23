cd calc
cargo build # --release
cp target/debug/libcalc.dylib ./
cd ..
cargo run libcalc.dylib