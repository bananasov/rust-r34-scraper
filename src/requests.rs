pub fn get(url: String) -> Result<String, Box<dyn std::error::Error>> {
    let res = reqwest::blocking::get(url.as_str())?;
    let body = res.text()?;

    Ok(body)
}