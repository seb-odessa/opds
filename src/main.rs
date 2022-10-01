use quick_xml;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
#[serde(rename = "feed")]
struct Feed {
    #[serde(rename = "xmlns")]
    xmlns: String,
    #[serde(rename = "xmlns:dc")]
    xmlns_dc: String,
    #[serde(rename = "xmlns:os")]
    xmlns_os: String,
    #[serde(rename = "xmlns:opds")]
    xmlns_opds: String,

    id: StringTag,
    title: StringTag,
    updated: StringTag,
    icon: StringTag,

    link: Vec<Link>,
    entry: Vec<Entry>,
}
impl Feed {
    pub fn new() -> Self {
        Self {
            xmlns: String::from("http://www.w3.org/2005/Atom"),
            xmlns_dc: String::from("http://purl.org/dc/terms/"),
            xmlns_os: String::from("http://a9.com/-/spec/opensearch/1.1/"),
            xmlns_opds: String::from("http://opds-spec.org/2010/catalog"),

            id: StringTag(String::from("tag:root")),
            title: StringTag(String::from("Flibusta catalog")),
            updated: StringTag(String::from("2022-09-28T10:33:31+02:00")),
            icon: StringTag(String::from("/favicon.ico")),

            link: vec![
                Link{
                    href: String::from("/opds-opensearch.xml"),
                    href_rel: String::from("search"),
                    href_type: String::from("application/opensearchdescription+xml"),
                },
                Link{
                    href: String::from("/opds/search?searchTerm={searchTerms}"),
                    href_rel: String::from("search"),
                    href_type: String::from("application/atom+xml"),
                },
                Link{
                    href: String::from("/opds"),
                    href_rel: String::from("start"),
                    href_type: String::from("application/atom+xml;profile=opds-catalog"),
                },
                Link{
                    href: String::from("/opds"),
                    href_rel: String::from("self"),
                    href_type: String::from("application/atom+xml;profile=opds-catalog"),
                },
            ],
            entry: vec![
                Entry{
                    id: StringTag(String::from("tag:root::new")),
                    title: StringTag(String::from("Новинки")),
                    updated: StringTag(String::from("2022-09-28T10:33:31+02:00")),
                    content: Content{
                        content_type: String::from("text"),
                        content_data: String::from("Новые поступления за неделю"),
                    },
                    link: Link{
                        href: String::from("/opds/new"),
                        href_rel: String::from("http://opds-spec.org/sort/new"),
                        href_type: String::from("application/atom+xml;profile=opds-catalog"),
                    },
                },
            ],
        }
    }
}

#[derive(Deserialize, Serialize)]
struct StringTag (String);


#[derive(Deserialize, Serialize)]
struct Link {
    #[serde(rename = "href")]
    href: String,
    #[serde(rename = "rel")]
    href_rel: String,
    #[serde(rename = "type")]
    href_type: String,
}


#[derive(Deserialize, Serialize)]
struct Entry {
    id: StringTag,
    title: StringTag,
    updated: StringTag,
    content: Content,
    link: Link,
}


#[derive(Deserialize, Serialize)]
struct Content {
    #[serde(rename = "type")]
    content_type: String,
    #[serde(rename = "$value")]
    content_data: String
}


fn main() {
    let feed = Feed::new();


    println!(r#"<?xml version="1.0" encoding="utf-8"?>"#);
    println!("{}", quick_xml::se::to_string(&feed).unwrap());
}
