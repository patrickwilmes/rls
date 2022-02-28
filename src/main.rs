/*
 * Copyright (c) 2022, Patrick Wilmes <patrick.wilmes@bit-lake.com>
 *
 * SPDX-License-Identifier: BSD-2-Clause
 */
use std::{env, str, fs};
use std::os::unix::fs::PermissionsExt;
use std::fs::DirEntry;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 1 {
        list_directories(0x0);
    } else {
        let flags = args.get(1).unwrap();
        let mut chars = flags.chars();
        chars.next();
        let bitmask = translate_to_bitmask(chars.as_str());
        list_directories(bitmask);
    }
}

fn translate_to_bitmask(value: &str) -> i8 {
    let mut bitmask: i8 = 0x0;
    if value.contains("p") {
        bitmask = bitmask | 0b0001;
    }
    if value.contains("h") {
        bitmask = bitmask | 0b0010;
    }
    return bitmask;
}

fn list_directories(listing_conditions: i8) {
    let current_wd = env::current_dir().unwrap();
    for entry in fs::read_dir(current_wd).unwrap() {
        let path = entry.unwrap();
        let permissions = fs::metadata(path.path()).unwrap().permissions();
        let is_dir = !fs::metadata(path.path()).unwrap().is_file();
        print_entry((listing_conditions & 0x2) != 0, is_dir, (listing_conditions & 0x1) != 0, &path, &permissions);
    }
}

fn print_with_permissions(dir_or_file_indicator: &str, path: &DirEntry, permissions: &fs::Permissions) {
    println!("{} {:6o} {}", dir_or_file_indicator, permissions.mode(), path.path().display());
}

fn print_without_permissions(dir_or_file_indicator: &str, path: &DirEntry) {
    println!("{} {}", dir_or_file_indicator, path.path().display());
}

fn print_entry(allow_hidden: bool, is_dir: bool, skip_permissions: bool, path: &DirEntry, permissions: &fs::Permissions) {
    let dir_or_file_indicator = match is_dir {
        false => "f",
        true => "d"
    };
    if allow_hidden { // display all files incl. hidden ones
        match skip_permissions {
            true => print_without_permissions(dir_or_file_indicator, &path),
            false => print_with_permissions(dir_or_file_indicator, &path, &permissions)
        }
    } else {
        if path.file_name().to_str().unwrap().chars().next().unwrap() != '.' {
            match skip_permissions {
                true => print_without_permissions(dir_or_file_indicator, &path),
                false => print_with_permissions(dir_or_file_indicator, &path, &permissions)
            }
        }
    }
}
