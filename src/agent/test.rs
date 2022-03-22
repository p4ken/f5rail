use std::ffi::OsString;

use anyhow::Result;

use crate::transition::{self, curve::{Diminish, Radius}};

use super::bat::*;

#[test]
fn コマンドライン引数をパースできる() {
    let args = vec![
        OsString::from("transition.exe"),
        OsString::from("/TRANSITION:1"),
        OsString::from("/R0:1.1"),
        OsString::from("/R1:2"),
        OsString::from("/TCL:3"),
        OsString::from("/FILE:./JWC_TEMP.TXT"),
    ];
    let args = Args::parse(args);
    let args = args.unwrap();
    let transition = args.unwrap_transition();
    assert_eq!(transition.0, "./JWC_TEMP.TXT");
    let param = transition.1.as_ref().unwrap();
    assert!(matches!(param.diminish, Diminish::Sine));
    assert_eq!(param.k.0.r(), Some(Radius(1.1)));
    assert_eq!(param.k.1.r(), Some(Radius(2.0)));
    assert_eq!(param.tcl, 3.0.into());
}

#[test]
fn コマンドライン引数にファイル名がなければエラー() {
    let args = vec![
        OsString::from("transition.exe"),
        OsString::from("/TRANSITION:1"),
    ];
    let args = Args::parse(args);
    let e = args.unwrap_err();
    assert_eq!(e.to_string(), "FILEを指定してください")
}

#[test]
fn 緩和曲線の長さ以外は省略可能() {
    let args = vec![
        OsString::from("transition.exe"),
        OsString::from("/TRANSITION:1"),
        OsString::from("/TCL:3"),
        OsString::from("/FILE:./JWC_TEMP.TXT"),
    ];
    let args = Args::parse(args);
    let args = args.unwrap();
    let transition = args.unwrap_transition();
    assert_eq!(transition.0, "./JWC_TEMP.TXT");
    let param = transition.1.as_ref().unwrap();
    assert!(matches!(param.diminish, Diminish::Sine));
    assert!(param.k.0.is_straight());
    assert!(param.k.1.is_straight());
    assert_eq!(param.tcl, 3.0.into());
}

#[test]
fn クロソイド曲線を指定できる() {
    let args = vec![
        OsString::from("transition.exe"),
        OsString::from("/TRANSITION:2"),
        OsString::from("/TCL:3"),
        OsString::from("/FILE:./JWC_TEMP.TXT"),
    ];
    let args = Args::parse(args).unwrap();
    let param = args.unwrap_transition().1.as_ref().unwrap();
    assert!(matches!(param.diminish, Diminish::Linear));
}

#[test]
fn 緩和曲線関数が間違っていればエラー() {
    let args = vec![
        OsString::from("transition.exe"),
        OsString::from("/TRANSITION:0"),
        OsString::from("/TCL:3"),
        OsString::from("/FILE:./JWC_TEMP.TXT"),
    ];
    let args = Args::parse(args).unwrap();
    let e = args.unwrap_transition().1.as_ref().unwrap_err();
    assert_eq!(e.to_string(), "緩和曲線関数に正しい値を入力してください");
}

#[test]
fn 緩和曲線の長さがなければエラー() {
    let args = vec![
        OsString::from("transition.exe"),
        OsString::from("/TRANSITION:1"),
        OsString::from("/FILE:./JWC_TEMP.TXT"),
    ];
    let args = Args::parse(args);
    let args = args.unwrap();
    let transition = args.unwrap_transition();
    assert_eq!(transition.0, "./JWC_TEMP.TXT");
    let e = transition.1.as_ref().unwrap_err();
    assert_eq!(e.to_string(), "TCLを指定してください");
}

#[test]
fn 緩和曲線の半径が文字列ならエラー() {
    let args = vec![
        OsString::from("transition.exe"),
        OsString::from("/TRANSITION:1"),
        OsString::from("/R0:abc"),
        OsString::from("/TCL:3"),
        OsString::from("/FILE:./JWC_TEMP.TXT"),
    ];
    let args = Args::parse(args);
    let args = args.unwrap();
    let transition = args.unwrap_transition();
    assert_eq!(transition.0, "./JWC_TEMP.TXT");
    let e = transition.1.as_ref().unwrap_err();
    assert_eq!(e.to_string(), "R0を数値で入力してください");
}

impl Args {
    fn unwrap_transition(&self) -> (&String, &Result<transition::Param>) {
        match self {
            Self::Transition(a, b) => (a, b),
            _ => panic!("This is not a transition."),
        }
    }
}
