## Rust Experimental
Initiative to list musl compatible library ( mostly the one that declared written in pure rust) for cross compile compatibility

## How
```sh
$ rustup target add x86_64-unknown-linux-musl
$ cargo build --release --target=x86_64-unknown-linux-musl
```
## Currently Tested
- [umya_spreadsheet](https://github.com/mathnya/umya-spreadsheet) (replacement for [xlsxwriter](https://docs.rs/xlsxwriter/latest/xlsxwriter/), able to read spreadsheet too!)

## Notes
- reqwest: configuring TLS to work with musl is a pain, just disabled features entirely.

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
