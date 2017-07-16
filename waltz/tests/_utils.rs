extern crate tempdir;
extern crate waltz;
extern crate pulldown_cmark;
extern crate unindent;

#[allow(unused_macros)]
macro_rules! assert_files_generated {
    ($name:ident: $input:expr => [$($filename:expr => $content:expr),+]) => {
        #[test]
        fn $name() {
            use std::fs::File;
            use std::io::Read;
            use std::path::Path;
            use tempdir::TempDir;

            use unindent::unindent;

            fn get_file(path: &Path) -> String {
                let mut res = String::new();
                let mut f = File::open(path).expect(&format!("no file at {:?}", path));
                f.read_to_string(&mut res).expect(&format!("failed to read {:?}", path));
                res
            }

            let input = unindent($input);
            let markdown = pulldown_cmark::Parser::new(&input);
            let code_blocks = waltz::extract_code_blocks(markdown).expect("extract_code_blocks");

            let tmp_dir = TempDir::new("waltz_to_file").expect("tempdir new");

            for codeblock in code_blocks.iter().filter(|cb| cb.has_filename()) {
                codeblock.to_file(tmp_dir.path()).expect(&format!("failed to write {:?}", codeblock));
            }

            $(
                assert_eq!(get_file(&tmp_dir.path().join($filename)), unindent($content));
            )*
        }
    }
}
