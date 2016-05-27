#[derive(Deserialize, Debug)]
pub struct OAuth {
    pub access_token: String,
    pub expires_in: u64,
}
