# minigrep

## 学習リソース

[12 章](https://doc.rust-jp.rs/book-ja/ch12-00-an-io-project.html)

## プロジェクトの作成方法

### 前提

- Docker Desktop のインストール
  - Version: 20.10.7
  - Backend:
    - OS: Microsoft Windows 10 Pro
    - WSL2: Ubuntu 20.04.2 LTS

### 親ディレクトリにてプロジェクト(minigrep)を新規作成

WSL2 にて以下のコマンドを実行

```bash
docker container run -v "$(pwd)":/tmp -w /tmp rust:latest cargo new minigrep
```

### VSCode でプロジェクトを開く

ローカルの Git Bash で以下を実行

```bash
code minigrep
```

### Git リポジトリの作成

ローカルの Git Bash で以下を実行

```bash
git init
```

### Remote-Container を起動する

VSCode で `Remote-Containers: Reopen in Container` を実行し、`Rust`を選択する
-> development container が作成される

### Remote-Container 内で開発

[入出力プロジェクト: コマンドラインプログラムを構築する](https://doc.rust-jp.rs/book-ja/ch12-00-an-io-project.html)
