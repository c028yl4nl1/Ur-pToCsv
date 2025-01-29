use crate::core::Parser;

impl Parser {
    /// Extract the query from the url.
    ///
    /// # Example
    /// ```rust
    /// use url_parse::core::Parser;
    /// let input = "https://www.example.co.uk:443/blog/article/search?docid=720&hl=en#dayone";
    /// let result = Parser::new(None).query(input).unwrap();
    /// assert_eq!(result, "docid=720&hl=en");
    /// ```
    pub fn query<'a>(&self, input: &'a str) -> Option<&'a str> {
        let position_questionmark = input.find('?');
        let position_pound = input.find('#');
    
        // Verifica se existe um '?' na string
        if let Some(v) = position_questionmark {
            // Define o índice final como o menor entre a posição do '#' e o comprimento da string
            let end = match position_pound {
                Some(p) if p > v => p, // Se '#' aparece depois de '?', usa a posição do '#'
                _ => input.len(), // Caso contrário, usa o comprimento total da string
            };
    
            // Verifica se end é maior que v + 1 para evitar slice inválido
            if end > v + 1 {
                let after = &input[v + 1..end];
                return Some(after);
            }
        }
        
        None // Retorna None se não houver query string válida
    }
    
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_query_works_when_typical() {
        let input = "https://www.example.co.uk:443/blog/article/search?docid=720&hl=en";
        let result = Parser::new(None).query(input).unwrap();
        assert_eq!(result, "docid=720&hl=en");
    }

    #[test]
    fn test_query_works_when_typical_with_anchor() {
        let input = "https://www.example.co.uk:443/blog/article/search?docid=720&hl=en#dayone";
        let result = Parser::new(None).query(input).unwrap();
        assert_eq!(result, "docid=720&hl=en");
    }
}
