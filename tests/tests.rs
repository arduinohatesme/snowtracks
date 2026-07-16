#[cfg(test)]
mod tests {
    use std::vec;

    use snowtracks::cli;

    #[test]
    fn test_cli() {
        cli().debug_assert();
    }

    #[test]
    fn test_setup_subcommand() {
        let matches = cli()
            .try_get_matches_from(vec!["snow", "setup", "--name", "my_db"])
            .unwrap();

        let (sc_name, sc_matches) = matches.subcommand().unwrap();
        let db_name: &String = sc_matches.get_one("name").unwrap();

        assert_eq!(sc_name, "setup");
        assert_eq!(db_name, "my_db");
    }
}
