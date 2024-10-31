//#[derive(Serialize, Deserialize, Debug)]
pub struct Book {
    pub(crate) title: String,
    pub(crate) author: String,
    pub(crate) isbn: String,
}