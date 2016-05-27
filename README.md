# clarifai-rs
Crate for image recognition / tagging using the [Clarifai](http://www.clarifai.com/) API.

[![Build Status](https://travis-ci.org/mbStavola/clarifai-rs.svg?branch=master)](https://travis-ci.org/mbStavola/clarifai-rs)

WIP

## Use
All calls are done through an instance of a Clarifai object.

For example, you can get a list of tags and their probability for an image (or multiple images) by doing:

```rust
let urls = vec!["http://i.imgur.com/DMOkjFF.jpg", "http://i.imgur.com/93VdKBI.png", "http://i.imgur.com/rfRu6ct.jpg"];
let client: Clarifai = Clarifai::new("my-client-id", "my-client-secret");
println!("{:#?}", client.tag(&urls));
```

## TODO
 * info()
 * tag(images: &Vec<Path>)
 * add_tags(doc_ids: &Vec<&str>, tags: &Vec<&str>)
 * remove_tags(doc_ids: &Vec<&str>, tags: &Vec<&str>)
 * add_similarity(doc_ids: &Vec<&str>, similars: &Vec<&str>)
 * add_dissimilarity(doc_ids: &Vec<&str>, dissimilars: &Vec<&str>)
 
 And of course, lot's of cleanup :)
