use std::error::Error;
use std::io::Write;
use std::process::Command;
use std::thread::sleep;
use std::time::Duration;

const BATCH_SIZE: usize = 500;
const PAUSE_SECONDS: u64 = 30;

pub struct MikrotikManager {
    host: String,
    username: String,
    list_name: String,
}

impl MikrotikManager {
    pub fn new(host: String, username: String, list_name: String) -> Self {
        Self {
            host,
            username,
            list_name,
        }
    }
    
    /// Обновляет address-list на MikroTik
    pub fn update_address_list(&self, cidrs: &[String]) -> Result<(), Box<dyn Error>> {
        self.print_connection_info();
        self.clear_address_list()?;
        self.add_cidrs_in_batches(cidrs)?;
        self.print_completion(cidrs.len());
        
        Ok(())
    }
    
    fn print_connection_info(&self) {
        println!("\n{}", "=".repeat(60));
        println!("Подключение к MikroTik: {}@{}", self.username, self.host);
        println!("Добавление по {} CIDR каждые {} секунд", BATCH_SIZE, PAUSE_SECONDS);
        println!("{}", "=".repeat(60));
    }
    
    fn print_completion(&self, total: usize) {
        println!("\n{}", "=".repeat(60));
        println!("✓ Завершено!");
        println!("  Всего добавлено {} CIDR блоков", total);
        println!("{}", "=".repeat(60));
    }
    
    /// Очищает существующий address-list
    fn clear_address_list(&self) -> Result<(), Box<dyn Error>> {
        println!("Очистка address-list `{}`...", self.list_name);
        
        let clear_cmd = format!(
            "/ip firewall address-list remove [find list=\"{}\"]\n",
            self.list_name
        );
        
        let status = self.execute_ssh_command(&clear_cmd)?;
        
        if !status.success() {
            return Err("Не удалось очистить address-list".into());
        }
        
        println!("✓ Address-list очищен");
        Ok(())
    }
    
    /// Добавляет CIDR блоки батчами
    fn add_cidrs_in_batches(&self, cidrs: &[String]) -> Result<(), Box<dyn Error>> {
        let total = cidrs.len();
        let batches = (total + BATCH_SIZE - 1) / BATCH_SIZE;
        
        for (i, chunk) in cidrs.chunks(BATCH_SIZE).enumerate() {
            let batch_number = i + 1;
            let start = i * BATCH_SIZE + 1;
            let end = start + chunk.len() - 1;
            
            println!(
                "\n▶ Батч {}/{} (CIDR {}–{})",
                batch_number, batches, start, end
            );
            
            self.add_batch(chunk, batch_number)?;
            
            println!("✓ Батч {} применён ({} CIDR)", batch_number, chunk.len());
            
            if batch_number < batches {
                println!("⏳ Ожидание {} секунд...", PAUSE_SECONDS);
                sleep(Duration::from_secs(PAUSE_SECONDS));
            }
        }
        
        Ok(())
    }
    
    /// Добавляет один батч CIDR блоков
    fn add_batch(&self, cidrs: &[String], batch_number: usize) -> Result<(), Box<dyn Error>> {
        let commands = self.build_add_commands(cidrs);
        
        let mut child = Command::new("ssh")
            .arg("-o")
            .arg("BatchMode=yes")
            .arg("-o")
            .arg("StrictHostKeyChecking=no")
            .arg(format!("{}@{}", self.username, self.host))
            .stdin(std::process::Stdio::piped())
            .stdout(std::process::Stdio::piped())
            .stderr(std::process::Stdio::piped())
            .spawn()?;
        
        if let Some(ref mut stdin) = child.stdin {
            stdin.write_all(commands.as_bytes())?;
        }
        
        let output = child.wait_with_output()?;
        
        if !output.status.success() {
            eprintln!("⚠ Ошибка в батче {}", batch_number);
            eprintln!("{}", String::from_utf8_lossy(&output.stderr));
            return Err("Ошибка добавления CIDR".into());
        }
        
        Ok(())
    }
    
    /// Строит команды для добавления CIDR блоков
    fn build_add_commands(&self, cidrs: &[String]) -> String {
        let mut commands = String::new();
        
        for cidr in cidrs {
            commands.push_str(&format!(
                "/ip firewall address-list add list=\"{}\" address=\"{}\" comment=\"Auto-generated from RIPE\"\n",
                self.list_name, cidr
            ));
        }
        
        commands
    }
    
    /// Выполняет SSH команду на MikroTik
    fn execute_ssh_command(&self, command: &str) -> Result<std::process::ExitStatus, Box<dyn Error>> {
        let status = Command::new("ssh")
            .arg("-o")
            .arg("BatchMode=yes")
            .arg("-o")
            .arg("StrictHostKeyChecking=no")
            .arg(format!("{}@{}", self.username, self.host))
            .arg(command)
            .status()?;
        
        Ok(status)
    }
}