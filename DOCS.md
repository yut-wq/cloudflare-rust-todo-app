# ドキュメントインデックス

このプロジェクトのドキュメントは以下のファイルに分かれています：

## 📖 メインドキュメント

- **[README.md](./README.md)**  
  プロジェクト概要、セットアップ手順、基本的な使い方

- **[ENVIRONMENT_SETUP.md](./ENVIRONMENT_SETUP.md)**  
  環境変数設定ガイド（API URLの設定方法など）

- **[openapi.yaml](./openapi.yaml)**  
  完全なAPI仕様（OpenAPI 3.0.3準拠）

- **[DEVELOPMENT.md](./DEVELOPMENT.md)**  
  開発者向けの詳細ガイド、コーディング規約、デプロイ手順

## 🚀 クイックスタート

新しい開発者の方は以下の順序で読むことをお勧めします：

1. [README.md](./README.md) - プロジェクト概要とセットアップ
2. [ENVIRONMENT_SETUP.md](./ENVIRONMENT_SETUP.md) - 環境変数の設定
3. [DEVELOPMENT.md](./DEVELOPMENT.md) - 開発環境の構築
4. [openapi.yaml](./openapi.yaml) - API仕様の確認

## 📋 API仕様の確認方法

### オンラインビューア

OpenAPI仕様書は以下のツールで視覚的に確認できます：

- **Swagger Editor**: https://editor.swagger.io/
  - [openapi.yaml](./openapi.yaml) の内容をコピー&ペーストして表示
- **Redoc**: https://redocly.github.io/redoc/
  - 読みやすいドキュメント形式で表示

### ローカルでの確認

```bash
# Swagger UIをローカルで起動
npx @apidevtools/swagger-parser validate openapi.yaml

# または、VS Codeの拡張機能を使用
# "OpenAPI (Swagger) Editor" 拡張機能をインストール
```

## 🔧 開発に関する質問

- **環境変数設定**: [ENVIRONMENT_SETUP.md](./ENVIRONMENT_SETUP.md)
- **セットアップ**: [DEVELOPMENT.md#開発環境のセットアップ](./DEVELOPMENT.md#開発環境のセットアップ)
- **API使用方法**: [README.md#api仕様](./README.md#api仕様)
- **デプロイ手順**: [DEVELOPMENT.md#デプロイメント](./DEVELOPMENT.md#デプロイメント)
- **トラブルシューティング**: [README.md#トラブルシューティング](./README.md#トラブルシューティング)

## 📝 ドキュメントの更新

ドキュメントの改善提案や誤りの報告は、Issues または Pull Request でお知らせください。
