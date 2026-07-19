# dotclaude

WakaTaira の Claude Code 資産（公開分）。skills / agents / hooks / statusline / keybindings を単一リポジトリで管理する。

Self-made Claude Code assets: skills, agents, hooks, a Rust statusline, and keybindings.

## 構成

| パス | 内容 |
|---|---|
| `skills/` | 自作スキル（relay / relay-opus / pc-power / hunk-watch / grill-me / creating-pull-requests-en / creating-pull-requests-ja） |
| `rules/` | 常時ロードの rules（`~/.claude/rules` から symlink） |
| `agents/` | relay 系サブエージェント定義 7 種 |
| `statusline/` | Rust 製 statusline（stdin 駆動・低 RSS）。`cargo build --release` で `target/release/statusline` を生成 |
| `keybindings.json` | Claude Code キーバインド設定 |

## 導入

ホームの `~/.claude/` から本リポジトリ内へ symlink を張って使う。
非公開資産と統合した親リポジトリ（dotclaude-private）の submodule `pub/` として運用するのが正位置。

## 備考

- `skills/grill-me` は [mattpocock/skills](https://github.com/mattpocock/skills) の grill-me 系スキルを起点に大幅に改変・統合したもの（詳細は同ディレクトリの PROVENANCE.md）
- `rules/i-have-adhd.md` は [ayghri/i-have-adhd](https://github.com/ayghri/i-have-adhd)（MIT License, Copyright (c) 2026 Ayoub Ghriss）の SKILL.md を日本語へ改変・圧縮したもの。適用範囲を「ユーザーの確認・行動を求める出力」に絞り、文体は [k16shikano 氏の日本語ライティング規範 gist](https://gist.github.com/k16shikano/eb2929f13ed19c97188393d297be8432)（Unlicense）を参考にした
- `skills/creating-pull-requests-en` は [google/eng-practices](https://github.com/google/eng-practices) の CL description ガイドライン（CC-BY 3.0, Copyright Google LLC）を基礎に大幅改変したもの。[tdhopper/dotfiles2.0](https://github.com/tdhopper/dotfiles2.0) の creating-pull-requests スキルから手法上の着想を得ているが、同リポジトリはライセンス表示が無いため文章表現は全面的に独自へ書き下ろしている（詳細は同ディレクトリの PROVENANCE.md）
- `skills/creating-pull-requests-ja` は日本語 OSS のマージ済み PR 約 150 件の実地調査に基づく自作。`references/examples.md` に出典 URL 明記付きで公開 PR 本文の引用を含む（詳細は同ディレクトリの PROVENANCE.md）
