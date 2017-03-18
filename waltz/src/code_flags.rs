use regex::Regex;

pub fn get_filename(flags: &str) -> Option<String> {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"^(?P<lang>\w+)(,(?P<props>\w+))*(,file=(?P<file>[^,]*))?$").unwrap();
    }

    if let Some(captures) = RE.captures(flags) {
        info!("found code block flags: {:?}", captures);

        if let Some(filename) = captures.name("file") {
            return Some(filename.as_str().to_string());
        }
    }

    None
}

#[cfg(test)]
mod test {
    use super::get_filename;

    macro_rules! flag_check {
        ($flags:expr => None) => {
            assert_eq!(get_filename($flags), None);
        };
        ($flags:expr => $filename:expr) => {
            assert_eq!(get_filename($flags), Some($filename.to_string()));
        };
    }

    #[test]
    fn simple_flags() {
        flag_check!("rust,file=Cargo.toml" => "Cargo.toml");
        flag_check!("rust,file=src/lib.rs" => "src/lib.rs");
        flag_check!("rust,file=../foo/__bar.rs" => "../foo/__bar.rs");
    }

    #[test]
    fn no_filename_in_flags() {
        flag_check!("rust,ignore" => None);
        flag_check!("rust,foo=bar" => None);
    }

    #[test]
    fn all_the_flags() {
        flag_check!("rust,ignore,file=Cargo.toml" => "Cargo.toml");
        flag_check!("rust,norun,file=src/lib.rs" => "src/lib.rs");
    }

    #[test]
    fn no_lang() {
        flag_check!("file=src/lib.rs" => None);
    }
}
