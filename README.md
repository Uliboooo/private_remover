# private_remover

**the README was written by Gemini**

URL Cleanerは、URLから不要なパラメータを削除し、共有に適した短いURLを生成するシンプルなコマンドラインツールです。特に、YouTubeのURLを短縮し、クリップボードにコピーする機能に特化しています。

## 主な機能

*   **YouTube URLの短縮:** `https://www.youtube.com/watch?v=VIDEO_ID` 形式のURLを `https://youtu.be/VIDEO_ID` 形式に変換します。
*   **汎用的なURLクリーニング:** URLからクエリパラメータを削除し、ベースURLのみを抽出します。
*   **クリップボードへのコピー:** 処理後のURLを自動的にクリップボードにコピーします。
*   **シンプルなコマンドラインインターフェース:** 簡単に使用できます。

## 使用技術

*   **Rust:** 高速で安全なシステムプログラミング言語。
*   **Regex:** 正規表現エンジン。
*   **copypasta:** クリップボード操作を行うためのクレート。

## 実行方法

コマンドラインから `url_cleaner` コマンドを実行し、URLを引数として渡します。

```bash
prr "https://www.youtube.com/watch?v=dQw4w9WgXcQ&ab_channel=RickAstley"
```

上記のコマンドを実行すると、ターミナルに以下のメッセージが表示され、クリップボードに短縮されたURLがコピーされます。

```
pasted to clipboard: https://youtu.be/dQw4w9WgXcQ
```

汎用的なURLクリーニングの例：

```bash
url_cleaner "https://example.com/path/to/page?param1=value1&param2=value2"
```

結果：

```
pasted to clipboard: https://example.com/path/to/page
```

## 使用例

1.  **YouTubeの動画を共有する:** YouTubeのURLをコマンドに渡し、短縮されたURLを友達に送ります。
2.  **長いURLを短くする:** ウェブサイトのURLからトラッキングパラメータを削除し、共有しやすくします。
3.  **スクリプトで利用する:** URLをプログラムで処理し、他のアプリケーションに渡します。

## その他の指示

*   引数としてURLを必ず渡してください。
*   URLが認識されない場合、エラーメッセージが表示されます。

```
Error: NotFoundURL
```

GitHubリポジトリ: https://github.com/Uliboooo/private_remover

