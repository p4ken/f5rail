
// JWWの直線・曲線はたぶんagent側で持たせた方がいい。JWW特有なので。
// あるいは共通のドメインオブジェクトgeoを作って持つとか。
// すでにJWWの関数に直接依存しているので、構造体だけtraitにする必要性に疑問。
// テストはモックせず、純関数になればそれでいい。

// Bveの相対座標もagentに持たせていいかも？ BVE特有なので。

trait Relative {

}
