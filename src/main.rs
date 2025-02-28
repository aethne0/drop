#![allow(unused)]

use std::collections::HashMap;
use std::fs;
use std::error::Error;

mod bencode;
use bencode::{BencodeObj};

fn main() -> Result<(), Box<dyn Error>> {
    //let mut rest: &str = "i35ei67e";
    //let mut encoded: &str = "12:hellohellobb";
    let mut encoded: &str = "d5:item1l12:hellohellobbi32eli32ei32ei32e5:helloi32eee3:key5:valuee";
    //let encoded: Vec<u8> = fs::read("sample.torrent")?;


    dbg!(bencode::decode(encoded));

    Ok(())
}


