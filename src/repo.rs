pub enum Status {
    Clean,
}
pub struct Repo {
    pub path: String,
    pub branch: String,
    pub status: Status,
    pub url: Option<String>,
}
