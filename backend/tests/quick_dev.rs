use anyhow::Result;
use serde_json::json;

#[tokio::test]
async fn quick_dev() -> Result<()> {
    let hc = httpc_test::new_client("http://localhost:3000")?;
    hc.do_get("/").await?.print().await?;
    let req_login = hc.do_post(
        "/api/users/login",
        json!({
            "username": "user",
            "password": "pass",
        }),
    );
    req_login.await?.print().await?;

    hc.do_get("/").await?.print().await?;
    hc.do_get("/cookies").await?.print().await?;
    hc.do_get("/cookies").await?.print().await?;
    hc.do_get("/cookies").await?.print().await?;
    Ok(())
}
