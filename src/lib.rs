use std::collections::BTreeMap;
use std::io::Read;

extern crate hyper;
use hyper::Client;
use hyper::header::{Authorization, Bearer, ContentType, Headers, UserAgent};
use hyper::mime::{Mime, SubLevel, TopLevel};
use hyper::status::StatusCode;

extern crate serde_json;
use serde_json::Value;

extern crate time;

mod model;
use model::TagResult;

#[test]
fn test() {
    let mut client = client.unwrap();
    println!("{:#?}", client);

    let urls = vec!["http://i.imgur.com/DMOkjFF.jpg", "http://i.imgur.com/93VdKBI.png", "http://i.imgur.com/rfRu6ct.jpg"];
    let results: Vec<TagResult> = client.tag(urls).unwrap();

    for result in results {
        println!("{:#?}", result);
    }

    let doc_ids = vec![];
    let tags = vec![];
    client.add_tags(doc_ids, tags);
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

    headers: Headers,
    expires_in: u32,

    acquired: i64
}

#[derive(Debug)]
pub struct Tag<'a> {
    doc_id: &'a str,
    tags: &'a BTreeMap<&'a str, f32>
}

impl <'a> Clarifai<'a> {
    pub fn new(client_id: &'a str, client_secret: &'a str) -> Result<Clarifai<'a>, String> {
        let mut client = Clarifai {
            client_id: client_id,
            client_secret: client_secret,

            headers: Headers::new(),
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

        if response.status == StatusCode::Ok {
            let mut json_string = String::new();
            response.read_to_string(&mut json_string);

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
                    self.headers.set(
                       Authorization(
                           Bearer {
                               token: val.to_string()
                           }
                       )
                    );
                    self.headers.set(
                        ContentType(
                            Mime(
                                TopLevel::Application,
                                SubLevel::WwwFormUrlEncoded,
                                vec![]
                            )
                        )
                    );
                    self.headers.set(
                        UserAgent("clarifai-rs".to_string())
                    );
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

    fn ensure_validity(&mut self) -> Result<(), String> {
        let delta = (time::get_time().sec - self.acquired) as u32;

        if delta >= self.expires_in {
            try!(self.get_access_token());
        }

        Ok(())
    }

    pub fn tag(&mut self, urls: Vec<&str>) -> Result<Vec<TagResult>, String> {
        try!(self.ensure_validity());

        let mut urls = urls;
        let mut body: String = format!("url={}", urls.pop().unwrap());

        for url in urls {
            let parameter = format!("&url={}", url);

            body.push_str(&parameter);
        }

        let client = Client::new();
        let mut response = client.post("https://api.clarifai.com/v1/tag/")
            .body(&body)
            .headers(self.headers.clone())
            .send()
            .unwrap();

        let mut json_string = String::new();
        response.read_to_string(&mut json_string);

        TagResult::from_json(json_string)
    }

    pub fn add_tags(&mut self, doc_ids: Vec<&str>, tags: Vec<&str>) -> Result<(), String> {
        try!(self.ensure_validity());

        let doc_ids = format!("docids={}", doc_ids.join(","));
        let tags = format!("tags={}", tags.join(","));
        let body = &format!("{}&{}", doc_ids, tags);

        let client = Client::new();
        let response = client.post("https://api.clarifai.com/v1/token/")
            .body(body)
            .headers(self.headers.clone())
            .send()
            .unwrap();

        if response.status == StatusCode::Ok {
            Ok(())
        } else {
            Err("Failed to add tags".to_string())
        }
    }
}
