pub struct Anagram {
    word1: String,
    word2: String,
}

impl Anagram {
    pub fn new() -> Self {
        Self {
            word1: String::new(),
            word2: String::new(),
        }
    }

    pub fn get_words(&mut self, word1: &str, word2: &str) {
        self.word1 = word1.to_string();
        self.word2 = word2.to_string();
    }

    pub fn check_if_anagram(&self) -> bool {
        if self.word1.len() != self.word2.len() {
            return false;
        }
        let mut is_anagram: bool = true;
        let mut vec1: Vec<char> = Vec::new();
        let mut vec2: Vec<char> = Vec::new();
        let mut pos1: usize = 0;

        for i in self.word1.chars() {
            vec1.push(i);
        }

        for i in self.word2.chars() {
            vec2.push(i);
        }
        vec1.sort();
        vec2.sort();

        while pos1 < vec1.len() {
            if vec1[pos1] != vec2[pos1] {
                is_anagram = false;
            }
            pos1 += 1;
        }
        return is_anagram;
    }
}
