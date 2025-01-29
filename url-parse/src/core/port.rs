use regex::Regex;

use crate::core::Parser;
use crate::utils::Utils;

impl Parser {
    /// Extract the port from the url. If no port is present, it will be deduced from the scheme.
    /// The default scheme provides well-known ports. The user can specify new schemes when constructing the Parser object with `new()`.
    ///
    /// # Example
    /// ```rust
    /// use url_parse::core::Parser;
    /// let input = "https://www.example.co.uk:443/blog/article/search?docid=720&hl=en#dayone";
    /// let port = Parser::new(None).port(input);
    /// assert_eq!(port.unwrap(), 443);
    /// ```
    pub fn port(&self, input: &str) -> Option<u32> {
        let rest = Utils::substring_after_login(self, input);
        let position_colon = rest.find(':');
    
        if let Some(v) = position_colon {
            let after = &rest[v + 1..];
            let re = Regex::new(r"^[0-9]+").ok()?; // Cria regex e lida com erro
            let caps = re.captures(after)?;
    
            // Tenta obter a captura e converter para u64
            if let Some(port_str) = caps.get(0) {
                // Tenta converter para u64 primeiro
                if let Ok(port) = port_str.as_str().trim().parse::<u64>() {
                    // Verifica se o valor está dentro do limite de u32
                    if port <= u32::MAX as u64 {
                        return Some(port as u32); // Converte para u32
                    } else {
                        println!("Número excede o limite permitido para u32.");
                        return None; // Retorna None em caso de overflow
                    }
                } else {
                    println!("Erro ao analisar a porta: não é um número válido.");
                    return None; // Retorna None em caso de erro na conversão
                }
            }
        }
    
        // Obtém a porta padrão com segurança
        match self.scheme(input) {
            Some((v, _)) => {
                self.port_mappings.get(&v).map(|&(port, _)| port) // Usando map para evitar unwrap
            }
            None => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_port_works_when_typical() {
        let input = "https://www.example.co.uk:443/blog/article/search?docid=720&hl=en#dayone";
        let port = Parser::new(None).port(input);
        assert_eq!(port.unwrap(), 443);
    }

    #[test]
    fn test_port_works_when_scheme_and_port_specified() {
        let input = "ftp://127.0.0.1:21/test";
        let port = Parser::new(None).port(input);
        assert_eq!(port.unwrap(), 21);
    }

    #[test]
    fn test_port_works_when_no_path() {
        let input = "https://www.example.co.uk:443";
        let port = Parser::new(None).port(input);
        assert_eq!(port.unwrap(), 443);
    }
    #[test]
    fn test_port_default_works_when_https() {
        let input = "https://www.example.co.uk";
        let port = Parser::new(None).port(input);
        assert_eq!(port.unwrap(), 443);
    }

    #[test]
    fn test_port_works_when_default_port_login_and_no_port() {
        let input =
            "https://user:pass@www.example.co.uk/blog/article/search?docid=720&hl=en#dayone";
        let result = Parser::new(None).port(input).unwrap();
        assert_eq!(result, 443);
    }
    #[test]
    fn test_port_works_when_login_and_no_port() {
        let input = "user:pass@www.example.co.uk/blog/article/search?docid=720&hl=en#dayone";
        let result = Parser::new(None).port(input);
        assert!(result.is_none());
    }

    #[test]
    fn test_port_works_when_login_and_no_port_with_numbers() {
        let input = "user:pass@www.example.co.uk/blog/article/720/test.txt";
        let result = Parser::new(None).port(input);
        assert!(result.is_none());
    }

    #[test]
    fn test_port_works_when_colon_in_url() {
        let input = "http://en.wikipedia.org/wiki/Template:Welcome";
        let result = Parser::new(None).port(input);
        assert!(result.is_none());
    }
}
