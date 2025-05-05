fn main() {
    if std::env::var_os("CARGO_CFG_UNIX").is_some() {
        println!("cargo::rustc-link-arg=-Wl,-init,initialize");
    }
}
