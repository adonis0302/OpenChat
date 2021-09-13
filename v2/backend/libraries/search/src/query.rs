pub struct Query {
    pub tokens: Vec<Token>,
}

pub struct Token {
    pub value: String,
    pub value_lower: String,
}

impl Query {
    pub fn parse(free_text: &str) -> Query {
        Query {
            tokens: free_text
                .split_whitespace()
                .map(|word| Token {
                    value: word.to_string(),
                    value_lower: word.to_lowercase(),
                })
                .collect(),
        }
    }
}
