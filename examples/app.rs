use apollo_client::open::{requests::OpenAppRequest, OpenApiClientBuilder};
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    env_logger::init();

    // Create open platform api client.
    let client = OpenApiClientBuilder::new(
        "http://127.0.0.1:8070/".parse()?,
        "391cc4053f8cce2e452a0e6db8925bbba503f434",
    )?
    .build()?;

    // Execute app fetching request.
    let responses = client
        .execute(
            OpenAppRequest::builder()
                .app_ids(vec!["SampleApp".into()])
                .build(),
        )
        .await?;

    dbg!(responses);

    Ok(())
}
