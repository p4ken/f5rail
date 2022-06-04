use std::{
    fmt::Display,
    fs::{File, OpenOptions},
    io::{self, BufRead, BufReader},
    path::{Path, PathBuf},
};

use anyhow::{bail, ensure, Context, Result};
use encoding_rs::SHIFT_JIS;
use encoding_rs_io::DecodeReaderBytesBuilder;

use crate::{
    track::polyline::Stroke,
    transition::unit::{Deg, Meter, Vector},
};

/// 入出力用の座標ファイル。
///
/// (参考) JWC_TEMP.TXTのフォーマット
/// http://mintleaf.sakura.ne.jp/cad/jwc_temp.html
pub struct JwcTemp;

impl JwcTemp {
    /// 座標ファイルを読み込む。
    pub fn open(path: &(impl AsRef<Path> + ?Sized)) -> Result<Read> {
        let path = path.as_ref();
        let file = OpenOptions::new()
            .read(true)
            .open(path)
            .with_context(|| format!("ファイル {} を開けませんでした", path.display()))?;
        let cache = None;
        Ok(Read { file, cache })
    }

    /// 座標ファイルを作成する。
    pub fn create(path: &(impl AsRef<Path> + ?Sized)) -> Result<Write> {
        let file = File::create(path).with_context(|| {
            format!(
                "JWC_TEMPファイル {} を作成できませんでした",
                path.as_ref().display()
            )
        })?;
        Ok(Write { file })
    }
}

pub struct Write {
    file: File,
}

impl Write {
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

pub struct Read {
    file: File,
    cache: Option<Cache>,
}

impl Read {
    /// トラック名
    pub fn track_name(&mut self) -> Result<&str> {
        self.cache().map(Cache::track_name)
    }

    /// 作業中のファイルがあるディレクトリ
    pub fn project_dir(&mut self) -> Result<PathBuf> {
        let path = self.project_path()?;

        let dir = Path::new(path)
            .parent()
            .with_context(|| format!("{} と同じフォルダに出力できません", path))?;

        Ok(dir.to_path_buf())
    }

    /// 図形データ
    pub fn figures(&mut self) -> Result<Vec<Figure>> {
        self.cache()?.figures()
    }

    /// 作業中のファイルパス
    fn project_path(&mut self) -> Result<&str> {
        let path = self
            .cache()?
            .project_path()
            .context("JWC_TEMPファイルにパスが出力されていません")?;

        ensure!(
            !path.is_empty(),
            "作業中のファイルに名前をつけて保存してください"
        );

        Ok(path)
    }

    fn cache(&mut self) -> Result<&Cache> {
        // 必要になったときに読み取る。
        if self.cache.is_none() {
            self.cache = Some(self.read()?);
        }
        self.cache.as_ref().context("")
    }

    fn read(&self) -> Result<Cache> {
        let decoder = DecodeReaderBytesBuilder::new()
            .encoding(Some(SHIFT_JIS))
            .build(&self.file);
        BufReader::new(decoder)
            .lines()
            .map(|a| a.context(""))
            .collect::<Result<Cache>>()
    }
}

#[derive(Debug, PartialEq, Default)]
struct Cache {
    buf: Vec<String>,
    project_path: Option<String>,
    figures: Vec<Figure>,
}

impl FromIterator<String> for Cache {
    fn from_iter<T: IntoIterator<Item = String>>(iter: T) -> Self {
        let mut cache = Self::default();
        cache.buf = iter.into_iter().collect();
        cache
    }
}

impl Cache {
    // 一度に全部パースする必要はない。エラーハンドリングしにくい。
    fn parse(iter: impl IntoIterator<Item = String>) -> Result<Self> {
        let mut cache = Self::default();
        for line in iter {
            if let Some(s) = line.strip_prefix("file=") {
                cache.project_path = Some(s.to_string());
            } else if let Some(_z0) = line.strip_prefix("/始点距離程:") {
                //
            }
        }
        Ok(cache)
    }

    fn track_name(&self) -> &str {
        for line in &self.buf {
            if let Some(s) = line.strip_prefix("/トラック名:") {
                return s;
            }
        }
        " "
    }

    fn project_path(&self) -> Option<&str> {
        self.project_path.as_ref().map(&String::as_str)
    }

    fn figures(&self) -> Result<Vec<Figure>> {
        self.buf
            .iter()
            .filter_map(|line| Figure::parse(line).transpose())
            .collect()
    }
}

/// JWW_TEMPファイルの図形データ
#[derive(Debug, PartialEq)]
pub enum Figure {
    /// 単線
    Straight([f64; 4]),

    /// 円弧
    Arc([f64; 5]),

    /// 円
    Circle,

    /// 楕円弧
    Ellipse,
    // TODO: ソリッド図形とか
}

impl Figure {
    fn parse(line: &str) -> Result<Option<Self>> {
        let str_v = line.split_whitespace().collect::<Vec<_>>();
        let figure = match str_v[..] {
            [_, _, _, _] if line.starts_with(' ') => match Self::parse_num(&str_v).try_into() {
                Ok(array) => Self::Straight(array),
                _ => bail!("単線 {} を数値にパースできません", line),
            },
            ["ci", _, _, _, _, _, "1", "0"] => match Self::parse_num(&str_v[1..=5]).try_into() {
                Ok(array) => Self::Arc(array),
                _ => bail!("円弧 {} を数値にパースできません", line),
            },
            ["ci", _, _, _, _, _, _, _] => Self::Ellipse,
            ["ci", _, _, _] => Self::Circle,
            _ => return Ok(None),
        };
        return Ok(Some(figure));
    }

    fn parse_num(v: &[&str]) -> Vec<f64> {
        v.iter()
            .filter_map(|s| s.parse().ok())
            .collect::<Vec<f64>>()
    }
}

#[derive(Debug, PartialEq)]
pub struct StraightLine([f64; 4]);

impl StraightLine {
    pub fn x0(&self) -> f64 {
        self.0[0]
    }
}

impl StraightLine {
    fn parse(line: &str) -> Option<Self> {
        // 数値でない -> エラー
        // 要素数4以外 -> エラー
        line.split(" ")
            .filter_map(|s| s.parse().ok())
            .collect::<Vec<f64>>()
            .try_into()
            .ok()
            .map(|array| Self(array))
    }
}

impl From<&Figure> for Stroke {
    fn from(_: &Figure) -> Self {
        // TODO
        Stroke::ToDo
    }
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    #[rstest]
    #[case(vec!["/トラック名:文字"], "文字")]
    fn トラック名をパースする(#[case] contents: Vec<&str>, #[case] expected: &str) {
        let cache = contents.into_iter().map(&str::to_string).collect::<Cache>();
        assert_eq!(cache.track_name(), expected);
    }

    #[rstest]
    #[case::直線(vec![" 1 2 3 4"], vec![Figure::Straight([1.,2.,3.,4.])])]
    #[case::円弧(vec!["ci 1 2 3 4 5 1 0"], vec![Figure::Arc([1.,2.,3.,4.,5.])])]
    #[case::楕円弧(vec!["ci 1 2 3 4 5 1.5 5"], vec![Figure::Ellipse])]
    #[case::円(vec!["ci 1 2 3"], vec![Figure::Circle])]
    fn 図形データをパースする(
        #[case] contents: Vec<&str>,
        #[case] expected: Vec<Figure>,
    ) {
        let cache = contents.into_iter().map(&str::to_string).collect::<Cache>();
        let figures = cache.figures();
        assert!(figures.is_ok());
        assert_eq!(figures.unwrap(), expected);
    }
}
