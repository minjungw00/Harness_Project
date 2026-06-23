pub fn codex_guidance_body(integration_id: &str, project_id: &str) -> String {
    guidance_body("Codex", integration_id, project_id)
}

pub fn claude_code_guidance_body(integration_id: &str, project_id: &str) -> String {
    guidance_body("Claude Code", integration_id, project_id)
}

fn guidance_body(host_label: &str, integration_id: &str, project_id: &str) -> String {
    format!(
        "## Volicord MCP guidance for {host_label}\n\
         \n\
         - This repository has optional Volicord guidance for `integration_id` `{integration_id}` and `project_id` `{project_id}`.\n\
         - Use Volicord for work that needs durable scope, state, write preparation, run evidence, user judgment, or close-readiness tracking.\n\
         - Use the exposed Volicord MCP tools rather than inventing Volicord state in prose.\n\
         - If the target Product Repository is unclear, call `volicord.list_projects`; do not guess `project_id` from paths, folder names, roots, labels, or memory.\n\
         - A Volicord record or `Write Authorization` does not independently grant filesystem, host, or user permission outside its documented meaning.\n\
         - Follow this repository's existing instructions. This guidance supplements Volicord MCP server instructions and does not override unrelated project rules.\n\
         - Volicord MCP server instructions and repository guidance can help tool selection, but they are not enforcement and cannot guarantee model behavior.\n"
    )
}
