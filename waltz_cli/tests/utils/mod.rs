#![allow(dead_code)]

extern crate tempdir;
extern crate unindent;
extern crate assert_cli;
extern crate difference;

use std::fs::{File, create_dir_all};
use std::io::{Read, Write};
use std::path::{Path, PathBuf};
use std::default::Default;

use self::tempdir::TempDir;
use self::unindent::unindent;
use self::assert_cli::Assert as CliAssert;
use self::difference::Changeset;

pub fn given(file_content: &str) -> Assert {
    Assert::with_file(file_content)
}

pub fn file(path: &str) -> FileAssert {
    FileAssert::with_path(path)
}

pub fn waltz(cwd: &Path) -> CliAssert {
    CliAssert::main_binary()
        .with_args(&["-vvv", cwd.join("test.md").to_str().unwrap(), "-o", cwd.to_str().unwrap()],)
        .succeeds()
}

pub fn main(cwd: &Path) -> CliAssert {
    CliAssert::command(&["cargo", "run", "--manifest-path", cwd.join("Cargo.toml").to_str().unwrap()],)
}

pub fn binary(cwd: &Path, name: &str) -> CliAssert {
    CliAssert::command(
        &[
            "cargo",
            "run",
            "--manifest-path",
            cwd.join("Cargo.toml").to_str().unwrap(),
            "--bin",
            name,
        ],
    )
}

fn cargo(cwd: &Path, subcommand: &str) -> CliAssert {
    CliAssert::command(&["cargo", subcommand, "--manifest-path", cwd.join("Cargo.toml").to_str().unwrap()],)
}

pub fn cargo_check(cwd: &Path) -> CliAssert {
    cargo(cwd, "check")
}

pub fn cargo_test(cwd: &Path) -> CliAssert {
    cargo(cwd, "test")
}

#[derive(Debug)]
pub struct Assert {
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

        create_dir_all(&a.output_dir).expect("error creating output dir");

        let mut f = File::create(a.output_dir.join("test.md")).expect("error create md file");
        f.write_all(unindent(content).as_bytes())
            .expect("error writing md file");

        a
    }

    pub fn running<F>(&self, cmd: F) -> &Self
    where
        F: for<'cwd> Fn(&'cwd Path) -> CliAssert,
    {
        cmd(&self.output_dir).unwrap();
        self
    }

    pub fn creates(&self, fa: FileAssert) -> &Self {
        fa.context(self.output_dir.to_owned()).unwrap();
        self
    }
}

#[derive(Debug)]
pub struct FileAssert {
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

    pub fn containing<I: Into<String>>(mut self, content: I) -> Self {
        self.content = Some(content.into());
        self
    }

    fn context<I: Into<PathBuf>>(mut self, dir: I) -> Self {
        self.working_dir = Some(dir.into());
        self
    }

    fn unwrap(self) {
        let dir = self.working_dir
            .expect(&format!("No working dir set for `{}`", self.path));
        let path = PathBuf::from(dir).join(&self.path);

        let mut f = File::open(&path).expect(&format!("no file at {:?}", path));

        if let Some(expected_content) = self.content {
            let mut content = String::new();
            f.read_to_string(&mut content)
                .expect(&format!("failed to read {:?}", path));

            let diff = Changeset::new(&content, &unindent(&expected_content), "\n");
            if diff.distance > 0 {
                panic!("Content of `{}` not as expected:\n{}", self.path, diff);
            }
        }
    }
}
