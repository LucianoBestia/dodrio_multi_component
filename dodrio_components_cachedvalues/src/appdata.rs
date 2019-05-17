#[derive(Default)]
pub struct AppData {
    pub title: String,
    pub description: String,
    pub author: String,
    pub counter1: i32,
    pub counter2: i32,
    pub counter3: i32,
}
impl AppData {
    ///constructor
    pub fn new() -> Self {
        //return from constructor
        Self {
            title: String::from("title"),
            description: String::from("description"),
            author: String::from("author"),
            counter1: 0,
            counter2: 0,
            counter3: 0,
        }
    }
}
