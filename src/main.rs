mod client;

use structopt::StructOpt;
use anyhow::Error;
use serde_json::Value;
use serde::{ Serialize, Deserialize };
use crate::client::Client;
use std::collections::{ HashSet, HashMap };
use anyhow::Context;

#[derive(Debug, Clone, StructOpt)]
struct Opts {
    /// A token that has the necessary scopes and such
    #[structopt(long, env = "TOKEN")]
    token: String,

    /// Channel ID to read messages from
    #[structopt(long)]
    channel: String
}

#[tokio::main]
async fn main() -> Result<(),Error> {

    let opts = Opts::from_args();
    eprintln!("{:?}", opts);

    let client = Client::new(Some(opts.token));

    // Get messages:
    let messages = get_messages(&client, &opts.channel).await.with_context(|| format!("Failed to retrieve messages"))?;

    // Get threads
    let thread_ids: HashSet<&str> = messages.iter().filter_map(|m| m.thread_ts.as_ref().map(|id| id.as_str())).collect();
    let mut threads: HashMap<&str, Vec<Message>> = HashMap::new();
    for thread_id in thread_ids {
        let messages = get_thread(&client, &opts.channel, thread_id).await.with_context(|| format!("Failed to retrieve a thread"))?;
        threads.insert(thread_id, messages);
    }

    // find out which user IDs are present:
    let mut user_ids: HashSet<&str> = HashSet::new();
    for msg in &messages {
        user_ids.insert(&*msg.user);
    }
    for msgs in threads.values() {
        for msg in msgs {
            user_ids.insert(&*msg.user);
        }
    }

    // Get proper usernames for those users:
    let mut users: HashMap<&str, String> = HashMap::new();
    for user_id in user_ids {
        let username = client.request("users.info", &[("user", user_id)])
            .await
            .map(|u: Value| u["user"]["name"].as_str().unwrap_or("").to_owned())?;
        if !username.is_empty() {
            users.insert(user_id, username);
        }
    }

    // Convert messages to output:
    let mut output: Vec<OutputMessage> = Vec::new();
    for msg in &messages {
        output.push(to_output_message(msg, &users, &threads));
    }

    println!("{}", serde_json::to_string_pretty(&output).unwrap());

    Ok(())
}

fn to_output_message(msg: &Message, users: &HashMap<&str,String>, threads: &HashMap<&str,Vec<Message>>) -> OutputMessage {
    let thread = if let Some(thread_id) = msg.thread_ts.as_ref() {
        threads.get(&**thread_id).map(|t| {
            t.iter().map(|msg| to_output_message(msg, users, &HashMap::new())).collect()
        })
    } else {
        None
    };

    let username = users
        .get(&*msg.user)
        .map(|u| u.to_owned())
        .unwrap_or("<unknown>".to_owned());

    OutputMessage {
        ts: msg.ts.clone(),
        name: username,
        text: msg.text.clone(),
        thread: thread
    }
}

async fn get_messages(client: &Client, channel: &str) -> Result<Vec<Message>,Error> {
    get_message_like(client, "conversations.history", &[("channel", channel)]).await
}

async fn get_thread(client: &Client, channel: &str, thread_id: &str) -> Result<Vec<Message>,Error> {
    let msgs = get_message_like(client, "conversations.replies", &[("ts", thread_id),("channel", &channel)]).await?;
    let filtered_msgs = msgs.into_iter().filter(|m| Some(&m.ts) != m.thread_ts.as_ref()).collect();
    Ok(filtered_msgs)
}

async fn get_message_like<P: Serialize>(client: &Client, api: &'static str, params: P) -> Result<Vec<Message>,Error> {
    #[derive(Deserialize,Debug)]
    struct Conversations {
        messages: Vec<Message>,
        response_metadata: Option<ResponseMetadata>
    }

    #[derive(Deserialize,Debug)]
    struct ResponseMetadata {
        next_cursor: Option<String>
    }

    let get_msgs = |cursor: Option<String>| async {
        let res: Conversations = client.request_with_cursor(
            api,
            &params,
            cursor
        ).await?;
        let cursor = res.response_metadata.and_then(|md| md.next_cursor);
        Ok::<_,Error>((res.messages, cursor))
    };

    let (mut messages, mut cursor) = get_msgs(None).await?;
    while let Some(this_cursor) = cursor {
        let (mut more_messages, next_cursor) = get_msgs(Some(this_cursor)).await?;
        messages.append(&mut more_messages);
        cursor = next_cursor;
    }

    messages.sort_by_key(|msg| msg.ts.to_owned());
    Ok(messages)
}

#[derive(Deserialize,Debug)]
struct Message {
    ts: String,
    text: String,
    user: String,
    r#type: String,
    thread_ts: Option<String>
}

#[derive(Serialize,Debug)]
struct OutputMessage {
    ts: String,
    name: String,
    text: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    thread: Option<Vec<OutputMessage>>
}