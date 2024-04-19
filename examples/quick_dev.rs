#![allow(unused)] // For beginning only.

use anyhow::Result;
use serde_json::json;

#[tokio::main]
async fn main() -> Result<()> {
	let hc = httpc_test::new_client("http://localhost:8000")?;

	hc.do_get("/index.html").await?.print().await?;

	let req_login = hc.do_post(
		"/api/login",
		json!({
			"username": "demo1",
			"pwd": "welcome"
		}),
	);
	req_login.await?.print().await?;

	let req_logoff = hc.do_post(
		"/api/logoff",
		json!({
			"logoff": true
		}),
	);
	req_logoff.await?.print().await?;

	hc.do_get("/hello").await?.print().await?;

	Ok(())
}

//  cargo watch -q -c -w src/ -w .cargo/ -x run
// cargo watch -q -c -x "test -- --nocapture"
// docker run --rm --name pg -p 5432:5432 -e POSTGRES_PASSWORD=welcome postgres:15