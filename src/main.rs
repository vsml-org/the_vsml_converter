use vsmlc::{args, iv};

fn main() {
    // 引数を取得
    let args = args::get_parsed_args();

    // VSMLファイルからIVデータに変換
    iv::convert_iv_data(args.input_path, &args.src_base_path);

    // 動画へと出力
    // output_video(iv_data);
}
