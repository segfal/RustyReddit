use serde::Deserialize;

use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

use roux::Reddit;

#[derive(Deserialize, Debug)]
pub struct Config {
    USER_AGENT: String,
    REDDIT_KEY : String,
    REDDIT_SECRET : String,
    REDDIT_USERNAME : String,
    REDDIT_PASSWORD : String,
}


pub fn read_config_from_file<P: AsRef<Path>>(path: P) -> Result<Config, Box<dyn Error>> {
    // Open the file in read-only mode with buffer.
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    // Read the JSON contents of the file as an instance of `Config`.
    let u = serde_json::from_reader(reader)?;

    // Return the `Config`.
    Ok(u)
}

pub async fn Reddit_Login() -> i32
{
    //if file does not exist return -1
    if !Path::new("config.json").exists()
    {
        println!("Config file does not exist");
        return -1;
    }


    let config = read_config_from_file("config.json").unwrap();
    let reddit = Reddit::new(user_agent: config.USER_AGENT, client_id: config.REDDIT_KEY, client_secret: config.REDDIT_SECRET).username(config.REDDIT_USERNAME).password(config.REDDIT_PASSWORD).login()
    println!("Reddit Login");
    return 1;


}


