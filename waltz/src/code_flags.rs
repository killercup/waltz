use std::str::FromStr;

#[derive(Debug, Clone, PartialEq, Hash)]
pub struct CodeFlags {
    lang: String,
    filename: Option<String>,
    run: Option<String>,
}

impl CodeFlags {
    pub fn lang(&self) -> &str {
        &self.lang
    }

    pub fn filename(&self) -> Option<String> {
        self.filename.clone()
    }

    pub fn run(&self) -> Option<String> {
        self.run.clone()
    }
}

error_chain! {
    types {
        Error, ErrorKind, ResultExt, CodeFlagsResult;
    }

    errors {
        NoFlags {
            description("Code block has no flags")
        }
        EmptyFilename {
            description("File name attribute exists but is empty")
        }
        DuplicateFilename {
           description("File name flag found twice")
        }
        DuplicateRun {
           description("Run flag found twice")
        }
    }
}

impl FromStr for CodeFlags {
    type Err = Error;

    fn from_str(flags: &str) -> Result<Self, Self::Err> {
        let mut flags = flags.split(',');
        let lang = flags.next().map(str::to_string).ok_or(ErrorKind::NoFlags)?;
        let mut filename = None;
        let mut run = None;

        for flag in flags {
            if let Some(f) = flag.splitn(2, "file=").nth(1) {
                ensure!(filename.is_none(), ErrorKind::DuplicateFilename);
                ensure!(!f.is_empty(), ErrorKind::EmptyFilename);
                filename = Some(f.to_string());
            }

            // Might want to allow `run` as well as `run=bash` later
            let run_prefix = "run=";
            if flag.starts_with(run_prefix) {
                ensure!(filename.is_none(), ErrorKind::DuplicateRun);
                let r = &flag[run_prefix.len()..];
                run = Some(r.to_string());
            }
        }

        Ok(CodeFlags {
            lang,
            filename,
            run,
        })
    }
}

#[cfg(test)]
mod test {
    use super::CodeFlags;

    macro_rules! flag_check {
        ($flags:expr => None) => {
            assert_eq!(
                $flags.parse::<CodeFlags>().unwrap().filename(),
                None
            );
        };
        ($flags:expr => $filename:expr) => {
            assert_eq!(
                $flags.parse::<CodeFlags>().unwrap().filename(),
                Some($filename.to_string())
            );
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
