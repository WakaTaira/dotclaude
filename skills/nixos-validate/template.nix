# 仮説: @HYPOTHESIS@
# 対象: @TARGET@（@SLUG@）
#
# 防御仮説の検証用 nixosTest である（nixos-validate スキルの雛形から生成）。
# 規約:
#   - 試験は「防御が有効な状態」を表明する。弱点が成立する場合に失敗させる。
#   - 判定は必ず verdict() で記録する。verdict() に達しない失敗は「判定不能」として扱われる。
#   - 対策の差分を当てた後も、同じ試験を変更せずに再実行して合否を確かめる。
#
# 実行: run.sh run <本ファイル> [--flake path:<複製先>] [--host <名前>]
{
  # 評価対象の nix-config。差分検証時は複製先を path: 参照で渡す。
  flakeRef ? "git+file://@NIX_CONFIG_DIR@",
  # 設定を参照するホスト（nixosConfigurations の属性名）。
  host ? "@HOST@",
}:
let
  flake = builtins.getFlake flakeRef;
  hostSystem = flake.nixosConfigurations.${host};
  # 実ホストの評価済み設定。VM に載せられない値（生成済み PAM 設定など）の参照に用いる。
  #   例: hostConfig.security.pam.services.greetd.text
  hostConfig = hostSystem.config;
  pkgs = hostSystem.pkgs;
in
pkgs.testers.runNixOSTest {
  name = "defense-@TARGET@-@SLUG@";

  # 検証に必要な最小構成のみを記述する。実ホストの設定をそのまま import すると
  # LUKS・実デバイス依存で起動しないため、仮説に関わる設定だけを写す。
  # 写した値が実ホストと一致することは hostConfig との比較で担保する。
  nodes.machine =
    { ... }:
    {
      # 例（TPM を要する検証）: virtualisation.tpm.enable = true;
    };

  testScript = ''
    def verdict(defended: bool, evidence: str) -> None:
        """判定を定型行として記録し、弱点が成立する場合は試験を失敗させる。"""
        state = "DEFENDED" if defended else "VULNERABLE"
        print(f"DEFENSE-VERDICT: {state} :: {evidence}")
        assert defended, evidence


    machine.wait_for_unit("multi-user.target")

    # ここに仮説の検証手順を記述し、最後に verdict(防御が有効か, 根拠) を呼ぶ。
    raise Exception("雛形のままである。検証手順を記述すること")
  '';
}
