use checkout::Client;
use checkout::models::metadata::CardMetadataSource;

fn client() -> Option<Client> {
    dotenvy::dotenv().ok();
    Client::from_env().ok()
}

#[tokio::test]
async fn metadata_card() {
    let Some(client) = client() else { return };
    let response = client
        .metadata()
        .get_card(
            CardMetadataSource::Card {
                number: "4273149019799094".to_owned(),
            },
            None,
        )
        .await
        .unwrap();

    println!("Response: {:#?}", response);

    assert_eq!(response.scheme, "visa");
    assert_eq!(response.issuer_country.as_deref(), Some("GB"));
}
