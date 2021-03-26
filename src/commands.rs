use serenity::{
    model::channel::Message,
    prelude::*
};
use crate::scraper::{Post, Request};

#[derive(Clone)]
pub struct Parsedmsg {
    pub command: String,
    pub arguments: Vec<String>,
    msg: Message
}

impl Parsedmsg {
    pub fn new(msg: Message) -> Option<Parsedmsg> {
       let mut message = msg.content.trim_start();
       match message.starts_with('!') {
            true => message = message.trim_start_matches('!'),
            false => {
                return None;
            }
       }
       let mut iter = message.split_whitespace();
       let mut arguments: Vec<String> = Vec::new();
       let command = match iter.next(){
           Some(s) => s.to_string(),
           None => {return None;},
       };
       while let Some(arg) = iter.next() {
           arguments.push(arg.to_string());
       }
       Some(Parsedmsg {command, arguments, msg})
    }
    
    pub async fn reddit(&self, context: Context) {
        let request = Request::new(self.clone());
        let post: Post = match request.get_post().await{
            Some(x) => {
                x
            },
            None => {return;},
        };
        let files = vec![&post.url[1..post.url.len()-1]];
        if &post.post_hint[1 .. post.post_hint.len()-1] != "image"{
            if let Err(why) = self.msg.channel_id.say(&context.http, format!("It might not be an image. Heres the link anyway {}", post.url)).await {
                println!("Error sending message: {:?}", why);
            }
        } else {
            if let Err(why) = self.msg.channel_id.send_files(&context.http, files, |m| {
                m.content(format!("Author: {}\nTitle: {}", post.author, post.title))
            }).await {
                println!("Error sending message: {:?}", why);
            };
        }
    }

    pub async fn ping(&self, context: Context) {
        println!("{} pinged the bot", self.msg.author.name);
        if let Err(why) = self.msg.channel_id.say(&context.http, "Pong!").await {
            println!("Error sending message: {:?}", why);
        }
    }

    pub async fn help(&self, context: Context) {
        if let Err(why) = self.msg.channel_id.say(&context.http, r#"
COMMANDS
!reddit <subreddit default=memes> <sorting=top|hot|new|rising default=hot> -- Get post from reddit
!ping -- Pings the bot
!help -- Prints this message"#).await {
            println!("Error sending message: {:?}", why);
        }
    }
}
