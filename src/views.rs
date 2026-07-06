use serde::Serialize;

#[derive(Debug, Serialize)]
pub(crate) struct PublicationView {
    pub(crate) authors: String,
    pub(crate) title: String,
    pub(crate) journal: String,
    pub(crate) citation: Option<String>,
    pub(crate) year: i32,
    pub(crate) identifier_html: Option<String>,
}

#[derive(Debug, Serialize)]
pub(crate) struct PublicationYear {
    pub(crate) year: i32,
    pub(crate) publications: Vec<PublicationView>,
}

#[derive(Debug, Serialize)]
pub(crate) struct AbstractView {
    pub(crate) authors: String,
    pub(crate) title: String,
    pub(crate) event: String,
    pub(crate) year: i32,
    pub(crate) location: String,
}
