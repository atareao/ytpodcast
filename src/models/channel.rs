use serde::{Serialize, Deserialize};
use std::fmt::{self, Display};
use regex::Regex;
use chrono::{DateTime, Utc, NaiveDateTime};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Channel {
    url: String,
    title: String,
    description: String,
    image: Option<String>,
    #[serde(default = "get_default_first")]
    first: DateTime<Utc>
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ChannelWithId {
    id: String,
    url: String,
    title: String,
    description: String,
    image: Option<String>,
}

fn get_default_first() -> DateTime<Utc>{
    DateTime::<Utc>::from_utc(NaiveDateTime::from_timestamp_opt(0, 0).unwrap(), Utc)
}

impl Display for Channel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.get_id(), self.url)
    }
}

impl Channel{
    pub fn get_complete(&self) -> ChannelWithId{
        ChannelWithId { 
            id: self.get_id(),
            url: self.url.clone(),
            title: self.title.clone(),
            description: self.description.clone(),
            image: self.image.clone(),
        }
    }
    pub fn get_id(&self) -> String{
        let re = Regex::new(r"[^a-zA-Z0-9_-]").unwrap();
        re.replace_all(&self.get_title().to_lowercase(), "_").to_string()
    }
    pub fn get_first(&self) -> DateTime<Utc>{
        self.first
    }
    pub fn get_url(&self) -> &str{
        &self.url
    }
    pub fn get_title(&self) -> &str{
        &self.title
    }
    pub fn get_image(&self) -> Option<String>{
        self.image.clone()
    }
    pub fn get_description(&self) -> &str{
        &self.description
    }
}


