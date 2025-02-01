#[allow(unused)]


use std::fmt::Debug;

use axum::{body, http, response::{IntoResponse, Response}};

use super::resp_json::Payload;

pub mod code {
    use super::Code;
    
    pub static ENV: Code =                  Code(1, "环境异常");
    pub static SYSTEM: Code =               Code(2, "系统异常");
    pub static READ_FILE: Code =            Code(3, "读取文件");
    pub static WRITE_FILE: Code =           Code(4, "写入文件");

    pub static REQUEST: Code =              Code(1000, "请求异常");
    pub static RESPONSE: Code =             Code(1001, "返回异常");

    pub static FORBIDDEN: Code =            Code(5000, "无访问权限");
    pub static TOKEN: Code =                Code(5001, "TOKEN异常");
    pub static VALIDATE: Code =             Code(5002, "校验失败");

    pub static THIRD_REQUEST: Code =        Code(2001, "三方请求失败");
    pub static THIRD_PAY: Code =            Code(2002, "三方支付失败");
    pub static THIRD_ACCESS_DATA: Code =    Code(2003, "三方支获取内容失败");
    pub static DB_CONNECT: Code =           Code(2100, "数据库连接失败");
    pub static DB_TRANS: Code =             Code(2101, "数据库创建事务报错");
    pub static DB_COMMIT: Code =            Code(2102, "数据库提交报错");
    pub static DB_ROLLBACLK: Code =         Code(2103, "数据库回滚报错");
    pub static DB_QUERY: Code =             Code(2104, "数据库查询列表报错");
    pub static DB_EXEC: Code =              Code(2105, "数据库执行报错");

    pub static REDIS_CONNECT: Code =        Code(2201, "REDIS连接失败");
    pub static REDIS_CMD: Code =            Code(2202, "REDIS操作失败");

    pub static KAFKA_CONNECT: Code =        Code(2221, "KAFKA连接失败");
    pub static KAFKA_ADMIN: Code =          Code(2222, "KAFKA管理失败");
    pub static KAFKA_PRODUCE: Code =        Code(2223, "KAFKA生产失败");
    pub static KAFKA_CONSUME: Code =        Code(2224, "KAFKA消费失败");

    pub static PARSE: Code =                Code(3001, "解析失败");
    pub static SERIALIZE: Code =            Code(3002, "序列化失败");
    pub static ENCRIPT: Code =              Code(3003, "加密失败");
    pub static DECRIPT: Code =              Code(3004, "解密失败");

    pub static NOT_ENOUGH: Code =           Code(6000, "资源不满足");
    pub static NOT_FOUND: Code =            Code(6001, "资源未找到");
    pub static ALREADY_EXSIT: Code =        Code(6002, "资源已存在");
    pub static OUT_OF_TIME: Code =          Code(6003, "资源已失效");
    pub static OUT_OF_LIMIT: Code =         Code(6004, "资源超过限制");
    pub static WRONG_STATUS: Code =         Code(6005, "非可处理状态");
    pub static RESERVED: Code =             Code(6006, "保留资源");

    pub static LOGIC_ERROR: Code =          Code(9000, "逻辑异常");

}


#[derive(Debug)]
pub struct Code(pub i32, pub &'static str);

#[derive(Debug)]
pub struct Error {
    pub code: &'static Code,
    pub message: String,
    pub trace: Option<String>,
}
impl Error {
    pub fn new(code: &'static Code, message: &str) -> Self {
        Error {
            code,
            message: message.to_string(),
            trace: None,
        }
    }
    pub fn error<E: Debug>(code: &'static Code, message: &str, err: E) -> Self {
        Error {
            code,
            message: message.to_string(),
            trace: Some(format!("{:?}", err).to_string()),
        }
    }

    fn to_payload(self) -> Payload<String> {
        Payload {
            code: self.code.0,
            message: self.message,
            data: None,
            trace: self.trace
        }
    }
}
impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Error:{}({}),{:?}", self.message, self.code.0, self.trace)
    }
}
impl IntoResponse for Error {
    fn into_response(self) -> Response<body::Body> {
        let payload = self.to_payload();
        let json = serde_json::to_string(&payload).unwrap();
        let body = body::Body::new(json);
        Response::builder()
            .status(http::StatusCode::OK)
            .header("Content-Type", "application/json")
            .body(body)
            .unwrap()
    }
}
