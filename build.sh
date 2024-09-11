#/bin/sh

rm mh3se-release.zip
mkdir ./mh3se-release

cargo build --release --target x86_64-pc-windows-gnu
cp ./target/x86_64-pc-windows-gnu/release/mh3se-cli.exe ./mh3se-release/mh3se-cli.exe
cp ./target/x86_64-pc-windows-gnu/release/mh3se-webui.exe ./mh3se-release/mh3se-webui.exe

cp -R ./web ./mh3se-release/web
cp ./LICENSE.md ./mh3se-release/LICENSE.md
cp ./README.md ./mh3se-release/README.md

zip -r mh3se-release.zip mh3se-release/

rm -rf mh3se-release/