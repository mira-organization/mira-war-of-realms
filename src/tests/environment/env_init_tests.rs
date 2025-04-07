#[cfg(test)]
mod tests {
    use environment_lib::environment::env_init::load_environments;

    #[test]
    fn test_load_environments_reads_directory_correctly() {

        let result = load_environments();

        assert_eq!(result.len(), 1);
    }
}