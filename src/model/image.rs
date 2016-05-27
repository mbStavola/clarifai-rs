#[derive(Deserialize, Debug)]
pub struct Image {
    pub id: String,
    pub url: String,
}

#[derive(Serialize, Debug)]
pub struct ImageUrl<'a> {
    pub url: &'a str,
}

#[derive(Serialize, Debug)]
pub struct ImageFile<'a> {
    pub base64: &'a str,
}
