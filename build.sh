mkdir -p  output/conf
cp script/* output 2>/dev/null
cp -r conf/* output/conf 2>/dev/null
cargo build --release
cp target/release/youtube_bot_run output
chmod +x output/*
