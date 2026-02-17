mod common;

use rmcp::model::{CallToolRequestParams, GetPromptRequestParams, ReadResourceRequestParams};
use std::borrow::Cow;

#[tokio::test(flavor = "current_thread")]
async fn list_tools_returns_three_tools() {
    let client = common::spawn_server(None).await;
    let tools = client.list_all_tools().await.unwrap();
    let names: Vec<&str> = tools.iter().map(|t| t.name.as_ref()).collect();
    assert!(
        names.contains(&"compose"),
        "missing compose tool: {names:?}"
    );
    assert!(names.contains(&"list"), "missing list tool: {names:?}");
    assert!(
        names.contains(&"resolve"),
        "missing resolve tool: {names:?}"
    );
    assert!(
        names.contains(&"recommend"),
        "missing recommend tool: {names:?}"
    );
    assert!(
        names.contains(&"check_update"),
        "missing check_update tool: {names:?}"
    );
    assert_eq!(tools.len(), 5);
}

#[tokio::test(flavor = "current_thread")]
async fn compose_persona_only() {
    let client = common::spawn_server(None).await;
    let result = client
        .call_tool(CallToolRequestParams {
            meta: None,
            name: Cow::Borrowed("compose"),
            arguments: Some(
                serde_json::from_value(serde_json::json!({
                    "persona": "backend-engineer"
                }))
                .unwrap(),
            ),
            task: None,
        })
        .await
        .unwrap();
    assert!(!result.is_error.unwrap_or(false));
    let text = result.content.first().unwrap();
    let body = format!("{text:?}");
    assert!(!body.is_empty());
    // Should NOT have section headers since we only requested persona
    assert!(
        !body.contains("## Expertise"),
        "should not have Expertise section"
    );
}

#[tokio::test(flavor = "current_thread")]
async fn compose_full_prompt() {
    let client = common::spawn_server(None).await;
    let result = client
        .call_tool(CallToolRequestParams {
            meta: None,
            name: Cow::Borrowed("compose"),
            arguments: Some(
                serde_json::from_value(serde_json::json!({
                    "persona": "backend-engineer",
                    "skills": ["go", "postgresql"],
                    "contexts": ["code-review"],
                    "tones": ["concise"],
                    "constraints": ["no-frameworks"]
                }))
                .unwrap(),
            ),
            task: None,
        })
        .await
        .unwrap();
    assert!(!result.is_error.unwrap_or(false));
    let text = format!("{:?}", result.content);
    assert!(text.contains("Expertise"), "should have Expertise section");
    assert!(text.contains("Context"), "should have Context section");
    assert!(
        text.contains("Communication Style"),
        "should have Communication Style section"
    );
    assert!(
        text.contains("Constraints"),
        "should have Constraints section"
    );
}

#[tokio::test(flavor = "current_thread")]
async fn compose_missing_persona_returns_error() {
    let client = common::spawn_server(None).await;
    let result = client
        .call_tool(CallToolRequestParams {
            meta: None,
            name: Cow::Borrowed("compose"),
            arguments: Some(
                serde_json::from_value(serde_json::json!({
                    "persona": "nonexistent-persona-xyz"
                }))
                .unwrap(),
            ),
            task: None,
        })
        .await
        .unwrap();
    assert!(result.is_error.unwrap_or(false), "should be an error");
    let text = format!("{:?}", result.content);
    assert!(
        text.contains("not found"),
        "error should mention 'not found'"
    );
}

#[tokio::test(flavor = "current_thread")]
async fn list_all_fragments() {
    let client = common::spawn_server(None).await;
    let result = client
        .call_tool(CallToolRequestParams {
            meta: None,
            name: Cow::Borrowed("list"),
            arguments: Some(serde_json::from_value(serde_json::json!({})).unwrap()),
            task: None,
        })
        .await
        .unwrap();
    assert!(!result.is_error.unwrap_or(false));
    let text = format!("{:?}", result.content);
    // Should be parseable JSON array
    assert!(
        text.contains("backend-engineer"),
        "should list backend-engineer"
    );
}

#[tokio::test(flavor = "current_thread")]
async fn list_filtered_by_category() {
    let client = common::spawn_server(None).await;
    let result = client
        .call_tool(CallToolRequestParams {
            meta: None,
            name: Cow::Borrowed("list"),
            arguments: Some(
                serde_json::from_value(serde_json::json!({
                    "category": "tone"
                }))
                .unwrap(),
            ),
            task: None,
        })
        .await
        .unwrap();
    assert!(!result.is_error.unwrap_or(false));
    let text = format!("{:?}", result.content);
    assert!(text.contains("tone"), "should only return tones");
    assert!(!text.contains("\"persona\""), "should not contain personas");
}

