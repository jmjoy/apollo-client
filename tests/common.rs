use std::{
    env,
    fs::File,
    path::PathBuf,
    process::{exit, Command},
    sync::Once,
    time::Duration,
};
use tokio::time::sleep;

#[cfg(feature = "open")]
use apollo_client::open::{OpenApiClient, OpenApiClientBuilder};
use reqwest::Client;
use tokio::{runtime::Handle, task::block_in_place, time};

static START: Once = Once::new();

pub fn setup() {
    START.call_once(|| {
        env_logger::init();
        // setup_mysql();
        // setup_docker();
    });
}

#[allow(dead_code)]
fn setup_docker() {
    let mut down = Command::new("docker-compose");
    let output = down.arg("down").output().unwrap();
    let down_stdout = String::from_utf8(output.stdout);
    let down_stderr = String::from_utf8(output.stderr);
    log::info!(
        "docker-compose down, stdout: {:?}, stderr: {:?}",
        down_stdout,
        down_stderr
    );
    assert!(output.status.success());

    let mut up = Command::new("docker-compose");
    let output = up.arg("up").arg("-d").output().unwrap();
    let up_stdout = String::from_utf8(output.stdout);
    let up_stderr = String::from_utf8(output.stderr);
    log::info!(
        "docker-compose up -d, stdout: {:?}, stderr: {:?}",
        up_stdout,
        up_stderr
    );
    assert!(output.status.success());

    let b = block_in_place(|| {
        Handle::current().block_on(async move {
            for _ in 0..12 {
                time::sleep(Duration::from_secs(5)).await;

                let client = Client::builder().build().unwrap();
                let response = client.get("http://localhost:8080").send().await;
                if response.is_err() {
                    continue;
                }
                // match response {
                //     Ok(response) => if !response.status().is_success() {
                //         continue;
                //     }
                //     Err(_) => continue,
                // }

                let response = client.get("http://localhost:8070").send().await;
                if response.is_err() {
                    continue;
                }
                // match response {
                //     Ok(response) => if !response.status().is_success() {
                //         continue;
                //     }
                //     Err(_) => continue,
                // }

                return true;
            }

            false
        })
    });

    if !b {
        panic!("docker-compose up failed");
    }
}

/// Restore sql to rebuild the fixture.
#[allow(dead_code)]
fn setup_mysql() {
    let mut sql_path = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    sql_path.push("sql");
    sql_path.push("apollodb.sql");
    let sql_file = File::open(sql_path).unwrap();

    let mut cmd = Command::new("mysql");
    let output = cmd
        .args(["-h", "127.0.0.1", "-u", "root"])
        .stdin(sql_file)
        .output()
        .unwrap();

    assert!(output.status.success());
}

#[allow(dead_code)]
pub fn ensure_timeout(dur: Duration) {
    tokio::spawn(async move {
        sleep(dur).await;
        log::error!("Test failed: {:?} timeout", dur);
        exit(1);
    });
}

#[cfg(feature = "open")]
pub fn create_open_client() -> OpenApiClient {
    OpenApiClientBuilder::new(
        "http://127.0.0.1:8070/".parse().unwrap(),
        "391cc4053f8cce2e452a0e6db8925bbba503f434",
    )
    .unwrap()
    .build()
    .unwrap()
}
