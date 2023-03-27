use reqwest::{Client, Error, header::{HeaderMap, HeaderValue, AUTHORIZATION, CONTENT_TYPE}};
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



#[derive(Debug, Deserialize, Serialize)]
struct NewProperty {
    id: String,
    name: String,
    r#type: String,
}

async fn add_property_to_database(api_key: &str, database_id: &str) -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new();
    let mut headers = HeaderMap::new();
    headers.insert(AUTHORIZATION, format!("Bearer {}", api_key).parse().unwrap());
    headers.insert(CONTENT_TYPE, "application/json".parse().unwrap());
    headers.insert("Notion-Version", "2022-06-28".parse().unwrap());

    let new_property = NewProperty {
        id: "description".to_string(),
        name: "Description".to_string(),
        r#type: "rich_text".to_string(),
    };
    let mut properties = HashMap::new();
    properties.insert("Description".to_string(), new_property);

    let body = serde_json::json!({
        "properties": {
            "New property1": {
              "rich_text": {}
            }
          }
    });

    let url = format!("https://api.notion.com/v1/databases/{}", database_id);
    let response = client
    .patch(&url)
    .headers(headers)
    .json(&body)
    .send()
    .await?;

    println!("Response: {:?}", response);
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    // let client = Client::new();

    // let notion_page = NotionPage {
    //     parent: Parent {
    //         type_: String::from("database_id"),
    //         database_id: String::from("35fd9e0dda20451297b9153388941669"),
    //     },
    //     properties: Properties {
    //         grocery_item: Title {
    //             type_: String::from("title"),
    //             title: vec![Text {
    //                 type_: String::from("text"),
    //                 text: Content {
    //                     content: String::from("Oleg"),
    //                 },
    //             }],
    //         },
    //         Price: Number {
    //             type_: String::from("number"),
    //             number: 1337,
    //         },
    //         last_ordered: Date {
    //             type_: String::from("date"),
    //             date: Start {
    //                 start: String::from("2021-04-11"),
    //             },
    //         },

            
    //     },

        
    // };

    
    
    // //get info from db
    // let get_response = reqwest::Client::new()
    //     .post("https://api.notion.com/v1/databases/35fd9e0dda20451297b9153388941669/query")
    //     .header("Authorization", "Bearer secret_t3oEqeeKdfn2vLAskJlj4nGavuM6fnKs9Tf833moaFT")
    //     .header("Content-Type", "application/json")
    //     .header("Notion-Version", "2022-06-28")
    //     .send()
    //     .await?
    //     .json::<serde_json::Value>()
    //     .await?;

    // println!("{:?}", get_response);

    // let row_ids = get_response["results"]
    //     .as_array()
    //     .unwrap()
    //     .iter()
    //     .map(|row| row["id"].as_str().unwrap().to_string())
    //     .collect::<Vec<String>>();

    // //println!("{:?}", row_ids);

    // for row_id in row_ids {
    //     let delete_response = reqwest::Client::new()
    //         .delete(&format!("https://api.notion.com/v1/blocks/{}", row_id))
    //         .header("Authorization", "Bearer secret_t3oEqeeKdfn2vLAskJlj4nGavuM6fnKs9Tf833moaFT")
    //         .header("Notion-Version", "2022-06-28")
    //         .send()
    //         .await?
    //         .text()
    //         .await?;
    //     println!("Row deleted: {}", delete_response);
    // }

    // //add info to a db
    // let add_response = client
    //     .post("https://api.notion.com/v1/pages")
    //     .header("Authorization", "Bearer secret_t3oEqeeKdfn2vLAskJlj4nGavuM6fnKs9Tf833moaFT")
    //     .header("Content-Type", "application/json")
    //     .header("Notion-Version", "2022-06-28")
    //     .json(&notion_page)
    //     .send()
    //     .await?;

    // //println!("{:?}", response);


    add_property_to_database("secret_t3oEqeeKdfn2vLAskJlj4nGavuM6fnKs9Tf833moaFT", "66caada96e3d46cc8a74bed36fc90a7f").await.unwrap();

    Ok(())
}
