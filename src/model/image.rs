#[derive(Deserialize, Debug)]
pub struct Image {
    pub id: String,
    pub url: String,
}

#[derive(Serialize)]
pub struct ImageUrl<'a> {
    pub url: &'a str,
}
