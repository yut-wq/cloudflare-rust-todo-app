# 開発者ガイド

このドキュメントでは、TODO Appの開発に関する詳細な情報を提供します。

## 目次

- [開発環境のセットアップ](#開発環境のセットアップ)
- [プロジェクト構造](#プロジェクト構造)
- [コーディング規約](#コーディング規約)
- [デバッグとログ](#デバッグとログ)
- [パフォーマンス](#パフォーマンス)
- [セキュリティ](#セキュリティ)
- [デプロイメント](#デプロイメント)

## 開発環境のセットアップ

### 必要な事前準備

1. **Node.js** (v18以上)
2. **pnpm** (v8以上)
3. **Rust** (v1.70以上)
4. **wrangler CLI**

### 初期セットアップ

```bash
# Cloudflareアカウントの設定
wrangler login

# 依存関係のインストール
cd front && pnpm install

# Rustツールチェインの確認
rustc --version
cargo --version
```

### Git フックのインストール

[prek](https://github.com/j178/prek) を使って `git push` 前にリンターとフォーマッターを自動チェックします。

**prek のインストール:**

```bash
# Windows (winget)
winget install --id j178.Prek

# Windows (scoop)
scoop install main/prek

# macOS
brew install prek

# その他: https://prek.j178.dev/installation/
```

**Git フックの有効化:**

```bash
# リポジトリルートで実行
prek install
```

これで `git push` 時に以下のチェックが自動実行されます：

| フック | 対象 | 内容 |
|--------|------|------|
| `frontend-lint` | `front/` 以下の変更時 | Prettier フォーマットチェック + ESLint |
| `rust-fmt` | `my-rust-worker/` 以下の `.rs` ファイル変更時 | `cargo fmt --check` |
| `rust-clippy` | `my-rust-worker/` 以下の `.rs` ファイル変更時 | `cargo clippy -D warnings` |

**手動実行（任意のタイミングでチェックしたい場合）:**

```bash
# 全ファイルに対してチェック
prek run --all-files --stage pre-push

# 特定のフックのみ実行
prek run frontend-lint --stage pre-push
```

### 開発サーバーの起動

2つのターミナルが必要です：

**ターミナル1: Rust Worker**

```bash
cd my-rust-worker
wrangler dev --port 8787
```

**ターミナル2: SvelteKit Frontend**

```bash
cd front
pnpm dev --port 5173
```

## プロジェクト構造

```
rust-todo-app/
├── README.md                  # プロジェクト概要
├── DEVELOPMENT.md            # 開発者ガイド（このファイル）
├── openapi.yaml              # API仕様書
├── .gitignore               # Git除外設定
│
├── front/                   # SvelteKitフロントエンド
│   ├── src/
│   │   ├── lib/
│   │   │   ├── api.ts       # API クライアント
│   │   │   ├── index.ts     # ライブラリエクスポート
│   │   │   └── pwa.ts       # PWA設定
│   │   ├── routes/
│   │   │   ├── +layout.svelte # レイアウト
│   │   │   └── +page.svelte   # メインページ
│   │   └── app.html         # HTMLテンプレート
│   ├── static/              # 静的ファイル
│   ├── e2e/                 # E2Eテスト
│   ├── package.json         # 依存関係
│   └── vite.config.ts       # Viteビルド設定
│
└── my-rust-worker/          # Rust Worker API
    ├── src/
    │   └── lib.rs           # メインAPI実装
    ├── Cargo.toml           # Rust依存関係
    └── wrangler.toml        # Cloudflare Workers設定
```

## コーディング規約

### TypeScript/Svelte（フロントエンド）

- **ESLint + Prettier** を使用
- **関数名**: camelCase
- **コンポーネント**: PascalCase
- **定数**: UPPER_SNAKE_CASE

```typescript
// ✅ 良い例
const apiClient = new ApiClient();
const MAX_RETRY_COUNT = 3;

interface TodoItem {
  id: number;
  text: string;
  completed: boolean;
}
```

### Rust（バックエンド）

- **cargo fmt** でフォーマット
- **構造体**: PascalCase
- **関数・変数**: snake_case
- **定数**: UPPER_SNAKE_CASE

```rust
// ✅ 良い例
#[derive(Debug, Serialize, Deserialize)]
pub struct Todo {
    pub id: u64,
    pub text: String,
    pub completed: bool,
}

async fn get_todos() -> Result<Json<Vec<Todo>>, StatusCode> {
    // 実装
}
```

### Git Commit メッセージ

Conventional Commits 規約を採用：

```
<type>(<scope>): <description>

[optional body]

[optional footer(s)]
```

例：

```
feat(api): add todo filtering by status
fix(frontend): resolve CORS issue in development
docs: update API documentation
```

**Type 一覧:**

- `feat`: 新機能
- `fix`: バグ修正
- `docs`: ドキュメント変更
- `style`: フォーマット変更
- `refactor`: リファクタリング
- `test`: テスト追加
- `chore`: その他の変更

## デバッグとログ

### フロントエンド

```typescript
// 開発環境でのみログ出力
import { dev } from "$app/environment";

if (dev) {
  console.log("Debug info:", data);
}
```

### Rust Worker

```rust
// コンソールログ出力
console_log!("Debug: {}", message);

// エラー処理
match some_result {
    Ok(value) => value,
    Err(err) => {
        console_log!("Error: {}", err);
        return Err(StatusCode::INTERNAL_SERVER_ERROR);
    }
}
```

### Cloudflareでのログ確認

```bash
# リアルタイムログの確認
wrangler tail my-rust-worker

# 特定の環境のログ
wrangler tail my-rust-worker --env production
```

## パフォーマンス

### フロントエンド最適化

1. **バンドルサイズ最適化**:

   ```bash
   pnpm run build
   pnpm run preview
   ```

2. **プリロード設定**:
   ```typescript
   // 重要なAPIエンドポイントのプリロード
   export async function load({ fetch }) {
     const todos = await fetch("/api/todos");
     return {
       todos: await todos.json(),
     };
   }
   ```

### Rust Worker最適化

1. **リリースビルド使用**:

   ```bash
   wrangler deploy --env production
   ```

2. **メモリ使用量の監視**:
   ```rust
   // 必要に応じてデータ構造を最適化
   use std::collections::BTreeMap; // HashMapの代わり
   ```

## セキュリティ

### CORS設定

本番環境では適切なオリジンのみを許可：

```rust
let cors = CorsLayer::new()
    .allow_origin("https://your-app.pages.dev".parse::<HeaderValue>().unwrap())
    .allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE])
    .allow_headers([CONTENT_TYPE]);
```

### 入力検証

```rust
async fn create_todo(Json(request): Json<CreateTodoRequest>) -> Result<Json<Todo>, StatusCode> {
    // 入力検証
    let text = request.text.trim();
    if text.is_empty() || text.len() > 1000 {
        return Err(StatusCode::BAD_REQUEST);
    }

    // サニタイゼーション（必要に応じて）
    let sanitized_text = html_escape::encode_text(text);

    // TODO作成処理...
}
```

### 環境変数の管理

機密情報はCloudflare Secretsで管理：

```bash
# シークレットの設定
wrangler secret put DATABASE_URL --env production
wrangler secret put API_KEY --env production

# シークレットの一覧表示
wrangler secret list --env production
```

## デプロイメント

### 開発→ステージング→本番の流れ

1. **開発環境での確認**
2. **ステージングデプロイ**
3. **本番デプロイ**

### 自動デプロイ（CI/CD）

GitHub Actionsの例：

```yaml
name: Deploy

on:
  push:
    branches: [main]

jobs:
  deploy-worker:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      - uses: cloudflare/wrangler-action@v3
        with:
          apiToken: ${{ secrets.CLOUDFLARE_API_TOKEN }}
          workingDirectory: "my-rust-worker"

  deploy-frontend:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      - run: |
          cd front
          npm install -g pnpm
          pnpm install
          pnpm run build
      - uses: cloudflare/pages-action@v1
        with:
          apiToken: ${{ secrets.CLOUDFLARE_API_TOKEN }}
          accountId: ${{ secrets.CLOUDFLARE_ACCOUNT_ID }}
          projectName: "todo-app"
          directory: "front/build"
```

### ロールバック手順

```bash
# 特定のバージョンにロールバック
wrangler deployments list my-rust-worker
wrangler rollback my-rust-worker --deployment-id <deployment-id>

# 前のバージョンにロールバック
wrangler rollback my-rust-worker
```

## トラブルシューティング

### よくある問題と解決方法

1. **CORS エラー**

   ```bash
   # ローカル開発でのプロキシ設定確認
   # vite.config.ts の proxy 設定を確認
   ```

2. **ビルドエラー**

   ```bash
   # 依存関係のクリーンインストール
   rm -rf node_modules pnpm-lock.yaml
   pnpm install
   ```

3. **Rust Worker デプロイエラー**
   ```bash
   # Wranglerの設定確認
   wrangler whoami
   wrangler kv:namespace list
   ```

### デバッグコマンド

```bash
# フロントエンドのビルド確認
cd front && pnpm run check

# Rustのコンパイル確認
cd my-rust-worker && cargo check

# E2Eテストの実行
cd front && pnpm run test:e2e

# APIテスト
curl -X GET http://localhost:8787/api/todos
```

## 今後の拡張予定

- [ ] データベース連携（D1 Database）
- [ ] ユーザー認証機能
- [ ] リアルタイム同期（WebSocket）
- [ ] オフライン対応の強化
- [ ] 管理者画面の追加
