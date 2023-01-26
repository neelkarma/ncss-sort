use percent_encoding::{percent_encode, NON_ALPHANUMERIC};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct OWHit {
    sku: String,
}

#[derive(Debug, Deserialize)]
struct OWResult {
    hits: Vec<OWHit>,
}

#[derive(Debug, Deserialize)]
struct OWResponse {
    results: Vec<OWResult>,
}

/// Sorts the provdided slice with Officeworks Sort out-of-place.
///
/// Time Complexity is `O(n log n)`.
///
/// This function may return an `Err` if:
/// - The Officeworks API request fails
/// - The Officeworks API returns a malformed object
///
/// This function may panic if:
/// - The Officeworks API doesn't return any results for any given item
pub fn officeworks_sort<T: ToString>(arr: &[T]) -> Result<Vec<String>, ureq::Error> {
    let agent = ureq::agent();

    let mut items: Vec<_> = arr
        .iter()
        .map(|item| -> Result<(String, String), ureq::Error> {
            let item_encoded =
                percent_encode(item.to_string().as_bytes(), NON_ALPHANUMERIC).to_string();

            let res: OWResponse = agent
                .post("https://k535caawve-3.algolianet.com/1/indexes/*/queries")
                .query_pairs([
                    ("x-algolia-application-id", "K535CAAWVE"),
                    ("x-algolia-api-key", "8a831febe0110932cfa06ff0e2024b4f"),
                ])
                .send_json(ureq::json!({
                    "requests": [
                        {
                            "indexName": "prod-product-wc-bestmatch-personal",
                            "params": format!("{}{}", "query=", item_encoded)
                        }
                    ]
                }))?
                .into_json()?;

            Ok((item.to_string(), res.results[0].hits[0].sku.clone()))
        })
        .collect::<Result<Vec<_>, _>>()?;

    items.sort_by_key(|(_, sku)| sku.clone());

    Ok(items.into_iter().map(|(item, _)| item).collect())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn officeworks_sort_works() {
        let arr = [
            "pencil case",
            "headphones",
            "school bag",
            "textbook",
            "printer",
            "excersise book",
        ];
        let res = officeworks_sort(&arr).unwrap();
        println!("{:?}", res);
    }
}
