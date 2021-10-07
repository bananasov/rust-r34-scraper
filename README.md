# rust-34-scraper
A rule34 scraper made in rust this time

# Building
- Clone the repository
- Execute `cargo build --release`
    - `--release` is an optimized build profile
- Your folder will be in `/path/to/rust-r34-scraper/target/release`

# Command line arguments
```
Rule34 Scraper 0.1
Scrapes explicit images from a list of sites

USAGE:
    rust-r34-scraper [FLAGS] [OPTIONS] --limit <LIMIT> --site <SITE> --tags <TAGS>

FLAGS:
    -d, --debug         Sets debug mode to enabled
    -h, --help          Prints help information
    -x, --links-only    Logs the links only, send the output to a file with piping
    -V, --version       Prints version information

OPTIONS:
        --dir <DIR>        Sets the directory for the downloaded images
    -l, --limit <LIMIT>    The amount of images you want to download (Max, 100)
    -s, --site <SITE>      Sets the site to scrape from (MUST BE A SUPPORTED SITE)
    -t, --tags <TAGS>      tags you want to scrape (Split apart by a space)
```

# Supported domains
- rule34.xxx
- rule34.paheal.net
- danbooru.donmai.us
- gelbooru.com
- e621.net
- safebooru.org
- e926.net

## Example

### Without a custom directory
```bash
rust-r34-scraper --site rule34.xxx --limit 100 --tags "nude sex video"
```
will download 100 videos of nude sex to the downloads folder.

### With a custom directory
```bash
rust-r34-scraper --site rule34.xxx --limit 100 --tags "nude sex video" --dir /usr/home/rule34
```
will download 100 videos of nude sex to `/usr/home/rule34`

### Links only example
```bash
rust-r34-scraper --site rule34.xxx --limit 100 --tags "nude sex video" --links-only
```
will output 100 links to nude sex video's in the console