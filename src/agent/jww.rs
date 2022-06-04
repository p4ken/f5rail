use std::{
    fmt::Display,
    fs::{File, OpenOptions},
    io::{self, BufRead, BufReader},
    path::{Path, PathBuf},
};

use anyhow::{bail, ensure, Context, Result};
use derive_more::Deref;
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

    /// 作業中のファイルが存在するディレクトリ
    pub fn project_dir(&mut self) -> Result<PathBuf> {
        self.cache()?.project_dir()
    }

    /// 図形データ
    pub fn figures(&mut self) -> Result<Vec<Figure>> {
        self.cache()?.figures()
    }

    fn cache(&mut self) -> Result<&Cache> {
        // 必要になったときに読み取る。
        if self.cache.is_none() {
            self.cache = Some(self.read_cache()?);
        }
        self.cache.as_ref().context("")
    }

    fn read_cache(&self) -> Result<Cache> {
        let decoder = DecodeReaderBytesBuilder::new()
            .encoding(Some(SHIFT_JIS))
            .build(&self.file);
        BufReader::new(decoder)
            .lines()
            .collect::<std::io::Result<Cache>>()
            .context("JWC_TEMPファイルの読み取りに失敗しました")
    }
}

#[derive(Debug, PartialEq)]
struct Cache(Vec<String>);

impl FromIterator<String> for Cache {
    fn from_iter<T: IntoIterator<Item = String>>(iter: T) -> Self {
        Self(<Vec<String>>::from_iter(iter))
    }
}

impl Cache {
    /// トラック名
    fn track_name(&self) -> &str {
        self.iter()
            .find_map(|line| line.strip_prefix("/トラック名:"))
            .unwrap_or(" ")
    }

    /// 作業中のファイルが存在するディレクトリ
    fn project_dir(&self) -> Result<PathBuf> {
        let path = self
            .iter()
            .find_map(|line| line.strip_prefix("file="))
            .context("JWC_TEMPファイルにパスが出力されていません")?;
        ensure!(
            !path.is_empty(),
            "作業中のファイルに名前をつけて保存してください"
        );
        let dir = Path::new(path)
            .parent()
            .with_context(|| format!("{} と同じフォルダに出力できません", path))?;
        Ok(dir.to_path_buf())
    }

    /// 始点距離程
    fn distance_0(&self) -> Result<f64> {
        let s = self
            .iter()
            .find_map(|line| line.strip_prefix("/始点距離程:"))
            .context("始点距離程を指定してください")?;
        s.parse()
            .with_context(|| format!("始点距離程 {} を数値にパースできません", s))
    }

    /// 図形データ
    fn figures(&self) -> Result<Vec<Figure>> {
        self.iter()
            .filter_map(|line| Figure::parse(&line).transpose())
            .collect()
    }

    /// 出力始点
    fn anchor_0(&self) -> Result<[f64; 2]> {
        let hp1 = self
            .iter()
            .find_map(|line| line.strip_prefix("hp1"))
            .context("JWC_TEMPファイルに指示点1がありません")?;
        hp1.split_whitespace()
            .filter_map(|s| s.parse().ok())
            .collect::<Vec<f64>>()
            .try_into()
            .ok()
            .with_context(|| format!("指示点 {} を数値にパースできません", hp1))
    }

    fn iter(&self) -> impl Iterator<Item = &String> {
        self.0.iter()
    }
}

/// JWW_TEMPファイルの図形データ
#[derive(Debug, PartialEq)]
pub enum Figure {
    /// 単線
    Straight([f64; 4]),

    /// 円弧
    Arc([f64; 5]),

    /// 円・楕円・楕円弧
    Ellipse,

    /// ブロック図形
    Block,

    /// ソリッド（線形・円周）
    Solid,
}

impl Figure {
    /// 図形データをパースする
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
            ["ci", ..] => Self::Ellipse,
            ["BL", ..] => Self::Block,
            ["sl", _, _, _, _] | ["se", ..] => Self::Solid,
            _ => return Ok(None),
        };
        return Ok(Some(figure));
    }

    /// 数値にパースする
    fn parse_num(v: &[&str]) -> Vec<f64> {
        v.iter()
            .filter_map(|s| s.parse().ok())
            .collect::<Vec<f64>>()
    }
}

impl From<&Figure> for Stroke {
    fn from(_: &Figure) -> Self {
        // TODO
        Stroke::ToDo
    }
}

