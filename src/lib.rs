use std::io::Read;
use std::collections::BTreeMap;

extern crate hyper;
use hyper::Client;
use hyper::status::StatusCode;
use hyper::header::ContentType;

extern crate serde_json;

pub fn get_access_token(client_id: &str, client_secret: &str) -> Option<Credentials> {
    let mut result: Option<Credentials> = None;

    let body =
        format!("grant_type=client_credentials&client_id={}&client_secret={}", client_id, client_secret);

    let client = Client::new();
    let mut response = client.post("https://api.clarifai.com/v1/token/")
        .body(&body)
        .header(ContentType::form_url_encoded())
        .send()
        .unwrap();

    println!("{:?}", response);

    if response.status == StatusCode::Ok {
        let mut json_body = String::new();
        response.read_to_string(&mut json_body);

        println!("{:?}", json_body);

        let map: BTreeMap<String, String> = serde_json::from_str(&json_body).unwrap();

        println!("Map");

        let access_token = map.get("access_token").cloned().unwrap();

        println!("Token");

        let expires_in = map.get("expires_in").cloned().unwrap().parse::<u32>().unwrap();

        println!("Expires");

        let credentials = Credentials::new(access_token, expires_in);

        result = Some(credentials);
    }

    result
}

#[test]
fn test() {
}

#[derive(Debug)]
pub struct Credentials {
    access_token: String,
    expires_in: u32,
}

impl Credentials {
    fn new(access_token: String, expires_in: u32) -> Credentials {
        Credentials {
            access_token: access_token,
            expires_in: expires_in
        }
    }
}
