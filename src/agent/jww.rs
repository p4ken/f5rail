use std::{
    fmt::Display,
    fs::{File, OpenOptions},
    io::{BufRead, BufReader, Write},
    path::{Path, PathBuf},
};

use anyhow::{ensure, Context, Result};
use encoding_rs::SHIFT_JIS;
use encoding_rs_io::DecodeReaderBytesBuilder;

/// create -> 書き込む
/// open -> 開く＆書き込む
/// どのみち書き込むので、どのみちmutでもいい

pub struct JwcTemp {
    file: File,
    // cache: Option<Cache>,
}

impl JwcTemp {
    pub fn read(path: &(impl AsRef<Path> + ?Sized)) -> Result<Cache> {
        let file = OpenOptions::new().read(true).open(path).with_context(|| {
            format!(
                "JWC_TEMPファイル {} を開けませんでした。",
                path.as_ref().to_string_lossy()
            )
        })?;
        let decoder = DecodeReaderBytesBuilder::new()
            .encoding(Some(SHIFT_JIS))
            .build(file);
        let mut cache = Cache::default();
        let line_iter = BufReader::new(decoder).lines().filter_map(|l| l.ok());
        for line in line_iter {
            if let Some((_, path)) = line.split_once("file=") {
                cache.project_path = Some(path.to_string());
            }
        }
        Ok(cache)
    }

    pub fn create(path: &(impl AsRef<Path> + ?Sized)) -> Result<Self> {
        let file = File::create(path).with_context(|| {
            format!(
                "JWC_TEMPファイル {} に書き込めませんでした。",
                path.as_ref().to_string_lossy()
            )
        })?;
        Ok(Self { file })
    }

    /// エラー `e` を書き込む。
    ///
    /// - 最初のエラーのみが表示される。
    /// - エラーがあれば、エラー以外の座標などはすべて無視される。
    pub fn error(&mut self, e: &impl Display) -> Result<()> {
        self.puts(&format!("he{}", e))
    }

    /// 注意を出力する。
    ///
    /// 最後の注意のみ表示される。
    ///
    /// 座標の間に出力すると、座標が途切れてしまう。
    pub fn notice<T: AsRef<str>>(&mut self, s: T) -> Result<()> {
        self.puts(format!("h#{}", s.as_ref()))
    }

    /// 文字列と改行を出力する。
    fn puts<T: AsRef<str>>(&mut self, s: T) -> Result<()> {
        // TODO:
        // SHIFT_JISではなくCP932にしたほうがいい。
        // - https://crates.io/crates/codepage
        // - https://crates.io/search?q=windows%20encoding&sort=downloads
        let (sjis, _, _) = SHIFT_JIS.encode(s.as_ref());
        for bytes in [&sjis[..], b"\r\n"] {
            Write::write_all(&mut self.file, bytes)
                .context("JWC_TEMP.TXTへの書き込みに失敗しました。")?;
        }
        Ok(())
    }
}

#[derive(Default)]
pub struct Cache {
    project_path: Option<String>,
}

impl Cache {
    pub fn project_dir(&self) -> Result<PathBuf> {
        let path = self.project_path()?;

        ensure!(
            !path.is_empty(),
            "作業中のファイルに名前をつけて保存してください。"
        );

        let dir = Path::new(path)
            .parent()
            .with_context(|| format!("{} と同じフォルダに出力できません。", path))?;

        Ok(dir.to_path_buf())
    }

    /// 作業中のファイルパス
    fn project_path(&self) -> Result<&String> {
        self.project_path
            .as_ref()
            .context("JWC_TEMPファイルにパスが出力されていません。")
    }
}
