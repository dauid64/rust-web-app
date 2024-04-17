#![allow(unused)] // For beginning only.

use anyhow::Result;
use serde_json::json;

#[tokio::main]
async fn main() -> Result<()> {
	let hc = httpc_test::new_client("http://localhost:8080")?;

	hc.do_get("/index.html").await?.print().await?;

	let req_login = hc.do_post(
		"/api/login",
		json!({
			"username": "demo1",
			"pwd": "welcome"
		}),
	);
	req_login.await?.print().await?;

	Ok(())
}

// cargo watch -q -c -w src/ -w .cargo/ -x run
// cargo watch -q -c -w examples/ -x "run --example quick_dev"
// docker run --rm --name pg -p 5432:5432 -e POSTGRES_PASSWORD=welcome postgres:15