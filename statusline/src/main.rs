// カスタムステータスライン - Claude Code statusline hook 用 (Rust 版)
//
// Claude Code が stdin で渡す JSON (model / context_window / rate_limits) を基本入力とする。
// 外部プロセス起動は行わない。ただし fable モデルの週間使用量のみ OAuth usage API
// (https://api.anthropic.com/api/oauth/usage) への HTTP 呼び出しで取得し、結果は短命
// キャッシュ経由で参照する。その他の表示は Python 版 (~/.claude/statusline.py) の
// 出力とバイト単位で互換になるよう整数化・丸め・配色をすべて移植している。

use chrono::{Local, TimeZone};
use serde_json::{json, Value};
use std::fs;
use std::io::{self, Read, Write};
use std::path::{Path, PathBuf};
use std::time::{Duration, SystemTime, UNIX_EPOCH};

// --- fable 週間使用量 (usage API) 設定 ---
const USAGE_API_URL: &str = "https://api.anthropic.com/api/oauth/usage";
const USAGE_CACHE_TTL_SECS: i64 = 120; // キャッシュ有効期間。API の呼び出し頻度を抑えるためのしきい値
const USAGE_HTTP_TIMEOUT: Duration = Duration::from_secs(2); // 接続・全体ともに 2 秒

// --- 表示設定 ---
const CONTEXT_LIMIT: u64 = 500_000; // コンテキストバーの上限
const BAR_WIDTH: usize = 10;

// --- ANSI カラー ---
const RESET: &str = "\x1b[0m";
const GREEN: &str = "\x1b[32m";
const YELLOW: &str = "\x1b[33m";
const RED: &str = "\x1b[1;31m"; // 太字赤 = 上限超過の警告
const CYAN: &str = "\x1b[36m"; // モデル名表示用

/// トークン数を K / M 表記に整形する (Python f"{n/1000:.0f}K" 相当)
fn fmt_tokens(n: u64) -> String {
    if n >= 1_000_000 {
        format!("{:.1}M", n as f64 / 1_000_000.0)
    } else {
        format!("{:.0}K", n as f64 / 1_000.0)
    }
}

/// Python の round() と一致させるための banker's rounding (round-half-to-even)
fn round_half_even(x: f64) -> i64 {
    let floor = x.floor();
    let frac = x - floor;
    if (frac - 0.5).abs() < 1e-9 {
        let fi = floor as i64;
        if fi.rem_euclid(2) == 0 {
            fi
        } else {
            fi + 1
        }
    } else if frac < 0.5 {
        floor as i64
    } else {
        floor as i64 + 1
    }
}

/// used / limit の割合でブロックを塗りつぶす (超過時は満タンで止める)
fn token_bar(used: u64, limit: u64, width: usize) -> String {
    let ratio = if limit > 0 {
        used as f64 / limit as f64
    } else {
        0.0
    };
    let raw = round_half_even(ratio * width as f64);
    let filled = raw.max(0).min(width as i64) as usize;
    let mut s = String::with_capacity(width * 3);
    for _ in 0..filled {
        s.push('█');
    }
    for _ in 0..(width - filled) {
        s.push('░');
    }
    s
}

/// コンテキスト使用率に応じた色 (1.0 以上 = 上限超過で赤)
fn context_color(ratio: f64) -> &'static str {
    if ratio >= 1.0 {
        RED
    } else if ratio >= 0.8 {
        YELLOW
    } else {
        GREEN
    }
}

/// 週間残量に応じた色 (残りが少ないほど警告色)
fn remain_color(remain_pct: f64) -> &'static str {
    if remain_pct > 50.0 {
        GREEN
    } else if remain_pct > 20.0 {
        YELLOW
    } else {
        RED
    }
}

/// Python の `x or y` セマンティクスを文字列にだけ適用する (空文字も falsy 扱い)
fn truthy_str(s: Option<&str>) -> Option<&str> {
    s.filter(|v| !v.is_empty())
}

/// 現在使用中のモデル名を表示する
fn render_model(data: &Value) -> String {
    let name = truthy_str(data["model"]["display_name"].as_str())
        .or_else(|| truthy_str(data["model"]["id"].as_str()))
        .unwrap_or("?");
    format!("{}🤖 {}{}", CYAN, name, RESET)
}

