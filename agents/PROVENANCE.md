# 出典

## defense-recon / defense-validator / defense-verifier / defense-patcher と skills/nixos-validate

自環境防御用のエージェント群とスキルは自作である。文章はすべて独自に書き下ろしており、以下の資料から構成・観点・手法を参考にした（取得日 2026-11-19）。

| 資料 | rev | ライセンス | 参考にした点 |
|---|---|---|---|
| [anthropics/claude-plugins-official](https://github.com/anthropics/claude-plugins-official) の `plugins/claude-security` | df9699a | 独自ライセンス（Copyright (c) 2026 Anthropic, PBC. 再配布・公開不可） | 調査 → 反証 → 修正案 → 修正の検証という役割分担、反証側の既定を誤検知とする考え方。**着想のみ**であり、文章・プロンプトは複製していない |
| [anthropics/defending-code-reference-harness](https://github.com/anthropics/defending-code-reference-harness) | d3bea6b | Apache-2.0 | 修正した当人に合否を判定させないこと、実行結果で合否を決める段階的な確認、失敗から成功へ変わる試験で修正を確かめること（`docs/pipeline.md`、`docs/patching.md`） |
| [lian-yue/finecomb](https://github.com/lian-yue/finecomb) | 165b69e | Apache-2.0 | ホストと OS の確認項目、実行環境と配備の確認項目（`references/targets/hosts-and-operating-systems.md`、`references/dimensions/34-runtime-environment-and-deployment-contract.md`）。defense-recon の「棚卸しの観点」は、これらを自宅端末の認証・鍵管理向けに絞り込み、日本語で書き直したものである |
| [mukul975/Anthropic-Cybersecurity-Skills](https://github.com/mukul975/Anthropic-Cybersecurity-Skills) の `skills/validating-tpm-measured-boot-attestation` | 54a7988 | Apache-2.0 | PCR の読み取りと、イベントログから再計算した値との照合。鍵の生成・封印の手順は TPM に書き込むため採用していない。個人による著作物であり、Anthropic の公式資料ではない |
| [VoltAgent/awesome-claude-code-subagents](https://github.com/VoltAgent/awesome-claude-code-subagents) の `categories/03-infrastructure/security-engineer.md` | ed74d11 | MIT（Copyright (c) 2025 VoltAgent） | OS 水準の硬化という観点の見出し |

- 更新方法: 上流の該当ファイルと照らし、観点の追加があれば自分の言葉で取り込む。`claude-security` の文面は公開リポジトリへ持ち込まない
- 取り込み時の確認: 参考にしたファイルは全文を読み、指示のすり替えや外部への送信が含まれないことを確かめた（finecomb は上記 2 ファイルと配布定義のみ）
