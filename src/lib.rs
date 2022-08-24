use std::{process::Command, io::{self, stdin}};



struct Menu<'a> {
    items: Vec<&'a str>
}


impl Menu<'_>{
    pub fn new<'a>(items: Vec<&'a str>) -> Menu {
        Menu {  items }
    }

    fn print_items(self){
        for (i, el) in self.items.iter().enumerate() {
            println!("{}. {}", i + 1, el);
        }
    }

    // fn select_item<'a>(&self) -> u8{
    //     
    // }
}


#[cfg(test)]
mod tests {
    use crate::{Menu};

    #[test]
    fn print_menu_items() {
        let menu = Menu::new(vec!["first", "second", "third"]);
        menu.print_items();

    }

    // #[test]
    // fn make_selection() {
    //     let menu = Menu{
    //         items: vec!["1","2","3"],
    //         config: Config{
    //             selected_item_color: MenuColor::Red
    //         }
    //     };
    //     let selection = menu.select_item();
    // }
}