#[cfg(test)]
mod 座標ファイルをパースする {
    use rstest::rstest;

    use super::*;

    #[rstest]
    #[case(vec!["file=parent/child/file"], Ok("parent/child"))]
    #[case(vec!["file=/"], Err("/ と同じフォルダに出力できません"))]
    #[case(vec!["file="], Err("作業中のファイルに名前をつけて保存してください"))]
    #[case(vec![""], Err("JWC_TEMPファイルにパスが出力されていません"))]
    fn プロジェクトファイル名(
        #[case] contents: Vec<&str>,
        #[case] expected: Result<&str, &str>,
    ) {
        let cache = contents.into_iter().map(&str::to_string).collect::<Cache>();
        match (cache.project_dir(), expected) {
            (Ok(x), Ok(expected)) => assert_eq!(x, PathBuf::from(expected)),
            (Err(e), Err(expected)) => assert_eq!(e.to_string(), expected.to_string()),
            (lhs, rhs) => panic!("{:?} vs {:?}", lhs, rhs),
        }
    }

    #[rstest]
    #[case(vec!["/始点距離程:5"], Ok(5.0))]
    #[case(vec!["/始点距離程:-5.1"], Ok(-5.1))]
    #[case(vec!["/始点距離程:a"], Err("始点距離程 a を数値にパースできません"))]
    #[case(vec![""], Err("始点距離程を指定してください"))]
    fn 始点距離程(#[case] contents: Vec<&str>, #[case] expected: Result<f64, &str>) {
        let cache = contents.into_iter().map(&str::to_string).collect::<Cache>();
        match (cache.distance_0(), expected) {
            (Ok(x), Ok(expected)) => assert_eq!(x, expected),
            (Err(e), Err(expected)) => assert_eq!(e.to_string(), expected.to_string()),
            _ => panic!(""),
        }
    }

    #[rstest]
    #[case(vec!["/トラック名:文字"], "文字")]
    fn トラック名(#[case] contents: Vec<&str>, #[case] expected: &str) {
        let cache = contents.into_iter().map(&str::to_string).collect::<Cache>();
        assert_eq!(cache.track_name(), expected);
    }

    #[rstest]
    #[case::直線(vec![" -47275.9875573158 -46216.5741820161 93751.1092168777 -72084.3161175"], vec![Figure::Straight([-47275.9875573158, -46216.5741820161, 93751.1092168777, -72084.3161175])])]
    #[case::円弧(vec!["ci 26701.9673429692 8497.03095908351 48568.3678393406 39.3304039946051 138.775693111848 1 0"], vec![Figure::Arc([26701.9673429692,8497.03095908351,48568.3678393406,39.3304039946051,138.775693111848])])]
    #[case::楕円弧(vec!["ci 1 2 3 4 5 1.5 5"], vec![Figure::Ellipse])]
    #[case::円(vec!["ci 1 2 3"], vec![Figure::Ellipse])]
    #[case::ブロック図形(vec!["BL 0 0 \"aaa"], vec![Figure::Block])]
    #[case::線形ソリッド(vec!["sl -11022.8155339806 26740.5339805825 11022.8155339806 -26740.5339805825"], vec![Figure::Solid])]
    #[case::円周ソリッド(vec!["se 0 0 18917.6057536967 1 0 2.99371813587285 2.42761894570543 0"], vec![Figure::Solid])]
    fn 図形データ(#[case] contents: Vec<&str>, #[case] expected: Vec<Figure>) {
        let cache = contents.into_iter().map(&str::to_string).collect::<Cache>();
        let figures = cache.figures();
        assert!(figures.is_ok());
        assert_eq!(figures.unwrap(), expected);
    }

    #[rstest]
    #[case::直線(vec!["hp1   136190.26326708 -45574.8870993316"], Ok([136190.26326708,-45574.8870993316]))]
    fn 出力始点(#[case] contents: Vec<&str>, #[case] expected: Result<[f64; 2]>) {
        let cache = contents.into_iter().map(&str::to_string).collect::<Cache>();
        match (cache.anchor_0(), expected) {
            (Ok(x), Ok(expected)) => assert_eq!(x, expected),
            (Err(e), Err(expected)) => assert_eq!(e.to_string(), expected.to_string()),
            _ => panic!(""),
        }
    }
}
