#!/usr/bin/env bash
# nixos-validate 実行補助スクリプト
#
# 防御仮説を検証する nixosTest の「検索・雛形生成・実行・判定」を担う。
# 試験ファイルは nix-config 配下の gitignore 済みディレクトリに置き、flake の外から評価する。
#
# 使い方:
#   run.sh find <対象> [語...]                    既存の試験を検索する
#   run.sh new [--host <名前>] <対象> <短縮名> <仮説>   雛形から試験を生成する
#   run.sh run [--flake <参照>] [--host <名前>] <試験ファイル>   試験を実行して判定する
#   run.sh sandbox <出力先>                       差分検証用に nix-config の複製を作る
#
# run の終了コード:
#   0 = 防御は有効（DEFENDED）  1 = 弱点が成立（VULNERABLE）  2 = 判定不能
set -euo pipefail

NIX_CONFIG_DIR="${NIX_CONFIG_DIR:-$HOME/Projects/nix-config}"
TESTS_DIR="${DEFENSE_TESTS_DIR:-$NIX_CONFIG_DIR/.defense-tests}"
SKILL_DIR="$(cd "$(dirname "$(readlink -f "${BASH_SOURCE[0]}")")" && pwd)"
TEMPLATE="$SKILL_DIR/template.nix"
VERDICT_MARK="DEFENSE-VERDICT: "
NAME_PATTERN='^[a-z0-9][a-z0-9-]*$'

# エラーメッセージを標準エラーへ出力し、判定不能の終了コードで終了する。
die() {
  echo "nixos-validate: $*" >&2
  exit 2
}

# 使い方（本ファイル冒頭のコメント）を表示する。
usage() {
  sed -n '2,15p' "${BASH_SOURCE[0]}" | sed 's/^# \{0,1\}//'
}

# 対象名・短縮名がファイル名として安全な形式であることを検査する。
require_name() {
  [[ $2 =~ $NAME_PATTERN ]] || die "$1 は英小文字・数字・ハイフンのみで指定する: $2"
}

# 試験ファイルの冒頭コメントから仮説の 1 行を取り出す。
hypothesis_of() {
  sed -n 's/^# 仮説: //p' "$1" | head -n 1
}

# 試験ファイルが、ファイル名または冒頭の仮説行に指定語をすべて含むかを判定する。
matches_all() {
  local file="$1" word haystack
  shift
  haystack="$(basename "$file") $(hypothesis_of "$file")"
  for word in "$@"; do
    grep -qiF -- "$word" <<<"$haystack" || return 1
  done
}

