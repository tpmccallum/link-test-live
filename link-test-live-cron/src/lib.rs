use serde_json::json;
use serde_json::Value;
use spin_cron_sdk::{cron_component, Metadata};
use spin_sdk::{
    http::{Method, Request, Response},
    key_value::Store,
    variables,
};

#[cron_component]
async fn handle_cron_event(metadata: Metadata) -> anyhow::Result<()> {
    let cloud_url;
    if variables::get("mode")? == "local" {
        println!("Test mode enabled ... using locahost ...");
        cloud_url = APIURL {
            url: "http://localhost:3000".to_string(),
        };
    } else {
        println!("Test mode disabled ... using cloud ...");
        cloud_url = APIURL {
            url: "https://link-test-live-cloud.fermyon.app".to_string(),
        };
    }
    let cloud_url_string: String = cloud_url.url_to_string();
    println!("Checking for API key, please wait ...");
    let store = Store::open_default()?;
    // TODO REMOVE THIS SET TO NO LINE
    store.set("busy", "no".as_bytes())?;
    //panic!("Just resetting the busy flag to no for testing purposes ...");
    match store.get("api_key")? {
        Some(value) => {
            match store.get("busy")? {
                Some(busy_value) => {
                    match std::str::from_utf8(&busy_value) {
                        Ok(busy_str) => {
                            match busy_str {
                                "yes" => {
                                    println!("Still busy checking the last batch of links. Will try again later ...");
                                }
                                "no" => {
                                    let mut json_result = json!({
                                        "urls": [],
                                    });
                                    println!("Fetching new batch of links to check ...");
                                    store.set("busy", "yes".as_bytes())?;

                                    let string_value = String::from_utf8(value)?;
                                    println!(
                                        "Using API Key {:?}",
                                        Some(string_value.to_string().trim_matches('"'))
                                    );
                                    let links_per_batch = variables::get("links")?;
                                    let host_whitelist = variables::get("hosts")?;
                                    let mut json_request_data = json!({
                                        "api_key": string_value.to_string().trim_matches('"'),
                                        "links_per_batch": links_per_batch.as_str(),
                                        "allowed_outbound_hosts": host_whitelist.as_str()
                                    });
                                    let request = Request::builder()
                                        .method(Method::Post)
                                        .uri(cloud_url_string.clone())
                                        .body(serde_json::to_vec(&json_request_data)?)
                                        .build();
                                    let response: Response = spin_sdk::http::send(request).await?;
                                    let json_data: serde_json::Value =
                                        serde_json::from_slice(&response.body())?;
                                    let mut json_result = json!({
                                        "results": [],
                                    });
                                    if let Some(urls) = json_data["urls"].as_array() {
                                        for url in urls {
                                            if let Some(url_str) = url.as_str() {
                                                println!("Processing: {}", url_str);
                                                let request = Request::builder()
                                                    .method(Method::Get)
                                                    .uri(url.as_str().unwrap())
                                                    .build();
                                                let response: Response =
                                                    spin_sdk::http::send(request).await?;
                                                let status_code = response.status();
                                                let url_entry =
                                                    json!({"url": url, "status_code": status_code});
                                                json_result["results"]
                                                    .as_array_mut()
                                                    .unwrap()
                                                    .push(url_entry);
                                            } else {
                                                println!("No 'url' string found in the response.");
                                            }
                                        }
                                        if json_request_data.get("results").is_none() {
                                            json_request_data["results"] = json!([]);
                                        }
                                        json_request_data["results"]
                                            .as_array_mut()
                                            .unwrap()
                                            .push(json_result.clone());
                                        println!("All checked\n\n: {:?}", json_result);
                                        let request = Request::builder()
                                            .method(Method::Post)
                                            .uri(cloud_url_string.clone())
                                            .body(serde_json::to_vec(&json_result)?)
                                            .build();
                                        let response: Response =
                                            spin_sdk::http::send(request).await?;
                                        //check response code
                                        let status_code: u16 = *response.status();
                                        if status_code == 200 {
                                            println!("Points awarded successfully!");
                                            store.set("busy", "no".as_bytes())?;
                                        } else {
                                            println!(
                                                "Failed to award points! Something went wrong ..."
                                            );
                                        }
                                    } else {
                                        println!("No 'urls' array found in the response.");
                                    }
                                }
                                _ => {
                                    store.set("busy", "no".as_bytes())?;
                                    println!("Initializing, please wait! Will fetch links to check soon ...");
                                }
                            }
                        }
                        Err(e) => {
                            println!("Failed to convert busy value to string: {:?}", e);
                            // Add your error handling logic here
                        }
                    }
                }
                None => {
                    println!("The 'busy' key is not present in the store.");
                    // Add your logic for the None case here
                }
            }

            Ok(())
        }

        None => {
            println!("No API Key found, generating a new one ...");
            let request = Request::builder()
                .method(Method::Get)
                .uri(cloud_url_string)
                .build();
            let response: Response = spin_sdk::http::send(request).await?;
            let json_data: Value = serde_json::from_slice(&response.body())?;
            let api_key = json_data["your_new_api_key"].clone().to_string();
            let api_key_trimmed = api_key.trim_matches('"');
            println!("New API Key: {:?}", api_key_trimmed);
            store.set("api_key", api_key_trimmed.as_bytes())?;
            Ok(())
        }
    }
}

#[derive(Debug)]
struct APIURL {
    url: String,
}

impl APIURL {
    fn url_to_string(&self) -> String {
        format!("{}", self.url)
    }
}

#[derive(Debug)]
struct CheckList {
    urls: Vec<String>,
}
