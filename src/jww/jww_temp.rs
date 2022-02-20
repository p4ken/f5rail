use std::fs;

/// 入出力用の座標ファイル。
/// 
/// JWC_TEMP.TXTのフォーマット（参考）
/// http://mintleaf.sakura.ne.jp/cad/jwc_temp.html
pub struct JwwTemp {
    path: String,
    message: Option<Message>,
}

pub enum Message {
    Notice(String),
    Error(String),
}

impl JwwTemp {
    pub fn new(path: &String) -> JwwTemp {
        JwwTemp {
            path: path.clone(),
            message: None,
        }
    }

    pub fn notice(&mut self, message: &str) {
        self.message = Some(Message::Notice(message.to_owned()));
    }

    /// todo: 文字列エンコード変換
    pub fn flush(&self) {
        if let message = self.message {
            let _ = fs::write(&self.path, "h#".to_owned() + message);
        }
        let mut content = "h#サイン半波長逓減";
        println!("h#サイン半波長逓減");
        println!("pl");
        println!("0 0");
        println!("100 -100");
        println!("200 -400");
        println!("300 -900");
        println!("#");
    }
}