# 既存の試験を検索し、「パス<TAB>仮説」の形式で列挙する。
cmd_find() {
  [ $# -ge 1 ] || die "find には対象を指定する"
  local file found=1
  [ -d "$TESTS_DIR" ] || return 1
  for file in "$TESTS_DIR"/*.nix; do
    [ -f "$file" ] || continue
    matches_all "$file" "$@" || continue
    printf '%s\t%s\n' "$file" "$(hypothesis_of "$file")"
    found=0
  done
  return "$found"
}

# 雛形のプレースホルダを置換した内容を標準出力へ書き出す。
render_template() {
  local host="$1" target="$2" slug="$3" hypothesis="$4" content
  content="$(<"$TEMPLATE")"
  content="${content//@HOST@/"$host"}"
  content="${content//@TARGET@/"$target"}"
  content="${content//@SLUG@/"$slug"}"
  content="${content//@NIX_CONFIG_DIR@/"$NIX_CONFIG_DIR"}"
  content="${content//@HYPOTHESIS@/"$hypothesis"}"
  printf '%s\n' "$content"
}

# 雛形から新しい試験ファイルを生成し、そのパスを出力する。既存ファイルは上書きしない。
cmd_new() {
  local host="t14"
  if [ "${1:-}" = "--host" ]; then
    host="${2:?--host には値が必要である}"
    shift 2
  fi
  [ $# -eq 3 ] || die "new には <対象> <短縮名> <仮説> を指定する"
  local target="$1" slug="$2" hypothesis file
  require_name "対象" "$target"
  require_name "短縮名" "$slug"
  require_name "ホスト名" "$host"
  hypothesis="$(tr '\n' ' ' <<<"$3")"
  file="$TESTS_DIR/$target-$slug.nix"
  [ ! -e "$file" ] || die "既に存在する（流用すること）: $file"
  mkdir -p "$TESTS_DIR"
  render_template "$host" "$target" "$slug" "${hypothesis% }" >"$file"
  echo "$file"
}

# ログから最後の判定行（DEFENDED / VULNERABLE と根拠）を取り出す。
# 判定行が無い場合は空文字列を返す（grep の不一致で呼び出し元を終了させない）。
last_verdict() {
  { grep -aF -- "$VERDICT_MARK" "$1" || true; } | tail -n 1 | sed "s/^.*$VERDICT_MARK//"
}

# nix build で試験を実行する。出力はログへ保存しつつ表示し、build の終了コードを返す。
build_test() {
  local file="$1" log="$2" out_file="$3"
  shift 3
  set +e
  nix build --impure --no-link --print-out-paths -L -f "$file" "$@" 2>&1 >"$out_file" | tee "$log"
  local rc="${PIPESTATUS[0]}"
  set -e
  return "$rc"
}

# 成功結果が nix のキャッシュから返った場合に、保存済みのビルドログを実行ログへ補う。
append_cached_log() {
  local log="$1" out_path="$2"
  [ -n "$out_path" ] || return 0
  grep -qaF -- "$VERDICT_MARK" "$log" && return 0
  echo "nixos-validate: 結果はキャッシュ済みである。保存済みのビルドログから判定を読む" | tee -a "$log"
  nix log "$out_path" >>"$log" 2>/dev/null || true
}

# build の終了コードと判定行を突き合わせ、最終判定を表示して終了コードを返す。
report_verdict() {
  local rc="$1" log="$2" verdict
  verdict="$(last_verdict "$log")"
  echo "----"
  echo "ログ: $log"
  case "$rc:$verdict" in
    0:DEFENDED*) echo "判定: 防御は有効（$verdict）" && return 0 ;;
    0:*) echo "判定: 判定不能（試験は成功したが判定行が無い。verdict() を呼んでいない）" && return 2 ;;
    *:VULNERABLE*) echo "判定: 弱点が成立（$verdict）" && return 1 ;;
    *) echo "判定: 判定不能（判定行に達する前に失敗した。評価エラーまたは試験の不備）" && return 2 ;;
  esac
}

# 試験ファイルを実行し、判定を報告する。
cmd_run() {
  local -a nix_args=()
  while [ $# -gt 1 ]; do
    case "$1" in
      --flake) nix_args+=(--argstr flakeRef "${2:?--flake には値が必要である}") ;;
      --host) nix_args+=(--argstr host "${2:?--host には値が必要である}") ;;
      *) die "不明な引数: $1" ;;
    esac
    shift 2
  done
  local file="${1:?run には試験ファイルを指定する}" log out_file
  [ -f "$file" ] || die "試験ファイルが無い: $file"
  mkdir -p "$TESTS_DIR/logs"
  log="$TESTS_DIR/logs/$(basename "$file" .nix)-$(date +%Y%m%d-%H%M%S).log"
  out_file="$(mktemp)"
  local rc=0
  build_test "$file" "$log" "$out_file" "${nix_args[@]}" || rc=$?
  [ "$rc" -ne 0 ] || append_cached_log "$log" "$(head -n 1 "$out_file")"
  rm -f "$out_file"
  report_verdict "$rc" "$log"
}

# 差分検証用に、nix-config の作業ツリー（未コミットの変更を含む）を複製する。
# 複製先は `--flake path:<出力先>` として run に渡す。本物のツリーには書き込まない。
cmd_sandbox() {
  local dest="${1:?sandbox には出力先を指定する}"
  [ ! -e "$dest" ] || die "出力先が既に存在する: $dest"
  mkdir -p "$dest"
  git -C "$NIX_CONFIG_DIR" ls-files -z --cached --others --exclude-standard |
    (cd "$NIX_CONFIG_DIR" && xargs -0 cp --parents -t "$dest" --)
  git -C "$dest" init -q
  git -C "$dest" add -A
  echo "$dest"
}

main() {
  local cmd="${1:-}"
  [ $# -eq 0 ] || shift
  case "$cmd" in
    find) cmd_find "$@" ;;
    new) cmd_new "$@" ;;
    run) cmd_run "$@" ;;
    sandbox) cmd_sandbox "$@" ;;
    *) usage && exit 2 ;;
  esac
}

main "$@"
