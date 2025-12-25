use std::error::Error;
use std::io::{self, Write};

pub struct UserConfig {
    pub host: String,
    pub username: String,
    pub list_name: String,
}

impl UserConfig {
    /// Создает конфигурацию из пользовательского ввода
    pub fn from_user_input() -> Result<Self, Box<dyn Error>> {
        let host = Self::read_input("Введите IP адрес MikroTik: ")?;
        let username = Self::read_input("Введите имя пользователя: ")?;
        let list_name = Self::read_input_with_default(
            "Введите имя address-list [RU]: ",
            "RU"
        )?;
        
        Ok(Self {
            host,
            username,
            list_name,
        })
    }
    
    /// Читает строку из stdin
    fn read_input(prompt: &str) -> Result<String, Box<dyn Error>> {
        print!("{}", prompt);
        io::stdout().flush()?;
        
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        
        Ok(input.trim().to_string())
    }
    
    /// Читает строку из stdin со значением по умолчанию
    fn read_input_with_default(prompt: &str, default: &str) -> Result<String, Box<dyn Error>> {
        let input = Self::read_input(prompt)?;
        
        Ok(if input.is_empty() {
            default.to_string()
        } else {
            input
        })
    }
}