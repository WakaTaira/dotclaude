# 日本語PRの実例集（マージ済みPRより引用）

2026年7月時点で収集した日本語OSSコミュニティのマージ済みPRの実例。文体・テンプレートの埋め方・セクション構成の生きた参考として使用する。各コードブロックは出典PRの本文からの引用（著作権は各執筆者に帰属。分析・学習目的の引用として出典を明記の上で掲載）。

---

## VOICEVOX/voicevox #3067

出典: <https://github.com/VOICEVOX/voicevox/pull/3067>

**タイトル**: `feat: エンジン分離版のビルド経路を追加`

**分析**: VOICEVOX標準テンプレ（内容/関連 Issue/スクリーンショット・動画など/その他）。本文はです・ます調1文で簡潔。該当なしセクションは削除せず「（なし）」と明記。関連Issueは `ref:` で参照のみ（自動クローズしない）。その他にCI実行結果のURLを貼って検証を示す。小規模PRの理想形。

```markdown
## 内容

エンジン分離版のビルドworkflowを追加します。

## 関連 Issue

- ref: #2944

## スクリーンショット・動画など

（なし）

## その他

https://github.com/sevenc-nanashi/voicevox/actions/workflows/build.yml?query=branch%3Afeat%2Fdownload-build-workflow
```

---

## misskey-dev/misskey #17742

出典: <https://github.com/misskey-dev/misskey/pull/17742>

**タイトル**: `refactor(frontend): メディアメニューのメニューボタンのスタイルを統一`

**分析**: misskeyのWhat/Whyバイリンガルテンプレ。テンプレートのHTMLコメント（記入ガイド）を残したまま提出する文化。本文は常体の体言止めで極めて簡潔（「統一」「コードの削減」）——テンプレが構造を保証するため文章量は最小でよい。チェックリストは実施済みのみチェック。

```markdown
<!-- ℹ お読みください / README
PRありがとうございます！ PRを作成する前に、コントリビューションガイドをご確認ください:
Thank you for your PR! Before creating a PR, please check the contribution guide:
https://github.com/misskey-dev/misskey/blob/develop/CONTRIBUTING.md
-->

## What
<!-- このPRで何をしたのか？ どう変わるのか？ -->
<!-- What did you do with this PR? How will it change things? -->

もともとは別だったが、どっちもほぼ同じ見た目になったため統一

## Why
<!-- なぜそうするのか？ どういう意図なのか？ 何が困っているのか？ -->
<!-- Why do you do it? What are your intentions? What is the problem? -->
コードの削減

## Additional info (optional)
<!-- テスト観点など -->
<!-- Test perspective, etc -->

## Checklist
- [x] Read the [contribution guide](https://github.com/misskey-dev/misskey/blob/develop/CONTRIBUTING.md)
- [ ] Test working in a local environment
- [ ] (If needed) Add story of storybook
- [ ] (If needed) Update CHANGELOG.md
- [ ] (If possible) Add tests
```

---

## sakura-editor/sakura #2532

出典: <https://github.com/sakura-editor/sakura/pull/2532>

**タイトル**: `エディター初期化完了イベントを導入する`

**分析**: sakura-editorの厳格テンプレ（`<!-- 必須 -->` 指示付き）。全必須セクションを埋め、「※厳密には〜」で正確性の但し書き、「本件、勝手にやります」のような率直な宣言も許容される文化。背景→仕様→影響範囲→テストの流れで、issue番号 #2531 を背景とresolvesの両方に記載。

