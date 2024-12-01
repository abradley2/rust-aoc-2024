#[derive(Debug)]
#[allow(dead_code)]
pub enum Error {
    MalformedLine(usize),
    MalformedInt((usize, std::num::ParseIntError)),
}

pub fn parse_input(input: &[u8]) -> Result<(Vec<u32>, Vec<u32>), Error> {
    let mut left_vec = Vec::with_capacity(1000);
    let mut right_vec = Vec::with_capacity(1000);

    let input_str = std::str::from_utf8(input).unwrap();
    for (line_idx, line) in input_str.lines().enumerate() {
        let mut line_words = line.split_whitespace();

        let left_str = line_words
            .next()
            .map(Ok)
            .unwrap_or(Err(Error::MalformedLine(line_idx)))?;
        let left_val = u32::from_str_radix(left_str, 10)
            .map_err(|err| Error::MalformedInt((line_idx, err)))?;

        let right_str = line_words
            .next()
            .map(Ok)
            .unwrap_or(Err(Error::MalformedLine(line_idx)))?;
        let right_val = u32::from_str_radix(right_str, 10)
            .map_err(|err| Error::MalformedInt((line_idx, err)))?;

        left_vec.push(left_val);
        right_vec.push(right_val);
    }

    left_vec.sort();
    right_vec.sort();

    Ok((left_vec, right_vec))
}
