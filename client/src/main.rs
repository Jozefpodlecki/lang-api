use crate::lang_api::LanguageApi;

mod lang_api;
mod models;

#[tokio::main]
async fn main() {
    let client = LanguageApi::new("https://localhost:3000".into());

    let result = client.get_language_metadata("cz").await.unwrap();
}
