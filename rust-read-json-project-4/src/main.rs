use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct Paragraph {
    name : String,
}


#[derive(Serialize, Deserialize)]
struct Article {
    article: String,
    author: String,
    paragraph: Vec<Paragraph>
}

impl Article {
    fn new(raw_json: &str) -> Article {
        let parsed = serde_json::from_str(raw_json).unwrap();
        parsed
    } 
}

fn main(){
    let json = r#" 
    {
        "article": "How to work with json in rust",
        "author": "vael",
        "paragraph": [
            {
                "name": "starting sentence"
            },
            {
                "name" : "body of the paragraph"
            },
            {
                "name" : "end of the paragraph"
            }
            
        ]
    }"#;

    let parsed = Article::new(json);
    println!("\n\n The name of the first paragraph is : {}", parsed.paragraph[2].name);

}   

//fn read_json_typed(raw_json: &str) -> Article {
//    let parsed: Article = serde_json::from_str(raw_json).unwrap();
//    parsed
//}
