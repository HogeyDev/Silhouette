pub struct Lexer {
    contents: String,
    character: char,
    index: usize,
}

impl Lexer {
    pub fn from(contents: String) -> Self {
        assert_ne!(contents.len(), 0);
        Self {
            contents: contents.clone(),
            character: contents.chars().nth(0).unwrap(),
            index: 0,
        }
    }
    fn advance(&mut self) {
        self.index += 1;
        self.character = self.contents.chars().nth(self.index).unwrap_or('\0');
    }
    fn peek(&self, offset: i64) -> Option<char> {
        let real: i64 = self.index as i64 + offset;
        if real < 0 || real as usize >= self.contents.len() {
            return None;
        }
        self.contents.chars().nth(real as usize)
    }
    fn skip_whitespace(&mut self) {
        while self.character.is_ascii_whitespace() {
            self.advance();
        }
    }
    fn identifier(&mut self) -> String {
        let mut id = String::new();
        while self.character.is_alphanumeric() {
            id.push(self.character);
            self.advance();
        }
        id
    }
    fn number(&mut self) -> String {
        let mut num = String::new();
        while self.character.is_numeric() {
            num.push(self.character);
            self.advance();
        }
        num
    }
    fn get_token(&mut self) -> Option<String> {
        if self.character == '\0' {
            return None;
        }
        self.skip_whitespace();
        if self.character.is_alphabetic() {
            return Some(self.identifier());
        }
        if self.character.is_numeric() {
            return Some(self.number());
        }
        let value = self.character;
        self.advance();
        // if self.index % 1000 <= 5 {
        //     eprintln!("{}: {}", self.index, self.character);
        // }
        Some(value.to_string())
    }
    pub fn tokens(&mut self) -> Vec<String> {
        std::iter::from_fn(|| self.get_token()).collect()
    }
}
