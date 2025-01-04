# the_vsml_converter

## このリポジトリは？
VSMLファイルを受け取り、動画へと変換するVSMLコンバータのリポジトリ。

Python版の旧コンバータは[こちら](https://github.com/vsml-org/vsml_converter_old)

## VSMLとは？
HTMLのような記法で動画を生成できる言語。

詳細は[VSML公式サイト](https://vsml.pigeons.house/)へ。

## 各Crateについて
### vsml_cli
コンバータの実行ファイル

`vsml_parser` から `vsml_encoder` まで一連の処理の呼び出しを行っている

### vsml_parser
VSMLやVSSの字句解析を行うライブラリ

VSMLの文字列から `vsml_ast` で定義した構造体のデータを生成する

### vsml_iv_converter
VSMLの時間、サイズ、位置などのデータ計算処理を行いIVData(Intermediate VSML Data)へ変換するライブラリ

`vsml_ast` の構造体を受け取りIVDataを生成する

### vsml_core
IVDataから画像1枚を生成する処理を定義したdomainライブラリ

IVDataの定義もここにある

### vsml_encoder
IVDataから `vsml_core` を利用して動画へと変換するライブラリ

IVDataと出力先パスを受け取り、そのパスへ動画を出力する

### vsml_ast
VSMLの字句解析を行うための構造体定義ライブラリ

`vsml_parser` や `vsml_iv_converter` で使用

### vsml_common_image
VSMLの画像生成の具体的な構造体を定義するライブラリ

`vsml_renderer` や `vsml_processer` などで使用される構造体を定義するためライブラリを切っている

### vsml_renderer
`vsml_core` のレンダリングの具体的な処理を定義したライブラリ

`Renderer` traitをimplementしている

### vsml_processor
VSMLのプリミティブなWrapタグを除く各タグの画像生成を行うライブラリ

ユーザ定義タグのプラグインなどはここで受け取る
