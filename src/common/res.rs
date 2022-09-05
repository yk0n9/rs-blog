#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Res<'a, T> {
    pub code: u32,
    pub msg: &'a str,
    pub data: T,
}

impl Res<T> {
    pub fn ok(data: T) -> Self {
        Res {
            code: 200,
            msg: "操作成功",
            data,
        }
    }

    pub fn none() -> Self {
        Res {
            code: 400,
            msg: "数据为空",
            data: (),
        }
    }

    pub fn err() -> Self {
        Res {
            code: 500,
            msg: "服务错误",
            data: (),
        }
    }
}