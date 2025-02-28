use std::collections::HashMap;
use std::error::Error;
use std::fs;

mod bencode;
use bencode::{print_bencode_obj, BencodeObj};

fn main() -> Result<(), Box<dyn Error>> {
    let encoded: Vec<u8> = fs::read("sample.torrent")?;

    let (v, e) = bencode::decode(&encoded);
    print_bencode_obj(v.unwrap(), None);

    Ok(())
}
