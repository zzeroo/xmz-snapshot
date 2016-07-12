Small command line utility for taking screen shots of a 'xMZ-Mod-Touch'.

Kleines Befehlszeilen Programm das ein Bildschirmausdruck einer 'xMZ-Mod-Touch'
anfertigen kann.

# Dependencies/ Abhängigkeiten

* [ImageMagic][1] `apt-get install imagemagick`
* [ffmpeg][2] `apt-get install ffmpeg`

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

All further information you can get from the internal help.

Alle weiteren Infos erfährt man über die Hilfe des Programms.

```bash
xmz-snapshot --help
```

# Links

* http://www.imagemagick.org/script/index.php
* https://ffmpeg.org/

[1]: http://www.imagemagick.org/script/index.php
[2]: https://ffmpeg.org/
