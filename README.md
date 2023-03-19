export ANKI_ROOT=$HOME/proj/rust/anki
export PATH=$ANKI_ROOT/out/extracted/protoc/bin:$PATH

$ protoc --version
libprotoc 3.21.8

cargo new --lib rslib
cargo new --lib rsbridge


cd rsbridge
mkdir -p out/pylib
python3 -m venv out/pyenv
source out/pyenv/bin/activate
pip install maturin
pip install protobuf
maturin develop
python main.py
