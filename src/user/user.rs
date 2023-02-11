#[derive(Clone)]
pub struct User{
    pub id: String,
    pub pw: String
}
impl User{
    pub fn new(id: &str, name: &str, pw: &str) -> Self{
        Self { id: id.to_string(),
            pw: pw.to_string()
        }
    }
}

impl std::fmt::Display for User{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.id, self.pw)
    }
}

impl std::fmt::Debug for User{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({:?}, {:?})", self.id, self.pw)
    }
}