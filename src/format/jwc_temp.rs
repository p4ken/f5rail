use std::fs;

use anyhow::Result;

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
    pub fn new(path: &str) -> JwwTemp {
        JwwTemp {
            path: path.to_owned(),
            message: None,
        }
    }

    // 最初のエラーのみが表示される。
    // エラーがあれば、図形は描画されない。

    pub fn notice(&mut self, message: &str) {
        // エラーがないときのみ、最後の注意が表示される。
        // 座標間の行に出力すると、座標が途切れてしまう。
        self.message = Some(Message::Notice(message.to_owned()));
    }

    /// todo: 文字コード変換
    pub fn save(&self) -> Result<()> {
        if let Some(message) = &self.message {
            match message {
                Message::Notice(s) => fs::write(&self.path, String::from("h#") + &s)?,
                _ => (),
            }
        }
        let mut content = "h#サイン半波長逓減";
        println!("h#サイン半波長逓減");
        println!("pl");
        println!("0 0");
        println!("100 -100");
        println!("200 -400");
        println!("300 -900");
        println!("#");
        Ok(())
    }
}
