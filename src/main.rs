
extern crate clap;
extern crate hyper;
extern crate reqwest;
extern crate pretty_env_logger;
extern crate url;
extern crate hyper_native_tls;
extern crate colored;
extern crate serde; 
extern crate serde_json; 
extern crate webbrowser;
use serde_json::{Value};
use clap::{App,SubCommand};
use colored::*;



fn main() {

let matches = App::new("hn")
        .version("0.1.0")
        .author("VP7 <shihan.viswa@gmail.com>")
        .about("Command Line CLI for HN")
        .subcommand(SubCommand::with_name("ask").about("Display Ask HN posts."))
        .subcommand(SubCommand::with_name("new").about("Display the latest posts."))
        .subcommand(SubCommand::with_name("show").about("Display Show HN posts."))
        .subcommand(SubCommand::with_name("top").about("Display the top recent posts."))
                          .get_matches();

 pretty_env_logger::init();

let ask_stories="https://hacker-news.firebaseio.com/v0/askstories.json?print=pretty";
let show_stories="https://hacker-news.firebaseio.com/v0/showstories.json?print=pretty";
let new_stories="https://hacker-news.firebaseio.com/v0/newstories.json?print=pretty";
let top_stories="https://hacker-news.firebaseio.com/v0/topstories.json?print=pretty";



match matches.subcommand_name() {
        Some("ask") => {
            println!("{}","We are going to Display Ask HN posts".blue());
            make_get_connection(ask_stories);
        },
        Some("new") => {
            println!("{}","We are going to Display the latest posts".blue());
            make_get_connection(new_stories);
            },
        Some("show") => {
            println!("{}","We are going to Display Show HN posts ".blue());
            make_get_connection(show_stories);
            },
        Some("top") => {
            println!("{}","We are going to Display the top recent posts ".blue());
            make_get_connection(top_stories);
        },
         Some("open") => {
            println!("{}","We are going to Display the top recent posts ".blue());
            if matches.is_present("debug") {
            println!("Printing debug info...");
        } else {
            println!("Printing normally...");
        }
        },
        None        => println!("No subcommand was used"),
        _           => unreachable!(), // Assuming you've listed all direct children above, this is unreachable
}


}

fn make_get_connection(uri: &str ){
let mut resp = reqwest::get(uri).unwrap(); 
   assert!(resp.status().is_success());

   let body = resp.text().unwrap();
    
           
           let s_slice: &str = &*body;  // s  : String 
           let all_array: Value = serde_json::from_str(s_slice).unwrap();
            
            for index in 0..10{
                    let id =(all_array[index]).to_string();
                    fetch_specific_data(id,index);
            }
}


fn fetch_specific_data(story_id:  String , current_index: usize){
    let mut owned_string: String = "https://hacker-news.firebaseio.com/v0/item/".to_owned();
    let another_owned_string: String = ".json?print=pretty".to_owned();
    
    owned_string.push_str(&story_id);
    owned_string.push_str(&another_owned_string);

    let mut resp = reqwest::get(&owned_string).unwrap(); 
   assert!(resp.status().is_success());

   let body = resp.text().unwrap();
    

    
     let s_slice: &str = &*body;  // s  : String 
           let json_value: Value = serde_json::from_str(s_slice).unwrap();

     
/*
{
  "by" : "pg",
  "id" : 160705,
  "poll" : 160704,
  "score" : 335,
  "text" : "Yes, ban them; I'm tired of seeing Valleywag stories on News.YC.",
  "time" : 1207886576,
  "type" : "pollopt"
}

*/

            let title = json_value.pointer("/title").unwrap().to_string();
            let score = json_value.pointer("/score").unwrap().to_string();
            let author = json_value.pointer("/by").unwrap().to_string();
         
         
            let mut owned_title: String = title;
            let mut owned_score: String = score;
            let mut owned_author: String = "by".to_owned();

            owned_score.push_str(" ");
            owned_score.push_str("points");
            owned_author.push_str(" ");
            owned_author.push_str(&*author);

let mut owned_index: String = (current_index+1).to_string();
owned_index.push_str(".");

            owned_title.retain(|c| c != '\"');
            owned_author.retain(|c| c != '\"');

           println!(" {}  {} ",owned_index.purple(), owned_title);
           println!(" {} {} ",owned_score.green(),owned_author.cyan());
           
               
}
    