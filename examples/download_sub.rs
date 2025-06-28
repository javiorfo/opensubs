use std::process::Command;

use opensubs::{Filters, Language, OrderBy, Response, SearchBy};

#[tokio::main]
async fn main() -> opensubs::Result {
    // blocking search movie "pulp fiction", norwegian and swedish subs, order by date
    let results = opensubs::search(SearchBy::MovieAndFilter(
        "pulp fiction",
        Filters::default()
            .languages(&[Language::Norwegian, Language::Swedish])
            .order_by(OrderBy::Uploaded)
            .build(),
    ))
    .await?;

    if let Response::Subtitle(_, subtitles) = results {
        // Example filtering by uploader and getting the download link
        let link = subtitles
            .iter()
            .find(|&sub| sub.uploader.as_ref().is_some_and(|s| s.contains("larza83")))
            .map(|sub| sub.download_link.as_str());

        println!("Link to download {link:#?}");

        // Download using wget
        let output = Command::new("wget")
            .arg("-O")
            .arg("subtitle.zip")
            .arg(link.unwrap())
            .output()
            .expect("Failed to execute wget");

        if output.status.success() {
            println!("Download successful!");
        } else {
            println!("Download failed.");
        }
    }

    Ok(())
}
