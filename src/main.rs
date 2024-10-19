use std::collections::HashMap;

use anyhow::Result;
use search_provider::{ResultID, ResultMeta, SearchProvider, SearchProviderImpl};
use steamlocate::SteamDir;

type GameResults = HashMap<String, String>;

#[derive(Debug)]
struct Application {
    results: GameResults,
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
        if terms.contains(&"game".to_owned()) {
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
                ResultMeta::builder(id.to_owned(), "Game Name")
                    .description("appid")
                    .build()
            })
            .collect()
    }
}

fn get_games() -> Result<GameResults> {
    let mut results = GameResults::new();
    let steam = SteamDir::locate()?;

    for library in steam.libraries()? {
        match library {
            Err(err) => eprintln!("failed reading library: {err}"),
            Ok(library) => {
                for app in library.apps() {
                    match app {
                        Err(err) => eprintln!("failed reading app: {err}"),
                        Ok(app) => {
                            results.insert(app.app_id.to_string(), app.name.unwrap());
                        }
                    }
                }
            }
        }
    }

    Ok(results)
}

#[tokio::main]
async fn main() -> Result<()> {
    let app = Application {
        results: get_games()?,
    };
    SearchProvider::new(
        app,
        "dev.trytonvanmeer.Steam.SearchProvider",
        "/dev/trytonvanmeer/Steam/SearchProvider",
    )
    .await?;

    Ok(())
}
