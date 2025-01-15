use std::{sync::Arc, time::SystemTime};
use reqwest::{redirect::Policy, ClientBuilder, Result};
use rustls::time_provider::TimeProvider;
use serde_json::{json, Value};
use std::fmt::Debug;
use crate::provider_example;
use rustls_platform_verifier::BuilderVerifierExt;


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

pub async fn fn_dua() -> Result<()> {
    println!("Terpanggil Fn Dua!");
    Ok(())
}

pub async fn fn_akses_http(paramurl: String) -> Result<Value> {
    fn_dua().await?;
    let root_store = rustls::RootCertStore::from_iter(
        webpki_roots::TLS_SERVER_ROOTS
            .iter()
            .cloned(),
    );

    println!("rootstore: {:#?}", root_store);

    let time_provider = Arc::new(SystemTimeProvider); 

    let config =
        rustls::ClientConfig::builder_with_details(provider_example::provider().into(), time_provider)
            .with_safe_default_protocol_versions()
            .unwrap()
            // .with_root_certificates(root_store)
            .with_platform_verifier()
            .with_no_client_auth();
     
    let url = format!("{}",
        paramurl
    );

    println!("url: {}", url);

    let mut headers = reqwest::header::HeaderMap::new();
    headers.insert("User-Agent", reqwest::header::HeaderValue::from_static("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/131.0.0.0 Safari/537.36 Edg/131.0.0.0"));
    
    // Consider marking security-sensitive headers with `set_sensitive`.
    // let mut auth_value = header::HeaderValue::from_static("secret");
    // auth_value.set_sensitive(true);
    // headers.insert(header::AUTHORIZATION, auth_value);

    let client = ClientBuilder::new()
        .cookie_store(true)
        // .use_rustls_tls()
        .danger_accept_invalid_certs(true) //https only
        .use_preconfigured_tls(config)
        // .tls_built_in_root_certs(true)
        .no_zstd()
        .redirect(Policy::limited(20))
        // .http1_allow_obsolete_multiline_headers_in_responses(true)
        .default_headers(headers)
        .build();

    println!("client built {:#?}", client);

    let res = client?
                        .get(url)
                        .send().await?;

    println!("req sent");
    // let res = client?
    //     .post(url)
    //     .header("Content-Type", "application/json")
    //     // .body(query_str)
    //     .send()
    //     .await?;

    println!("iniii {:#?}", res);

    // let data = res.json::<Value>().await?;
    // Ok(data)    

    // return following line for debugging content
    let data = res.text().await?;
    Ok(json!({"content": data}))    
}
