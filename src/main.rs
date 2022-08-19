#![doc = include_str!("../README.md")]

use anyhow::Result;
use clap::Parser;
use std::env;

use self::args::Args;

use log::LevelFilter;
use metrics::{counter, gauge, histogram};
use metrics_datadog_exporter::DataDogBuilder;

mod args;

#[tokio::main(flavor = "multi_thread")]
async fn main() -> Result<()> {
    let args = Args::parse();
    let lowercased_outputs = args.outputs.to_lowercase();
    let outputs: Vec<&str> = lowercased_outputs.split(',').collect();

    let mut dd_api_key: String = "".to_string();
    let mut dd_api_host: String = "".to_string();
    if outputs.contains(&"api") {
        match env::var("DD_API_KEY") {
            Ok(val) => dd_api_key = val,
            Err(_e) => {
                eprintln!("Error: DD_API_KEY not set in environment");
                std::process::exit(78);
            }
        }
        match env::var("DD_API_HOST") {
            Ok(val) => dd_api_host = val,
            Err(_e) => dd_api_host = "https://api.datadoghq.com/api/v1".to_string(),
        }
    }

    let mut tags = vec![];
    for tag in args.tags.split(',') {
        if tag.contains(':') {
            let mut tag_split = tag.split(':');
            tags.push((
                tag_split.next().unwrap().to_string(),
                tag_split.next().unwrap().to_string(),
            ));
        } else if !tag.is_empty() {
            eprintln!("Invalid tag format. Ignoring tag: {}", tag);
        }
    }

    if args.verbose {
        env_logger::builder()
            .filter_level(LevelFilter::Debug)
            .init();
    }

    let metrics = DataDogBuilder::default()
        .tags(tags)
        .api_host(dd_api_host)
        .write_to_stdout(outputs.contains(&"stdout"))
        .write_to_api(outputs.contains(&"api"), Some(dd_api_key.to_string()))
        .build()?
        .install()
        .unwrap();

    match args.r#type.to_lowercase().as_str() {
        "counter" => counter!(args.name.to_string(), args.value.round() as u64),
        "gauge" => gauge!(args.name.to_string(), args.value),
        "histogram" => histogram!(args.name.to_string(), args.value),
        _ => {
            eprintln!("Error: unknown metric type");
            std::process::exit(78);
        }
    };
    match metrics.flush().await {
        Ok(_) => {
            println!(
                "{} {} sent to Datadog with value {:?}",
                args.r#type, args.name, args.value
            );
            Ok(())
        }
        Err(e) => {
            eprintln!(
                "Error sending {} {} to Datadog: {}",
                args.name, args.r#type, e
            );
            std::process::exit(1);
        }
    }
}
