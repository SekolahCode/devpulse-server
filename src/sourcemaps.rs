use serde_json::{json, Value};
use sqlx::{PgPool, Row};
use std::collections::HashMap;
use uuid::Uuid;

/// Resolves every frame in `stacktrace` (a JSON array of `{file, line, column, ...}` objects)
/// against any source map uploaded for `release`, rewriting matched frames in place — same
/// `file`/`line`/`function`/`context.lines` shape the dashboard already renders for PHP/Laravel
/// stacks, so no frontend change is needed to display resolved JS frames.
///
/// A frame with no matching map, or that fails to resolve for any reason (parse error, token
/// not found), is left completely untouched. This must be a no-op when nothing has been
/// uploaded for the release.
pub async fn resolve_stacktrace(pool: &PgPool, project_id: Uuid, release: &str, stacktrace: &mut Value) {
    let Some(frames) = stacktrace.as_array_mut() else { return };

    // Parsing a source map is the expensive part — a single minified bundle can produce
    // dozens of frames from the same file, so cache per (file -> parsed map) for this call.
    let mut cache: HashMap<String, Option<sourcemap::SourceMap>> = HashMap::new();

    for frame in frames {
        resolve_frame(pool, project_id, release, frame, &mut cache).await;
    }
}

async fn resolve_frame(
    pool:    &PgPool,
    project_id: Uuid,
    release: &str,
    frame:   &mut Value,
    cache:   &mut HashMap<String, Option<sourcemap::SourceMap>>,
) {
    let Some(file) = frame.get("file").and_then(|v| v.as_str()).map(str::to_string) else { return };
    let Some(line) = frame.get("line").and_then(|v| v.as_u64()) else { return };
    let Some(column) = frame.get("column").and_then(|v| v.as_u64()) else { return };

    if !cache.contains_key(&file) {
        let map = fetch_and_parse(pool, project_id, release, &file).await;
        cache.insert(file.clone(), map);
    }

    let Some(Some(sm)) = cache.get(&file) else { return };

    // Stack traces are 1-based (line 1, column 1); sourcemap tokens are 0-based.
    let Some(token) = sm.lookup_token(line.saturating_sub(1) as u32, column.saturating_sub(1) as u32) else { return };

    let src_line = token.get_src_line();
    let src_col  = token.get_src_col();

    if let Some(src) = token.get_source() {
        frame["file"] = json!(src);
    }
    frame["line"]   = json!(src_line + 1);
    frame["column"] = json!(src_col + 1);
    if let Some(name) = token.get_name() {
        frame["function"] = json!(name);
    }
    frame["resolved"] = json!(true);

    // Embedded `sourcesContent` lets us show a real code snippet, same as the
    // PHP SDK's `context.lines` — only present if the build tool inlined sources.
    if let Some(view) = token.get_source_view() {
        let start = src_line.saturating_sub(5);
        let end   = src_line + 5;
        let mut lines = serde_json::Map::new();
        for n in start..=end {
            if let Some(text) = view.get_line(n) {
                lines.insert((n + 1).to_string(), json!(text));
            }
        }
        if !lines.is_empty() {
            frame["context"] = json!({ "start": start + 1, "lines": Value::Object(lines) });
        }
    }
}

async fn fetch_and_parse(pool: &PgPool, project_id: Uuid, release: &str, file: &str) -> Option<sourcemap::SourceMap> {
    let row = sqlx::query(
        "SELECT source_map FROM source_maps WHERE project_id = $1 AND release = $2 AND artifact_name = $3"
    )
    .bind(project_id)
    .bind(release)
    .bind(file)
    .fetch_optional(pool)
    .await
    .ok()??;

    let source_map: Value = row.try_get("source_map").ok()?;
    let bytes = serde_json::to_vec(&source_map).ok()?;
    sourcemap::SourceMap::from_slice(&bytes).ok()
}
