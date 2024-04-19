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

	let req_logoff = hc.do_post(
		"/api/logoff",
		json!({
			"logoff": true
		}),
	);

	let req_create_task = hc.do_post(
		"/api/rpc",
		json!({
			"id": 1,
			"method": "create_task",
			"params": {
				"data": {
					"title": "task AAA"
				}
			}
		}),
	);
	req_create_task.await?.print().await?;

	let req_edit_task = hc.do_post(
		"/api/rpc",
		json!({
			"id": 1,
			"method": "update_task",
			"params": {
				"id": 1000,
				"data": {
					"title": "task BB"
				}
			}
		}),
	);
	req_edit_task.await?.print().await?;

	let req_delete_task = hc.do_post(
		"/api/rpc",
		json!({
			"id": 1,
			"method": "delete_task",
			"params": {
				"id": 1001
			}
		})
	);
	req_delete_task.await?.print().await?;

	let req_list_tasks = hc.do_post(
		"/api/rpc",
		json!({
			"id": 1,
			"method": "list_tasks"
		})
	);
	req_list_tasks.await?.print().await?;

	// req_logoff.await?.print().await?;

	// hc.do_get("/hello").await?.print().await?;

	Ok(())
}

//  cargo watch -q -c -w src/ -w .cargo/ -x run
// cargo watch -q -c -x "test -- --nocapture"
// docker run --rm --name pg -p 5432:5432 -e POSTGRES_PASSWORD=welcome postgres:15