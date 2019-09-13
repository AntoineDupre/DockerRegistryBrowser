extern crate reqwest;
extern crate skim;
use skim::{Skim, SkimOptionsBuilder};
use std::collections::HashMap;
use std::io::Cursor;
use std::process::Command;

extern crate argparse;

use argparse::{ArgumentParser, Store, StoreTrue};

fn main() -> Result<(), Box<std::error::Error>> {
    // Setup arg parse
    let mut run_image = false;
    let mut registry_name = "".to_string();
    {
        let mut ap = ArgumentParser::new();
        ap.set_description("Docker registry image browser");
        ap.refer(&mut registry_name).add_option(
            &["-u", "--url"],
            Store,
            "URL of the docker registry",
        );
        ap.refer(&mut run_image).add_option(
            &["-r", "--run"],
            StoreTrue,
            "Run the selected image (default is pull only)",
        );
        ap.parse_args_or_exit();
    }

    // Convert url
    let url = format!("https://{}/v2/_catalog", registry_name);
    let url_slash = format!("{}/", registry_name);

    // Http query on the docker registry
    let resp_json: HashMap<String, Vec<String>> = reqwest::get(&url)?.json()?;
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
    let mut image = String::from(url_slash);
    for item in selected_items.iter() {
        image.push_str(item.get_output_text().to_string().trim());
    }

    // Run docker in interactive mode
    if run_image {
        Command::new("docker")
            .arg("run")
            .arg("-i")
            .arg("-t")
            .arg(&image)
            .status()
            .expect("failed to execute process");

    // Only pull the container
    } else {
        Command::new("docker")
            .arg("pull")
            .arg(&image)
            .status()
            .expect("failed to execute process");
    }

    Ok(())
}
