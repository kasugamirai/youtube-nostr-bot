use nostr_sdk::prelude::*;

pub async fn publish_text_note(my_keys:Keys,username: &str, message: &str) -> Result<()> {
    // show keys
    let bech32_pubkey: String = my_keys.public_key().to_bech32()?;
    println!("Bech32 PubKey: {}", bech32_pubkey);

    // create client
    let client = Client::new(&my_keys);

    // add relays
    let relays = vec![
        "wss://relay.damus.io",
        "wss://freerelay.xyz",
        "wss://relayable.org",
        "wss://nos.lol",
        "wss://offchain.pub",
        "wss://relay.snort.social",
    ];
    client.add_relays(relays).await?;

    // connect to the network
    client.connect().await;

    let metadata = Metadata::new()
    .name("username")
    .display_name(username)
    .about("Description")
    .picture(Url::parse("https://example.com/avatar.png")?)
    .banner(Url::parse("https://example.com/banner.png")?)
    .nip05("username@example.com")
    .lud16("yuki@getalby.com")
    .custom_field("custom_field", "my value");

    // Update metadata
    client.set_metadata(&metadata).await?;


    // post a text note
    client.publish_text_note(message, []).await?;

    Ok(())
}