error_chain! {
    foreign_links {
        Utf8(::std::str::Utf8Error);
        FromUtf16(::std::string::FromUtf16Error);
        Io(::std::io::Error);
        Nix(::nix::Error) #[cfg(any(target_os = "linux", target_os = "macos"))];
    }

    errors {
        Os(error: u32) {
            description("Win32 error occurred")
            display("Win32 error occurred: {}", error)
        }
    }
}
