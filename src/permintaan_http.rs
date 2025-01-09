use reqwest::{ClientBuilder, Result};
use serde_json::Value;

pub async fn fn_dua() -> Result<()> {
    println!("Terpanggil Fn Dua!");
    let a = fn_akses_http().await?;
    println!("Hasil Dua {:#?}", a);
    Ok(())
}

pub async fn fn_akses_http() -> Result<Value> {
    println!("Terpanggil Fn Tiga!");
    let url = format!(
        // "http://nominatim.openstreetmap.org/search?q=Grand%20Indonesia&format=json&addressdetails=1"
        "http://goweather.herokuapp.com/weather/Jakarta"
    );

    println!("url: {}", url);

    let client = ClientBuilder::new()
        .cookie_store(true)
        // .danger_accept_invalid_certs(true) //https only
        // .redirect(Policy::limited(20))
        .build();

    let res = client.unwrap()
                        .get(url)
                        .send().await?;

    // let res = client?
    //     .post(url)
    //     .header("Content-Type", "application/json")
    //     // .body(query_str)
    //     .send()
    //     .await?;

    println!("{:?}", res);

    let data = res.json::<Value>().await?;

    Ok(data)
    // Ok(())
}
