extern crate clarifai;
use clarifai::Clarifai;

#[test]
fn add_image() {
    let client: Clarifai = Clarifai::new("Lkr0GNgteFBLYcOpfDP7rMb_JQIXcfcIfGhm3zmR",
                                         "HX4AUbn6qnZT5d1Y7p_yQaj_PiDRYuSsFPmwsXiJ")
                               .unwrap();

    let response = client.add_image_from_url("https://www.rust-lang.org/logos/rust-logo-512x512.\
                                              png");

    println!("{:#?}", response);

    let response = client.get_images(100, 1);

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
