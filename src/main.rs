// export xmz_ip=192.168.89.101
//
// cd /tmp
// ssh -i ~/development/custom_image/id_rsa root@$xmz_ip cat /dev/fb0 > framebuffer.img
// avconv -f rawvideo -pix_fmt rgb0 -s 1024x600 -i framebuffer.img framebuffer.png -y
// display framebuffer.png
#[macro_use] extern crate clap;

mod cmd;

fn main() {
    println!("{:?}", cmd::read_command());
}
