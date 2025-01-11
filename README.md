## Rust Experimental
![image](https://upload.wikimedia.org/wikipedia/commons/d/d5/Rust_programming_language_black_logo.svg)
![image](https://upload.wikimedia.org/wikipedia/commons/thumb/5/52/Heart_icon_red_hollow.svg/45px-Heart_icon_red_hollow.svg.png)
![image](https://upload.wikimedia.org/wikipedia/commons/f/f0/Musl_libc.svg)

Initiative to list musl compatible library ( mostly the one that declared written in pure rust) for cross compile compatibility

## Disclaimer
Compatibility with all Linux distributions is not guaranteed. A list of distributions on which this has been successfully tested is provided in the 'Currently Tested' section.

## Environment Specification
| Toolchain      | Version      |
| ------------- | ------------- |
| [rustup](https://static.rust-lang.org/rustup/dist/x86_64-pc-windows-msvc/rustup-init.exe) | 1.27.1 (54dd3d00f 2024-04-24) |
| rustc  | 1.83.0 (90b35a623 2024-11-26) |



## How (Windows)
Simply follow instruction to [setup Visual C++ Build Tools for Rust development](https://learn.microsoft.com/en-us/windows/dev-environment/rust/setup)

Below is summarized version
- Download and Open [vs_BuildTools](https://aka.ms/vs/17/release/vs_BuildTools.exe). Make sure to check Desktop development with C++
- Create/Modify Cargo.toml file in %USERPROFILE%/.cargo/ folder as follows :
```toml
[target.x86_64-unknown-linux-musl]
linker = "rust-lld"
```
Add musl target and compile as follows :
```sh
$ rustup target add x86_64-unknown-linux-musl
$ cargo build --release --target=x86_64-unknown-linux-musl
```
## Currently Tested (Env : Rocky Linux)
- [umya_spreadsheet](https://github.com/mathnya/umya-spreadsheet) (replacement for [xlsxwriter](https://docs.rs/xlsxwriter/latest/xlsxwriter/), able to read spreadsheet too!)

## What to Avoid
- OpenSSL
- zstd

## Notes
- reqwest: configuring TLS to work with musl is a pain, just disabled features entirely.

https://rhysd.github.io/actionlint/

## Caveats
- reqwest: No support for accessing https resource

## To Be Explored
- reqwest : add https support
```sh
rustls = { version = "0.23.20", default-features = false, features = ["std"] }
webpki-roots = "0.26.7"
chacha20poly1305 = "0.10.1"
der = "0.7.9"
ecdsa = "0.16.9"
hmac = "0.12.1"
hpke-rs = "0.2.0"
hpke-rs-crypto = "0.2.0"
hpke-rs-rust-crypto = "0.2.0"
p256 = "0.13.2"
pkcs8 = "0.10.2"
rand_core = "0.6.4"
rsa = "0.9.7"
sha2 = "0.10.8"
signature = "2.2.0"
x25519-dalek = "2.0.1"
webpki = { package = "rustls-webpki", version = "0.102.8", default-features = false, features = ["alloc"] }
```
