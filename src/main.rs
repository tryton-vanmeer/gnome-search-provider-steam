use std::{
    collections::HashMap,
    io::{self, IsTerminal},
};

use anyhow::Result;
use search_provider::{ResultID, ResultMeta, SearchProvider, SearchProviderImpl};
use steamlocate::SteamDir;
use tracing::{debug, error};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

type GameResults = HashMap<String, String>;

#[derive(Debug)]
struct Application {
    games: GameResults,
}

impl SearchProviderImpl for Application {
    fn activate_result(&self, identifier: ResultID, _terms: &[String], _timestamp: u32) {
        let uri = format!("steam://rungameid/{identifier}");

        let _ = gio::AppInfo::launch_default_for_uri(&uri, gio::AppLaunchContext::NONE);
    }

    fn initial_result_set(&self, terms: &[String]) -> Vec<ResultID> {
        let mut results = Vec::<ResultID>::new();

        for (id, name) in self.games.iter() {
            let name_lower = name.to_lowercase();
            for term in terms {
                if name_lower.contains(&term.to_lowercase()) {
                    debug!("found game '{}' ({}) for term '{}'", name, id, term);
                    results.push(id.clone());
                }
            }
        }

        results
    }

    fn result_metas(
        &self,
        identifiers: &[search_provider::ResultID],
    ) -> Vec<search_provider::ResultMeta> {
        identifiers
            .iter()
            .map(|id| {
                ResultMeta::builder(id.to_owned(), self.games.get(id).unwrap())
                    .description(id)
                    .gicon(&format!("steam_icon_{id}").to_owned())
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
            Err(err) => error!("failed reading library: {err}"),
            Ok(library) => {
                for app in library.apps() {
                    match app {
                        Err(err) => error!("failed reading app: {err}"),
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
    if io::stdin().is_terminal() {
        tracing_subscriber::fmt::init();
    } else {
        tracing_subscriber::registry()
            .with(tracing_journald::layer()?)
            .init();
    }

    let app = Application {
        games: get_games()?,
    };
    SearchProvider::new(
        app,
        "dev.trytonvanmeer.Steam.SearchProvider",
        "/dev/trytonvanmeer/Steam/SearchProvider",
    )
    .await?;

    Ok(())
}
