use askama::Template;

#[derive(Template)]
#[template(path = "../prompt.txt")]
pub struct PromptTemplate<'a> {
    pub context: &'a str,
}
