use std::collections::HashMap;

#[derive(Debug)]
pub enum BencodeObj {
    Num(isize),
    Str(Vec<u8>),
    List(Vec<BencodeObj>),
    Dict(HashMap<Vec<u8>, Box<BencodeObj>>),
}

#[derive(Debug)]
enum State {
    Ready,
    RdInt,
    RdStrLen,
    RdStrCon,
    RdLst,
    RdDic,
}

pub fn decode(encoded: &[u8]) -> (Result<BencodeObj, ()>, &[u8] /* remaining */) {
    // 1 pass parser
    let mut state = State::Ready;
    let mut start_pointer: usize = 0;
    let mut end_pointer: usize = 0;

    let mut unexpected = false;

    // we could split these up but ill just make these and use the one i need for now
    let mut str_len: usize = 0;

    for ch in encoded.iter() {
        match state {
            State::Ready => match ch {
                b'0'..=b'9' => {
                    state = State::RdStrLen;
                    end_pointer += 1;
                }
                b'i' => {
                    state = State::RdInt;
                    start_pointer += 1;
                }
                b'l' => {
                    state = State::RdLst;
                    start_pointer += 1;
                }
                b'd' => {
                    state = State::RdDic;
                    start_pointer += 1;
                }
                _ => {
                    unexpected = true;
                }
            },

            State::RdInt => match ch {
                b'0'..=b'9' => end_pointer += 1,
                b'e' => {
                    end_pointer += 1;
                    break;
                }
                _ => {
                    unexpected = true;
                }
            },

            State::RdStrLen => {
                match ch {
                    b':' => {
                        state = State::RdStrCon;
                        let num_str =
                            String::from_utf8(encoded[start_pointer..end_pointer].to_vec())
                                .unwrap();

                        str_len = match num_str.parse::<usize>() {
                            Ok(num) => num,
                            Err(_) => {
                                unexpected = true;
                                0
                            }
                        };
                        end_pointer += 1; // now both pointing to first rune
                        start_pointer = end_pointer;
                    }
                    b'0'..=b'9' => {
                        end_pointer += 1;
                    }
                    _ => {
                        unexpected = true;
                    }
                }
            }
            State::RdStrCon => {
                if str_len > 0 {
                    end_pointer += 1;
                    str_len -= 1;
                } else {
                    break;
                }
            }

            State::RdLst => {}

            State::RdDic => {}
        }
    }

    if unexpected {
        return (Err(()), &encoded[start_pointer..]);
    }

    match state {
        State::Ready => {
            return (Err(()), &encoded[start_pointer..]);
        }
        State::RdInt => {
            let num_str = String::from_utf8(encoded[start_pointer..end_pointer].to_vec()).unwrap();

            return (
                match num_str.parse::<isize>() {
                    Ok(num) => Ok(BencodeObj::Num(num)),
                    Err(_) => Err(()),
                },
                &encoded[end_pointer + 1..],
            );
        }
        State::RdStrLen => {
            return (Err(()), &encoded[start_pointer..]);
        }
        State::RdStrCon => {
            return (
                Ok(BencodeObj::Str(
                    encoded[start_pointer..end_pointer].to_vec(),
                )),
                &encoded[end_pointer..],
            );
        }
        State::RdLst => {
            let mut list: Vec<BencodeObj> = vec![];
            let mut remaining = &encoded[start_pointer..];

            loop {
                if remaining.starts_with(b"e") {
                    return (Ok(BencodeObj::List(list)), &remaining[1..]);
                }

                let (val, rem) = decode(remaining);
                match val {
                    Ok(bencode_obj) => {
                        list.push(bencode_obj);
                    }
                    Err(_) => {
                        return (Err(()), remaining);
                    }
                }
                remaining = rem;
            }
        }
        State::RdDic => {
            let mut dict: HashMap<Vec<u8>, Box<BencodeObj>> = Default::default();

            let mut remaining = &encoded[start_pointer..];

            loop {
                if remaining.starts_with(b"e") {
                    return (Ok(BencodeObj::Dict(dict)), &remaining[1..]);
                }

                let (key, rem) = decode(remaining);
                remaining = rem;
                let (val, rem) = decode(remaining);

                match (key, val) {
                    (Ok(BencodeObj::Str(k)), Ok(v)) => {
                        dict.insert(k, Box::new(v));
                    }
                    _ => {
                        return (Err(()), remaining);
                    }
                }
                remaining = rem;
            }
        }
    }
}

pub fn print_bencode_obj(obj: BencodeObj, indent: Option<usize>) {
    let actual_indent = indent.unwrap_or_default();

    match obj {
        BencodeObj::Num(n) => {
            println!("{:actual_indent$}{}", "", n);
        }
        BencodeObj::Str(s) => {
            let maybe_utf8 = String::from_utf8(s.clone());

            match maybe_utf8 {
                Ok(utf_str) => {
                    println!("{:actual_indent$}{:?}", "", utf_str);
                }
                Err(_) => {
                    println!("{:actual_indent$}{:?}", "", "...bytes...");
                }
            };
        }
        BencodeObj::List(l) => {
            println!("{:actual_indent$}[", "");

            for el in l {
                print_bencode_obj(el, Some(actual_indent + 2));
            }

            println!("{:actual_indent$}]", "");
        }
        BencodeObj::Dict(d) => {
            println!("{:actual_indent$}{{", "");
            for (k, v) in d {
                let i = actual_indent + 2;
                let k_str = String::from_utf8(k).unwrap();
                println!("{:i$}{:?}", "", k_str);
                print_bencode_obj(*v, Some(i));
            }
            println!("{:actual_indent$}}}", "");
        }
    }
}
