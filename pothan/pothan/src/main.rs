use std::{
    env::{args, current_dir},
    fs::{File, OpenOptions, remove_file, remove_dir_all},
    io::Result,
    process::Command,
};

use tar::Archive;
use xz::{read::XzDecoder, write::XzEncoder};

const MODE_UNPACK: &str = "0";
const MODE_PACK: &str = "1";

fn main() -> Result<()> {
    let mut args = args();
    args.next();

    if let Some(mode) = args.next() {
        if mode.eq(MODE_UNPACK) {
            println!("Unpacking...");

            let app_compressed_path = "App.cmp";

            let app_compressed = File::open(app_compressed_path).expect("App.cmp missing!");
            let app_decoder = XzDecoder::new(app_compressed);
            let mut archive = Archive::new(app_decoder);

            archive.unpack(".").expect("Error extracting App.cmp!");

            drop(archive);
            remove_file(app_compressed_path).expect("Error removing App.cmp!");

            Command::new("pothan\\shortcut.exe")
                .arg("/f:Xiphos.lnk")
                .arg("/a:c")
                .arg("/t:".to_owned() + current_dir().unwrap().to_str().unwrap() + "\\App\\launcher.exe")
                .arg("/w:".to_owned() + current_dir().unwrap().to_str().unwrap() + "\\App")
                .status()
                .expect("Error creating shortcut!");
        } else if mode.eq(MODE_PACK) {
            println!("Packing...");

            let app_compressed_path = "App.cmp";

            let app_compressed = OpenOptions::new().write(true).create_new(true)
                .open(app_compressed_path).expect("App.cmp exists!");
            let app_encoder = XzEncoder::new(app_compressed, 9);
            let mut archive = tar::Builder::new(app_encoder);

            archive.append_dir_all("App", "App").expect("Error compressing App!");

            archive.finish().expect("Error compressing App!");
            drop(archive);
            match remove_file("Xiphos.lnk") {
                Ok(_) => {},
                Err(_) => {},
            };
            remove_dir_all("App").expect("Error removing App!");
        }
    }

    Ok(())
}
