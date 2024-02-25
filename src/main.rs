use std::{cmp::Ordering, ops::Index};
use sha256::try_digest;

fn get_sha(filename: String) -> String{
    return try_digest(filename).unwrap();
}

fn compare(sha1: String, sha2: String) -> bool {
    return sha1.cmp(&sha2) == Ordering::Equal;
}

fn show_sha(filename: String) {
    println!("{}", get_sha(filename));
}

fn show_compare(filename: String, given_sha: String) {
    println!("{}", compare(get_sha(filename), given_sha));
}

fn main() {
    let args = std::env::args().collect::<Vec<String>>();
    
    match args.len() {
        1 => eprintln!("You need to insert file name as an argument"),
        2 => show_sha(args.index(1).to_string()),
        3 => show_compare(args.index(1).to_string(), args.index(2).to_string()),
        _ => panic!("Panic when checking args length")
    }
}
