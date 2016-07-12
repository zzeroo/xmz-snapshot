Small command line utility for linux to taking screenshots
from a 'xMZ-Mod-Touch'.

Kleines Befehlszeilen Programm für Linux, dass ein Bildschirmausdruck
einer 'xMZ-Mod-Touch' anfertigen kann.

# Dependencies/ Abhängigkeiten

* [Rust][0] installation via [rustup][3] `curl https://sh.rustup.rs -sSf | sh -s -- --default-toolchain nightly` of course ...nee klar ne?
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

* https://www.rust-lang.org
* https://www.rustup.rs/
* http://www.imagemagick.org/script/index.php
* https://ffmpeg.org/
* Todo http://www.steveklabnik.com/automatically_update_github_pages_with_travis_example/

[0]: https://www.rust-lang.org
[1]: http://www.imagemagick.org/script/index.php
[2]: https://ffmpeg.org/
[3]: https://www.rustup.rs/


