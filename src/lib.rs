pub mod anagram {
    use tracing::info;

    pub fn get_anagrams(word: &str) -> Vec<String> {
        let mut result = Vec::new();
        let mut chars: Vec<char> = word.chars().collect();
        let length = chars.len();
        info!("generating all possible passwords");
        generate_anagrams(length, &mut chars, &mut result);

        result
    }

    fn generate_anagrams(length: usize, chars: &mut Vec<char>, result: &mut Vec<String>) {
        if length == 1 || length >= 9 {
            result.push(chars.iter().collect());
            return;
        }

        for i in 0..length {
            generate_anagrams(length - 1, chars, result);
            if length % 2 == 0 {
                chars.swap(i, length - 1);
            } else {
                chars.swap(0, length - 1);
            }
        }

        result.sort();
        result.dedup()
    }

    pub fn generate_passwords() -> Vec<String> {
        let mut passwords = Vec::new();
        let letters = [
            'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q',
            'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z',
        ];

        generate_password_alphabet(&mut passwords, &letters, String::new(), 4);

        passwords
    }

    fn generate_password_alphabet(
        passwords: &mut Vec<String>,
        letters: &[char],
        current_password: String,
        length: usize,
    ) {
        if length == 0 {
            passwords.push(current_password);
            return;
        }

        for letter in letters {
            let new_password = format!("{}{}", current_password, letter);
            generate_password_alphabet(passwords, letters, new_password, length - 1);
        }
    }

    pub const BANNER: &str = r#"
    _                           _                             
   |_)  _   _   _|  _   _ _    |_) _   _  _       _   _ _|  _ 
   | \ (_| | | (_| (_) | | |   |  (_| _> _> \/\/ (_) | (_| _> 
   "#;

    pub fn print_banner() {
        println!("{BANNER}");
    }

    pub const FILE_PATH: &str = "./passwords.txt";
}

#[cfg(test)]
mod anagram_tests {
    use super::anagram::get_anagrams;

    #[test]
    fn test_get_anagrams() {
        let list = get_anagrams("test");

        assert_eq!(12, list.len());
    }
}
