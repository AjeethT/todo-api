pub struct Todo{
    pub item_id:i32,
    pub item_desc:String,
    pub item_tags:String,
    pub is_active:bool,
    pub remainder_date:String,
    pub created_date:String,
    pub modified_date:String
}

pub trait TodoManager {
    fn create_item(&self);
//     fn get_item(id:i32) -> Todo;
//     fn get_all_items() -> Vec<Todo>;
}

impl TodoManager for Todo {
    // Implement the Todo trait methods here
    fn create_item(&self) {
        // Implement the logic to create a new todo item
        //print the new todo item details
        println!("New Todo Item: ID: {}, Description: {}, Tags: {}, Active: {}, Remainder Date: {}, Created Date: {}, Modified Date: {}", self.item_id, self.item_desc, self.item_tags, self.is_active, self.remainder_date, self.created_date, self.modified_date);
    }
    // fn get_item(id: i32) -> Todo {
    //     // Implement the logic to fetch a todo item by id
    // }
    // fn get_all_items() -> Vec<Todo> {
    //     // Implement the logic to fetch all todo items
    // }
}