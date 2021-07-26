# 数当てゲーム

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

## 使い方

### コンテナを起動する

VSCode で `guessing_game` を開き、 `Remote-Container: Reopen in Container` を実行

### コンテナ内で以下を実行する

```bash
cargo run
```
