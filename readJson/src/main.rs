use serde::{Deserialize, Serialize};
use serde_json::json;
#[derive(Serialize, Deserialize)]
struct Paragraph {
    name: String,
}
#[derive(Serialize, Deserialize)]
struct Artical {
    artical: String,
    author: String,
    paragraph: Vec<Paragraph>,
}

fn main() {
    let json = r#"{"artical": "How to work in json i n rust",
"author":"akhile",
"paragraph":[
    {
    "name":"first name"
},
{
    "name":"first name"
},
{
    "name":"first name"
}
] }"#;

    let parsed: Artical = read_json_typed(json);
    println!(
        "\n\n The name of the frist paragraph is : 
{}",
        parsed.paragraph[0].name
    )
}
fn read_json_typed(raw_json : &str) -> Artical  {
let parsed :Artical =  serde_json::from_str(raw_json).unwrap();
return  parsed;
}
