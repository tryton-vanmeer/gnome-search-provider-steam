use std::collections::HashMap;

use search_provider::{ResultID, ResultMeta, SearchProvider, SearchProviderImpl};

#[derive(Debug)]
struct Application {
    results: HashMap<String, String>,
}

impl SearchProviderImpl for Application {
    fn activate_result(&self, identifier: ResultID, _terms: &[String], _timestamp: u32) {
        let result = self.results.get(&identifier);

        println!(
            "activating result {:#?} identified by {}",
            result, identifier
        );
    }

    fn initial_result_set(&self, terms: &[String]) -> Vec<ResultID> {
        // Here do your search logic
        if terms.contains(&"some_value".to_owned()) {
            vec!["some_key".to_owned()]
        } else {
            vec![]
        }
    }

    fn result_metas(
        &self,
        identifiers: &[search_provider::ResultID],
    ) -> Vec<search_provider::ResultMeta> {
        identifiers
            .iter()
            .map(|id| {
                ResultMeta::builder(id.to_owned(), "Some name")
                    .description("some description for the current identifier")
                    .build()
            })
            .collect()
    }
}

#[tokio::main]
async fn main() -> zbus::Result<()> {
    let mut results = HashMap::new();
    results.insert("some_key".to_string(), "some_value".to_string());

    let app = Application { results };
    SearchProvider::new(
        app,
        "dev.trytonvanmeer.Steam.SearchProvider",
        "/dev/trytonvanmeer/Steam/SearchProvider",
    )
    .await?;

    Ok(())
}
