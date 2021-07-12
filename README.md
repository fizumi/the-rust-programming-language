# The Book 7 章

## プロジェクトの作成方法

### 前提

- Docker Desktop のインストール
  - Version: 20.10.7
  - Backend:
    - OS: Microsoft Windows 10 Pro
    - WSL2: Ubuntu 20.04.2 LTS

### 親ディレクトリにてプロジェクト(restaurant)を新規作成

WSL2 にて以下のコマンドを実行

```bash
docker container run -v "$(pwd)":/tmp -w /tmp rust:latest cargo new --lib restaurant
```

### VSCode でプロジェクトを開く

ローカルの Git Bash で以下を実行

```bash
code restaurant
```

### Git リポジトリの作成

ローカルの Git Bash で以下を実行

```bash
git init
```

### Remote-Container を起動する

VSCode で `Reopen in Container` を実行し、Rust を選択する
-> development container が作成される
