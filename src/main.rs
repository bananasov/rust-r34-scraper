extern crate clap;
use clap::{Arg, App};

mod requests;
mod json;
mod scraper;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let matches = App::new("Rule34 Scraper")
        .version("0.1")
        .about("Scrapes explicit images from a list of sites")
        .arg(Arg::with_name("site")
                .short("s")
                .long("site")
                .value_name("SITE")
                .takes_value(true)
                .required(true)
                .help("Sets the site to scrape from (MUST BE A SUPPORTED SITE)")
        )
        .arg(Arg::with_name("limit")
                .short("l")
                .long("limit")
                .value_name("LIMIT")
                .takes_value(true)
                .required(true)
                .help("The amount of images you want to download (Max, 100)")
        ).arg(Arg::with_name("tags")
                .short("t")
                .long("tags")
                .value_name("TAGS")
                .takes_value(true)
                .required(true)
                .help("tags you want to scrape (Split apart by a space)") 
    ).arg(Arg::with_name("dir")
            .long("dir")
            .value_name("DIR")
            .takes_value(true)
            .required(false)
            .help("Sets the directory for the downloaded images")
    ).arg(Arg::with_name("debug")
        .short("d")
        .long("debug")
        .value_name("DEBUG")
        .takes_value(false)
        .required(false)
        .help("Sets debug mode to enabled")
    ).arg(Arg::with_name("links-only")
        .short("x")
        .long("links-only")
        .help("Logs the links only, send the output to a file with piping")
    ).get_matches();

    // shit begins here
    let site = matches.value_of("site").unwrap_or("rule34.xxx");
    let limit = matches.value_of("limit").unwrap_or("1");
    let tags = matches.value_of("tags").unwrap();
    let dir = matches.value_of("dir").unwrap_or("./downloads");

    let dbg = matches.is_present("debug");
    let links_only = matches.is_present("links-only");
    let mut start = 1;

    // setup the scraper
    scraper::init(dir);

    let actualtags = tags.replace(" ", "+");
    let url =  format!("https://api.r34.app/booru/gelbooru/posts?baseEndpoint={}&limit={}&tags={}", site, limit, actualtags);

    println!("---------------------------------------");
    println!("| URL: {}", site);
    println!("| Limit: {}", limit);
    println!("| Tags: {}", tags);
    println!("---------------------------------------");

    println!("Started Scraping like the horny bitch you are");

    let cock = requests::get(url)?;
    let datas: Vec<json::File> = serde_json::from_str(cock.as_str())?;

    for data in datas.iter() {
        let file_ext = data.high_res_file.url.split(".").last().unwrap();
        let filename = format!("{}.{}", data.id, file_ext);

        if links_only {
            println!("{}", data.high_res_file.url);
        } else {
            if dbg {
                println!("scraping file {} ({}/{})", filename, start, limit);
            }

            scraper::download_file(data.high_res_file.url.clone(), String::from(filename), dir)?;
            start = start + 1;
        }  
    }

    Ok(())
}
