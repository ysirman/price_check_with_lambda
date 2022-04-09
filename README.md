# 必要なもの
- Docker
- SendGridの APIキー

# 準備
## .env.sample を編集して .env にする
```shell
# スクレイピング先
TARGET_URL="https:// your target url"

# メール設定
SENDGRID_API_KEY="your api key"
EMAIL_SEND_TO="to@example.com"
EMAIL_SEND_FROM="from@example.com"

# 割引率がEXPECT_RATE以下の場合にメール通知する
EXPECT_RATE=90
```

## util#get_price_and_rates をスクレイピング対象のHTML構造にあわせて修正
```rust
// 要修正箇所
let wrapper_selector = ...
let price_and_rate_select = ...
```

# 実行方法（ローカル環境）
## 準備（ビルド）
```shell
# コンテナに入る
docker-compose up && docker-compose exec builder bash

# AWS Lambda 用にビルド
cargo build --release --target=x86_64-unknown-linux-musl
cp ./target/x86_64-unknown-linux-musl/release/app ./bootstrap # app は Cargo.toml の name = "app" の部分
zip lambda.zip bootstrap
```

## 実行
```shell
# bootstrap があるディレクトリでコマンド実行
docker run -it --rm -v $(pwd):/var/task:ro,delegated -e DOCKER_LAMBDA_USE_STDIN=1 -e AWS_LAMBDA_FUNCTION_MEMORY_SIZE=128 -e RUST_LOG=info lambci/lambda:provided
# コマンド実行後、引数の待ち状態になるので {} を入力して Ctrl+D を押下 (本ツールは引数を想定していないので {} で OK)

# 実行結果例
START RequestId: 3387c069-4e4e-12d9-86aa-182c38367bf4 Version: $LATEST
END RequestId: 3387c069-4e4e-12d9-86aa-182c38367bf4
REPORT RequestId: 3387c069-4e4e-12d9-86aa-182c38367bf4	Init Duration: 4524.22 ms	Duration: 3.58 ms	Billed Duration: 4 ms	Memory Size: 128 MB	Max Memory Used: 10 MB

{"message":"success"}
```
