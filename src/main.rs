extern crate reqwest;
extern crate skim;
use skim::{Skim, SkimOptionsBuilder};
use std::collections::HashMap;
use std::io::Cursor;

fn main() -> Result<(), Box<std::error::Error>> {
    // Http query on the docker registry
    let resp_json: HashMap<String, Vec<String>> =
        reqwest::get("https://docker.maxiv.lu.se/v2/_catalog")?.json()?;
    // Parse json response
    let mut stack = Vec::new();
    for repositories in resp_json.get("repositories") {
        for docker_image in repositories {
            stack.push(docker_image.to_string());
        }
    }

    // Format reponse
    let input = stack.join("\n");

    // Setup skim
    let options = SkimOptionsBuilder::default()
        .height(Some("50%"))
        .multi(false)
        .build()
        .unwrap();

    // Run Skim
    let selected_items = Skim::run_with(&options, Some(Box::new(Cursor::new(input))))
        .map(|out| out.selected_items)
        .unwrap_or_else(|| Vec::new());

    // Append domain name
    let mut image = String::from("docker.maxiv.lu.se/");
    for item in selected_items.iter() {
        image.push_str(item.get_output_text().to_string().trim());
        // Print in stdout the result
        print!("{}", image);
    }

    Ok(())
}
