nim c --noMain --noLinking --nimcache:src/ src/nom.nim
cargo build
rm src/*.nim.c src/*.nim.c.* src/*.json
