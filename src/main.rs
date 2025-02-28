#![allow(unused)]

use std::collections::HashMap;
use std::fs;
use std::error::Error;

mod bencode;
use bencode::{BencodeObj,print_bencode_obj};

fn main() -> Result<(), Box<dyn Error>> {
    //let mut encoded = b"li32ei64e5:helloe";
    //let mut encoded = b"d5:hello3:keye";
    //let mut encoded = b"d5:item1l12:hellohellobbi32eli32ei32ei32e5:helloi32eee3:key5:valuee";
    let encoded: Vec<u8> = fs::read("sample.torrent")?;


    let (v, e) = bencode::decode(&encoded);
    print_bencode_obj(v.unwrap(), None);

    Ok(())
}

