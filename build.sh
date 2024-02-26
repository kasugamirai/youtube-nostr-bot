mkdir -p  output/conf
cp script/* output 2>/dev/null
cp -r conf/* output/conf 2>/dev/null
cp data/migrations output/data -r 2>/dev/null
cargo build --bin bootstrap --release
cp target/release/bootstrap output
chmod +x output/*
