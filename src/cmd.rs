use std::process;
use std::io::prelude::*;
use std::fs::File;


#[derive(Debug)]
pub enum Command {
    Invalid,
    Execute { remote_ip: String }
}

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
    ).get_matches();

    if let Some(ip) = matches.value_of("remote_ip") {
        let mut buffer = File::create("framebuffer.img").unwrap();
        let output = process::Command::new("ssh")
                                .args(&["-i", "~/development/custom_image/id_rsa", format!("root@{}", ip).as_str(), "cat", "/dev/fb0"])
                                .output()
                                .expect("failed to execute process on remote side via ssh");

        buffer.write(&output.stdout).unwrap();

        // Convert raw framebuffer to an graphical image file
        let output = process::Command::new("avconv")
                                .args(&["-f", "rawvideo", "-pix_fmt", "rgb0", "-s", "1024x600", "-i", "framebuffer.img", "framebuffer.png", "-y"])
                                .output()
                                .expect("failed to convert framebuffer to graphical image file");

        buffer.write(&output.stdout).unwrap();

        // Show image
        let _ = process::Command::new("display")
                                .arg("framebuffer.png")
                                .spawn()
                                .expect("framebuffer.png could not shown by display");

    }
}
