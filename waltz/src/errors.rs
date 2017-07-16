use code_flags;

error_chain! {
    types {
        Error, ErrorKind, ResultExt, Result;
    }

    links {
        CodeFlags(code_flags::Error, code_flags::ErrorKind);
    }

    foreign_links {
        Io(::std::io::Error);
    }
}
