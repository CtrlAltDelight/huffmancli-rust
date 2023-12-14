mod tests {
    #[test]
    fn it_works() {
        use huffmancli::frequencies::{calc_frequencies, Frequencies};
        let mut frequencies: Frequencies = [0; 256];
        let _ = calc_frequencies(&mut frequencies, "./tests/frequencies.txt");
        assert_eq!(frequencies['a' as usize], 3);
        assert_eq!(frequencies['b' as usize], 2);
        assert_eq!(frequencies['c' as usize], 1);
        assert_eq!(frequencies['\n' as usize], 1);
    }
}
