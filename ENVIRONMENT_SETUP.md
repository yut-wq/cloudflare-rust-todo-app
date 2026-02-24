# 環境変数設定ガイド

このプロジェクトでは、API URLを動的に設定するために環境変数を使用します。

## 📋 環境変数一覧

| 変数名                | 必須 | デフォルト値 | 説明                       |
| --------------------- | ---- | ------------ | -------------------------- |
| `PUBLIC_API_BASE_URL` | ❌   | 自動判定     | Rust Worker APIのベースURL |

## 🚀 設定方法

### 1. 環境設定ファイルの作成

```bash
# .env.example をコピーして環境設定ファイルを作成
cp .env.example .env.local
```

### 2. 環境変数の編集

`.env.local` ファイルを編集してAPIのURLを設定：

```bash
# 開発環境（ローカル）
PUBLIC_API_BASE_URL=http://localhost:8787

# 本番環境
PUBLIC_API_BASE_URL=https://my-rust-worker.your-subdomain.workers.dev

# ステージング環境
PUBLIC_API_BASE_URL=https://staging.my-rust-worker.your-subdomain.workers.dev
```

## 🌍 環境別設定例

### 開発環境

```bash
# .env.local
PUBLIC_API_BASE_URL=http://localhost:8787
```

```bash
# 開発サーバー起動
pnpm dev
```

### 本番環境

**セキュリティ重要**: 本番URLは公開リポジトリにコミットしないでください

```bash
# 方法1: 環境変数ファイルを使用（推奨）
cp .env.production.example .env.production
# .env.production を編集して実際のURLに変更

# 方法2: 環境変数を直接設定
export PUBLIC_API_BASE_URL=https://your-actual-worker-url.workers.dev

# 環境変数が設定されていることを確認してビルド
pnpm run build:check

# または通常のビルド（環境変数は外部で設定済み想定）
pnpm run build:production
```

**重要**: `.env.production` は `.gitignore` に含まれており、Gitにコミットされません。

### 複数環境の管理

異なる環境用に複数の設定ファイルを作成：

```bash
.env.local           # ローカル開発用
.env.development     # 開発環境用
.env.staging         # ステージング環境用
.env.production      # 本番環境用
```

## 🔧 動的な設定方法

### 1. 環境変数なしの場合

環境変数が設定されていない場合、自動的に適切なデフォルト値を使用：

- **開発環境**: `http://localhost:8787`
- **本番環境**: `https://my-rust-worker.your-subdomain.workers.dev`

### 2. 実行時設定の確認

開発環境では、コンソールで現在の設定を確認できます：

```javascript
// ブラウザのコンソールに表示される
console.log("API Base URL:", "http://localhost:8787");
console.log("Environment variables:", {
  PUBLIC_API_BASE_URL: "http://localhost:8787",
  dev: true,
});
```

## 🚢 デプロイ時の設定

### 手動デプロイ

```bash
# 1. 実際のWorker URLを設定
export PUBLIC_API_BASE_URL=https://your-actual-worker.workers.dev

# 2. 設定確認付きビルド
cd front
pnpm run build:check

# 3. デプロイ
pnpm run deploy
```

### CI/CDでの設定

### Cloudflare Pages

Cloudflare Pagesのダッシュボードで環境変数を設定：

1. Pagesプロジェクトの設定を開く
2. 「Environment variables」セクション → 「Add variable」
3. 変数名: `PUBLIC_API_BASE_URL`
4. 値: `https://your-actual-worker.workers.dev`
5. Environment: `Production` を選択
6. 「Save」をクリック
7. 再デプロイ実行

### GitHub Actions

```yaml
name: Deploy to Cloudflare Pages

on:
  push:
    branches: [main]

jobs:
  deploy:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3

      - name: Setup Node.js
        uses: actions/setup-node@v3
        with:
          node-version: "18"

      - name: Install pnpm
        uses: pnpm/action-setup@v2
        with:
          version: 8

      - name: Install dependencies
        run: |
          cd front
          pnpm install

      - name: Build with environment variables
        run: |
          cd front
          PUBLIC_API_BASE_URL=${{ secrets.PUBLIC_API_BASE_URL }} pnpm run build:check
        env:
          PUBLIC_API_BASE_URL: ${{ secrets.PUBLIC_API_BASE_URL }}

      - name: Deploy to Cloudflare Pages
        uses: cloudflare/pages-action@v1
        with:
          apiToken: ${{ secrets.CLOUDFLARE_API_TOKEN }}
          accountId: ${{ secrets.CLOUDFLARE_ACCOUNT_ID }}
          projectName: "your-project-name"
          directory: "front/build"
```

**GitHub Secrets 設定**:

1. リポジトリ設定 → Secrets and variables → Actions
2. 以下のシークレットを追加:
   - `PUBLIC_API_BASE_URL`: 実際のWorker URL
   - `CLOUDFLARE_API_TOKEN`: Cloudflare API トークン
   - `CLOUDFLARE_ACCOUNT_ID`: Cloudflare アカウント ID

### Vercel

```bash
# Vercelプロジェクトで環境変数を設定
vercel env add PUBLIC_API_BASE_URL production
# https://my-rust-worker.your-subdomain.workers.dev
```

### Netlify

Netlifyの管理画面で設定：

1. Site settings → Environment variables
2. `PUBLIC_API_BASE_URL` を追加
3. 値を設定して保存

## 🛠️ トラブルシューティング

### CORS エラーが発生する場合

APIのCORS設定で、フロントエンドのドメインが許可されていることを確認：

```rust
// my-rust-worker/src/lib.rs
let cors = CorsLayer::new()
    .allow_origin("https://your-frontend-domain.com".parse::<HeaderValue>().unwrap())
    .allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE])
    .allow_headers([CONTENT_TYPE]);
```

### 環境変数が反映されない場合

1. `.env.local` が正しい場所にあるか確認
2. 開発サーバーを再起動
3. 変数名が `PUBLIC_` で始まっているか確認
4. ブラウザのキャッシュをクリア

### API URL の確認方法

開発環境でブラウザのコンソールを確認：

```bash
# 正常な場合の表示例
API Base URL: http://localhost:8787
API Request: http://localhost:8787/api/todos GET 200
```

## 💡 ベストプラクティス

1. **機密情報は含めない**: API URLは公開情報として扱われます
2. **環境ごとに設定**: 開発、ステージング、本番で適切なURLを設定
3. **バリデーション**: 無効なURLが設定された場合のエラーハンドリング
4. **ドキュメント化**: チームで共有できるよう設定方法を文書化
