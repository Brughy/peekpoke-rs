use std::io;
use memmap2::{MmapMut, MmapOptions};
use std::fs::File;
use clap::Parser;
use std::str::FromStr;

type U = u32;

fn todec<T: Sized + FromStr>(value: String) -> T where <T as FromStr>::Err: std::fmt::Debug {
    match value.starts_with("0x") {
        true => T::from_str(&value.strip_prefix("0x").unwrap()).unwrap(),
        false => value.parse::<T>().unwrap(),
    }
}

pub struct MyCfgAccess {
    pub offset: u64,
    pub size: usize,
    pub mode: String,
    pub data: U,
    pub num: usize,
}

pub fn open_mmap(s: &str) -> File {
    std::fs::OpenOptions::new()
                    .read(true)
                    .write(true)
                    .open(s)
                    .unwrap()
}

fn create_mmap(f: &File, offset: u64, size: usize) -> MmapMut {
    unsafe {
            MmapOptions::new()
                .offset(offset)
                .len(size)
                .map_mut(f)
                .unwrap()
    }
}

fn read<T: Sized> (m: &MmapMut)  -> T {
    unsafe {
        let register = m.as_ptr() as *mut T;
        std::ptr::read::<T>(register)
    }
}

fn write<T: Sized> (m: &MmapMut, v: T) {
    unsafe {
        let register = m.as_ptr() as *mut T;
        std::ptr::write::<T>(register, v);
    }
}

#[derive(Parser, Debug)]
#[command(name = "Peek/Poke ++",
          version = "0.1.0",
          author = "Luca Brugnera <luca.brugnera@gmail.com>",
          about = "Read and write on file or char device (ex. /dev/mem)")]
struct Args {
    /// Set offset (default 0x0)
    #[arg(short, long, value_name = "OFFSET")]
    offset: Option<String>,
    /// Set size (default 4 --> 32bit arch)
    #[arg(short, long, value_name = "SIZE")]
    size: Option<usize>,
    /// Set mode : r, read or w, write (default r)
    #[arg(short, long, value_name = "MODE")]
    mode: Option<String>,
    /// Set data (default 0x0)
    #[arg(short, long, value_name = "DATA")]
    data: Option<String>,
    /// Set repetitions (default 1)
    #[arg(short, long, value_name = "NUM")]
    num: Option<usize>,
    /// Set data (default 0x0)
    #[arg(short, long, value_name = "DEVICE")]
    file: Option<String>,    
}

fn op(f: &File, a: &mut MyCfgAccess) -> U {
    let m = create_mmap(&f, a.offset, a.size);
    let v;
    if a.mode == "w" || a.mode == "W" || a.mode == "write" || a.mode == "WRITE" {
        write::<U>(&m, a.data);
        v = a.data;
    } else if a.mode == "r" || a.mode == "R" || a.mode == "read" || a.mode == "READ" {
        v = read::<U>(&m);
        //println!("offset 0x{:#08x} value {:#x}", offset.clone(), v);
    } else {
        write::<U>(&m, a.data);
        v = read::<U>(&m);
    }
    v
}

fn rep( f: &File, a: &mut MyCfgAccess) {
    for i in 0..a.num {
        a.offset = a.offset + (a.size as u64) * i as u64;
        let v = op(f, a);
        println!("offset 0x{:#08x} value {:#x}", a.offset, v);
    }
}

fn main() -> Result<(), io::Error> {
    let args = Args::parse();
    let offset = args.offset.unwrap_or("0x0".to_string());
    let data = args.data.unwrap_or("0x0".to_string());
    let device = args.file.unwrap_or("/dev/mem".to_string());

    let mut a = MyCfgAccess {
        offset: todec::<U>(offset.clone()) as u64,
        size: args.size.unwrap_or(4),
        mode: args.mode.unwrap_or("r".to_string()),
        data: todec::<U>(data) as U,
        num: args.num.unwrap_or(1),
      };

    let f = open_mmap(&device);
    rep(&f, &mut a);
    Ok(())
}

#[cfg(test)]
mod test;
