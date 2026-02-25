# TODO App - SvelteKit + Cloudflare Pages + Rust Worker

Cloudflare Pages（フロントエンド）とRust Worker（API）で構築されたTODOアプリケーションです。

## 構成

- **フロントエンド**: SvelteKit + TypeScript
- **API**: Rust + Axum + Cloudflare Workers
- **ホスティング**: Cloudflare Pages + Cloudflare Workers

## セットアップ

### 前提条件

- Node.js (18以上)
- pnpm
- Rust (1.70以上)
- wrangler CLI

```bash
# wrangler CLIのインストール
npm install -g wrangler

# Cloudflareにログイン
wrangler login
```

### 1. Rust Workerのデプロイ

```bash
cd my-rust-worker

# ビルドとデプロイ
wrangler deploy

# デプロイされたURLを確認
# 例: https://my-rust-worker.your-subdomain.workers.dev
```

### 2. 環境変数の設定

環境変数でAPI URLを設定します。環境設定ファイルを作成してください：

```bash
cd front

# 環境設定ファイルを作成
cp .env.example .env.local

# .env.local を編集してAPI URLを設定
# 例：
# PUBLIC_API_BASE_URL=https://my-rust-worker.your-subdomain.workers.dev
```

📋 **詳細な環境変数設定については [ENVIRONMENT_SETUP.md](./ENVIRONMENT_SETUP.md) を参照してください。**

### 3. フロントエンドのビルドとデプロイ

```bash
cd front

# 依存関係のインストール
pnpm install

# ビルド
pnpm run build

# Cloudflare Pagesにデプロイ
```

Cloudflare Pagesでのデプロイ設定：

- **ビルドコマンド**: `pnpm run build`
- **ビルド出力フォルダ**: `build`
- **ルートディレクトリ**: `front`

## 開発環境

### 1. 環境変数の設定

最初に環境変数を設定してください：

```bash
cd front

# 開発用環境設定ファイルを作成
cp .env.example .env.local

# 設定例（開発環境）
echo "PUBLIC_API_BASE_URL=http://localhost:8787" > .env.local
```

### 2. Rust Workerの開発

```bash
cd my-rust-worker

# 開発サーバーを起動（localhost:8787）
wrangler dev
```

### 3. フロントエンドの開発

```bash
cd front

# 依存関係のインストール
pnpm install

# 開発サーバーを起動（localhost:5173）
pnpm run dev
```

💡 **環境変数の詳細設定や本番環境での使い方については [ENVIRONMENT_SETUP.md](./ENVIRONMENT_SETUP.md) を参照してください。**

## API エンドポイント

| メソッド | エンドポイント   | 説明                         |
| -------- | ---------------- | ---------------------------- |
| GET      | `/api/todos`     | 全TODO取得                   |
| POST     | `/api/todos`     | TODO作成                     |
| PUT      | `/api/todos/:id` | TODO更新（完了状態切り替え） |
| DELETE   | `/api/todos/:id` | TODO削除                     |

### リクエスト例

```bash
# TODO作成
curl -X POST https://your-worker-url.workers.dev/api/todos \
  -H "Content-Type: application/json" \
  -d '{"text": "新しいTODO"}'

# TODO一覧取得
curl https://your-worker-url.workers.dev/api/todos

# TODO更新
curl -X PUT https://your-worker-url.workers.dev/api/todos/1 \
  -H "Content-Type: application/json" \
  -d '{"completed": true}'

# TODO削除
curl -X DELETE https://your-worker-url.workers.dev/api/todos/1
```

## ファイル構成

```
├── front/                      # SvelteKitフロントエンド
│   ├── src/
│   │   ├── lib/
│   │   │   └── api.ts         # API クライアント
│   │   └── routes/
│   │       └── +page.svelte   # メインページ
│   ├── package.json
│   └── wrangler.jsonc         # Cloudflare Pages設定
└── my-rust-worker/            # Rust Worker API
    ├── src/
    │   └── lib.rs             # メインロジック
    ├── Cargo.toml
    └── wrangler.toml          # Cloudflare Workers設定
```

## 機能

- ✅ TODO作成・編集・削除
- ✅ 完了状態の切り替え
- ✅ 完了済みTODOの一括削除
- ✅ レスポンシブデザイン
- ✅ PWA対応
- ✅ エラーハンドリング
- ✅ ローディング状態

