// このファイルは環境変数ベースの実装に変更したため、現在は使用されません。
// 将来的により複雑な設定管理が必要になった場合に再利用できます。

import { env } from '$env/dynamic/public';

// 現在は env から直接 PUBLIC_API_BASE_URL を使用します
// 詳細は src/lib/api.ts を参照してください

export { env };
