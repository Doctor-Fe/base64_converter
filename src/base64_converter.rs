pub struct Base64Encoder;

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
                result.push('=');
            }
            i += 1;
        }

        for _ in 0..(result.len() % 4) {
            result.push('=');
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
