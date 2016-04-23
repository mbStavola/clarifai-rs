extern crate clarifai;
use clarifai::Clarifai;

#[test]
fn add_image() {
    let mut client: Clarifai = Clarifai::new().unwrap();

    let response = client.add_image("https://www.rust-lang.org/logos/rust-logo-512x512.png");

    println!("{:#?}", response);
}

// #[test]
// fn add_tags() {
//     let mut client = client.unwrap();
//
//     let doc_ids = vec!["caaf9494eeb4e0cd61bb9dbd1948b13b"]
//     let tags = vec!["rust", "metal", "gear"];
//     client.add_tags(&doc_ids, &tags).unwrap();
// }
//
// #[test]
// fn remove_tags() {
//     let mut client = client.unwrap();
//
//     let doc_ids = vec!["caaf9494eeb4e0cd61bb9dbd1948b13b"]
//     let tags = vec!["c++", "wood", "gear"];
//     client.remove_tags(&doc_ids, &tags);
// }
