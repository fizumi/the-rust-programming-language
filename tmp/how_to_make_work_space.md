# ワークスペースの作成方法

## 前提

- Docker Desktop のインストール
  - Version: 20.10.7
  - Backend:
    - OS: Microsoft Windows 10 Pro
    - WSL2: Ubuntu 20.04.2 LTS
- VSCode の最新版のインストール
  - エクステンション
    - [Remote - Containers](https://marketplace.visualstudio.com/items?itemName=ms-vscode-remote.remote-containers)
    - [Remote - WSL](https://marketplace.visualstudio.com/items?itemName=ms-vscode-remote.remote-wsl)

## コンテナを起動する

VSCode で プロジェクトのディレクトリを開く

`Remote-Container: Reopen in Container` を実行する  
`Rsut` テンプレートを選択する

## コンテナ内の操作

コンテナ内で以下のコマンドを実行する

```bash
cargo new 【パッケージ名】
```

プロジェクト直下に `Cargo.toml` ファイルを作成する。
下記の[member]部分を【パッケージ名】に変更して保存する。

```toml
[workspace]

members = [
    "[member]",
]
```

（通常、`Cargo.toml` の保存で Rust server は Restart されるようであるが、変更が反映されない場合、VSCode の`Rust: Restart The Rust server`を実行すると変更が反映される）

`main.rs` がある場合、以下のコマンドでパッケージを実行する。

```bash
cargo run -p 【パッケージ名】
```
