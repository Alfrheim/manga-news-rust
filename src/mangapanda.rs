use select::document::Document;
use select::node::Node;
use select::predicate::Class;
use std::fs;

#[derive(Debug)]
struct Manga {
    name: String,
    chapter: String,
    link: String,
}

impl PartialEq for Manga {
    fn eq(&self, other: &Self) -> bool {
        self.link == other.link
            && self.chapter == other.chapter
            && self.name == other.name
    }
}

fn extract_manga(contents: String) -> Vec<Manga> {
    let document = Document::from(&contents[..]);

    // extract_chaptersrec(&document);

    //    let chapters = extract_chaptersrec(&document);
    //
    //    for chapter in chapters {
    //        println!("{}", chapter);
    //    }
    //
    println!("searching for elements");
    let mangas: Vec<Manga> = document
        .find(Class("chaptersrec"))
        .map(|node| get_manga_from(&node))
        .collect();

    for node in document.find(Class("chaptersrec")) {
        // println!("{:#?}", node);
        // println!("searching in node {:?}", node);
        // println!("printing attr: {:#?}", last);
        get_manga_from(&node);
    }

    return mangas;
}

fn get_manga_from(node: &Node) -> Manga {
    let link = node.attr("href").unwrap_or("link/not/found/0");
    let chapter = link.split("/").last().unwrap_or("0");

    Manga {
        name: link.to_string(),
        chapter: chapter.to_string(),
        link: link.to_string(),
    }
}

fn extract_chapters(document: &Document) -> Vec<String> {
    let chapters: Vec<String> = document.find(Class("chapter")).map(|n| n.text()).collect();
    chapters
}

fn extract_chaptersrec(document: &Document) -> Vec<String> {
    let chapters = document
        .find(Class("chaptersrec"))
        //todo I was trying to filter just one node to print
        // .filter(|node| node.attr("href"))
        .map(|n| println!("{:#?}", n.raw()))
        .count();
    Vec::new()
}

#[test]
fn should_extract_manga_info_from_string() {
    let contents = fs::read_to_string("src/mangapanda_source_example.html")
        .expect("Something went wrong reading the file");

    let manga = Manga{
        name: String::from("onepunch-man"),
        chapter: String::from("160"),
        link: String::from("/onepunch-man/160")};

    assert_eq!(extract_manga(contents).first().expect("can't extract any content"), &manga)
}
