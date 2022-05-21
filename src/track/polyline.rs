// JWWの直線・曲線はたぶんagent側で持たせた方がいい。JWW特有なので。
// あるいは共通のドメインオブジェクトgeoを作って持つとか。
// すでにJWWの関数に直接依存しているので、構造体だけtraitにする必要性に疑問。
// ドメインサービスが純関数にさえなればモック不要で十分テストしやすい。

use anyhow::Result;

/// 連続線
pub struct Polyline {}

impl Polyline {
    // 円・直線はgeoかagentだけど、連続線はtrack特有。
    pub fn new() -> Result<Self> {
        Ok(Self {})
    }
}
