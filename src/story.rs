use select::node::Node;
use select::predicate::{Class, Name, Predicate};

#[derive(Debug, Clone)]
pub struct Story {
    pub title: String,
    pub url: String,
    pub domain: Option<String>,
    pub tags: Vec<String>,
    pub score: u32,
    pub author: String,
    pub timestamp: String
}

impl<'a> From<Node<'a>> for Story {
    fn from(story_node: Node) -> Story {
        let link = story_node
            .find(Class("link").descendant(Name("a")))
            .nth(0)
            .unwrap();

        let title = link.text();
        let url = link.attr("href").unwrap();

        let domain = story_node
            .find(Class("domain"))
            .nth(0)
            .and_then(|domain_node| Some(domain_node.text()));

        let tags = story_node
            .find(Class("tag"))
            .map(|tag_node: Node| tag_node.text())
            .collect();

        let score: u32 = match story_node.find(Class("score")).next() {
            Some(score_node) => {
                score_node.text().parse().unwrap_or(0)
            },
            None => 0
        };

        let author = story_node
            .find(Class("byline").descendant(Name("a")))
            .nth(1)
            .unwrap()
            .text();

        let timestamp = story_node
            .find(Class("byline").descendant(Name("span")))
            .nth(0)
            .unwrap()
            .text();

        Story {
            title: title,
            url: url.to_string(),
            domain: domain,
            tags: tags,
            score: score,
            author: author,
            timestamp: timestamp
        }
    }
}
