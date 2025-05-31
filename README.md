# rs-csv-validator

JSONスキーマを使用してCSVファイルを検証するRustコマンドラインツールです。

## 概要

rs-csv-validatorは、CSVファイルの各行がJSONスキーマの仕様に準拠しているかを検証するツールです。データの整合性確保やバリデーション処理の自動化に役立ちます。

## ✨ 特徴

- **🔍 柔軟な型推論**: 文字列、数値（整数・浮動小数点）、真偽値、null値を自動識別
- **📋 JSONスキーマ対応**: 業界標準のJSONスキーマ形式での検証ルール定義
- **📊 詳細なエラー報告**: 行番号、列名、エラー内容を含む詳細なエラー情報
- **⚡ 高性能**: Rustによる高速な処理

## 使用方法

### 基本的な使い方

```bash
rs-csv-validator --csv <CSVファイルパス> --schema <JSONスキーマファイルパス>
```

### オプション

- `-c, --csv <FILE>`: 検証対象のCSVファイルパス（必須）
- `-s, --schema <FILE>`: 検証に使用するJSONスキーマファイルパス（必須）
- `-h, --help`: ヘルプメッセージを表示
- `-V, --version`: バージョン情報を表示

### 使用例

```bash
# 基本的な検証
rs-csv-validator --csv data.csv --schema schema.json

# 短縮形オプション使用
rs-csv-validator -c data.csv -s schema.json
```

## スキーマファイルの例

```json
{
  "$schema": "http://json-schema.org/draft-07/schema#",
  "type": "object",
  "properties": {
    "name": {
      "type": "string",
      "minLength": 1
    },
    "age": {
      "type": "integer",
      "minimum": 0,
      "maximum": 150
    },
    "email": {
      "type": "string",
      "format": "email"
    },
    "active": {
      "type": "boolean"
    }
  },
  "required": ["name", "age", "email"]
}
```

## CSVファイルの例

```csv
name,age,email,active
田中太郎,25,tanaka@example.com,true
佐藤花子,30,sato@example.com,false
山田三郎,35,yamada@example.com,true
```

## エラー出力形式

検証エラーが発生した場合、以下のようなJSON形式でエラー情報が標準エラー出力に表示されます：

```json
[
  {
    "line": 2,
    "column": "age",
    "error": "-5 is less than the minimum of 0"
  },
  {
    "line": 3,
    "column": "email",
    "error": "\"invalid-email\" is not a \"email\""
  }
]
```

各エラーオブジェクトには以下の情報が含まれます：

- `line`: エラーが発生した行番号（ヘッダー行を除く）
- `column`: エラーが発生した列名
- `error`: エラーの詳細説明

## データ型の自動変換

CSVの文字列値は以下の順序で型推論されます：

1. **整数**: `123`, `-456` など
2. **浮動小数点数**: `3.14`, `-2.5` など
3. **真偽値**: `true`, `false`（大文字小文字を区別しない）
4. **null値**: 空文字列
5. **文字列**: 上記以外のすべての値

## 依存関係

- `anyhow`: エラーハンドリング
- `clap`: コマンドライン引数解析
- `csv`: CSV読み込み処理
- `jsonschema`: JSONスキーマ検証
- `serde`: シリアライゼーション
- `serde_json`: JSON処理