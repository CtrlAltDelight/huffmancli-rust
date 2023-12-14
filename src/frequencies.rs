use std::fs;
use std::io::{self, Read, BufReader};

pub type Frequencies = [u64; 256];

pub fn calc_frequencies(freqs: &mut Frequencies, path: &str) -> io::Result<()>{
    let file = fs::File::open(path)?;
    let reader = BufReader::new(file);

    for byte_result in reader.bytes() {
        let byte: u8 = byte_result?;
        freqs[byte as usize] += 1;
    }

    Ok(())
}
