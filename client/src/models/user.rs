use serde::{Deserialize, Serialize};

/// 用户信息
#[derive(Debug, Clone, Deserialize, PartialEq, Serialize)]
pub  struct User {
    pub id: u32,
    pub login:String,
    pub avatar_url: String,
    pub is_admin: bool
}

/// 用于 OAuth 登录时从路径中提取 query 参数和向后端发起请求
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Login {
    pub code: String,
}