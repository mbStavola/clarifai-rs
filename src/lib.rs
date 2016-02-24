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
use model::tag_result::TagResult;

mod error;
use error::ClarifaiError;

#[test]
fn test() {
    let client = Clarifai::new("placeholder",
                               "use-your-own");
    let mut client = client.unwrap();
    println!("{:#?}", client);

    let urls = vec!["https://www.rust-lang.org/logos/rust-logo-512x512.png"];
    let mut results = client.tag(&urls).unwrap();

    let mut doc_ids: Vec<String> = vec![];
    for result in results {
        doc_ids.push(result.doc_id);
    }

    let tags = vec!["clarifai", "serde", "hyper"];
    client.add_tags(&doc_ids, &tags);

    results = client.tag(&urls).unwrap();

    for result in results {
        println!("{:#?}", result);
    }

    client.remove_tags(&doc_ids, &tags);

    results = client.tag(&urls).unwrap();

    for result in results {
        println!("{:#?}", result);
    }

    // let client = Clarifai::new("client_id",  "client_secret")
    // client.info()
    // client.tag(images: Vec<Path>)
    // client.tag(urls: Vec<&str>)
    // client.add_tags(doc_ids: Vec<&str>, tags: Vec<&str>)
    // client.remove_tags(doc_ids: Vec<&str>, tags: Vec<&str>)
    // client.add_similarity(doc_ids: Vec<&str>, similars: Vec<&str>)
    // client.add_dissimilarity(doc_ids: Vec<&str>, dissimilars: Vec<&str>)
}

#[derive(Debug)]
pub struct Clarifai<'a> {
    client_id: &'a str,
    client_secret: &'a str,

    headers: Headers,
    expires_in: u64,

    acquired: i64,
}

impl<'a> Clarifai<'a> {
    pub fn new(client_id: &'a str, client_secret: &'a str) -> Result<Clarifai<'a>, ClarifaiError> {
        let mut client = Clarifai {
            client_id: client_id,
            client_secret: client_secret,

            headers: Headers::new(),
            expires_in: 0,

            acquired: 0,
        };

        try!(client.get_access_token());

        Ok(client)
    }

    fn get_access_token(&mut self) -> Result<(), ClarifaiError> {
        let body = format!("grant_type=client_credentials&client_id={}&client_secret={}",
                           self.client_id,
                           self.client_secret);

        let client = Client::new();
        let mut response = try!(client.post("https://api.clarifai.com/v1/token/")
                                      .body(&body)
                                      .header(ContentType::form_url_encoded())
                                      .send());

        if response.status == StatusCode::Ok {
            let mut json_string = String::new();
            response.read_to_string(&mut json_string);

            let json: Value = try!(serde_json::from_str(&json_string));
            let object: &BTreeMap<String, Value> = try!(json.as_object()
                                                            .ok_or(ClarifaiError::ConversionError));

            let access_token: &Value = try!(object.get("access_token")
                                                  .ok_or(ClarifaiError::MissingFieldError));
            let access_token: &str = try!(access_token.as_string()
                                                      .ok_or(ClarifaiError::ConversionError));
            self.headers.set(Authorization(Bearer { token: access_token.to_string() }));
            self.headers
                .set(ContentType(Mime(TopLevel::Application, SubLevel::WwwFormUrlEncoded, vec![])));
            self.headers.set(UserAgent("clarifai-rs".to_string()));

            let expires_in: &Value = try!(object.get("expires_in")
                                                .ok_or(ClarifaiError::MissingFieldError));
            self.expires_in = try!(expires_in.as_u64().ok_or(ClarifaiError::ConversionError));

            self.acquired = time::get_time().sec;

            return Ok(());
        }

        Err(ClarifaiError::HttpStatusError)
    }

    fn ensure_validity(&mut self) -> Result<(), ClarifaiError> {
        let delta = (time::get_time().sec - self.acquired) as u64;

        if delta >= self.expires_in {
            try!(self.get_access_token());
        }

        Ok(())
    }

    pub fn tag(&mut self, urls: &Vec<&str>) -> Result<Vec<TagResult>, ClarifaiError> {
        try!(self.ensure_validity());

        let mut url_iter = urls.iter();
        let mut body: String = format!("url={}", url_iter.next().unwrap());

        for url in url_iter {
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

    pub fn add_tags(&mut self,
                    doc_ids: &Vec<String>,
                    tags: &Vec<&str>)
                    -> Result<(), ClarifaiError> {
        try!(self.ensure_validity());

        let doc_ids = format!("docids={}", doc_ids.join(","));
        let tags = format!("add_tags={}", tags.join(","));
        let body = &format!("{}&{}", doc_ids, tags);

        let client = Client::new();
        let response = client.post("https://api.clarifai.com/v1/feedback/")
                             .body(body)
                             .headers(self.headers.clone())
                             .send()
                             .unwrap();

        if response.status == StatusCode::Ok {
            Ok(())
        } else {
            Err(ClarifaiError::HttpStatusError)
        }
    }

    fn remove_tags(&mut self,
                   doc_ids: &Vec<String>,
                   tags: &Vec<&str>)
                   -> Result<(), ClarifaiError> {
        let doc_ids = format!("docids={}", doc_ids.join(","));
        let tags = format!("remove_tags={}", tags.join(","));
        let body = &format!("{}&{}", doc_ids, tags);

        let client = Client::new();
        let response = client.post("https://api.clarifai.com/v1/feedback/")
                             .body(body)
                             .headers(self.headers.clone())
                             .send()
                             .unwrap();

        if response.status == StatusCode::Ok {
            Ok(())
        } else {
            Err(ClarifaiError::HttpStatusError)
        }
    }
}
