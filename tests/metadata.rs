use checkout::*;
use once_cell::sync::OnceCell;

fn client() -> Option<&'static Client> {
    dotenvy::dotenv().ok();
    static INSTANCE: OnceCell<Client> = OnceCell::new();
    INSTANCE.get_or_try_init(Client::from_env).ok()
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
