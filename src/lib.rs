use std::io::Read;
use std::collections::BTreeMap;

extern crate hyper;
use hyper::Client;
use hyper::status::StatusCode;
use hyper::header::ContentType;

extern crate serde_json;
use serde_json::Value;

extern crate time;

#[test]
fn test() {
    //let client = Clarifai::new("client_id",  "client_secret")
    //client.info()
    //client.tag(images: Vec<Path>)
    //client.tag(urls: Vec<&str>)
    //client.add_tags(doc_ids: Vec<&str>, tags: Vec<&str>)
    //client.remove_tags(doc_ids: Vec<&str>, tags: Vec<&str>)
    //client.add_similarity(doc_ids: Vec<&str>, similars: Vec<&str>)
    //client.add_dissimilarity(doc_ids: Vec<&str>, dissimilars: Vec<&str>)
}

#[derive(Debug)]
pub struct Clarifai<'a> {
    client_id: &'a str,
    client_secret: &'a str,

    access_token: String,
    expires_in: u32,

    acquired: i64
}

impl <'a> Clarifai<'a> {
    pub fn new(client_id: &'a str, client_secret: &'a str) -> Result<Clarifai<'a>, String> {
        let mut client = Clarifai {
            client_id: client_id,
            client_secret: client_secret,

            access_token: String::new(),
            expires_in: 0,

            acquired: 0
        };

        try!(client.get_access_token());

        Ok(client)
    }

    fn get_access_token(&mut self) -> Result<(), String> {
        let body =
            &format!("grant_type=client_credentials&client_id={}&client_secret={}", self.client_id, self.client_secret);

        let client = Client::new();
        let mut response = client.post("https://api.clarifai.com/v1/token/")
            .body(body)
            .header(ContentType::form_url_encoded())
            .send()
            .unwrap();

        println!("{:?}", response);

        if response.status == StatusCode::Ok {
            let mut json_string = String::new();
            response.read_to_string(&mut json_string);

            println!("{:?}", json_string);

            let json: Value;
            if let Ok(val) = serde_json::from_str(&json_string) {
                json = val;
            } else {
                return Err("Could not convert Json".to_string());
            }

            if json.is_object() {
                let object: &BTreeMap<String, Value>;

                if let Some(map) = json.as_object() {
                    object = map;
                } else {
                    return Err("Could not convert Json".to_string());
                }

                let access_token: &Value = object.get("access_token").unwrap();
                let expires_in: &Value = object.get("expires_in").unwrap();

                if let Some(val) = access_token.as_string() {
                    self.access_token = val.to_string();
                } else {
                    return Err("Could not convert access_token to String".to_string());
                }

                if let Some(val) = expires_in.as_u64() {
                    self.expires_in = val as u32;
                } else {
                    return Err("Could not convert expires_in to u32".to_string());
                }

                self.acquired = time::get_time().sec;

                return Ok(());
            }

            return Err("Invalid Json response".to_string());
        }

        Err(format!("Status Code: {}", response.status))
    }

    fn ensure_validity(&mut self) {
        let delta = (time::get_time().sec - self.acquired) as u32;
        if delta >= self.expires_in {
            self.get_access_token();
        }
    }
}
