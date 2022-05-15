use std::{
    fmt::Display,
    fs::{File, OpenOptions},
    io::{self, BufRead, BufReader},
    path::{Path, PathBuf},
};

use anyhow::{ensure, Context, Result};
use encoding_rs::SHIFT_JIS;
use encoding_rs_io::DecodeReaderBytesBuilder;

use crate::transition::unit::{Deg, Meter, Vector};

/// 入出力用の座標ファイル。
///
/// (参考) JWC_TEMP.TXTのフォーマット
/// http://mintleaf.sakura.ne.jp/cad/jwc_temp.html
pub struct JwcTemp {
    file: File,
}

// struct Write {
//     file: File,
// }

// struct Read {
//     path: Path,
//     cache: Option<Cache>
// }

impl JwcTemp {
    /// 座標ファイルを読み込む。
    pub fn read(path: &(impl AsRef<Path> + ?Sized)) -> Result<Cache> {
        let file = OpenOptions::new().read(true).open(path).with_context(|| {
            format!(
                "JWC_TEMPファイル {} を開けませんでした",
                path.as_ref().to_string_lossy()
            )
        })?;
        let decoder = DecodeReaderBytesBuilder::new()
            .encoding(Some(SHIFT_JIS))
            .build(file);
        let cache = BufReader::new(decoder)
            .lines()
            .filter_map(|l| l.ok())
            .collect::<Cache>();
        Ok(cache)
    }

    /// 座標ファイルを作成する。
    pub fn create(path: &(impl AsRef<Path> + ?Sized)) -> Result<Self> {
        let file = File::create(path).with_context(|| {
            format!(
                "JWC_TEMPファイル {} に書き込めませんでした",
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

    /// 曲線を出力する。
    pub fn curve(
        &mut self,
        c: &impl Vector,
        r: &impl Meter,
        a0: &impl Deg,
        a1: &impl Deg,
    ) -> Result<()> {
        let (cx, cy) = (c.x(), c.y());
        let r = r.meter().abs();
        let (a0, a1) = (a0.deg(), a1.deg());
        let (a0, a1) = if a0 < a1 { (a0, a1) } else { (a1, a0) };
        self.puts(format!("ci {cx} {cy} {r} {a0} {a1}"))
    }

    /// 直線を出力する。
    pub fn straight(&mut self, p0: &impl Vector, p1: &impl Vector) -> Result<()> {
        self.puts(format!("{} {} {} {}", p0.x(), p0.y(), p1.x(), p1.y()))
    }

    /// 文字列と改行を出力する。
    fn puts<T: AsRef<str>>(&mut self, s: T) -> Result<()> {
        // TODO:
        // SHIFT_JISではなくCP932にしたほうがいい。
        // - https://crates.io/crates/codepage
        // - https://crates.io/search?q=windows%20encoding&sort=downloads
        let (sjis, _, _) = SHIFT_JIS.encode(s.as_ref());
        for bytes in [&sjis[..], b"\r\n"] {
            io::Write::write_all(&mut self.file, bytes)
                .context("JWC_TEMP.TXTへの書き込みに失敗しました。")?;
        }
        Ok(())
    }
}

#[derive(Default)]
pub struct Cache {
    track_name: Option<String>,
    project_path: Option<String>,
}

impl Cache {
    /// トラック名
    pub fn track_name(&self) -> &str {
        self.track_name.as_ref().map_or(" ", |s| s.as_str())
    }

    /// 作業中のファイルがあるディレクトリ
    pub fn project_dir(&self) -> Result<PathBuf> {
        let path = self.project_path()?;

        let dir = Path::new(path)
            .parent()
            .with_context(|| format!("{} と同じフォルダに出力できません", path))?;

        Ok(dir.to_path_buf())
    }

    /// 作業中のファイルパス
    fn project_path(&self) -> Result<&String> {
        let path = self
            .project_path
            .as_ref()
            .context("JWC_TEMPファイルにパスが出力されていません")?;

        ensure!(
            !path.is_empty(),
            "作業中のファイルに名前をつけて保存してください"
        );

        Ok(path)
    }
}

impl FromIterator<String> for Cache {
    fn from_iter<T: IntoIterator<Item = String>>(iter: T) -> Self {
        let mut cache = Self::default();
        for line in iter {
            if let Some(s) = line.strip_prefix("file=") {
                cache.project_path = Some(s.to_string());
            } else if let Some(a) = line.strip_prefix("ci ") {
                let v = a.split(" ").collect::<Vec<_>>();
                if let [_a, _b, _c, _d, _e, _f, _g] = v.as_slice() {
                    // cache.curve.push()
                }
            } else if let Some(_straight) = line.strip_prefix(" ") {
                //
            } else if let Some(s) = line.strip_prefix("/トラック名:") {
                cache.track_name = Some(s.to_string());
            } else if let Some(_z0) = line.strip_prefix("/始点距離程:") {
                //
            }
        }
        cache
    }
}
