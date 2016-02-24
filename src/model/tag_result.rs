use std::collections::BTreeMap;

extern crate serde_json;
use serde_json::Value;

use super::super::error::ClarifaiError;

#[derive(Debug)]
pub struct TagResult {
    pub doc_id: String,
    pub source: String,

    pub concept_ids: Vec<String>,
    pub tag_map: BTreeMap<String, f64>,
}

impl TagResult {
    pub fn from_json(json_string: String) -> Result<Vec<TagResult>, ClarifaiError> {
        let json: BTreeMap<String, Value> = try!(serde_json::from_str(&json_string));

        let raw_results: &Value = json.get("results").unwrap();
        let results: Vec<BTreeMap<String, Value>> = raw_results.as_array()
                                                               .unwrap()
                                                               .iter()
                                                               .map(|v| {
                                                                   v.as_object().unwrap().clone()
                                                               })
                                                               .collect();

        let mut tag_results: Vec<TagResult> = vec![];
        for result in results {
            let doc_id = result.get("docid_str").unwrap().as_string().unwrap().to_string();

            let source = result.get("url").unwrap().as_string().unwrap().to_string();

            let inner_result: &BTreeMap<String, Value> = result.get("result")
                                                               .unwrap()
                                                               .as_object()
                                                               .unwrap()
                                                               .get("tag")
                                                               .unwrap()
                                                               .as_object()
                                                               .unwrap();
            let concept_ids: Vec<String> = inner_result.get("concept_ids")
                                                       .unwrap()
                                                       .as_array()
                                                       .unwrap()
                                                       .iter()
                                                       .map(|v| v.as_string().unwrap().to_string())
                                                       .collect();

            let mut classes: Vec<String> = inner_result.get("classes")
                                                       .unwrap()
                                                       .as_array()
                                                       .unwrap()
                                                       .iter()
                                                       .map(|v| v.as_string().unwrap().to_string())
                                                       .collect();
            let mut probs: Vec<f64> = inner_result.get("probs")
                                                  .unwrap()
                                                  .as_array()
                                                  .unwrap()
                                                  .iter()
                                                  .map(|v| v.as_f64().unwrap())
                                                  .collect();

            let mut tag_map: BTreeMap<String, f64> = BTreeMap::new();

            while classes.len() != 0 {
                tag_map.insert(classes.pop().unwrap(), probs.pop().unwrap());
            }

            tag_results.push(TagResult {
                doc_id: doc_id,
                source: source,

                concept_ids: concept_ids,
                tag_map: tag_map,
            });
        }

        Ok(tag_results)
    }
}