#[tokio::test(flavor = "current_thread")]
async fn resolve_with_persona() {
    let client = common::spawn_server(None).await;
    let result = client
        .call_tool(CallToolRequestParams {
            meta: None,
            name: Cow::Borrowed("resolve"),
            arguments: Some(
                serde_json::from_value(serde_json::json!({
                    "persona": "backend-engineer"
                }))
                .unwrap(),
            ),
            task: None,
        })
        .await
        .unwrap();
    assert!(!result.is_error.unwrap_or(false));
    let text = format!("{:?}", result.content);
    assert!(
        text.contains("persona:backend-engineer"),
        "should contain persona node"
    );
    assert!(text.contains("nodes"), "should have nodes field");
    assert!(text.contains("edges"), "should have edges field");
}

#[tokio::test(flavor = "current_thread")]
async fn list_resources_returns_fragments() {
    let client = common::spawn_server(None).await;
    let resources = client.list_all_resources().await.unwrap();
    assert!(!resources.is_empty(), "should have resources");
    // Check URI format
    let first_uri = &resources[0].uri;
    assert!(
        first_uri.starts_with("mimic://fragments/"),
        "URI should start with mimic://fragments/, got: {first_uri}"
    );
}

#[tokio::test(flavor = "current_thread")]
async fn read_resource_returns_body() {
    let client = common::spawn_server(None).await;
    let result = client
        .read_resource(ReadResourceRequestParams {
            uri: "mimic://fragments/personas/backend-engineer".to_string(),
            meta: None,
        })
        .await
        .unwrap();
    assert!(!result.contents.is_empty());
}

#[tokio::test(flavor = "current_thread")]
async fn read_resource_not_found() {
    let client = common::spawn_server(None).await;
    let result = client
        .read_resource(ReadResourceRequestParams {
            uri: "mimic://fragments/personas/nonexistent-xyz".to_string(),
            meta: None,
        })
        .await;
    assert!(result.is_err(), "should return error for missing resource");
}

#[tokio::test(flavor = "current_thread")]
async fn list_prompts_returns_persona_prompts() {
    let client = common::spawn_server(None).await;
    let prompts = client.list_all_prompts().await.unwrap();
    assert!(!prompts.is_empty(), "should have prompts");
    assert!(
        prompts.iter().all(|p| p.name.starts_with("mimic-")),
        "all prompts should have mimic- prefix"
    );
}

#[tokio::test(flavor = "current_thread")]
async fn get_prompt_returns_composed_text() {
    let client = common::spawn_server(None).await;
    let result = client
        .get_prompt(GetPromptRequestParams {
            name: "mimic-backend-engineer".to_string(),
            arguments: None,
            meta: None,
        })
        .await
        .unwrap();
    assert!(!result.messages.is_empty(), "should have messages");
}

#[tokio::test(flavor = "current_thread")]
async fn server_info_has_correct_name() {
    let client = common::spawn_server(None).await;
    let info = client.peer_info().expect("should have server info");
    assert_eq!(info.server_info.name, "mimic");
}

#[tokio::test(flavor = "current_thread")]
async fn compose_with_project_local_fragment() {
    let tmp = tempfile::tempdir().unwrap();
    let personas_dir = tmp.path().join("personas");
    std::fs::create_dir_all(&personas_dir).unwrap();
    std::fs::write(
        personas_dir.join("custom-test.md"),
        "---\ndescription: A custom test persona\ntags: [test]\nlevel: junior\nskill_groups: [general]\n---\nYou are a custom test persona.",
    ).unwrap();

    let client = common::spawn_server(Some(tmp.path().to_path_buf())).await;
    let result = client
        .call_tool(CallToolRequestParams {
            meta: None,
            name: Cow::Borrowed("compose"),
            arguments: Some(
                serde_json::from_value(serde_json::json!({
                    "persona": "custom-test"
                }))
                .unwrap(),
            ),
            task: None,
        })
        .await
        .unwrap();
    assert!(!result.is_error.unwrap_or(false));
    let text = format!("{:?}", result.content);
    assert!(
        text.contains("custom test persona"),
        "should use project-local fragment"
    );
}
