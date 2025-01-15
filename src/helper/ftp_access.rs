use std::str;
// use std::io::Cursor;
use std::time::SystemTime;
use rustls::time_provider::TimeProvider;
use suppaftp::FtpStream;

// use std::sync::Arc;
// use rustls_platform_verifier::BuilderVerifierExt;
// use crate::provider_example;
// use suppaftp::{RustlsFtpStream, RustlsConnector};
// use suppaftp::rustls::ClientConfig;

#[derive(Debug)]
pub struct SystemTimeProvider;

impl TimeProvider for SystemTimeProvider {

    fn current_time(&self) -> Option<rustls::pki_types::UnixTime> {
        SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .ok()
            .map(rustls::pki_types::UnixTime::since_unix_epoch)
    }
}

pub fn init_ftp() {
    // Create a connection to an FTP server and authenticate to it.
    let mut ftp_stream = FtpStream::connect("test.rebex.net:21").unwrap();
    let _ = ftp_stream.login("demo", "password").unwrap();

    // Get the current directory that the client will be reading from and writing to.
    println!("Current directory: {}", ftp_stream.pwd().unwrap());

    // Change into a new directory, relative to the one we are currently in.
    // let _ = ftp_stream.cwd("pub/example").unwrap();

    let list = ftp_stream.list(Some("/pub/example"));

    println!("Daftar File: {:#?}", list.expect("Gagal List FTP"));

    // Retrieve (GET) a file from the FTP server in the current working directory.
    let data = ftp_stream.retr_as_buffer("/pub/example/readme.txt").unwrap();
    println!("Read file with contents\n{}\n", str::from_utf8(&data.into_inner()).unwrap());

    // Store (PUT) a file from the client to the current working directory of the server.

    // let mut reader = Cursor::new("Hello from the Rust \"ftp\" crate!".as_bytes());
    // let _ = ftp_stream.put_file("greeting.txt", &mut reader);
    // println!("Successfully wrote greeting.txt");

    // Terminate the connection to the server.
    let _ = ftp_stream.quit();
}


// fn init_ftps() {
//     let mut root_store = rustls::RootCertStore::empty();
//     root_store.add(webpki_roots::TLS_SERVER_ROOTS.0.iter().map(|ta| {
//         rustls::OwnedTrustAnchor::from_subject_spki_name_constraints(
//             ta.subject,
//             ta.spki,
//             ta.name_constraints,
//         )
//     }));
//     let config = ClientConfig::builder()
//         .with_safe_defaults()
//         .with_root_certificates(root_store)
//         .with_no_client_auth();
//     // Create a connection to an FTP server and authenticate to it.
//     let config = Arc::new(rustls_config());
//     let mut ftp_stream = RustlsFtpStream::connect("test.rebex.net:21")
//         .unwrap()
//         .into_secure(RustlsConnector::from(Arc::clone(&config)), "test.rebex.net")
//         .unwrap();
//     // Terminate the connection to the server.
//     let _ = ftp_stream.quit();
// }