pub mod oauth;
pub mod image;

#[derive(Deserialize, Debug)]
pub struct ClarifaiResponse<T> {
    status_code: String,
    status_msg: String,
    pub results: T,
}
