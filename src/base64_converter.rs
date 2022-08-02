use std::io::{Error, ErrorKind};

pub struct Base64Encoder;
pub struct Base64Decoder;

impl Base64Encoder {
    pub fn encode(data: Vec<u8>) -> String {
        let mut d = data.clone();
        for _ in 0..(3 - d.len() % 3) {
            d.push(0);
        }
        let iter = d.chunks(3);
        let mut tmp: Vec<u8> = Vec::new();
        for t in iter {
            let mut a = Base64Encoder::convert_chunk(t);
            tmp.append(a.as_mut());
        }

        let mut result: String = String::new();
        let mut i = 0;
        for t in tmp {
            if (i as f64) < ((data.len() * 4) as f64 / 3.0).ceil() {
                result.push(Base64Encoder::convert_to_char(t));
            } else {
                break;
            }
            i += 1;
        }

        if result.len() % 4 != 0 {
            for _ in 0..(4 - result.len() % 4) {
                result.push('=');
            }
        }

        return result;
    }

    fn convert_chunk(chunk: &[u8]) -> Vec<u8> {
        return vec![
            chunk[0] >> 2,
            (chunk[0] & 3) << 4 | chunk[1] >> 4,
            (chunk[1] & 15) << 2 | chunk[2] >> 6,
            chunk[2] & 63,
        ];
    }

    fn convert_to_char(d: u8) -> char {
        if d <= 25 {
            return (d + 65) as char;
        } else if d <= 51 {
            return (d + 71) as char;
        } else if d <= 61 {
            return (d - 4) as char;
        } else {
            return (43 as u8 + (if d & 1 == 0 { 0 } else { 4 })) as char;
        }
    }
}

impl Base64Decoder {
    pub fn decode(string: String) -> Result<Vec<u8>, Error> {
        match Base64Decoder::convert_all(string) {
            Ok(iter) => {
                let mut result: Vec<u8> = Vec::new();
                for t in iter.chunks(4) {
                    match Base64Decoder::convert_chunk(t.to_vec()) {
                        Ok(mut tmp) => {
                            result.append(&mut tmp);
                        }
                        Err(e) => {
                            return Err(e);
                        }
                    }
                }
                return Ok(result);
            }
            Err(error) => {
                return Err(error);
            }
        }
    }

    fn convert_chunk(chunk: Vec<u8>) -> Result<Vec<u8>, Error> {
        if chunk[1] == 64 || chunk[0] == 64 {
            return Err(Error::new(ErrorKind::InvalidInput, "Invalid format."));
        } else if chunk[2] == 64 {
            return Ok(vec![chunk[0] << 2 | chunk[1] >> 4]);
        } else if chunk[3] == 64 {
            return Ok(vec![
                chunk[0] << 2 | chunk[1] >> 4,
                (chunk[1] & 15) << 4 | chunk[2] >> 2,
            ]);
        } else {
            return Ok(vec![
                chunk[0] << 2 | chunk[1] >> 4,
                (chunk[1] & 15) << 4 | chunk[2] >> 2,
                (chunk[2] & 3) << 6 | chunk[3],
            ]);
        }
    }

    fn convert_all(s: String) -> Result<Vec<u8>, Error> {
        let mut result: Vec<u8> = Vec::new();
        for c in s.as_bytes() {
            match Base64Decoder::convert_to_bin(*c) {
                Ok(p) => {
                    result.push(p);
                }
                Err(e) => {
                    return Err(e);
                }
            }
        }

        return Ok(result);
    }

    fn convert_to_bin(d: u8) -> Result<u8, Error> {
        if 65 <= d && d <= 0x5a {
            return Ok(d - 65);
        } else if 0x61 <= d && d <= 0x7a {
            return Ok(d - 71);
        } else if 48 <= d && d <= 58 {
            return Ok(d + 4);
        } else if d == 0x2b {
            return Ok(62);
        } else if d == 0x2f {
            return Ok(63);
        } else if d == 0x3d {
            return Ok(64);
        } else {
            return Err(Error::new(
                ErrorKind::InvalidInput,
                format!("There was invalid number \'{}\'", d),
            ));
        }
    }
}
