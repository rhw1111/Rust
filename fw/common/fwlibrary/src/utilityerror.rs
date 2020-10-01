
pub struct Utilityerror{
    pub code:i32,
    pub level:i32,
    pub etype:i32,
    pub message:String,
}

impl Utilityerror{
    pub fn tostring(&self)->&str
    {
        &*self.message
    }
}