## API仕様

本アプリケーションのAPI仕様は OpenAPI 3.0.3 形式で記述されています。

### 📋 API仕様書

詳細なAPI仕様は [openapi.yaml](./openapi.yaml) を参照してください。

### 🔗 主要エンドポイント

| メソッド | パス              | 説明         |
| -------- | ----------------- | ------------ |
| `GET`    | `/api/todos`      | TODO一覧取得 |
| `POST`   | `/api/todos`      | TODO作成     |
| `PUT`    | `/api/todos/{id}` | TODO更新     |
| `DELETE` | `/api/todos/{id}` | TODO削除     |

### 📊 データモデル

#### Todo

```typescript
interface Todo {
  id: number; // Unix時刻（ミリ秒）
  text: string; // TODOテキスト
  completed: boolean; // 完了状態
}
```

#### API リクエスト

```typescript
// TODO作成
interface CreateTodoRequest {
  text: string; // 必須、空文字不可
}

// TODO更新
interface UpdateTodoRequest {
  completed: boolean; // 完了状態
}
```

### 🌐 API使用例

```bash
# TODO一覧取得
curl -X GET https://my-rust-worker.your-subdomain.workers.dev/api/todos

# TODO作成
curl -X POST https://my-rust-worker.your-subdomain.workers.dev/api/todos \
  -H "Content-Type: application/json" \
  -d '{"text": "新しいタスク"}'

# TODO更新（完了）
curl -X PUT https://my-rust-worker.your-subdomain.workers.dev/api/todos/1708781445000 \
  -H "Content-Type: application/json" \
  -d '{"completed": true}'

# TODO削除
curl -X DELETE https://my-rust-worker.your-subdomain.workers.dev/api/todos/1708781445000
```

## 技術仕様

### アーキテクチャ

```
┌─────────────────┐    ┌─────────────────┐
│   SvelteKit     │    │   Rust Worker   │
│   Frontend      │◄──►│   (Axum API)    │
│ (Cloudflare     │    │ (Cloudflare     │
│  Pages)         │    │  Workers)       │
└─────────────────┘    └─────────────────┘
```

### フロントエンド技術

- **フレームワーク**: SvelteKit 5.x
- **言語**: TypeScript
- **スタイル**: CSS（独自スタイル）
- **ビルドツール**: Vite
- **パッケージマネージャー**: pnpm
- **ホスティング**: Cloudflare Pages
- **PWA**: Service Worker対応

### バックエンド技術

- **言語**: Rust 1.70+
- **フレームワーク**: Axum（WebAssembly対応）
- **ランタイム**: Cloudflare Workers
- **ビルドツール**: worker-build
- **CORS**: tower-http
- **データ保存**: インメモリ（HashMap）

### 開発環境

- **Node.js**: 18.x 以上
- **Rust**: 1.70.x 以上
- **Wrangler CLI**: 3.x 以上
- **pnpm**: 8.x 以上

## 開発ガイド

### ローカル開発の開始

```bash
# リポジトリのクローン
git clone <repository-url>
cd rust-todo-app

# 依存関係のインストール
cd front
pnpm install

# Rust Worker の起動（新しいターミナル）
cd ../my-rust-worker
wrangler dev

# フロントエンドの起動（別のターミナル）
cd ../front
pnpm run dev
```

### ビルドとデプロイ

```bash
# Rust Worker のデプロイ
cd my-rust-worker
wrangler deploy

# フロントエンドのビルドとデプロイ
cd ../front
pnpm run build
pnpm run deploy
```

### テスト実行

```bash
# フロントエンドのテスト
cd front
pnpm run test        # 単体テスト
pnpm run test:e2e    # E2Eテスト
```

## トラブルシューティング

### CORS エラーが発生する場合

Rust Worker側でCORS設定を確認してください。現在の実装では全オリジンを許可していますが、本番環境では適切に制限することをお勧めします。

### API URLの設定

開発環境と本番環境でAPIのURLが異なることを確認してください：

- 開発: `http://localhost:8787`
- 本番: `https://my-rust-worker.your-subdomain.workers.dev`

## ライセンス

MIT
