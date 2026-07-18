---
name: pc-power
description: >-
  Control the power state of the host Windows machine from either WSL or
  native Windows Claude Code. Supports sleep, hibernate, lock, shutdown,
  and restart. Invokable as /pc-power or via natural language. Arguments
  are interpreted as natural language in any language — no rigid keyword
  matching. TRIGGER phrases (non-exhaustive):
  "PC寝かせて", "ロックして", "シャットダウンして", "電源切って",
  "再起動して", "休止して", "sleep the pc", "lock screen",
  "shut it down", "restart laptop", "hibernate". When the argument's intent
  is ambiguous, ask for clarification instead of guessing — destructive
  actions (shutdown/restart) must never be triggered by inference alone.
origin: personal
---

# pc-power — Windows ホスト電源操作スキル

Windows ホストの電源状態を切り替える。スリープ／休止／ロック／シャットダウン／再起動の 5 動作を、
自然言語の引数で振り分けて実行する。**WSL 経由でも Windows ネイティブでも**動く。

## 動作環境

このスキルは 2 環境で動く：

1. **WSL (Linux/zsh)** — `/mnt/c/Windows/System32/*.exe` をフルパス指定して叩く
2. **Windows native (PowerShell)** — `System32` は PATH 上なので exe 名直叩き

## 実行前の環境判定

コマンドを発射する前に **必ず環境判定**する。確実な見分け方：

| 判定方法 | 結果 → 環境 |
|----------|-------------|
| `[ -e /mnt/c/Windows ]` が成功 | **WSL** |
| `uname` が `Linux` を返す（WSL 含む） | Linux 系 → 上の `/mnt/c/Windows` 有無で WSL/native Linux 区別 |
| `$env:OS` が `Windows_NT` または `$IsWindows` が `True` | **Windows native (PowerShell)** |
| `/mnt/c/Windows` が無い & `uname` がエラー | **Windows native** |

迷ったら `uname` を実行 → 結果で分岐。

## ディスパッチ表

ユーザの引数（または直前の発話）を **自然言語として解釈** し、下表のどれかに振り分けて
**環境に応じたコマンド**を実行する。キーワードに完全一致する必要はない。意図が読めれば良い。

| 意図 | 例（参考） | WSL でのコマンド | Windows native でのコマンド | 補足 |
|------|------------|-------------------|------------------------------|------|
| **sleep** | スリープ／寝かせる／suspend／sleep | `/mnt/c/Windows/System32/rundll32.exe powrprof.dll,SetSuspendState 0,1,0` | `rundll32.exe powrprof.dll,SetSuspendState 0,1,0` | **既定**。引数無しならこれ |
| **hibernate** | 休止／hibernate／ハイバネート | `/mnt/c/Windows/System32/rundll32.exe powrprof.dll,SetSuspendState 1,1,0` | `rundll32.exe powrprof.dll,SetSuspendState 1,1,0` | |
| **lock** | ロック／画面ロック／lock screen | `/mnt/c/Windows/System32/rundll32.exe user32.dll,LockWorkStation` | `rundll32.exe user32.dll,LockWorkStation` | |
| **shutdown** | シャットダウン／電源切る／落とす／power off | `/mnt/c/Windows/System32/shutdown.exe /s /t 0` | `shutdown.exe /s /t 0` | **破壊的**: 即時シャットダウン |
| **restart** | 再起動／リスタート／reboot | `/mnt/c/Windows/System32/shutdown.exe /r /t 0` | `shutdown.exe /r /t 0` | **破壊的**: 即時再起動 |

## 解釈ルール

- **引数なし** → `sleep` を実行
- **引数あり** → 自然言語として読んで上表のいずれかに振る
- **曖昧／表に無い意図** → 黙って実行せず確認質問
- **比喩・疑問形・冗談** → 走らせない（"スリープしたら困るよね？" みたいな反語に発火しないこと）

## 破壊的アクションの追加配慮

`shutdown` と `restart` は進行中の作業を失わせる可能性がある。以下のいずれかが当てはまるなら **実行前にひと声かけて確認** すること：
- 直前まで何かを編集／実行していた形跡がある
- 引数が短く曖昧（"切って" だけ等、終了系か電源系か判別しづらい）
- 他のセッションが動いている可能性が示唆されている

## PowerShell 実行時の注意

Windows native (PowerShell) で `rundll32.exe powrprof.dll,SetSuspendState 0,1,0` を実行する際、
PowerShell がカンマを配列演算子として解釈する可能性がある。確実に動かすには引数の区切りを意識する：

```powershell
# 推奨: そのまま叩く（外部 .exe 呼び出しなのでカンマも引数の一部として渡る）
rundll32.exe powrprof.dll,SetSuspendState 0,1,0

# 万一動かない場合の代替:
Start-Process rundll32.exe -ArgumentList 'powrprof.dll,SetSuspendState 0,1,0'
```

WSL の zsh ではクォート不要で素通り。

## 重要: schtasks 経由は使わない（WSL のみの注意点）

過去の試行で、WSL から Scheduled Task 経由のスリープは **動作しなかった**。schtasks が走らせるプロセスは
Session 0 寄りで対話デスクトップから分離されるため、`SetSuspendState` が黙殺される（exit 0 で返るが実際は何も起きない）。

5 種すべて **admin 不要・直叩きで通る** ので、schtasks は使わない。Windows native では当然この問題は無関係。

## 補足: admin 必須コマンドのテンプレ（このスキル外）

このスキルの 5 動作は admin 不要だが、別件で `powercfg /hibernate off` 等を WSL から UAC 無しで叩きたい場合は
`/rl HIGHEST` 付きで Scheduled Task を登録して `schtasks /run` で起動する手段がある。詳細は本スキルの管轄外。

## 関連メモ

- **ウェイク方法**: スリープ／休止からは蓋開け／電源ボタン／Wake-on-LAN
- **長時間離席**: スリープより休止やシャットダウンのほうが省電力
- **許可リスト** (`~/.claude/settings.json` の `permissions.allow`)：環境ごとに別エントリが必要
  ```jsonc
  // WSL 用
  "Bash(/mnt/c/Windows/System32/rundll32.exe powrprof.dll,SetSuspendState 0,1,0)"

  // Windows native 用（PowerShell）
  "PowerShell(rundll32.exe powrprof.dll,SetSuspendState 0,1,0)"
  ```
- 誤爆防止のため shutdown / restart は許可リストに入れず都度確認とするのが安全