/// 現在セッションのコンテキスト消費を 500k 上限のバーで表示する
fn render_context(data: &Value) -> String {
    let ctx = &data["context_window"];
    let used = ctx["total_input_tokens"].as_u64().unwrap_or(0)
        + ctx["total_output_tokens"].as_u64().unwrap_or(0);
    if used == 0 {
        return "🧠 --".to_string();
    }
    let ratio = used as f64 / CONTEXT_LIMIT as f64;
    let bar = token_bar(used, CONTEXT_LIMIT, BAR_WIDTH);
    format!(
        "{}🧠 {}/{} [{}]{}",
        context_color(ratio),
        fmt_tokens(used),
        fmt_tokens(CONTEXT_LIMIT),
        bar,
        RESET
    )
}

/// Unix 秒をローカルの HH:MM に整形する
fn fmt_reset(ts: i64) -> String {
    match Local.timestamp_opt(ts, 0).single() {
        Some(dt) => dt.format("%H:%M").to_string(),
        None => String::new(),
    }
}

/// レート制限の残量を表示する (5時間 / 週間で共通)
fn render_limit(data: &Value, key: &str, icon: &str, label: &str, show_reset: bool) -> String {
    let info = &data["rate_limits"][key];
    let used_pct = match info["used_percentage"].as_f64() {
        Some(v) => v,
        None => return format!("{} {} --", icon, label),
    };
    let remain = (100.0 - used_pct).max(0.0);
    let mut text = format!("{} {} {:.0}% left", icon, label, remain);
    if show_reset {
        if let Some(ts) = info["resets_at"].as_i64() {
            let r = fmt_reset(ts);
            if !r.is_empty() {
                text.push_str(&format!(" → {}", r));
            }
        }
    }
    format!("{}{}{}", remain_color(remain), text, RESET)
}

/// ホームディレクトリを解決する (HOME、なければ Windows の USERPROFILE)
fn resolve_home() -> Option<String> {
    std::env::var("HOME")
        .ok()
        .filter(|s| !s.is_empty())
        .or_else(|| std::env::var("USERPROFILE").ok().filter(|s| !s.is_empty()))
}

/// 現在時刻を Unix 秒で返す (取得失敗時は 0)
fn now_unix() -> i64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs() as i64)
        .unwrap_or(0)
}

/// usage キャッシュファイルのパス ($HOME/.cache/claude-statusline/usage.json)
fn usage_cache_path(home: &str) -> PathBuf {
    Path::new(home)
        .join(".cache")
        .join("claude-statusline")
        .join("usage.json")
}

/// OAuth アクセストークンを認証情報ファイルから読み取る (refreshToken には触れない)
fn read_access_token(home: &str) -> Option<String> {
    let path = Path::new(home).join(".claude").join(".credentials.json");
    let content = fs::read_to_string(path).ok()?;
    let v: Value = serde_json::from_str(&content).ok()?;
    v["claudeAiOauth"]["accessToken"]
        .as_str()
        .filter(|s| !s.is_empty())
        .map(|s| s.to_string())
}

/// usage API へ問い合わせ、レスポンス JSON を返す (失敗時は None)
fn fetch_usage(token: &str) -> Option<Value> {
    let agent = ureq::AgentBuilder::new()
        .timeout_connect(USAGE_HTTP_TIMEOUT)
        .timeout(USAGE_HTTP_TIMEOUT)
        .build();
    let resp = agent
        .get(USAGE_API_URL)
        .set("Authorization", &format!("Bearer {}", token))
        .set("anthropic-beta", "oauth-2025-04-20")
        .call()
        .ok()?;
    let body = resp.into_string().ok()?;
    serde_json::from_str(&body).ok()
}

/// キャッシュファイルを読み、{ attempted_at, payload } オブジェクトを返す
fn read_usage_cache(path: &Path) -> Option<Value> {
    let content = fs::read_to_string(path).ok()?;
    serde_json::from_str(&content).ok()
}

