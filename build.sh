mkdir -p output/bin output/conf
cp script/* output 2>/dev/null

cargo build --release --target=x86_64-unknown-linux-gnu
cp /target/release/YOUTUBE
chmod +x output/*
