mod tulis_excel;
use tulis_excel::*;
mod permintaan_http;
use permintaan_http::*;
use tokio::net::TcpListener;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
// use tokio;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    println!("Hello, world!");
    fn_satu();
    fn_tulis_xlsx();
    println!("Berhasil menulis!");
    // fn_dua().await;
    // http_request::fn_akses_http().await;
    let a = fn_akses_http().await?;
    println!("Hasil Dua {:#?}", a);
    Ok(())
}