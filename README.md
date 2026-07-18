# dotclaude

WakaTaira の Claude Code 資産（公開分）。skills / agents / hooks / statusline / keybindings を単一リポジトリで管理する。

Self-made Claude Code assets: skills, agents, hooks, a Rust statusline, and keybindings.

## 構成

| パス | 内容 |
|---|---|
| `skills/` | 自作スキル（relay / relay-opus / pc-power / hunk-watch / grill-me） |
| `agents/` | relay 系サブエージェント定義 7 種 |
| `statusline/` | Rust 製 statusline（stdin 駆動・低 RSS）。`cargo build --release` で `target/release/statusline` を生成 |
| `keybindings.json` | Claude Code キーバインド設定 |

## 導入

ホームの `~/.claude/` から本リポジトリ内へ symlink を張って使う。
非公開資産と統合した親リポジトリ（dotclaude-private）の submodule `pub/` として運用するのが正位置。

## 備考

- `skills/grill-me` は [mattpocock/skills](https://github.com/mattpocock/skills) の grill-me 系スキルを起点に大幅に改変・統合したもの（詳細は同ディレクトリの PROVENANCE.md）
