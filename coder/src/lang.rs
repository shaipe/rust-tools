/**
 * 开发语言类型
 * 
 */



/// 开发语言
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum Language {
    // Rust语言
    Rust,
    // C#
    CSharp,
    // Java
    Java
}

impl Language {

    /// 支持的语言语类型
    #[allow(dead_code)]
    pub fn as_str(&self) -> &'static str {
        match *self {
            Language::Rust => "rust",
            Language::Java => "java",
            Language::CSharp => "csharp"
        }
    }

    /// 开发放言对应的文件类型
    #[allow(dead_code)]
    pub fn file_ext_str(&self) -> &'static str {
        match *self {
            Language::Rust => ".rs",
            Language::Java => ".java",
            Language::CSharp => ".cs"
        }
    }
}
