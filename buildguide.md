ビルドするときは`cl.exe`のパスのために`Developer Command Prompt`を使ってください
```bash
cargo build --release -p パッケージ --features words_lib/cuda # build for cuda
cargo build --release -p パッケージ --features words_lib/cpu # build for cpu
```