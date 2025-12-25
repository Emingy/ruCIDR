mod clients;
mod utils;
mod adapters;
mod cli;

use std::error::Error;
use clients::ripe::RipeClient;
use utils::cidr::CidrConverter;
use adapters::mikrotik::MikrotikManager;
use cli::user_config::UserConfig;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    print_header();
    
    let config = UserConfig::from_user_input()?;
    
    println!("\n{}", "=".repeat(60));
    println!("Загрузка данных из RIPE...");
    println!("{}", "=".repeat(60));
    
    // Получаем данные из RIPE
    let ripe_client = RipeClient::new();
    let ipv4_subnets = ripe_client.fetch_russian_ips().await?;
    
    println!("\n✓ Найдено {} IPv4 записей", ipv4_subnets.len());
    println!("\nПреобразование в CIDR...");
    
    // Конвертируем в CIDR
    let converter = CidrConverter::new();
    let all_cidrs = converter.convert_subnets(&ipv4_subnets)?;
    
    println!("✓ Получено {} CIDR блоков", all_cidrs.len());
    
    // Добавляем в MikroTik
    let mikrotik = MikrotikManager::new(
        config.host,
        config.username,
        config.list_name,
    );
    
    mikrotik.update_address_list(&all_cidrs)?;
    
    Ok(())
}

fn print_header() {
    println!("{}", "=".repeat(60));
    println!("MikroTik Address-List Updater для российских IP");
    println!("{}", "=".repeat(60));
}