use serde_json::Value;
use crate::commands::Parsedmsg;

const BASE_URL: &str = "https://reddit.com/r/";

pub struct Request {
    subreddit: String,
    sort: String
}

pub struct Post {
    pub author: String,
    pub title: String,
    pub url: String,
    pub post_hint: String
}

impl Request {
    pub fn new(msg: Parsedmsg) -> Request{
        let mut subreddit: String = String::new();
        let sort: String;
        let mut args = msg.arguments;
        // Get the sorting method
        if let Some(index) = args.iter().position(|x| 
            *x == "hot" ||
            *x == "new" ||
            *x == "top" ||
            *x == "rising") {
                sort = String::from(args.remove(index));
        } else {
            sort = String::from("hot");
        }
        args.iter().for_each(|x| subreddit.push_str(&x[..]));
        if subreddit == "".to_string() {
            subreddit = "memes".to_string();
        }
        Request{subreddit, sort}
    }
    pub async fn get_post(&self) -> Option<Post> {
        let url = format!("{}{}/{}.json?limit=4", BASE_URL, self.subreddit, self.sort);
        // let url = format!("https://reddit.com/r/memes/hot?limit=1");
        let body: Option<String> = match reqwest::get(url).await {
            Ok(resp) => match resp.text().await {
                Ok(body) => Some(body),
                Err(_) => None,
            }
            Err(_) => None,
        };
        if let Some(body) = body {
            let x: Value = serde_json::from_str(&body).unwrap();
            let url = x["data"]["children"][0]["data"]["url"].clone().to_string();
            let author = x["data"]["children"][0]["data"]["author"].clone().to_string();
            let title = x["data"]["children"][0]["data"]["title"].clone().to_string();
            let post_hint = x["data"]["children"][0]["data"]["post_hint"].clone().to_string();
            return Some(Post{author, title, url, post_hint});
        }
        None
    }
}
