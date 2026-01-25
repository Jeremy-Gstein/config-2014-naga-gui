# Config 2014 Naga GUI
Linux GUI client to configure side buttons 1..12 on a 2014 Razer Naga Mouse.

---
<img width="500" height="420" alt="image" src="https://github.com/user-attachments/assets/14ff8975-9bdf-483c-81b0-24aa4f892e7d" />

<sub>testest on Garuda Linux x86_64-Linux 6.18.2-zen2-1-zen</sub>

## Quickstart:
App Image available via github releases
```bash
# download latest release from github
URL="https://github.com/Jeremy-Gstein/config-2014-naga-gui/releases/latest/download/config-2014-naga-gui.AppImage"
curl -L -o config-2014-naga-gui.AppImage "$URL"
# make the appimage executeable
chmod +x config-2014-naga-gui.AppImage
# run appimage to start GUI
./config-2014-naga-gui.AppImage
```
## Build locally:
```bash
# clone the repo
git clone https://github.com/Jeremy-Gstein/config-2014-naga-gui
# build with cargo
cargo build
# run with cargo
cargo run
```
Build appimage locally:
download [appimagetool](https://appimage.github.io/appimagetool/) from web or see below for cli.
```bash
# get appimagetool
wget https://github.com/AppImage/AppImageKit/releases/download/continuous/appimagetool-$(uname -m).AppImage -O /usr/local/bin/appimagetool
chmod +x /usr/local/bin/appimagetool
# get cargo-appimage
cargo install cargo-appimage
# build with cargo appimage
cargo appimage
# produces .AppImage in target/appimage/
```
