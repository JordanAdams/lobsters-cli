use story::Story;
use select::document::Document;
use select::predicate::Class;
use select::node::Node;

#[derive(Debug, Clone)]
pub struct Page {
    pub stories: Vec<Story>,
}

impl Page {
    pub fn parse_str(body: &str) -> Page {
        let document = Document::from(body);

        let stories = document
            .find(Class("story"))
            .map(|node: Node| Story::from(node))
            .collect();

        Page { stories: stories }
    }
}
