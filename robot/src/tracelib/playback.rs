pub struct PlaybackStruct {
    pub t: f32,
    pub vl: f32,
    pub vr: f32,
}

pub enum ParseError {
    InvalidFormat,
    NaN,
    DataOutofRange,
}

fn get_char_freq(text: &String, find: char) -> u8 {
    let mut count = 0u8;
    for letter in text.chars() {
        if letter == find {
            count += 1;
        }
    }
    count
}

fn get_raw_from_string(text: &String) -> [f32; 3] {
    let mut str_parts = [String::from(""), String::from(""), String::from("")];
    let mut on_index = 0usize;
    let mut ret: [f32; 3] = [0f32, 0f32, 0f32];
    for letter in text.chars() {
        if letter == ',' {
            on_index += 1;
        } else {
            // add new thing onto thing
            str_parts[on_index] = format!("{}{}", str_parts[on_index], letter);
        }
    }
    for i in 0..3 {
        ret[i] = str_parts[i].parse::<f32>().unwrap();
    }
    ret
}

pub fn get_vec(raw: &Vec<String>) -> Result<Vec<PlaybackStruct>, ParseError>  {
    let mut vec: Vec<PlaybackStruct> = vec![];
    for line in raw {
        if get_char_freq(line, ',') != 2 {
            return Err(ParseError::InvalidFormat);
        }
        let parts = get_raw_from_string(&line);
        for part in parts.iter() {
            if part.is_nan() {
                return Err(ParseError::NaN);
            }
            if !(*part <= 1f32 && *part >= -1f32) {
                return Err(ParseError::DataOutofRange);
            }
        }
        vec.push(PlaybackStruct {
            t: parts[0],
            vl: parts[1],
            vr: parts[2],
        });
    }
    Ok(vec)
}
