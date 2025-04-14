use std::fmt;
use std::error::Error as StdError;

#[derive(Debug)]
pub struct UnipusError {
    pub message: String,
    source: Option<Box<dyn StdError + 'static>>, // 可选：存储原始错误
}

impl UnipusError {
    pub fn new(message: &str) -> Self {
        UnipusError {
            message: message.to_string(),
            source: None,
        }
    }

    /// 获取内部错误（如果有）
    pub fn source_error(&self) -> Option<&(dyn StdError + 'static)> {
        self.source.as_deref()
    }
}

// 实现 Display 和 Debug
impl fmt::Display for UnipusError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "UnipusError: {}", self.message)
    }
}

// 关键点：为所有实现了 Error 的类型实现 From<T>
impl<E: StdError + 'static> From<E> for UnipusError {
    fn from(err: E) -> Self {
        UnipusError {
            message: err.to_string(), // 错误信息转为字符串
            source: Some(Box::new(err)), // 保留原始错误（可选）
        }
    }
}