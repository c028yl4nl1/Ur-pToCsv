use crate::core::scheme_separator::SchemeSeparator;
use crate::core::Parser;
use std::collections::HashMap;
pub struct Utils;

impl Utils {
    /// Get substring immediately after scheme.
    ///
    /// # Example
    /// ```rust
    /// use url_parse::utils::Utils;
    /// use url_parse::core::Parser;
    /// let input =
    ///     "https://user:pass@www.example.co.uk:443/blog/article/search?docid=720&hl=en#dayone";
    /// let expected =
    ///     "user:pass@www.example.co.uk:443/blog/article/search?docid=720&hl=en#dayone".to_string();
    /// let parser = Parser::new(None);
    /// let result = Utils::substring_after_scheme(&parser, input);
    /// assert_eq!(result, expected);
    /// ```
    pub fn substring_after_scheme<'a>(parser: &Parser, input: &'a str) -> &'a str {
        let scheme = parser.scheme(input);
        match scheme {
            Some((v, separator)) => {
                let start_index = v.len() + <SchemeSeparator as Into<usize>>::into(separator);
                // Verifica se o índice está dentro dos limites da string
                if start_index < input.len() {
                    &input[start_index..] // Retorna a parte após o esquema
                } else {
                    "" // Retorna uma string vazia se o índice estiver fora dos limites
                }
            },
            None => input, // Se não houver esquema, retorna a string original
        }
    }
    
    

    /// Get substring immediately after login. Eliminates scheme to ensure no colon present in remainder.
    ///
    /// # Example
    /// ```rust
    /// use url_parse::utils::Utils;
    /// use url_parse::core::Parser;
    /// let input =
    ///     "https://user:pass@www.example.co.uk:443/blog/article/search?docid=720&hl=en#dayone";
    /// let expected = "www.example.co.uk:443/blog/article/search?docid=720&hl=en#dayone".to_string();
    /// let parser = Parser::new(None);
    /// let result = Utils::substring_after_login(&parser, input);
    /// assert_eq!(result, expected);
    /// ```
    pub fn substring_after_login<'a>(parser: &Parser, input: &'a str) -> &'a str {
        let input = Utils::substring_after_scheme(parser, input);
        match input.find('@') {
            Some(pos) => &input[pos + 1..],
            None => input,
        }
    }

    /// Get substring immediately after port. Eliminates scheme to ensure no colon present in remainder.
    ///
    /// # Example
    /// ```rust
    /// use url_parse::utils::Utils;
    /// use url_parse::core::Parser;
    /// let input =
    ///     "https://user:pass@www.example.co.uk:443/blog/article/search?docid=720&hl=en#dayone";
    /// let expected = "www.example.co.uk:443/blog/article/search?docid=720&hl=en#dayone".to_string();
    /// let parser = Parser::new(None);
    /// let result = Utils::substring_after_login(&parser, input);
    /// assert_eq!(result, expected);
    /// ```
    pub fn substring_after_port<'a>(parser: &Parser, input: &'a str) -> &'a str {
        let input = Utils::substring_after_scheme(parser, input);
        let port = parser.port(input);

        if input.find(':').is_some() {
            let (pos_port, len_port_string) = match port {
                Some(v) => (input.find(&v.to_string()).unwrap(), v.to_string().len() + 1),
                None => (0, 0),
            };

            let substring_after_port = input.get(pos_port + len_port_string..);
            return substring_after_port.unwrap_or_default();
        }
        input
    }

    /// Get substring immediately before port.
    ///
    /// # Example
    /// ```rust
    /// use url_parse::utils::Utils;
    /// use url_parse::core::Parser;
    /// let input = "https://www.example.co.uk:443/blog/article/search?docid=720&hl=en#dayone";
    /// let expected = "https://www.example.co.uk".to_string();
    /// let parser = Parser::new(None);
    /// let result = Utils::substring_before_port(&parser, input);
    /// assert_eq!(result, expected);
    /// ```
    pub fn substring_before_port<'a>(parser: &Parser, input: &'a str) -> &'a str {
        let port = parser.port(input);
    
        // Determine a posição do port
        let pos_port = match port {
            Some(v) => {
                // Tenta encontrar a posição do port; se não for encontrado, retorna o comprimento da string
                match input.find(&v.to_string()) {
                    Some(pos) if pos > 0 => pos - 1, // Subtrai 1 se a posição for maior que 0
                    _ => return "", // Retorna uma string vazia se não encontrar ou se pos for 0
                }
            },
            None => input.len(), // Se não houver port, usa o comprimento total da string
        };
    
        // Retorna a substring antes da posição do port, se for válida
        if pos_port > 0 { // Verifica se pos_port é maior que 0 para evitar slice inválido
            &input[..pos_port]
        } else {
            "" // Retorna uma string vazia se pos_port não for válido
        }
    }
    

    /// Get substring starting at path field. Eliminates scheme to ensure no colon present in remainder.
    ///
    /// # Example
    /// ```rust
    /// use url_parse::utils::Utils;
    /// use url_parse::core::Parser;
    /// let input =
    ///     "https://user:pass@www.example.co.uk:443/blog/article/search?docid=720&hl=en#dayone";
    /// let expected =
    ///     "/blog/article/search?docid=720&hl=en#dayone".to_string();
    /// let parser = Parser::new(None);
    /// let result = Utils::substring_from_path_begin(&parser, input).unwrap_or("");
    /// assert_eq!(result, expected);
    /// ```
    pub fn substring_from_path_begin<'a>(parser: &Parser, input: &'a str) -> Option<&'a str> {
        let input = Utils::substring_after_scheme(parser, input);
        match input.find('/') {
            Some(pos) => Some(&input[pos..]),
            None => None,
        }
    }

    /// Partially matches a subpath in a path. Useful for i.e. GitHub absolute paths from URL hrefs.
    /// # Example
    /// ```rust
    /// use url_parse::utils::Utils;
    /// use url_parse::core::Parser;
    /// let input = "https://github.com/mihaigalos/aim/releases/tag/1.5.4";
    /// let subpath = "mihaigalos/aim/releases/download/1.5.4/aim-1.5.4-x86_64-unknown-linux-gnu.tar.gz";
    /// let expected = "https://github.com/mihaigalos/aim/releases/download/1.5.4/aim-1.5.4-x86_64-unknown-linux-gnu.tar.gz";
    /// let result = Utils::canonicalize(&Parser::new(None), input, subpath);
    /// assert_eq!(result, expected);
    pub fn canonicalize<'a>(parser: &Parser, input: &'a str, subpath: &'a str) -> String {
        let mut result = parser
            .scheme(input)
            .map(|s| s.0.to_string() + &<SchemeSeparator as Into<String>>::into(s.1))
            .unwrap_or_default();

        let subpath = Self::trim_leading_slash(subpath);
        let (similarity, input_splits) = Utils::compute_similarity(parser, input, subpath);
        let key_with_max_value = similarity.iter().max_by_key(|entry| entry.1).unwrap().0;

        result += &input_splits[0..*key_with_max_value].join("/");
        if *key_with_max_value != 0 || input.is_empty() {
            result += "/";
        }
        result += subpath;

        result
    }

    fn compute_similarity<'a>(
        parser: &Parser,
        input: &'a str,
        subpath: &'a str,
    ) -> (HashMap<usize, usize>, Vec<&'a str>) {
        let input = Utils::substring_after_scheme(parser, input);
        let input_splits = input.split('/').collect::<Vec<&str>>();
        let subpath_splits = subpath.split('/').collect::<Vec<&str>>();

        let mut similarity: HashMap<usize, usize> = HashMap::new();
        let mut pos_subpath = 0;
        let mut pos_match = 0;
        for (pos_input, input_split) in input_splits.iter().enumerate() {
            if input_split == &subpath_splits[pos_subpath] {
                if pos_subpath == 0 {
                    pos_match = pos_input;
                }
                pos_subpath += 1;
                *similarity.entry(pos_match).or_insert(0) += 1;
            } else {
                pos_subpath = 0;
            }
        }
        (similarity, input_splits)
    }

    fn trim_leading_slash(subpath: &str) -> &str {
        if subpath.starts_with('/') {
            return &subpath[1..subpath.len()];
        }
        subpath
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_substring_after_scheme_works_when_typical() {
        let input =
            "https://user:pass@www.example.co.uk:443/blog/article/search?docid=720&hl=en#dayone";
        let expected = "user:pass@www.example.co.uk:443/blog/article/search?docid=720&hl=en#dayone"
            .to_string();
        let parser = Parser::new(None);
        let result = Utils::substring_after_scheme(&parser, input);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_substring_after_scheme_works_when_simple_schema() {
        let input =
            "https:user:pass@www.example.co.uk:443/blog/article/search?docid=720&hl=en#dayone";
        let expected = "user:pass@www.example.co.uk:443/blog/article/search?docid=720&hl=en#dayone"
            .to_string();
        let parser = Parser::new(None);
        let result = Utils::substring_after_scheme(&parser, input);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_substring_after_port_works_when_typical() {
        let input = "https://www.example.co.uk:443/blog/article/search?docid=720&hl=en#dayone";
        let expected = "blog/article/search?docid=720&hl=en#dayone".to_string();
        let parser = Parser::new(None);
        let result = Utils::substring_after_port(&parser, input);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_substring_after_port_works_when_no_scheme() {
        let input = "user:pass@www.example.co.uk:443/blog/article/search?docid=720&hl=en#dayone";
        let expected = "blog/article/search?docid=720&hl=en#dayone".to_string();
        let parser = Parser::new(None);
        let result = Utils::substring_after_port(&parser, input);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_substring_before_port_works_when_typical() {
        let input = "https://www.example.co.uk:443/blog/article/search?docid=720&hl=en#dayone";
        let expected = "https://www.example.co.uk".to_string();
        let parser = Parser::new(None);
        let result = Utils::substring_before_port(&parser, input);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_substring_after_login_works_when_typical() {
        let input =
            "https://user:pass@www.example.co.uk:443/blog/article/search?docid=720&hl=en#dayone";
        let expected =
            "www.example.co.uk:443/blog/article/search?docid=720&hl=en#dayone".to_string();
        let parser = Parser::new(None);
        let result = Utils::substring_after_login(&parser, input);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_substring_from_path_begin_works_when_typical() {
        let input = "https://www.example.co.uk:443/blog/article/search?docid=720&hl=en#dayone";
        let expected = "/blog/article/search?docid=720&hl=en#dayone".to_string();
        let parser = Parser::new(None);
        let result = Utils::substring_from_path_begin(&parser, input).unwrap();
        assert_eq!(result, expected);
    }

    #[test]
    fn test_substring_from_path_begin_works_when_no_port() {
        let input = "https://www.example.co.uk/blog/article/search?docid=720&hl=en#dayone";
        let expected = "/blog/article/search?docid=720&hl=en#dayone".to_string();
        let parser = Parser::new(None);
        let result = Utils::substring_from_path_begin(&parser, input).unwrap();
        assert_eq!(result, expected);
    }

    #[test]
    fn test_substring_after_port_works_when_colon_in_url() {
        let input = "http://en.wikipedia.org/wiki/Template:Welcome";
        let expected = "en.wikipedia.org/wiki/Template:Welcome".to_string();
        let parser = Parser::new(None);
        let result = Utils::substring_after_port(&parser, input);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_substring_after_port_works_when_nothing_after_port() {
        let input = "http://192.168.0.100:8080";
        let expected = "".to_string();
        let parser = Parser::new(None);
        let result = Utils::substring_after_port(&parser, input);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_compute_similarity_hashmap_works_when_typical() {
        let input = "https://github.com/mihaigalos/aim/releases/tag/1.5.4";
        let subpath =
            "mihaigalos/aim/releases/download/1.5.4/aim-1.5.4-x86_64-unknown-linux-gnu.tar.gz";
        let expected_pos_begin_match: usize = 1;
        let expected_count_path_matches: usize = 3;

        let parser = Parser::new(None);
        let (hashmap, _) = Utils::compute_similarity(&parser, input, subpath);
        assert_eq!(
            hashmap[&expected_pos_begin_match],
            expected_count_path_matches
        );
    }

    #[test]
    fn test_compute_similarity_input_splits_works_when_typical() {
        let input = "https://github.com/mihaigalos/aim/releases/tag/1.5.4";
        let subpath =
            "mihaigalos/aim/releases/download/1.5.4/aim-1.5.4-x86_64-unknown-linux-gnu.tar.gz";
        let expected_input_splits: Vec<&str> = vec![
            "github.com",
            "mihaigalos",
            "aim",
            "releases",
            "tag",
            "1.5.4",
        ];

        let parser = Parser::new(None);
        let (_, input_splits) = Utils::compute_similarity(&parser, input, subpath);
        assert_eq!(input_splits, expected_input_splits);
    }

    #[test]
    fn test_compute_similarity_works_when_multiple_partial_matches() {
        let input = "https://github.com/mihaigalos/aim/fake/path/mihaigalos/aim/releases/tag/1.5.4";
        let subpath =
            "mihaigalos/aim/releases/download/1.5.4/aim-1.5.4-x86_64-unknown-linux-gnu.tar.gz";
        let expected_pos_begin_match: usize = 5;
        let expected_count_path_matches: usize = 3;

        let parser = Parser::new(None);
        let (hashmap, _) = Utils::compute_similarity(&parser, input, subpath);
        assert_eq!(
            hashmap[&expected_pos_begin_match],
            expected_count_path_matches
        );
    }

    #[test]
    fn test_canonicalize_works_when_typical() {
        let input = "https://github.com/mihaigalos/aim/releases/tag/1.5.4";
        let subpath =
            "mihaigalos/aim/releases/download/1.5.4/aim-1.5.4-x86_64-unknown-linux-gnu.tar.gz";
        let expected = "https://github.com/mihaigalos/aim/releases/download/1.5.4/aim-1.5.4-x86_64-unknown-linux-gnu.tar.gz";

        let parser = Parser::new(None);
        let result = Utils::canonicalize(&parser, input, subpath);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_canonicalize_works_when_domain_with_path_and_no_scheme() {
        let input = "https://github.com/mihaigalos/aim/releases/tag/1.5.4";
        let subpath =
            "github.com/mihaigalos/aim/releases/download/1.5.4/aim-1.5.4-x86_64-unknown-linux-gnu.tar.gz";
        let expected = "https://github.com/mihaigalos/aim/releases/download/1.5.4/aim-1.5.4-x86_64-unknown-linux-gnu.tar.gz";

        let parser = Parser::new(None);
        let result = Utils::canonicalize(&parser, input, subpath);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_canonicalize_works_when_multiple_partial_matches() {
        let input = "https://github.com/mihaigalos/aim/fake/path/mihaigalos/aim/releases/tag/1.5.4";
        let subpath =
            "mihaigalos/aim/releases/download/1.5.4/aim-1.5.4-x86_64-unknown-linux-gnu.tar.gz";
        let expected = "https://github.com/mihaigalos/aim/fake/path/mihaigalos/aim/releases/download/1.5.4/aim-1.5.4-x86_64-unknown-linux-gnu.tar.gz";

        let parser = Parser::new(None);
        let result = Utils::canonicalize(&parser, input, subpath);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_canonicalize_works_when_scheme_with_colon() {
        let input = "https:github.com/mihaigalos/aim/fake/path/mihaigalos/aim/releases/tag/1.5.4";
        let subpath =
            "mihaigalos/aim/releases/download/1.5.4/aim-1.5.4-x86_64-unknown-linux-gnu.tar.gz";
        let expected = "https:github.com/mihaigalos/aim/fake/path/mihaigalos/aim/releases/download/1.5.4/aim-1.5.4-x86_64-unknown-linux-gnu.tar.gz";

        let parser = Parser::new(None);
        let result = Utils::canonicalize(&parser, input, subpath);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_canonicalize_works_when_no_scheme() {
        let input = "github.com/mihaigalos/aim/fake/path/mihaigalos/aim/releases/tag/1.5.4";
        let subpath =
            "mihaigalos/aim/releases/download/1.5.4/aim-1.5.4-x86_64-unknown-linux-gnu.tar.gz";
        let expected = "github.com/mihaigalos/aim/fake/path/mihaigalos/aim/releases/download/1.5.4/aim-1.5.4-x86_64-unknown-linux-gnu.tar.gz";

        let parser = Parser::new(None);
        let result = Utils::canonicalize(&parser, input, subpath);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_canonicalize_works_when_no_scheme_and_path_begins_with_slash() {
        let input = "github.com/mihaigalos/aim/fake/path/mihaigalos/aim/releases/tag/1.5.4";
        let subpath =
            "/mihaigalos/aim/releases/download/1.5.4/aim-1.5.4-x86_64-unknown-linux-gnu.tar.gz";
        let expected = "github.com/mihaigalos/aim/fake/path/mihaigalos/aim/releases/download/1.5.4/aim-1.5.4-x86_64-unknown-linux-gnu.tar.gz";

        let parser = Parser::new(None);
        let result = Utils::canonicalize(&parser, input, subpath);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_canonicalize_works_when_empty() {
        let input = "";
        let subpath = "";
        let expected = "/";

        let parser = Parser::new(None);
        let result = Utils::canonicalize(&parser, input, subpath);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_trim_leading_slash_works_when_typical() {
        let input =
            "/mihaigalos/aim/releases/download/1.5.4/aim-1.5.4-x86_64-unknown-linux-gnu.tar.gz";
        let expected =
            "mihaigalos/aim/releases/download/1.5.4/aim-1.5.4-x86_64-unknown-linux-gnu.tar.gz";

        let result = Utils::trim_leading_slash(input);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_trim_leading_slash_works_when_no_leading_slash() {
        let input =
            "mihaigalos/aim/releases/download/1.5.4/aim-1.5.4-x86_64-unknown-linux-gnu.tar.gz";
        let expected =
            "mihaigalos/aim/releases/download/1.5.4/aim-1.5.4-x86_64-unknown-linux-gnu.tar.gz";

        let result = Utils::trim_leading_slash(input);
        assert_eq!(result, expected);
    }
}
