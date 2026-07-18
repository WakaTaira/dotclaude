---
name: hunk-watch
description: Hunk TUI の新規 user コメントを常駐 Monitor でセッションに流し込み、コメント駆動の修正ループを自動化する。ユーザーが Hunk セッションを開いて「hunk 見といて」「hunk watch」「コメント拾って」などと監視を頼んだときに使用する。
---

# hunk-watch

Hunk は pull 型の設計であり、確定済みコメントをエージェントへ push する仕組みを持たない。そのため、確定コメントの検知はエージェント側の常駐 Monitor で行う。

## 手順

1. `hunk session list` でライブセッションを確認する。
   - 0 件: ユーザーに `hunk diff --watch` の起動を依頼して終了する
   - 1 件: 自動選択する
   - 複数件: 対象 repo をユーザーに確認する
2. Monitor ツールで以下のスクリプトを張る（`persistent: true`）。`<REPO>` は対象セッションの repo 絶対パス、`<SEEN>` はスクラッチパッド配下の一意なファイルパスに置換する。

```sh
command -v jq >/dev/null || { echo "jq が無いため監視不能"; exit 1; }
SEEN="<SEEN>"
REPO="<REPO>"
hunk session comment list --repo "$REPO" --type user --json 2>/dev/null | jq -r '.comments[].noteId' > "$SEEN" || :
fails=0
while true; do
  out=$(hunk session comment list --repo "$REPO" --type user --json 2>&1)
  if [ $? -ne 0 ]; then
    fails=$((fails+1))
    [ $fails -ge 3 ] && { echo "HUNK-SESSION-LOST: $(echo "$out" | command head -1)"; exit 1; }
  else
    fails=0
    echo "$out" | jq -r '.comments[] | "\(.noteId)\t\(.filePath):\(.newRange[0])\t\(.body | gsub("\n"; " / "))"' | while IFS=$'\t' read -r id loc body; do
      command grep -qxF "$id" "$SEEN" 2>/dev/null || { echo "NEW-COMMENT [$loc] $body"; echo "$id" >> "$SEEN"; }
    done
  fi
  sleep 2
done
```

3. `NEW-COMMENT` イベントを受けたら:
   - コメントの指示に従って対象ファイルを修正する（差分は Hunk 側の `--watch` が自動反映する）
   - `hunk session comment add --repo <REPO> --file <path> --new-line <n> --summary "<対応内容>" --author "<自分の呼び名>"` で TUI に返信する
   - CLI 操作の詳細は hunk-review スキルに従う
4. `HUNK-SESSION-LOST` を受けたら、監視が終了した旨をユーザーに一言報告する。

## 注意

- Monitor はセッション寿命であり、新しい Claude セッションでは張り直しが必要（この「一言で張り直す」運用を自動化しすぎない — hunk が動いていないセッションで常駐させても監視が即座に死ぬだけである）
- 監視を止めるよう頼まれたら TaskStop で該当 Monitor を止める
