extern crate tempdir;
extern crate unindent;
extern crate assert_cli;

macro_rules! generate_file {
    ($path:expr => $input:expr) => {{
        use std::fs::{File, create_dir_all};
        use std::io::Write;
        use tempdir::TempDir;

        use unindent::unindent;

        let tmpdir = TempDir::new("waltz_cli_test").expect("tempdir new");

        create_dir_all(tmpdir.path().join($path).parent().unwrap()).unwrap();

        let mut f = File::create(tmpdir.path().join($path))
            .expect("error create md file");
        f.write_all(unindent($input).as_bytes())
            .expect("error writing md file");

        tmpdir
    }}
}

macro_rules! do_the_waltz {
    ($input:expr) => {{
        use assert_cli::Assert;

        let tmpdir = generate_file!("input/test.md" => $input);
        let output_dir = tmpdir.path().join("output");

        Assert::main_binary()
            .with_args(&[
                "-vvv",
                tmpdir.path().join("input/test.md").to_str().unwrap(),
                "-o", output_dir.to_str().unwrap(),
            ])
            .succeeds()
            .unwrap();

        (tmpdir, output_dir)
    }}
}

macro_rules! assert_file {
    ($path:expr => $content:expr) => {{
        use std::fs::File;
        use std::io::Read;
        use std::path::Path;

        use unindent::unindent;

        fn get_file(path: &Path) -> String {
            let mut res = String::new();
            let mut f = File::open(path).expect(&format!("no file at {:?}", path));
            f.read_to_string(&mut res).expect(&format!("failed to read {:?}", path));
            res
        }

        assert_eq!(get_file(&$path), unindent($content));
    }}
}
