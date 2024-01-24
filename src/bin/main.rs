use youtube_bot::youtube::scraper::RssFetcher;
#[tokio::main]
async fn main() {
    let fetcher = RssFetcher::new("https://rsshub.app/youtube/channel/CHANNEL_ID");

    match fetcher.fetch().await {
        Ok(feed) => {
            // todo: do something with the feed
        }
        Err(e) => eprintln!("Failed to fetch RSS feed: {}", e),
    }
}