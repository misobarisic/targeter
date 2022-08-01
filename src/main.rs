mod files;
mod opt;
mod util;

use std::ops::Range;
use std::path::Path;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};
use std::{fs, thread};

use byte_unit::{Byte, ByteUnit};
use colorful::{Color, Colorful};
use indicatif::{HumanDuration, MultiProgress, ProgressBar, ProgressStyle};
use promptly::prompt_default;
use rand::Rng;
use scan_dir::ScanDir;
use structopt::StructOpt;

use crate::files::get_all_cargo_dirs;
use crate::opt::Opt;

const COSMETIC_DELAY: u64 = 15;
const COSMETIC_DELAY_RANGE: Range<u64> = 1..2;

fn main() {
    let opt = Opt::from_args();
    let n_jobs = if opt.jobs == 0 { num_cpus::get() } else { opt.jobs };

    let mut rng = rand::thread_rng();
    let input = opt.input;

    if !Path::new(&input).is_dir() {
        panic!("Input parameter does not represent a valid dir.")
    }

    println!("Starting a scan in {}", input.display().to_string().bold());

    let started = Instant::now();
    let spinner_style = ProgressStyle::with_template("{prefix:.bold.dim} {spinner} {wide_msg}").unwrap().tick_chars("⠁⠂⠄⡀⢀⠠⠐⠈ ");
    let all_target_dirs = get_all_cargo_dirs(input);

    let map = Arc::new(Mutex::new(Vec::new()));
    let jobs_to_execute = all_target_dirs.len();

    let m = Arc::new(Mutex::new(MultiProgress::new()));
    let jobs_executed = Arc::new(Mutex::new(0));
    let pool = rayon::ThreadPoolBuilder::new().num_threads(n_jobs).build().unwrap();

    for (i, buf) in all_target_dirs.iter().enumerate() {
        let num = Arc::clone(&jobs_executed);
        let map = Arc::clone(&map);
        let m = Arc::clone(&m);
        let count = rng.gen_range(1..80);
        let spinner_style = spinner_style.clone();

        let mut buf = buf.clone();
        buf.set_file_name("");

        pool.spawn(move || {
            let m = m.lock().unwrap();
            let pb = m.add(ProgressBar::new(count));
            let map = Arc::clone(&map);
            pb.set_style(spinner_style.clone());
            pb.set_prefix(format!("[{}/{}]", i + 1, jobs_to_execute));
            #[cfg(feature = "delay")]
            thread::sleep(Duration::from_millis(COSMETIC_DELAY));
            thread::spawn(move || {
                let mut rng = rand::thread_rng();
                let pkg = buf.file_name().unwrap_or_else(|| "./".as_ref()).to_str().unwrap_or_default().to_owned();
                let files = ScanDir::files()
                    .walk(&buf, |iter| {
                        iter.map(|(ref entry, ref name)| {
                            pb.set_message(format!("{}: {}", pkg, name));
                            pb.inc(1);
                            if rng.gen_bool(0.1) {
                                #[cfg(feature = "delay")]
                                thread::sleep(Duration::from_millis(rng.gen_range(COSMETIC_DELAY_RANGE)));
                            }
                            entry.metadata().unwrap().len()
                        })
                        .collect::<Vec<u64>>()
                    })
                    .unwrap();
                map.lock().unwrap().push((buf, files.iter().sum::<u64>()));
                pb.finish_with_message(format!("{}: done...", pkg));

                let mut num = num.lock().unwrap();
                *num += 1;
            });
        })
    }

    while *jobs_executed.lock().unwrap() < jobs_to_execute {
        thread::sleep(Duration::from_millis(100));
    }
    println!("Scanned in {}", HumanDuration(started.elapsed()));

    let mut map = map.lock().unwrap().to_owned();
    map.sort_by(|a, b| b.1.cmp(&a.1));
    for (i, (fpath, size)) in map.clone().into_iter().enumerate() {
        let byte = Byte::from_bytes(size.into());
        let adjusted_byte = byte
            .get_appropriate_unit(true)
            .to_string()
            .color(if byte <= Byte::from_unit(512.0, ByteUnit::MiB).unwrap() {
                Color::Green
            } else if byte <= Byte::from_unit(3.0, ByteUnit::GiB).unwrap() {
                Color::Yellow
            } else if byte <= Byte::from_unit(6.5, ByteUnit::GiB).unwrap() {
                Color::DarkOrange
            } else {
                Color::Red
            })
            .bold();

        print!(" {} {} ", (i + 1).to_string().color(Color::LightGray).bold(), "->".color(Color::LightGray).bold());
        let mut folder_str = String::from(fpath.to_str().unwrap());
        folder_str.pop();

        #[cfg(feature = "delay")]
        thread::sleep(Duration::from_millis(COSMETIC_DELAY));

        println!("{} - {}", folder_str, adjusted_byte);
    }

    let mut to_remove = Vec::new();
    println!("An empty input or 0 will result in going to the next step.");
    loop {
        let mut age = jobs_to_execute + 1;
        while age > jobs_to_execute {
            age = prompt_default("Index to remove", 0).unwrap_or(0);
        }
        if age == 0 {
            break;
        }
        to_remove.push(age - 1);
    }

    if to_remove.is_empty() {
        println!("No files to remove. Exiting.");
        return;
    }

    for i in to_remove {
        let (mut buf, _) = map[i].clone();
        buf.push("target");
        match fs::remove_dir_all(&buf) {
            Ok(_) => {
                println!("Successfully cleaned {}", buf.display().to_string().bold());
            }
            Err(_) => {
                println!("Failed to clean {}", buf.display().to_string().bold());
            }
        }
    }
}
