Kleines Befehlszeilen Programm das ein Bildschirmausdruck einer 'xMZ-Mod-Touch'
anfertigen kann

# build

```bash
git clone https://github.com/zzeroo/xmz-snapshot
cd xmz-snapshot
cargo build --release
```

# Installation

```bash
sudo cp target/release/xmz-snapshot /usr/bin/
```

Alle weiteren Infos erfährt man über die Hilfe des Programms.

```bash
xmz-snapshot --help
```
