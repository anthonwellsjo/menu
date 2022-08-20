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
    pub fn new<'a>(items: Vec<&'a str>, config: Option<Config>) -> Menu {
        let config = match config {
            Some(config) => config,
            None => Config{ selected_item_color: MenuColor::White}
        };

        Menu { config, items }
    }

    fn print_items(&self){
        for item in &self.items {
            println!("{}", item);
        }
    }

    fn select_item<'a>(&self) -> u8{
        
    }
}


#[cfg(test)]
mod tests {
    use crate::{Menu, Config, MenuColor};

    #[test]
    fn make_selection() {
        let menu = Menu{
            items: vec!["1","2","3"],
            config: Config{
                selected_item_color: MenuColor::Red
            }
        };
        let selection = menu.select_item();
    }
}
