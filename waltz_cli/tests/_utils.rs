extern crate tempdir;
extern crate unindent;
extern crate assert_cli;

use std::fs::{File, create_dir_all};
use std::io::{Read, Write};
use std::path::PathBuf;
use std::default::Default;
use tempdir::TempDir;

use unindent::unindent;
use assert_cli::Assert as CliAssert;

fn given(file_content: &str) -> Assert {
    Assert::with_file(file_content)
}

fn file(path: &str) -> FileAssert {
    FileAssert::with_path(path)
}

#[derive(Debug)]
struct Assert {
    tmpdir: TempDir,
    output_dir: PathBuf,
}

impl Default for Assert {
    fn default() -> Self {
        let tmpdir = TempDir::new("waltz_cli_test").expect("tempdir new");
        let output_dir = tmpdir.path().join("output").to_owned();

        Assert {
            tmpdir: tmpdir,
            output_dir: output_dir,
        }
    }
}

impl Assert {
    fn with_file(content: &str) -> Self {
        let a = Assert::default();

        create_dir_all(&a.output_dir)
            .expect("error creating output dir");

        let mut f = File::create(a.output_dir.join("test.md"))
            .expect("error create md file");
        f.write_all(unindent(content).as_bytes())
            .expect("error writing md file");

        a
    }

    fn waltz(&self) -> &Self {
        CliAssert::main_binary()
            .with_args(&[
                "-vvv",
                self.output_dir.join("test.md").to_str().unwrap(),
                "-o", self.output_dir.to_str().unwrap(),
            ])
            .succeeds()
            .unwrap();
        self
    }

    fn creates(&self, fa: FileAssert) -> &Self {
        fa.context(self.output_dir.to_owned())
            .unwrap();
        self
    }

    fn cargo_run<F>(&self, cli_assertions: F) where
        F: Fn(CliAssert) -> CliAssert,
     {
        let cmd = CliAssert::command(&[
            "cargo", "run",
            "--manifest-path", self.output_dir.join("Cargo.toml").to_str().unwrap(),
        ]);

        cli_assertions(cmd).unwrap();
    }
}

#[derive(Debug)]
struct FileAssert {
    path: String,
    content: Option<String>,
    working_dir: Option<PathBuf>,
}

impl FileAssert {
    fn with_path<I: Into<String>>(path: I) -> Self {
        Self {
            path: path.into(),
            content: None,
            working_dir: None,
        }
    }

    fn containing<I: Into<String>>(mut self, content: I) -> Self {
        self.content = Some(content.into());
        self
    }

    fn context<I: Into<PathBuf>>(mut self, dir: I) -> Self {
        self.working_dir = Some(dir.into());
        self
    }

    fn unwrap(self) {
        let dir = self.working_dir.expect(&format!("No working dir set for `{}`", self.path));
        let path = PathBuf::from(dir).join(self.path);

        let mut f = File::open(&path).expect(&format!("no file at {:?}", path));

        if let Some(expected_content) = self.content {
            let mut content = String::new();
            f.read_to_string(&mut content).expect(&format!("failed to read {:?}", path));

            assert_eq!(content, unindent(&expected_content));
        }
    }
}
