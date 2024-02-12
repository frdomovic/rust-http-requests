use reqwest::blocking::{Client, ClientBuilder};
use reqwest::redirect::Policy;

fn main() {
    let http_client = Client::new();
    let http_result = http_client.get("url").send();


    if http_result.is_ok() {
        println!("Body: {:#?}", http_result.ok().unwrap().text().unwrap());
    } else if http_result.is_err() {
        println!("Error: {:#?}", http_result.err());
    }

    let post_result = http_client.post("url").body("{\"first_name\":\"Trevor\"}")
    .header("User-Agent", "Rust Application on Mac")
    .send();
    println!("{:#?}", post_result.ok().unwrap().text().unwrap());

    let redir_policy = Policy::limited(5);

    let http_client = ClientBuilder::new().redirect(redir_policy).build().ok().unwrap();

    let result = http_client.get("url").send().ok().unwrap();

    println!("{:#?}", result);

}