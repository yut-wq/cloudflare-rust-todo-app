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

### 2. フロントエンドの設定

デプロイしたRust WorkerのURLを`front/src/lib/api.ts`で更新してください：

```typescript
const API_BASE_URL = dev
  ? "http://localhost:8787" // 開発時のワーカーURL
  : "https://my-rust-worker.your-subdomain.workers.dev"; // ← ここを実際のURLに変更
```

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

### Rust Workerの開発

```bash
cd my-rust-worker

# 開発サーバーを起動（localhost:8787）
wrangler dev
```

### フロントエンドの開発

```bash
cd front

# 依存関係のインストール
pnpm install

# 開発サーバーを起動（localhost:5173）
pnpm run dev
```

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

## トラブルシューティング

### CORS エラーが発生する場合

Rust Worker側でCORS設定を確認してください。現在の実装では全オリジンを許可していますが、本番環境では適切に制限することをお勧めします。

### API URLの設定

開発環境と本番環境でAPIのURLが異なることを確認してください：

- 開発: `http://localhost:8787`
- 本番: `https://my-rust-worker.your-subdomain.workers.dev`

## ライセンス

MIT
