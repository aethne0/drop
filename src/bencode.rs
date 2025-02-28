use std::collections::HashMap;

#[derive(Debug)]
pub enum BencodeObj {
    Num(isize),
    Str(String),
    List(Vec<BencodeObj>),
    Dict(HashMap<String, Box<BencodeObj>>)
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

pub fn decode(encoded: &str) -> (Result<BencodeObj, ()>, &str /* remaining */) {
    // 1 pass parser
    let mut state = State::Ready;
    let mut start_pointer: usize = 0;
    let mut end_pointer: usize = 0;

    let mut unexpected = false;

    // we could split these up but ill just make these and use the one i need for now
    let mut str_len: usize = 0;

    for ch in encoded.chars() {
        match state {
            State::Ready => {
                match ch {
                    '0'..='9' => { state = State::RdStrLen; end_pointer += 1; },
                    'i'       => { state = State::RdInt; start_pointer += 1; },
                    'l'       => { state = State::RdLst; start_pointer += 1; },
                    'd'       => { state = State::RdDic; start_pointer += 1; },
                    _         => { unexpected = true; }
                }
            },

            State::RdInt => {
                match ch {
                    '0'..='9' => end_pointer += 1,
                    'e' => {
                        end_pointer += 1;
                        break;
                    },
                    _ => { unexpected = true; }
                }
            },

            State::RdStrLen=> {
                match ch {
                    ':' => {
                        state = State::RdStrCon;
                        str_len = match &encoded[start_pointer..end_pointer].parse::<usize>() {
                            Ok(num) => *num,
                            Err(_) => { unexpected = true; 0 },
                        };
                        end_pointer += 1; // now both pointing to first rune
                        start_pointer = end_pointer;
                    },
                    '0'..='9' => { end_pointer += 1; },
                    _ => { unexpected = true; }
                }
            },
            State::RdStrCon=> {
                if str_len > 0 {
                    end_pointer += 1;
                    str_len -= 1;
                } else {
                    break;
                }
            },

            State::RdLst=> {
            },

            State::RdDic=> {},
        }
    };

    if unexpected { 
        return (Err(()), &encoded[start_pointer..]);
    }

    match state {
        State::Ready => {
            return (Err(()), &encoded[start_pointer..]);
        },
        State::RdInt => {
            return (
                match &encoded[start_pointer..end_pointer].parse::<isize>() {
                    Ok(num) => Ok(BencodeObj::Num(*num)),
                    Err(_) => Err(())
                },
                &encoded[end_pointer+1..]
            );
        },
        State::RdStrLen => { return (Err(()), &encoded[start_pointer..]); },
        State::RdStrCon => {
            return (
                Ok(BencodeObj::Str(encoded[start_pointer..end_pointer].to_string())),
                &encoded[end_pointer..]
            );
        },
        State::RdLst => {
            let mut list: Vec<BencodeObj> = vec![];
            let mut remaining = &encoded[start_pointer..];

            loop {
                if remaining.starts_with('e') {
                    return (Ok(BencodeObj::List(list)), &remaining[1..]);
                }

                let (val, rem) = decode(remaining);
                match val {
                    Ok(bencode_obj) => {
                        list.push(bencode_obj);
                    },
                    Err(_) => { return (Err(()), remaining); },
                }
                remaining = rem;
            }
        },
        State::RdDic => {
            let mut dict: HashMap<String, Box<BencodeObj>> = Default::default();

            let mut remaining = &encoded[start_pointer..];

            loop {
                if remaining.starts_with('e') {
                    return (Ok(BencodeObj::Dict(dict)), &remaining[1..]);
                }

                dbg!(remaining);
                let (key, rem) = decode(remaining);
                remaining = rem;
                let (val, rem) = decode(remaining);

                match (key, val) {
                    (Ok(k), Ok(v)) => {
                        match k {
                            BencodeObj::Num(k_num) => {
                                dict.insert(k_num.to_string(), Box::new(v));
                            },
                            BencodeObj::Str(k_str) => {
                                dict.insert(k_str, Box::new(v));
                            },
                            _ => { return (Err(()), remaining); },
                        }
                    },
                    _ => { return (Err(()), remaining); },
                }
                remaining = rem;
            }
        },
    }

    (Err(()), &encoded[start_pointer..])
}
