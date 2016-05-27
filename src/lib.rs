#![feature(custom_derive, plugin)]
#![plugin(serde_macros)]

use std::cell::{Cell, RefCell};
use std::ops::Deref;
use std::fs::File;

extern crate curs;
use curs::{Request, DecodableResult, Method, CursError};
use curs::hyper::header::{Authorization, Basic, Bearer, UserAgent};

extern crate serde;

extern crate rustc_serialize;
use rustc_serialize::base64::ToBase64;

extern crate time;

mod model;
use model::ClarifaiResponse;
use model::image::{Image, ImageUrl, ImageFile};
use model::oauth::OAuth;

static CLARIFAI_URL: &'static str = "https://api2-prod.clarifai.com/v2";

pub struct Clarifai<'a> {
    client_id: &'a str,
    client_secret: &'a str,

    access_token: RefCell<String>,

    expires_in: Cell<u64>,
    acquired: Cell<i64>,
}

impl<'a> Clarifai<'a> {
    pub fn new(client_id: &'a str, client_secret: &'a str) -> ClarifaiResult<Clarifai<'a>> {
        let client = Clarifai {
            client_id: client_id,
            client_secret: client_secret,

            access_token: RefCell::new(String::new()),

            expires_in: Cell::new(0),
            acquired: Cell::new(0),
        };

        try!(client.get_access_token());

        Ok(client)
    }

    fn get_access_token(&self) -> ClarifaiResult<()> {
        let request_url = &request_url("token");

        let response: OAuth = try!(Request::new(Method::Post, request_url)
                                       .header(Authorization(Basic {
                                           username: self.client_id.to_string(),
                                           password: Some(self.client_secret.to_string()),
                                       }))
                                       .header(UserAgent("clarifai-rs".to_string()))
                                       .send()
                                       .decode_success());


        let mut access_token = self.access_token.borrow_mut();
        access_token.clear();
        access_token.push_str(&response.access_token);

        self.expires_in.set(response.expires_in);
        self.acquired.set(time::get_time().sec);

        Ok(())
    }

    fn ensure_validity(&self) -> ClarifaiResult<()> {
        let delta = (time::get_time().sec - self.acquired.get()) as u64;

        if delta >= self.expires_in.get() {
            try!(self.get_access_token());
        }

        Ok(())
    }

    fn add_headers(&self, request: &mut Request) {
        request.header(Authorization(Bearer {
                   token: self.access_token.borrow().deref().to_string(),
               }))
               .header(UserAgent("clarifai-rs".to_string()));
    }

    pub fn add_image_from_url(&self, image_url: &str) -> ClarifaiResult<Image> {
        try!(self.ensure_validity());

        let request_url = &request_url("images");

        let mut request = Request::new(Method::Post, request_url);
        request.json(ImageUrl { url: image_url });

        self.add_headers(&mut request);

        let response: ClarifaiResponse<Image> = try!(request.send().decode_success());
        Ok(response.results)
    }

    pub fn add_images_from_url(&self, image_urls: Vec<&str>) -> ClarifaiResult<Vec<Image>> {
        try!(self.ensure_validity());

        let request_url = &request_url("images/bulk");

        let mut bulk = vec![];

        for image_url in image_urls {
            bulk.push(ImageUrl { url: image_url });
        }

        let mut request = Request::new(Method::Post, request_url);
        request.json(bulk);

        self.add_headers(&mut request);

        let response: ClarifaiResponse<Vec<Image>> = try!(request.send().decode_success());
        Ok(response.results)
    }

    pub fn get_image(&self, id: &str) -> ClarifaiResult<Image> {
        try!(self.ensure_validity());

        let request_url = &request_url(&format!("images/{}", id));

        let mut request = Request::new(Method::Get, request_url);

        self.add_headers(&mut request);

        let response: ClarifaiResponse<Image> = try!(request.send().decode_success());
        Ok(response.results)
    }

    pub fn get_images(&self, per_page: i32, page: i32) -> ClarifaiResult<Vec<Image>> {
        try!(self.ensure_validity());

        let per_page = &per_page.to_string();
        let page = &page.to_string();

        let request_url = &request_url("images");

        let mut request = Request::new(Method::Get, request_url);
        request.params(vec![("per_page", &per_page[..]), ("page", &page[..])]);

        self.add_headers(&mut request);

        let response: ClarifaiResponse<Vec<Image>> = try!(request.send().decode_success());
        Ok(response.results)
    }

    pub fn image_iter(&self, per_page: i32) -> ImageLibrary {
        ImageLibrary {
            client: self,

            per_page: per_page,
            page: 0,
        }
    }
}

pub struct ImageLibrary<'a> {
    client: &'a Clarifai<'a>,

    per_page: i32,
    page: i32,
}

impl<'a> Iterator for ImageLibrary<'a> {
    type Item = Vec<Image>;

    fn next(&mut self) -> Option<Vec<Image>> {
        if let Ok(result) = self.client.get_images(self.per_page, self.page) {
            self.page += 1;
            Some(result)
        } else {
            None
        }
    }
}

fn request_url(route: &str) -> String {
    format!("{}/{}", CLARIFAI_URL, route)
}

pub type ClarifaiResult<T> = Result<T, CursError>;
