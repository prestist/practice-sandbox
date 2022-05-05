use core::panic;
use std::{io::Write, fs::{OpenOptions, File}};


fn main() {
    make_mbr_img();
    read_mbr_partitions();
}

fn make_mbr_img() {
    let file_name = "mbr.img";
    println!("Creating a file named {}, to store the mbr partitions",file_name);
    let mut drive = File::create(&file_name).unwrap();
    let sector_size = 512; // sector size
    println!("Preparing MBR partitions for {}.",file_name);
    let mut mbr = mbrman::MBR::new_from(&mut drive, sector_size as u32, [0xff; 4])
    .expect("could not create partition table");
    mbr.write_into(&mut drive).expect("Not able to save the file.");
    println!("File saved.");
}

fn read_mbr_partitions() {

    let file_name = "mbr.img";
    let mut f = std::fs::File::open(&file_name)
        .expect("could not open disk");
    let mbr = mbrman::MBR::read_from(&mut f, 512)
        .expect("could not find MBR");
    println!("Disk signature: {:?}", mbr.header.disk_signature);
    for (i, p) in mbr.iter() {
        // NOTE: The first four partitions are always provided by iter()
        if p.is_used() {
            println!("Partition #{}: type = {:?}, size = {} bytes, starting lba = {}",
                i,
                p.sys,
                p.sectors * mbr.sector_size,
                p.starting_lba);
        }
    }
}
