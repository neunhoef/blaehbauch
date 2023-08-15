use std::env;
use std::time;
use std::thread;

fn main() {
    let args : Vec<String> = env::args().collect();
    let size = if args.len() < 2 {
        4096 as u64    // gigabytes, virtually infinity
    } else {
        args[1].parse::<u64>().expect("Need number in gigabytes as size")
    };
    let delay = if args.len() < 3 {
        0.0 as f64
    } else {
        args[2].parse::<f64>().expect("Need number in seconds as delay")
    };
    let end_delay = if args.len() < 4 {
        0 as f64
    } else {
        args[3].parse::<f64>().expect("Need number in seconds as end delay")
    };
    println!("Hello, this is blaehbauch, I will use {} gigabytes of RAM.", size);
    println!("I will delay for {} seconds after each gigabyte and for {} seconds at the end.",
             delay, end_delay);
    let mut pattern : Vec<u8> = Vec::with_capacity(1024*1024);
    for i in 0..1024*1024 {
        pattern.push((i & 0xff) as u8);
    }
    let mut ram : Vec<Vec<u8>> = vec![];
    ram.reserve((size * 1024) as usize);  // Chunks of 1 MB
    for i in 1..=size * 1024 {
        //let v = Vec::with_capacity(1024*1024);
        let v = pattern.clone();
        ram.push(v);
        if i % 1024 == 0 {
            println!("Have allocated {} gigabytes.", i / 1024);
            thread::sleep(time::Duration::from_secs_f64(delay));
        }
    }
    println!("Done. Waiting for {} seconds...", end_delay);
    thread::sleep(time::Duration::from_secs_f64(end_delay));
}
