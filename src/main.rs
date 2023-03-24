use reqwest::{Client, Error};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize)]
struct NotionPage {
    parent: Parent,
    properties: Properties,
}

#[derive(Serialize, Deserialize)]
struct Parent {
    #[serde(rename = "type")]
    type_: String,
    database_id: String,
}

#[derive(Serialize, Deserialize)]
struct Properties {
    #[serde(rename = "Grocery item")]
    grocery_item: Title,
    Price: Number,
    #[serde(rename = "Last ordered")]
    last_ordered: Date,
}

#[derive(Serialize, Deserialize)]
struct Title {
    #[serde(rename = "type")]
    type_: String,
    title: Vec<Text>,
}

#[derive(Serialize, Deserialize)]
struct Text {
    #[serde(rename = "type")]
    type_: String,
    text: Content,
}

#[derive(Serialize, Deserialize)]
struct Content {
    content: String,
}

#[derive(Serialize, Deserialize)]
struct Number {
    #[serde(rename = "type")]
    type_: String,
    number: i64,
}

#[derive(Serialize, Deserialize)]
struct Date {
    #[serde(rename = "type")]
    type_: String,
    date: Start,
}

#[derive(Serialize, Deserialize)]
struct Start {
    start: String,
}


#[tokio::main]
async fn main() -> Result<(), Error> {
    let client = Client::new();

    let notion_page = NotionPage {
        parent: Parent {
            type_: String::from("database_id"),
            database_id: String::from("35fd9e0dda20451297b9153388941669"),
        },
        properties: Properties {
            grocery_item: Title {
                type_: String::from("title"),
                title: vec![Text {
                    type_: String::from("text"),
                    text: Content {
                        content: String::from("Oleg"),
                    },
                }],
            },
            Price: Number {
                type_: String::from("number"),
                number: 1337,
            },
            last_ordered: Date {
                type_: String::from("date"),
                date: Start {
                    start: String::from("2021-04-11"),
                },
            },

            
        },

        
    };

    
    
    //get info from db
    let get_response = reqwest::Client::new()
        .post("https://api.notion.com/v1/databases/35fd9e0dda20451297b9153388941669/query")
        .header("Authorization", "Bearer secret_t3oEqeeKdfn2vLAskJlj4nGavuM6fnKs9Tf833moaFT")
        .header("Content-Type", "application/json")
        .header("Notion-Version", "2022-06-28")
        .send()
        .await?
        .json::<serde_json::Value>()
        .await?;

    println!("{:?}", get_response);

    let row_ids = get_response["results"]
        .as_array()
        .unwrap()
        .iter()
        .map(|row| row["id"].as_str().unwrap().to_string())
        .collect::<Vec<String>>();

    //println!("{:?}", row_ids);

    for row_id in row_ids {
        let delete_response = reqwest::Client::new()
            .delete(&format!("https://api.notion.com/v1/blocks/{}", row_id))
            .header("Authorization", "Bearer secret_t3oEqeeKdfn2vLAskJlj4nGavuM6fnKs9Tf833moaFT")
            .header("Notion-Version", "2022-06-28")
            .send()
            .await?
            .text()
            .await?;
        println!("Row deleted: {}", delete_response);
    }

    //add info to a db
    let add_response = client
        .post("https://api.notion.com/v1/pages")
        .header("Authorization", "Bearer secret_t3oEqeeKdfn2vLAskJlj4nGavuM6fnKs9Tf833moaFT")
        .header("Content-Type", "application/json")
        .header("Notion-Version", "2022-06-28")
        .json(&notion_page)
        .send()
        .await?;

    //println!("{:?}", response);

    Ok(())
}
