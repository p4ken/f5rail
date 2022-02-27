use std::{fs::File, io::Write};

use anyhow::{bail, Context, Result};

use crate::transition::polyline::Polyline;

/// 入出力用の座標ファイル。
///
/// JWC_TEMP.TXTのフォーマット（参考）
/// http://mintleaf.sakura.ne.jp/cad/jwc_temp.html
pub struct JwcTemp {
    file: File,
}

impl JwcTemp {
    pub fn new(path: &str) -> Result<Self> {
        let file = File::open(path).context("JWC_TEMP.TXTを開けませんでした。")?;
        Ok(Self { file })
    }

    pub fn save(&mut self, polyline: &Result<Polyline>) -> Result<()> {
        // 最初のエラーのみが表示される。
        // エラーがあれば、図形は描画されない。
        if let Err(e) = polyline {
            return self.append(&format!("he{}", e));
        }

        // エラーがないときのみ、最後の注意が表示される。
        // 座標間の行に出力すると、座標が途切れてしまう。

        bail!("未実装")
    }

    fn append(&mut self, s: &str) -> Result<()> {
        self.file
            .write_all(s.as_bytes())
            .context("JWC_TEMP.TXTへの書き込みに失敗しました。")
    }

    fn a(&self) -> Result<()> {
        // if let Some(message) = &self.message {
        //     match message {
        //         Message::Notice(s) => fs::write(&self.path, String::from("h#") + &s)?,
        //         _ => (),
        //     }
        // }

        // todo: 文字コード変換
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