```markdown
<!-- これはコメントです。ブラウザで表示されません。 -->
<!-- Preview のシートで見た目のチェックができます。 -->

# <!-- 必須 --> PR対象

<!-- PR対象を以下のテンプレートより選択してください。 -->
<!-- 該当するものがなければ追加してください。 -->

- アプリ(サクラエディタ本体)
- テストコード

## <!-- 必須 --> カテゴリ

<!-- 編集 必須 -->
<!-- 以下のテンプレは自由に編集してください。 -->

- 改善

## <!-- 必須 --> PR の背景

<!-- PR前に作成したissue番号を記載してください。 -->
<!-- issueを作成していない場合、背景の説明で代替しても良いです。 -->
- #2531

サクラエディタには、エディターの初期化完了を検出する方法がありません。

このため、編集ウインドウの作成、ファイルを開く、Grep実行などのテストを書くことができません。

※厳密には、書くこと自体はできるが、初期化完了を検知できないのでちょくちょく失敗する。

## <!-- 必須 --> 仕様・動作説明

<!-- ふるまいを変えない変更の場合は省略可。 -->
- コントロールプロセス側の初期化完了イベントを参考に、エディター初期化完了イベントを追加します。
- アプリ側変更を最小限にし、テストで使えるかどうか検証するための仮仕様として実装します。

本件、勝手にやります。

## <!-- わかる範囲で --> PR の影響範囲

<!-- 影響範囲を記載してください。 -->
- この変更により、Grep実行のテストで余分な完了待ちをする必要がなくなります。

## <!-- 必須 --> テスト内容

<!-- PR内容の妥当性をどのように確認したかについて記載してください。 -->

<!-- レビュアーが確認する再現手順があれば記載してください。 -->

## <!-- なければ省略可 --> 関連 issue, PR

<!-- 関連する issue, PR の情報を記載してください。 -->
<!-- #xxx と書くと チケット xxx に対して自動的にリンクが張られます。 -->
<!-- 参考: https://help.github.com/en/articles/closing-issues-using-keywords-->
<!-- issue, PR の URL をそのまま貼り付けても OK -->
- resolves #2531


## <!-- なければ省略可 --> 参考資料

<!-- 参考になる資料の URL 等あればここに記載御願いします -->
<!-- 説明に必要なスクリーンショットがあれば貼り付けお願いします。-->
<!-- 画像ファイルをこの欄にドラッグ＆ドロップすれば画像が貼り付けられます -->
```

---

## rurema/doctree #3231

出典: <https://github.com/rurema/doctree/pull/3231>

**タイトル**: `制御構造の修飾子見出しにアンカーを追加(検索対応)`

**分析**: テンプレート無しリポジトリでの自由形式。冒頭1段落で上流PRとの関係と目的を提示。箇条書きで「やったこと・対応不要だった理由・対象外とした理由・判断保留の報告（必要ならご指示ください）」を明示——見送り事項の明記が信頼を生む好例。検証段落で具体的な確認手順を記述。AI帰属表記付きのままマージされている。

```markdown
rurema/bitclust#262(doc ページのアンカー付き見出しを検索インデックスに追加)のフォローアップです。制御構造のページでアンカーが無かった見出しに `{#id}` を付与し、検索でヒットするようにします。

- 追加した5つ(既存の snake_case 慣例に準拠・基本キーワードの既存アンカーとは区別): `{#if_modifier}`・`{#unless_modifier}`・`{#while_modifier}`・`{#until_modifier}`・`{#rescue_modifier}`
- if/unless/case/while/until/for/break/next/redo/retry/raise/begin/return/BEGIN/END は既にアンカーあり。def.md・call.md・literal.md・operator.md も全見出しアンカー済みで対応不要でした
- ensure・(修飾子でない)rescue は begin 節の本文中にあり独立見出しが無いため、構成変更なしにはアンカーを付けられず対象外としています
- 判断保留の報告: eval.md に多数のアンカー無しキーワード見出し(`#### if` 等 15 個程度)がありますが、control.md 等と重複する要約ページのため、検索ヒットが二重になることを避けて今回は見送りました(必要ならご指示ください)

検証: scratch DB(3.4)で render し `<h3 id='if_modifier'>` 等の出力を確認。SearchIndexGenerator の実行で5アンカー全てが重複なしの heading エントリとして載ることも確認済み。

🤖 Generated with [Claude Code](https://claude.com/claude-code)
```

---

## コーパス統計（149件の日本語マージ済みPR）

- 頻出セクション見出し: 内容(28) / その他(25) / What(25) / Why(25) / Checklist(25) / 関連 Issue(24) / テスト内容(16) / 影響範囲(16) / スクリーンショット・動画など(15) / 概要(6) / 背景(4)
- タイトルprefix: なし(71) / fix(28) / chore(12) / feat(12) / test(8) / enhance(6) / ci(4) / refactor(3)
- Issue参照: 「関連 issue」セクション(40) / fixes #(16) / ref(11) / fix #(8) / close #(6) / resolves(4)
- 本文にです・ます調を含む: 113/149 (76%)
