use std::process;
use std::io::prelude::*;
use std::fs;
use std::fs::File;


pub fn read_command() {
    // Pull version information out of Cargo.toml
    let version = format!("{}.{}.{}{}",
                     env!("CARGO_PKG_VERSION_MAJOR"),
                     env!("CARGO_PKG_VERSION_MINOR"),
                     env!("CARGO_PKG_VERSION_PATCH"),
                     option_env!("CARGO_PKG_VERSION_PRE").unwrap_or(""));


    let matches = clap_app!(xmz_snapshot =>
        (version: version.as_str())
        (author: env!("CARGO_PKG_AUTHORS"))
        (about: env!("CARGO_PKG_DESCRIPTION"))
        (@arg remote_ip: +required "IP adress of the 'xMZ-Mod-Touch'")
        (@arg keep_image: -i --image "Keep image file")
        (@arg keep_raw: -r --raw "Keep raw image file")
    ).get_matches();

    if let Some(ip) = matches.value_of("remote_ip") {
        // connect via ssh to the given remote ip and call `cat /dev/fb0` the stdout is later then
        // "piped" in an file
        let mut buffer = File::create("framebuffer.img").unwrap();
        let output = process::Command::new("ssh")
                                .args(&["-i", "~/development/custom_image/id_rsa", format!("root@{}", ip).as_str(), "cat", "/dev/fb0"])
                                .output()
                                .expect("failed to execute process on remote side via ssh");

        buffer.write(&output.stdout).unwrap();

        // Convert raw framebuffer to an graphical image file
        let output = process::Command::new("avconv")
                                .args(&["-f", "rawvideo", "-pix_fmt", "bgr0", "-s", "1024x600", "-i", "framebuffer.img", "framebuffer.png", "-y"])
                                .output()
                                .expect("failed to convert framebuffer to graphical image file");

        buffer.write(&output.stdout).unwrap();

        // Show image
        let _ = process::Command::new("display")
                                .arg("framebuffer.png")
                                .output()
                                .expect("framebuffer.png could not shown by display");

        // If keep_image parameter (-i, default not set) is not set delete the image
        if !matches.is_present("keep_image") {
            fs::remove_file("framebuffer.png");
        }

        // If keep_raw parameter (-r, default not set) is not set delete the image
        if !matches.is_present("keep_raw") {
            fs::remove_file("framebuffer.img");
        }


    }
}
