use std::{process::Command, io::{self, stdin}};


struct Config {
    selected_item_color: MenuColor
}

enum MenuColor {
    Red,
    Green,
    White
}

struct Menu<'a> {
    config: Config,
    items: Vec<&'a str>
}


impl Menu<'_>{
    pub fn new<'a>(items: Vec<&'a str>, config: Config) -> Menu {
        Menu { config, items }
    }

    fn print_items(&self){
        for item in &self.items {
            println!("{}", item);
        }
    }

    // fn select_item<'a>(&self) -> u8{
    //     
    // }
}


#[cfg(test)]
mod tests {
    use crate::{Menu, Config, MenuColor};

    #[test]
    fn print_menu_items() {
        let menu = Menu::new(vec!["first", "second", "third"], Config{selected_item_color: MenuColor::Red});
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
