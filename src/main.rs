use std::env;
use std::path::Path;
mod post;

use serenity::async_trait;
use serenity::model::prelude::Message;
use serenity::model::prelude::RoleId;
use serenity::prelude::*;
use serenity::framework::standard::macros::{command, group};
use serenity::framework::standard::{StandardFramework, CommandResult};

#[group]
#[commands(post)]
struct General;

struct Handler;

#[async_trait]
impl EventHandler for Handler {}

#[tokio::main]
async fn main() {

    println!("Bootstrapping repository");
    clone_repo();

    println!("Setting up bot framework");
    let framework = StandardFramework::new()
        .configure(|c| c.prefix("~")) // set the bot's prefix to "~"
        .group(&GENERAL_GROUP);

    // Login with a bot token from the environment
    let token = env::var("DISCORD_TOKEN").expect("token");
    let intents = GatewayIntents::non_privileged() | GatewayIntents::MESSAGE_CONTENT;
    let mut client = Client::builder(token, intents)
        .event_handler(Handler)
        .framework(framework)
        .await
        .expect("Error creating client");

    // start listening for events by starting a single shard
    println!("Starting discord client");
    if let Err(why) = client.start().await {
        println!("An error occurred while running the client: {:?}", why);
    }
}


fn clone_repo() {
  let repo_dir = env::var("REPO_DIR").expect("repo dir");
  let git_repo = env::var("GIT_REPO").expect("git repo");
  println!("repo dir: {}", repo_dir);
  println!("git repo: {}", git_repo);

  let git_dir = format!("{}/.git", &repo_dir);
  if !Path::new(git_dir.as_str()).exists() {
    let output = std::process::Command::new("git")
                                                      .arg("clone")
                                                      .arg(git_repo)
                                                      .arg(repo_dir)
                                                      .output()
                                                      .expect("Failed to clone repo");
    println!("stdout: {}",  String::from_utf8_lossy(&output.stdout));
    println!("stderr: {}",  String::from_utf8_lossy(&output.stderr));
    assert!(output.status.success());
  };
}

fn git_commit_and_push(path: &str, post_name: &str) {
  let repo_dir = env::var("REPO_DIR").expect("repo dir");

  let output = std::process::Command::new("git")
                                                      .current_dir(&repo_dir)
                                                      .arg("add")
                                                      .arg(path)
                                                      .output()
                                                      .expect("Failed to add file");
  println!("stdout: {}",  String::from_utf8_lossy(&output.stdout));
  println!("stderr: {}",  String::from_utf8_lossy(&output.stderr));
  assert!(output.status.success());

  let output = std::process::Command::new("git")
                                                      .current_dir(&repo_dir)
                                                      .arg("commit")
                                                      .arg("-m")
                                                      .arg(format!("\"add {post_name}\""))
                                                      .output()
                                                      .expect("Failed to commit");
  println!("stdout: {}",  String::from_utf8_lossy(&output.stdout));
  println!("stderr: {}",  String::from_utf8_lossy(&output.stderr));
  assert!(output.status.success());

  let output = std::process::Command::new("git")
                                                      .current_dir(&repo_dir)
                                                      .arg("push")
                                                      .output()
                                                      .expect("Failed to push");
  println!("stdout: {}",  String::from_utf8_lossy(&output.stdout));
  println!("stderr: {}",  String::from_utf8_lossy(&output.stderr));
  assert!(output.status.success());
}



#[command]
async fn post(ctx: &Context, msg: &Message) -> CommandResult {
    msg.reply(ctx, "Writing blog post...").await?;

    let guild_id = msg.guild_id.unwrap();
    let admin_role_id = RoleId(922534457297227826);
    let can_post = msg.author.has_role(ctx, guild_id, admin_role_id).await?;
    if !can_post {
      panic!("User is not an admin.");
    }
    let post = msg.referenced_message.to_owned().unwrap();

    // if post.kind != MessageType::ThreadStarterMessage {
    //   panic!("This isn't a thread starter!")
    // }
    let content = &post.content;
    let channel = post.channel(ctx).await?.guild().unwrap();

    
    let post_title = channel.name;
    let message = format!("Found post title: {post_title}, content: {content}");
    
    let post = post::Post{
      content: String::from(content),
      title: post_title,
      categories: vec![],
      tags: vec![],
    };

    let repo_dir = env::var("REPO_DIR").expect("repo dir");
    let message_id = msg.id.to_string();
    let sanitized_title = post.sanitized_title();
    let path = format!("{repo_dir}/content/posts/{message_id}-{sanitized_title}.txt");
    // write post to git, push to remote
    if !Path::new(&path).exists() {
      post.write_to_file(&path);
      git_commit_and_push(&path, &sanitized_title);
    }
    
    msg.reply(ctx, format!("blog post pushed! Go check it out at tdoot.com/{} soon.", sanitized_title)).await?;
    println!("blog post pushed");

    Ok(())
}

