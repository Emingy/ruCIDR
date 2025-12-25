use std::error::Error;
use std::net::Ipv4Addr;

pub struct CidrConverter;

impl CidrConverter {
    pub fn new() -> Self {
        Self
    }
    
    /// Конвертирует список подсетей в CIDR формат
    pub fn convert_subnets(&self, subnets: &[String]) -> Result<Vec<String>, Box<dyn Error>> {
        let mut all_cidrs = Vec::new();
        
        for subnet in subnets {
            if subnet.contains('/') {
                // Уже в CIDR формате
                all_cidrs.push(subnet.clone());
            } else if subnet.contains('-') {
                // Диапазон адресов - конвертируем в CIDR
                let parts: Vec<&str> = subnet.split('-').collect();
                if parts.len() == 2 {
                    match self.range_to_cidrs(parts[0], parts[1]) {
                        Ok(cidrs) => all_cidrs.extend(cidrs),
                        Err(e) => {
                            eprintln!("✗ Ошибка обработки диапазона {}: {}", subnet, e);
                        }
                    }
                }
            }
        }
        
        Ok(all_cidrs)
    }
    
    /// Конвертирует диапазон IP адресов в список CIDR блоков
    fn range_to_cidrs(&self, start_ip: &str, end_ip: &str) -> Result<Vec<String>, Box<dyn Error>> {
        let start: Ipv4Addr = start_ip.parse()?;
        let end: Ipv4Addr = end_ip.parse()?;
        
        let mut start_num = ip_to_u32(start);
        let end_num = ip_to_u32(end);
        
        let mut cidrs = Vec::new();
        
        while start_num <= end_num {
            let trailing_zeros = if start_num == 0 {
                32
            } else {
                start_num.trailing_zeros()
            };
            
            let remaining = end_num - start_num + 1;
            let max_prefix_by_alignment = 32 - trailing_zeros;
            let max_prefix_by_size = 32 - remaining.leading_zeros();
            let prefix_len = std::cmp::max(max_prefix_by_alignment, max_prefix_by_size);
            
            let cidr = format!("{}/{}", u32_to_ip(start_num), prefix_len);
            cidrs.push(cidr);
            
            let block_size = 1u64 << (32 - prefix_len);
            if block_size >= u64::from(u32::MAX) {
                break;
            }
            
            let next_start = (start_num as u64) + block_size;
            if next_start > u64::from(u32::MAX) {
                break;
            }
            
            start_num = next_start as u32;
            
            if start_num == 0 && end_num > 0 {
                break;
            }
        }
        
        Ok(cidrs)
    }
}

/// Конвертирует IP адрес в u32
fn ip_to_u32(ip: Ipv4Addr) -> u32 {
    u32::from(ip)
}

/// Конвертирует u32 обратно в IP адрес
fn u32_to_ip(num: u32) -> Ipv4Addr {
    Ipv4Addr::from(num)
}