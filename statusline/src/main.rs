// カスタムステータスライン - Claude Code statusline hook 用 (Rust 版)
//
// Claude Code が stdin で渡す JSON (model / context_window / rate_limits) のみを使用する。
// 外部プロセス起動・ネットワーク呼び出しはなし。Python 版 (~/.claude/statusline.py) の
// 出力とバイト単位で互換になるよう整数化・丸め・配色をすべて移植している。

use chrono::{Local, TimeZone};
use serde_json::Value;
use std::io::{self, Read, Write};

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
fn truthy_str<'a>(s: Option<&'a str>) -> Option<&'a str> {
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

fn main() {
    let mut input = String::new();
    let _ = io::stdin().read_to_string(&mut input);
    let data: Value = serde_json::from_str(&input).unwrap_or(Value::Null);

    let parts = [
        render_model(&data),
        render_context(&data),
        render_limit(&data, "five_hour", "⏰", "5h", true),
        render_limit(&data, "seven_day", "📅", "7d", false),
    ];
    let out = parts.join("  ");

    let stdout = io::stdout();
    let mut lock = stdout.lock();
    let _ = lock.write_all(out.as_bytes());
    let _ = lock.flush();
}
