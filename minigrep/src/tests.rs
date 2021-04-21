#[cfg(test)]
mod tests {
    use crate::helpers;

    #[test]
    fn should_read_file_correctly() {
        let args = vec!["anya".to_string(), "txt.txt".to_string()];
        let config = helpers::Config::new(&args).unwrap();

        let res = helpers::run(&config).unwrap();

        assert_eq!(res, ())
    }

    #[test]
    fn should_create_new_config() {
        let args = vec!["anya".to_string(), "txt.txt".to_string()];
        let config = helpers::Config::new(&args).unwrap();

        assert_eq!(config.filename(), &args[1]);
        assert_eq!(config.query(), &args[0])
    }

    #[test]
    fn should_handle_errors_when_creating_config_correctly() {
        let args = vec!["anya".to_string()];
        let err = helpers::Config::new(&args).unwrap_err();

        assert!(err.contains("File name was not supplied"))
    }

    #[test]
    fn should_return_correct_search_result() {
        let query = "duct";
        let content = "\
        Rust:
safe, fast, productive.
Pick three.
        ";
        assert_eq!(vec!["safe, fast, productive."], helpers::search(query, content))
    }

    #[test]
    fn should_be_case_insensitive() {
        let query = "Duct";
        let content = "\
        Rust:
safe, fast, productive.
Pick three.
        ";
        assert_eq!(vec!["safe, fast, productive."], helpers::search_case_insenitive(query, content))
    }
}
