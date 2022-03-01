use std::borrow::Cow;

use encoding_rs::SHIFT_JIS;

/// 文字列をSHIFT_JISに変換する
pub fn to_sjis(s: &str) -> Cow<[u8]> {
    let (cow, _, _) = SHIFT_JIS.encode(s);
    cow
}
