use reqwest::{blocking::Client, Error};

fn main() -> Result<(), Error> {
    let client = Client::new();

    let user = "sampleuser".to_string();
    let passwd: Option<String> = None;

    let response = client
    .get("http://httpbin.org/get")
    .basic_auth(user, passwd)
    .send()?;

    println!("Response: {:?}", response);
    println!("Status: {:?}", response.status());
    println!("Headers: {:?}", response.headers());
    println!("Url: {:?}", response.url());
    Ok(())
}
