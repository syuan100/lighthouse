use clap::ArgMatches;
use std::fs::create_dir_all;
use std::path::{Path, PathBuf};

pub fn ensure_dir_exists<P: AsRef<Path>>(path: P) -> Result<(), String> {
    let path = path.as_ref();

    if !path.exists() {
        create_dir_all(path).map_err(|e| format!("Unable to create {:?}: {:?}", path, e))?;
    }

    Ok(())
}

pub fn base_wallet_dir(matches: &ArgMatches, arg: &'static str) -> Result<PathBuf, String> {
    clap_utils::parse_path_with_default_in_home_dir(
        matches,
        arg,
        PathBuf::new().join(".lighthouse").join("wallets"),
    )
}

#[cfg(test)]
mod test {
    use account_utils::strip_off_newlines;

    #[test]
    fn test_strip_off() {
        let expected = "hello world".as_bytes().to_vec();

        assert_eq!(
            strip_off_newlines("hello world\n".as_bytes().to_vec()),
            expected
        );
        assert_eq!(
            strip_off_newlines("hello world\n\n\n\n".as_bytes().to_vec()),
            expected
        );
        assert_eq!(
            strip_off_newlines("hello world\r".as_bytes().to_vec()),
            expected
        );
        assert_eq!(
            strip_off_newlines("hello world\r\r\r\r\r".as_bytes().to_vec()),
            expected
        );
        assert_eq!(
            strip_off_newlines("hello world\r\n".as_bytes().to_vec()),
            expected
        );
        assert_eq!(
            strip_off_newlines("hello world\r\n\r\n".as_bytes().to_vec()),
            expected
        );
        assert_eq!(
            strip_off_newlines("hello world".as_bytes().to_vec()),
            expected
        );
    }
}
