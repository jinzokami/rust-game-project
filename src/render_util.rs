
pub fn load_ascii(path: &str) -> String
{
    use std::io::BufReader;
    use std::fs::File;
    use std::io::BufRead;

    let file = File::open(path).unwrap();
    let lines = BufReader::new(file);
    let mut text = String::new();

    for line in lines.lines() {
        text.push_str(&line.unwrap());
        text.push('\n');
    }

    text
}

pub fn load_mesh_binary(path: &str)
{

}

pub fn load_mesh_obj(path: &str)
{

}
