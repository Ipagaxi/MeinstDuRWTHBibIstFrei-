use reqwest::{Error};


pub async fn send_get_request(url: &str) -> Result<String, Error> {

    let client = reqwest::Client::new();

    let response = client.get(url)
        .header("User-Agent", "Mozilla/5.0 (X11; Ubuntu; Linux x86_64; rv:130.0) Gecko/20100101 Firefox/130.0")
        .header("sec-ch-ua-platform", "Linux")
        .header("Referer", "https://online.rwth-aachen.de/RWTHonline/ee/ui/ca2/app/desktop/")
        .header("Sec-Fetch-Dest", "empty")
        .header("Sec-Fetch-Mode", "cors")
        .header("Sec-Fetch-Site", "same-origin")
        .header("Accept-Language", "en")
        .header("Connection", "keep-alive")
        .header("Accept", "application/json")
        .header("Content-Type", "application/json")
        //.json(&body)
        .send()
        .await?;
    
    response.text().await
}