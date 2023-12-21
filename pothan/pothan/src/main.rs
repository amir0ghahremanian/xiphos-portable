use std::{
    env::{args, current_dir},
    fs::{File, OpenOptions, remove_file, remove_dir_all},
    io::{Result, Read, Write, Seek, SeekFrom, BufWriter, stdin, stdout},
    sync::{Arc, Mutex, Condvar},
    process::Command,
    thread,
};

use tar::{Archive, Builder};
use xz::{read::XzDecoder, write::XzEncoder};

const MODE_UNPACK: &str = "0";
const MODE_PACK: &str = "1";

const APP_PATH: &str = "App";
const APP_COMPRESSED_PATH: &str = "App.cmp";
const APP_TARBALL_PATH: &str = "App.tmp";

fn main() -> Result<()> {
    let mut args = args();
    args.next();

    if let Some(mode) = args.next() {
        if mode.eq(MODE_UNPACK) {
            println!("Unpacking...");

            let mut app_compressed = File::open(APP_COMPRESSED_PATH.to_owned() + ".0").expect("App data missing!");
            let mut app_parts_no: [u8; 1] = [0];
            app_compressed.read_exact(&mut app_parts_no)?;
            let app_parts_no = app_parts_no[0];

            let app_tarball = Arc::new(Mutex::new(File::create(APP_TARBALL_PATH).unwrap()));

            let write_cond_pair = Arc::new((Mutex::new(1), Condvar::new()));
            let mut decompressor_threads = Vec::new();
            for i in 1..(app_parts_no + 1) {
                let write_cond_pair_thread = write_cond_pair.clone();
                let app_tarball_thread = app_tarball.clone();
                let thread = thread::spawn(move || {
                    let app_i_compressed = File::open(APP_COMPRESSED_PATH.to_owned() + "." + i.to_string().as_str())
                        .unwrap();
                    let mut app_i_decoder = XzDecoder::new(app_i_compressed);

                    let mut buffer: BufWriter<Vec<u8>> = BufWriter::new(Vec::new());
                    loop {
                        let mut decode_buffer: [u8; 64] = [0; 64];
                        let n = app_i_decoder.read(&mut decode_buffer).unwrap();
                        if n == 0 { break; }
                        buffer.write_all(&decode_buffer[0..n]).unwrap();
                    }
                    let buffer = buffer.into_inner().unwrap();

                    let (stage, cvar) = write_cond_pair_thread.as_ref();
                    let mut stage = stage.lock().unwrap();
                    while *stage != i {
                        stage = cvar.wait(stage).unwrap();
                    }

                    app_tarball_thread.lock().unwrap().write_all(&buffer).unwrap();

                    *stage += 1;
                    cvar.notify_all();
                });

                decompressor_threads.push(thread);
            }

            for thread in decompressor_threads {
                thread.join().unwrap();
            }

            app_tarball.lock().unwrap().sync_all()?;
            let app_tarball = File::open(APP_TARBALL_PATH)?;

            let mut archive = Archive::new(app_tarball);

            archive.unpack(".")?;

            drop(archive);
            remove_file(APP_TARBALL_PATH)?;
            for i in 0..(app_parts_no + 1) {
                remove_file(APP_COMPRESSED_PATH.to_owned() + "." + i.to_string().as_str())?;
            }

            Command::new("pothan\\shortcut.exe")
                .arg("/f:Xiphos.lnk")
                .arg("/a:c")
                .arg("/t:".to_owned() + current_dir().unwrap().to_str().unwrap() + "\\App\\launcher.exe")
                .arg("/w:".to_owned() + current_dir().unwrap().to_str().unwrap() + "\\App")
                .status()
                .expect("Error creating shortcut!");
        } else if mode.eq(MODE_PACK) {
            let mut threads_no = String::new();

            print!("Number of threads: ");
            stdout().flush().unwrap();
            stdin().read_line(&mut threads_no).expect("Unexpected input!");

            let threads_no = threads_no.trim().parse::<u8>().expect("Unexpected input!");

            println!("Packing...");

            let mut app_compressed = OpenOptions::new().write(true).create_new(true)
                .open(APP_COMPRESSED_PATH.to_owned() + ".0").expect("App.cmp exists!");
            let app_tarball = File::create(APP_TARBALL_PATH)?;
            let mut archive = Builder::new(app_tarball);

            archive.append_dir_all(APP_PATH, APP_PATH).expect("Error creating App tarball!");
            archive.into_inner().unwrap().sync_all()?;

            let mut app_tarball = File::open(APP_TARBALL_PATH).unwrap();
            let buffer_size = (app_tarball.metadata().unwrap().len()) as usize / threads_no as usize;

            let mut compressor_threads = Vec::new();
            for i in 1..threads_no {
                let thread = thread::spawn(move || {
                    let mut app_tarball = File::open(APP_TARBALL_PATH).unwrap();
                    let app_i_compressed = File::create(APP_COMPRESSED_PATH.to_owned() + "." + i.to_string().as_str())
                        .unwrap();
                    let mut app_encoder = XzEncoder::new(app_i_compressed, 9);

                    let mut buffer: Vec<u8> = vec![0; buffer_size];
                    app_tarball.seek(SeekFrom::Start((i as u64 - 1)*buffer_size as u64)).unwrap();
                    app_tarball.read(&mut buffer).unwrap();
                    app_encoder.write(&buffer).unwrap();
                    app_encoder.finish().unwrap().flush().unwrap();
                });

                compressor_threads.push(thread);
            }            

            let size = app_tarball.metadata().unwrap().len();
            let new_buffer_size = size as usize - ((threads_no as usize - 1) * buffer_size);

            let app_last_compressed = File::create(APP_COMPRESSED_PATH.to_owned() + "." + threads_no.to_string().as_str())
                .unwrap();
            let mut app_encoder = XzEncoder::new(app_last_compressed, 9);

            let mut buffer: Vec<u8> = vec![0; new_buffer_size];
            app_tarball.seek(SeekFrom::Start((threads_no as u64 - 1)*buffer_size as u64)).unwrap();
            app_tarball.read(&mut buffer).unwrap();
            app_encoder.write(&buffer).unwrap();
            app_encoder.finish().unwrap().flush().unwrap();

            for thread in compressor_threads {
                thread.join().unwrap();
            }

            app_compressed.write_all(&[threads_no])?;
            app_compressed.sync_all()?;

            match remove_file(APP_TARBALL_PATH) {
                Ok(_) => {},
                Err(_) => {},
            };
            match remove_file("Xiphos.lnk") {
                Ok(_) => {},
                Err(_) => {},
            };
            remove_dir_all(APP_PATH).expect("Error removing App!");
        }
    }

    Ok(())
}