/// キャッシュを一時ファイル経由で原子的に書き込む (並行起動のレース対策)。失敗しても panic しない
fn write_usage_cache(path: &Path, attempted_at: i64, payload: &Value) {
    let dir = match path.parent() {
        Some(d) => d,
        None => return,
    };
    if fs::create_dir_all(dir).is_err() {
        return;
    }
    let obj = json!({ "attempted_at": attempted_at, "payload": payload });
    let serialized = match serde_json::to_string(&obj) {
        Ok(s) => s,
        Err(_) => return,
    };
    let tmp = dir.join(format!("usage.json.tmp.{}", std::process::id()));
    if fs::write(&tmp, serialized.as_bytes()).is_err() {
        return;
    }
    let _ = fs::rename(&tmp, path);
}

/// usage payload を取得する。TTL 内はキャッシュ、TTL 切れは API を呼び出して更新する。
/// API 失敗時は attempted_at のみ更新し旧 payload を保持する (オフライン時のバックオフ)
fn load_usage_payload() -> Option<Value> {
    let home = resolve_home()?;
    let path = usage_cache_path(&home);
    let now = now_unix();

    let cache = read_usage_cache(&path);
    if let Some(ref c) = cache {
        if let Some(at) = c.get("attempted_at").and_then(|v| v.as_i64()) {
            // 経過時間が負 (時計の巻き戻り) の場合は新鮮とみなさず再取得に落とす
            if (0..USAGE_CACHE_TTL_SECS).contains(&(now - at)) {
                // TTL 内はキャッシュ済み payload をそのまま用いる (null も含む)
                return c.get("payload").cloned();
            }
        }
    }

    // TTL 切れまたはキャッシュ不在。トークンを読み API 取得を試みる
    let fetched = read_access_token(&home).and_then(|token| fetch_usage(&token));
    match fetched {
        Some(payload) => {
            write_usage_cache(&path, now, &payload);
            Some(payload)
        }
        None => {
            // 取得失敗。旧 payload を保持したまま attempted_at のみ更新する
            let old_payload = cache
                .as_ref()
                .and_then(|c| c.get("payload").cloned())
                .unwrap_or(Value::Null);
            write_usage_cache(&path, now, &old_payload);
            Some(old_payload)
        }
    }
}

/// usage payload から fable の週間使用率 (percent) を取り出す。
/// 優先: scope.model.display_name が "fable" を含むもの。なければ最初の weekly_scoped
fn fable_weekly_percent(payload: &Value) -> Option<f64> {
    let limits = payload.get("limits")?.as_array()?;
    let weekly: Vec<&Value> = limits
        .iter()
        .filter(|e| e.get("kind").and_then(|k| k.as_str()) == Some("weekly_scoped"))
        .collect();
    if weekly.is_empty() {
        return None;
    }
    let chosen = weekly
        .iter()
        .find(|e| {
            e["scope"]["model"]["display_name"]
                .as_str()
                .map(|n| n.to_lowercase().contains("fable"))
                .unwrap_or(false)
        })
        .copied()
        .unwrap_or(weekly[0]);
    chosen.get("percent")?.as_f64()
}

/// fable 週間残量セグメントを描画する。データが無い場合は None を返し出力しない
fn render_fable_weekly(payload: &Value) -> Option<String> {
    let used_pct = fable_weekly_percent(payload)?;
    let remain = (100.0 - used_pct).max(0.0);
    let text = format!("📅 7d(F) {:.0}% left", remain);
    Some(format!("{}{}{}", remain_color(remain), text, RESET))
}

fn main() {
    let mut input = String::new();
    let _ = io::stdin().read_to_string(&mut input);
    let data: Value = serde_json::from_str(&input).unwrap_or(Value::Null);

    let mut parts: Vec<String> = vec![
        render_model(&data),
        render_context(&data),
        render_limit(&data, "five_hour", "⏰", "5h", true),
        render_limit(&data, "seven_day", "📅", "7d", false),
    ];
    // fable 週間使用量を seven_day の直後に並記する (取得不能時はセグメント自体を出さない)
    if let Some(payload) = load_usage_payload() {
        if let Some(seg) = render_fable_weekly(&payload) {
            parts.push(seg);
        }
    }
    let out = parts.join("  ");

    let stdout = io::stdout();
    let mut lock = stdout.lock();
    let _ = lock.write_all(out.as_bytes());
    let _ = lock.flush();
}
