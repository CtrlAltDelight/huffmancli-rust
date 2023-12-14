use std::fs;

pub type Frequencies = [u64; 256];

pub fn calc_frequencies(freqs: &mut Frequencies, path: &str) {
    let content = fs::read_to_string(path).expect("Could not read the file.");
    for byte in content.bytes() {
        freqs[byte as usize] += 1;
    }
}
