use youtube_bot::youtube::scraper::RssFetcher;
#[tokio::main]
async fn main() {
    let fetcher = RssFetcher::new("https://rsshub.app/youtube/channel/UC4YaOt1yT-ZeyB0OmxHgolA");

    match fetcher.fetch().await {
        Ok(videos) => {
            for video in videos {
                println!("Title: {}, Link: {}", video.title, video.link);
                //todo: save to database
            }
        }
        Err(e) => eprintln!("Failed to fetch RSS feed: {}", e),
    }
}
