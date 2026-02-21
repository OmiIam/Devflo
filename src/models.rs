
#[derive(Debug)]
pub enum Severity{
    
    Low,
    Medium,
    High,
    Critical,
}


#[derive(Debug)]
pub struct Finding {
    pub line_number: u32,
    pub file_path: String,
    pub matched_text: String,
    pub secret_type: String,
    pub severity: Severity,
    }

