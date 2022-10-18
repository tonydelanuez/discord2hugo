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

struct GitConfig {
  repo_dir: String,
  git_repo: String,
}

const ADMIN_ROLE: u64 = 922534457297227826;

#[tokio::main]
async fn main() {

    let git_config = GitConfig{
      repo_dir: env::var("REPO_DIR").expect("repo dir"),
      git_repo: env::var("GIT_REPO").expect("git repo")
    };
    println!("repo dir: {}", git_config.repo_dir);
    println!("git repo: {}", git_config.git_repo);

    clone_repo(&git_config);

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


fn run_git_command(args: Vec<&str>, config: &GitConfig){
    let output = std::process::Command::new("git")
                                                      .current_dir(&config.repo_dir)
                                                      .args(&args)
                                                      .output()
                                                      .expect(&format!("Failed to run git command. Command: {:?}", &args ));
    println!("stdout: {}",  String::from_utf8_lossy(&output.stdout));
    println!("stderr: {}",  String::from_utf8_lossy(&output.stderr));
    assert!(output.status.success());
}

fn clone_repo(git_config: &GitConfig) {
  let git_dir = format!("{}/.git", &git_config.repo_dir);
  if !Path::new(git_dir.as_str()).exists() {
    println!("Repository not present, cloning");
    run_git_command(vec!["clone", &*git_config.git_repo, &*git_config.repo_dir], git_config);
  } else {
    println!("pulling latest refs");
    run_git_command(vec!["pull"], git_config);
  }
}

fn git_commit_and_push(path: &str, post_name: &str, git_config: &GitConfig) {

  println!("Adding file to git index");
  run_git_command(vec!["add", path], git_config);

  println!("Committing file");
  run_git_command(vec!["commit", "-m", &format!("\"add {post_name}\"")], git_config);

  println!("Pushing to remote");
  run_git_command(vec!["push"], git_config);
}



#[command]
async fn post(ctx: &Context, msg: &Message) -> CommandResult {
    msg.reply(ctx, "Writing blog post...").await?;

    let guild_id = msg.guild_id.unwrap();
    let admin_role_id = RoleId(ADMIN_ROLE);
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
    
    let post = post::Post{
      content: String::from(content),
      title: post_title,
      categories: vec![],
      tags: vec![],
    };

    let git_config = GitConfig{
      repo_dir: env::var("REPO_DIR").expect("repo dir"),
      git_repo: env::var("GIT_REPO").expect("git repo")
    };
    let sanitized_title = post.sanitized_title();
    let repo_dir = &git_config.repo_dir;
    let path = format!("{repo_dir}/content/posts/{sanitized_title}.md");
    // write post to git, push to remote
    if !Path::new(&path).exists() {
      post.write_to_file(&path);
      git_commit_and_push(&path, &sanitized_title, &git_config);
    }
    
    msg.reply(ctx, format!("blog post pushed! Go check it out at tdoot.com/{} soon.", sanitized_title)).await?;
    println!("blog post pushed");

    Ok(())
}

