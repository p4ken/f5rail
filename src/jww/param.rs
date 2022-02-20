#[derive(Debug, Default)]
pub struct Param {
    func: String,
}

impl Param {
    pub fn parse(args: impl IntoIterator<Item = String>) -> Param {
        let mut param = Param::default();

        for arg in args {
            let arg = arg
                .trim_start_matches(ARG_PREFIX)
                .split(ARG_SEPARATOR)
                .collect::<Vec<_>>();
            if let Some(value) = arg.get(1) {
                match arg.get(0) {
                    Some(&ARG_KEY_FUNC) => param.func = value.to_string(),
                    _ => (),
                }
            }
        }

        return param;
    }
}

const ARG_PREFIX: &str = "/";
const ARG_SEPARATOR: &str = ":";
const ARG_KEY_FUNC: &str = "FUNC";

#[test]
fn パース() {
    let v = vec![
        String::from("transition.exe"),
        String::from("/FUNC:sin"),
        String::from("/FILE:./JWC_TEMP.TXT"),
    ];
    dbg!(&v);
    let param = Param::parse(v);
    assert_eq!(param.func, "sin");
    dbg!(&param);
}
