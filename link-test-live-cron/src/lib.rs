use serde_json::Value;
use spin_cron_sdk::{cron_component, Error, Metadata};

#[cron_component]
fn handle_cron_event(metadata: Metadata) -> Result<(), Error> {
    use serde_json::Value;
    use spin_cron_sdk::{cron_component, Metadata};
    use spin_sdk::{
        http::{Method, Request, Response},
        key_value::Store,
    };

    #[cron_component]
    async fn handle_cron_event(metadata: Metadata) -> anyhow::Result<()> {
        // Build request to the Cloud API
        let request = Request::builder()
            .method(Method::Get)
            // need to add json body here that bundles up the links_per_batch and the user's api key 
            .uri("https://link-test-live-cloud.fermyon.app")
            .build();
        // Await the response from the API, then store the response
        let response: Response = spin_sdk::http::send(request).await?;
        // Extract the JSON data from the body of the response
        let json_data: Value = serde_json::from_slice(&response.body())?;
        // Create a new instance of the struct (an index variable) that stores the timestamp and uvi
        let url = json_data["url"].to_string().as_str();
        // Check the URL and create a new json object to send to the cloud
        println!("Processed {:?} at timestamp: [{}]", url, index.get_timestamp());
        Ok(())
    }
}
