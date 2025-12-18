// hardware-diagnostic - Ferramenta de diagnóstico de hardware
// Copyright (C) 2025  Seu Nome
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

//! Módulo `engine` - Coleta e estrutura informações do sistema
//! 
//! Este módulo fornece funcionalidades para coletar informações de hardware
//! como CPU, RAM e discos de armazenamento no Windows usando a crate `sysinfo`.

use sysinfo::{System, Disks};

/// Representa as informações coletadas da CPU do sistema
#[derive(Debug, Clone)]
pub struct CpuInfo {
    /// Número total de CPUs/cores lógicos detectados
    pub number_cpus: usize,
    /// Percentual de uso total da CPU (0.0 a 100.0)
    pub cpu_usage: f32,
    /// Frequência atual da CPU em MHz
    pub frequency: u64,
    /// Nome/modelo da CPU
    pub name: String,
    /// Número de núcleos físicos (se disponível)
    pub physical_cores: Option<usize>,
}

/// Representa as informações coletadas da memória RAM
#[derive(Debug, Clone)]
pub struct RamInfo {
    /// Memória RAM total em bytes
    pub total_ram: u64,
    /// Memória RAM usada em bytes
    pub used_ram: u64,
    /// Memória RAM livre em bytes
    pub free_ram: u64,
    /// Memória SWAP total em bytes
    pub total_swap: u64,
    /// Memória SWAP usada em bytes
    pub used_swap: u64,
    /// Percentual de uso da RAM (0.0 a 100.0)
    pub ram_usage_percent: f64,
    /// Percentual de uso do SWAP (0.0 a 100.0)
    pub swap_usage_percent: f64,
}

/// Representa informações de um disco individual
#[derive(Debug, Clone)]
pub struct DiskInfo {
    /// Nome do dispositivo (ex: "C:")
    pub name: String,
    /// Ponto de montagem (ex: "C:\")
    pub mount_point: String,
    /// Espaço total em bytes
    pub total_space: u64,
    /// Espaço disponível em bytes
    pub available_space: u64,
    /// Espaço usado em bytes (calculado)
    pub used_space: u64,
    /// Percentual de uso (0.0 a 100.0)
    pub usage_percent: f64,
    /// Sistema de arquivos (ex: "NTFS")
    pub file_system: String,
    /// Tipo de disco
    pub disk_type: String,
}

/// Representa a pontuação de desempenho da máquina
#[derive(Debug, Clone)]
pub struct PerformanceScore {
    /// Pontuação geral (0.0 a 10.0)
    pub overall_score: f64,
    /// Pontuação da CPU (0.0 a 10.0)
    pub cpu_score: f64,
    /// Pontuação da RAM (0.0 a 10.0)
    pub ram_score: f64,
    /// Pontuação dos discos (0.0 a 10.0)
    pub disk_score: f64,
    /// Categoria de desempenho
    pub category: PerformanceCategory,
    /// Recomendações específicas
    pub recommendations: Vec<String>,
}

/// Categorias de desempenho da máquina
#[derive(Debug, Clone, PartialEq)]
pub enum PerformanceCategory {
    /// 1-2 pontos: Descarte ou upgrade completo necessário
    Descarte,
    /// 3-4 pontos: Manutenção urgente necessária
    Manutencao,
    /// 5-6 pontos: Uso com precaução/monitoramento
    Precaução,
    /// 7+ pontos: Máquina em bom estado de uso
    BomEstado,
}

impl PerformanceCategory {
    /// Retorna a descrição da categoria
    pub fn description(&self) -> &str {
        match self {
            PerformanceCategory::Descarte => "DESCARTE - Upgrade completo necessário",
            PerformanceCategory::Manutencao => "MANUTENÇÃO URGENTE - Requer ações corretivas",
            PerformanceCategory::Precaução => "USO COM PRECAUÇÃO - Monitorar constantemente",
            PerformanceCategory::BomEstado => "BOM ESTADO - Adequado para uso normal",
        }
    }
    
    /// Retorna a cor ANSI para exibição (opcional)
    pub fn color_code(&self) -> &str {
        match self {
            PerformanceCategory::Descarte => "\x1b[31m", // Vermelho
            PerformanceCategory::Manutencao => "\x1b[33m", // Amarelo
            PerformanceCategory::Precaução => "\x1b[93m", // Amarelo claro
            PerformanceCategory::BomEstado => "\x1b[32m", // Verde
        }
    }
    
    /// Retorna o código de reset ANSI
    pub fn reset_color() -> &'static str {
        "\x1b[0m"
    }
}

/// Coleta informações detalhadas da CPU
/// 
/// # Retorno
/// Retorna uma instância de `CpuInfo` com:
/// - Número de CPUs/cores lógicos
/// - Percentual de uso atual
/// - Frequência em MHz
/// - Nome do modelo
/// - Contagem de núcleos físicos
/// 
/// # Exemplo
/// ```
/// let cpu_info = cpu_info();
/// println!("CPU: {}", cpu_info.name);
/// println!("Uso: {:.1}%", cpu_info.cpu_usage);
/// ```
pub fn cpu_info() -> CpuInfo {
    // Cria uma nova instância do System
    let mut sys = System::new();
    
    // Atualiza apenas as informações da CPU
    sys.refresh_cpu();
    
    // Aguarda um breve período para medição precisa do uso
    std::thread::sleep(std::time::Duration::from_millis(500));
    sys.refresh_cpu();
    
    // Obtém informações dos CPUs
    let cpus = sys.cpus();
    
    // Calcula uso médio de todos os cores
    let total_usage: f32 = cpus.iter().map(|cpu| cpu.cpu_usage()).sum();
    let avg_usage = if !cpus.is_empty() {
        total_usage / cpus.len() as f32
    } else {
        0.0
    };
    
    // Obtém informações do primeiro CPU para nome e frequência
    let cpu_name = if let Some(first_cpu) = cpus.first() {
        first_cpu.brand().to_string()
    } else {
        "Desconhecido".to_string()
    };
    
    let cpu_frequency = if let Some(first_cpu) = cpus.first() {
        first_cpu.frequency()
    } else {
        0
    };
    
    CpuInfo {
        number_cpus: cpus.len(),
        cpu_usage: avg_usage,
        frequency: cpu_frequency,
        name: cpu_name,
        physical_cores: sys.physical_core_count(),
    }
}

/// Coleta informações detalhadas da memória RAM e SWAP
/// 
/// # Retorno
/// Retorna uma instância de `RamInfo` com:
/// - Totais e usos de RAM e SWAP em bytes
/// - Percentuais de uso calculados
/// 
/// # Exemplo
/// ```
/// let ram_info = ram_info();
/// println!("RAM: {:.1} GB / {:.1} GB", 
///     bytes_to_gb(ram_info.used_ram),
///     bytes_to_gb(ram_info.total_ram)
/// );
/// ```
pub fn ram_info() -> RamInfo {
    let mut sys = System::new();
    
    // Atualiza informações de memória
    sys.refresh_memory();
    
    let total_ram = sys.total_memory();
    let used_ram = sys.used_memory();
    let free_ram = sys.free_memory();
    let total_swap = sys.total_swap();
    let used_swap = sys.used_swap();
    
    // Calcula percentuais de uso
    let ram_usage_percent = if total_ram > 0 {
        (used_ram as f64 / total_ram as f64) * 100.0
    } else {
        0.0
    };
    
    let swap_usage_percent = if total_swap > 0 {
        (used_swap as f64 / total_swap as f64) * 100.0
    } else {
        0.0
    };
    
    RamInfo {
        total_ram,
        used_ram,
        free_ram,
        total_swap,
        used_swap,
        ram_usage_percent,
        swap_usage_percent,
    }
}

/// Coleta informações de todos os discos do sistema
/// 
/// # Retorno
/// Retorna um vetor contendo `DiskInfo` para cada disco encontrado
/// 
/// # Exemplo
/// ```
/// let disks = disk_info();
/// for disk in disks {
///     println!("Disco {}: {:.1} GB livre", 
///         disk.name, 
///         bytes_to_gb(disk.available_space)
///     );
/// }
/// ```
pub fn disk_info() -> Vec<DiskInfo> {
    // Cria uma lista atualizada de discos
    let disks = Disks::new_with_refreshed_list();
    let mut disk_info_list = Vec::new();
    
    for disk in &disks {
        let total_space = disk.total_space();
        let available_space = disk.available_space();
        let used_space = total_space - available_space;
        let usage_percent = if total_space > 0 {
            (used_space as f64 / total_space as f64) * 100.0
        } else {
            0.0
        };
        
        // Converte &OsStr para String usando to_string_lossy
        let file_system = disk.file_system()
            .to_string_lossy()
            .to_string();
        
        disk_info_list.push(DiskInfo {
            name: disk.name().to_string_lossy().to_string(),
            mount_point: disk.mount_point().to_string_lossy().to_string(),
            total_space,
            available_space,
            used_space,
            usage_percent,
            file_system,
            disk_type: format!("{:?}", disk.kind()),
        });
    }
    
    disk_info_list
}

/// Calcula a pontuação de desempenho da máquina
/// 
/// # Retorno
/// Retorna uma instância de `PerformanceScore` com:
/// - Pontuações individuais e geral
/// - Categoria de desempenho
/// - Recomendações específicas
/// 
/// # Exemplo
/// ```
/// let score = calculate_performance_score();
/// println!("Pontuação: {:.1}/10 - {}", score.overall_score, score.category);
/// ```
pub fn calculate_performance_score() -> PerformanceScore {
    let cpu_info = cpu_info();
    let ram_info = ram_info();
    let disks_info = disk_info();
    
    // 1. PONTUAÇÃO DA CPU (0-10)
    let cpu_score = calculate_cpu_score(&cpu_info);
    
    // 2. PONTUAÇÃO DA RAM (0-10)
    let ram_score = calculate_ram_score(&ram_info);
    
    // 3. PONTUAÇÃO DOS DISCOS (0-10)
    let disk_score = calculate_disk_score(&disks_info);
    
    // 4. PONTUAÇÃO GERAL (média ponderada)
    let overall_score = cpu_score * 0.4 + ram_score * 0.3 + disk_score * 0.3;
    
    // 5. DETERMINAR CATEGORIA
    let category = determine_category(overall_score);
    
    // 6. GERAR RECOMENDAÇÕES
    let recommendations = generate_recommendations(&cpu_info, &ram_info, &disks_info, overall_score);
    
    PerformanceScore {
        overall_score,
        cpu_score,
        ram_score,
        disk_score,
        category,
        recommendations,
    }
}

/// Calcula a pontuação da CPU baseada em múltiplos fatores
fn calculate_cpu_score(cpu_info: &CpuInfo) -> f64 {
    let score: f64; // Declare sem valor inicial
    
    // Fator 1: Número de núcleos
    let cores_score = match cpu_info.number_cpus {
        0..=1 => 2.0,  // Muito baixo
        2 => 4.0,      // Baixo
        3..=4 => 6.0,  // Médio
        5..=8 => 8.0,  // Bom
        _ => 10.0,     // Excelente
    };
    
    // Fator 2: Uso atual da CPU (quanto menor o uso, melhor)
    let usage_score = if cpu_info.cpu_usage < 30.0 {
        10.0 // Excelente (baixo uso)
    } else if cpu_info.cpu_usage < 60.0 {
        7.0  // Bom
    } else if cpu_info.cpu_usage < 85.0 {
        4.0  // Regular
    } else {
        1.0  // Crítico
    };
    
    // Fator 3: Frequência da CPU (quanto maior, melhor)
    let freq_score = if cpu_info.frequency < 2000 {
        3.0  // Muito baixa
    } else if cpu_info.frequency < 3000 {
        6.0  // Baixa
    } else if cpu_info.frequency < 4000 {
        8.0  // Boa
    } else {
        10.0 // Excelente
    };
    
    // Média dos fatores com pesos
    score = cores_score * 0.4 + usage_score * 0.4 + freq_score * 0.2;
    
    // Garante entre 0 e 10
    if score < 0.0 {
        0.0
    } else if score > 10.0 {
        10.0
    } else {
        score
    }
}

/// Calcula a pontuação da RAM
fn calculate_ram_score(ram_info: &RamInfo) -> f64 {
    let score: f64;
    
    // Fator 1: Uso da RAM (quanto menor, melhor)
    let ram_usage_score = if ram_info.ram_usage_percent < 60.0 {
        10.0 // Excelente
    } else if ram_info.ram_usage_percent < 75.0 {
        7.0  // Bom
    } else if ram_info.ram_usage_percent < 90.0 {
        4.0  // Regular
    } else {
        1.0  // Crítico
    };
    
    // Fator 2: Uso do SWAP (quanto menor, melhor)
    let swap_score = if ram_info.total_swap == 0 {
        8.0 // Sem SWAP configurado (neutro)
    } else if ram_info.swap_usage_percent < 10.0 {
        10.0 // Excelente
    } else if ram_info.swap_usage_percent < 30.0 {
        7.0  // Bom
    } else if ram_info.swap_usage_percent < 50.0 {
        4.0  // Regular
    } else {
        1.0  // Crítico (muito uso de SWAP)
    };
    
    // Fator 3: Quantidade total de RAM
    let total_ram_gb = ram_info.total_ram as f64 / 1_073_741_824.0;
    let capacity_score = if total_ram_gb < 4.0 {
        3.0  // Muito baixa
    } else if total_ram_gb < 8.0 {
        6.0  // Baixa
    } else if total_ram_gb < 16.0 {
        8.0  // Boa
    } else {
        10.0 // Excelente
    };
    
    score = ram_usage_score * 0.5 + swap_score * 0.3 + capacity_score * 0.2;
    
    // Garante entre 0 e 10
    if score < 0.0 {
        0.0
    } else if score > 10.0 {
        10.0
    } else {
        score
    }
}

/// Calcula a pontuação dos discos
fn calculate_disk_score(disks: &[DiskInfo]) -> f64 {
    if disks.is_empty() {
        return 5.0; // Pontuação neutra se não houver discos
    }
    
    let mut total_score = 0.0;
    let mut count = 0;
    
    for disk in disks {
        let disk_score: f64;
        
        // Fator 1: Uso do disco (quanto menor, melhor)
        let usage_score = if disk.usage_percent < 70.0 {
            10.0 // Excelente
        } else if disk.usage_percent < 85.0 {
            7.0  // Bom
        } else if disk.usage_percent < 95.0 {
            4.0  // Regular
        } else {
            1.0  // Crítico
        };
        
        // Fator 2: Tipo de disco
        let type_score = if disk.disk_type.contains("SSD") || disk.disk_type.contains("NVMe") {
            10.0 // SSD (rápido)
        } else if disk.disk_type.contains("HDD") {
            6.0  // HDD (lento)
        } else {
            8.0  // Outro/desconhecido
        };
        
        // Fator 3: Espaço livre
        let free_gb = disk.available_space as f64 / 1_000_000_000.0;
        let free_space_score = if free_gb > 100.0 {
            10.0 // Excelente
        } else if free_gb > 50.0 {
            8.0  // Bom
        } else if free_gb > 20.0 {
            6.0  // Regular
        } else if free_gb > 10.0 {
            4.0  // Baixo
        } else {
            1.0  // Crítico
        };
        
        disk_score = usage_score * 0.5 + type_score * 0.3 + free_space_score * 0.2;
        
        // Garante entre 0 e 10
        let clamped_score = if disk_score < 0.0 {
            0.0
        } else if disk_score > 10.0 {
            10.0
        } else {
            disk_score
        };
        
        total_score += clamped_score;
        count += 1;
    }
    
    if count > 0 {
        total_score / count as f64
    } else {
        5.0
    }
}

/// Determina a categoria baseada na pontuação geral
fn determine_category(score: f64) -> PerformanceCategory {
    match score {
        s if s < 3.0 => PerformanceCategory::Descarte,     // 0-2.9: Descarte
        s if s < 5.0 => PerformanceCategory::Manutencao,   // 3-4.9: Manutenção
        s if s < 7.0 => PerformanceCategory::Precaução,    // 5-6.9: Precaução
        _ => PerformanceCategory::BomEstado,               // 7+: Bom estado
    }
}

/// Gera recomendações baseadas no estado da máquina
fn generate_recommendations(
    cpu_info: &CpuInfo,
    ram_info: &RamInfo,
    disks: &[DiskInfo],
    overall_score: f64,
) -> Vec<String> {
    let mut recommendations = Vec::new();
    
    // Recomendações baseadas na pontuação geral
    if overall_score < 3.0 {
        recommendations.push("🛑 CONSIDERE DESCARTE: A máquina está em estado crítico".to_string());
        recommendations.push("💡 Sugestão: Upgrade completo ou substituição do equipamento".to_string());
    } else if overall_score < 5.0 {
        recommendations.push("⚠️ MANUTENÇÃO URGENTE: A máquina requer intervenção imediata".to_string());
    } else if overall_score < 7.0 {
        recommendations.push("🔶 USO COM PRECAUÇÃO: Monitore o desempenho regularmente".to_string());
    } else {
        recommendations.push("✅ BOM ESTADO: A máquina está adequada para uso normal".to_string());
    }
    
    // Recomendações específicas para CPU
    if cpu_info.cpu_usage > 80.0 {
        recommendations.push("🔴 CPU: Uso muito alto. Verifique processos desnecessários".to_string());
    }
    if cpu_info.number_cpus < 2 {
        recommendations.push("🟡 CPU: Apenas 1 núcleo detectado. Limitação para multitarefa".to_string());
    }
    
    // Recomendações específicas para RAM
    if ram_info.ram_usage_percent > 85.0 {
        recommendations.push("🔴 RAM: Uso acima de 85%. Considere adicionar mais memória".to_string());
    }
    if ram_info.total_ram < 4 * 1024 * 1024 * 1024 { // Menos de 4GB
        recommendations.push("🟡 RAM: Memória insuficiente para sistemas modernos".to_string());
    }
    if ram_info.swap_usage_percent > 50.0 {
        recommendations.push("🔴 SWAP: Uso excessivo de memória virtual. Otimize a RAM".to_string());
    }
    
    // Recomendações específicas para discos
    for disk in disks {
        if disk.usage_percent > 90.0 {
            recommendations.push(format!("🔴 DISCO {}: Capacidade quase esgotada ({:.1}%)", 
                disk.name, disk.usage_percent));
        }
        if disk.disk_type.contains("HDD") && overall_score < 7.0 {
            recommendations.push(format!("🟡 DISCO {}: HDD pode estar limitando performance", 
                disk.name));
        }
        if disk.available_space as f64 / 1_000_000_000.0 < 10.0 {
            recommendations.push(format!("🔴 DISCO {}: Menos de 10GB livres", disk.name));
        }
    }
    
    // Recomendação final baseada na categoria
    match determine_category(overall_score) {
        PerformanceCategory::Descarte => {
            recommendations.push("📋 Ação recomendada: Substituir equipamento".to_string());
        }
        PerformanceCategory::Manutencao => {
            recommendations.push("📋 Ação recomendada: Manutenção técnica urgente".to_string());
        }
        PerformanceCategory::Precaução => {
            recommendations.push("📋 Ação recomendada: Monitoramento contínuo".to_string());
        }
        PerformanceCategory::BomEstado => {
            recommendations.push("📋 Ação recomendada: Manutenção preventiva regular".to_string());
        }
    }
    
    recommendations
}

/// Exibe a pontuação de forma formatada
pub fn display_performance_score(score: &PerformanceScore) -> String {
    let mut output = String::new();
    
    output.push_str(&format!("{}\n", "=".repeat(60)));
    output.push_str("           📊 PONTUAÇÃO DE DESEMPENHO DA MÁQUINA           \n");
    output.push_str(&format!("{}\n\n", "=".repeat(60)));
    
    // Barra de pontuação visual
    let bar_width = 40;
    let filled = ((score.overall_score / 10.0) * bar_width as f64).round() as usize;
    let empty = bar_width - filled;
    
    output.push_str(&format!("PONTUAÇÃO GERAL: {:.1}/10.0\n", score.overall_score));
    output.push_str(&format!("[{}{}]\n\n", "█".repeat(filled), "░".repeat(empty)));
    
    // Categoria com cor (opcional)
    output.push_str(&format!("CATEGORIA: {}{}{}\n\n", 
        score.category.color_code(),
        score.category.description(),
        PerformanceCategory::reset_color()
    ));
    
    // Pontuações detalhadas
    output.push_str("PONTUAÇÕES DETALHADAS:\n");
    output.push_str(&format!("  • CPU:      {:.1}/10.0\n", score.cpu_score));
    output.push_str(&format!("  • RAM:      {:.1}/10.0\n", score.ram_score));
    output.push_str(&format!("  • Discos:   {:.1}/10.0\n\n", score.disk_score));
    
    // Legenda das categorias
    output.push_str("LEGENDA DAS CATEGORIAS:\n");
    output.push_str("  1-2  → DESCARTE/UPGRADE COMPLETO\n");
    output.push_str("  3-4  → MANUTENÇÃO URGENTE\n");
    output.push_str("  5-6  → USO COM PRECAUÇÃO\n");
    output.push_str("  7-10 → BOM ESTADO DE USO\n\n");
    
    // Recomendações
    if !score.recommendations.is_empty() {
        output.push_str("RECOMENDAÇÕES:\n");
        for (i, rec) in score.recommendations.iter().enumerate() {
            output.push_str(&format!("  {}. {}\n", i + 1, rec));
        }
    }
    
    output
}

/// Funções utilitárias para formatação de dados
pub mod utils {
    use super::*;
    
    /// Converte bytes para gigabytes com formatação
    /// 
    /// # Argumentos
    /// * `bytes` - Quantidade em bytes
    /// 
    /// # Retorno
    /// String formatada em GB com 2 casas decimais
    pub fn bytes_to_gb(bytes: u64) -> String {
        format!("{:.2}", bytes as f64 / 1_000_000_000.0)
    }
    
    /// Converte bytes para gigabytes como valor numérico
    pub fn bytes_to_gb_f64(bytes: u64) -> f64 {
        bytes as f64 / 1_000_000_000.0
    }
    
    /// Formata uma barra de progresso ASCII para representar percentuais
    /// 
    /// # Argumentos
    /// * `percent` - Percentual (0.0 a 100.0)
    /// * `width` - Largura da barra em caracteres
    /// 
    /// # Retorno
    /// String representando a barra de progresso
    pub fn progress_bar(percent: f64, width: usize) -> String {
        let filled = ((percent / 100.0) * width as f64).round() as usize;
        let empty = width.saturating_sub(filled);
        
        format!("[{}{}]", "█".repeat(filled), " ".repeat(empty))
    }
    
    /// Gera um relatório formatado de informações do sistema
    pub fn generate_report() -> String {
        let cpu = cpu_info();
        let ram = ram_info();
        let disks = disk_info();
        
        let mut report = String::new();
        
        // Seção CPU
        report.push_str("=== INFORMACOES DA CPU ===\n");
        report.push_str(&format!("Modelo: {}\n", cpu.name));
        report.push_str(&format!("Núcleos lógicos: {}\n", cpu.number_cpus));
        if let Some(physical) = cpu.physical_cores {
            report.push_str(&format!("Núcleos físicos: {}\n", physical));
        }
        report.push_str(&format!("Frequência: {} MHz\n", cpu.frequency));
        report.push_str(&format!("Uso atual: {:.1}%\n", cpu.cpu_usage));
        report.push_str(&format!("Barra: {}\n\n", progress_bar(cpu.cpu_usage as f64, 20)));
        
        // Seção Memória
        report.push_str("=== INFORMACOES DE MEMORIA ===\n");
        report.push_str(&format!("RAM Total: {} GB\n", bytes_to_gb(ram.total_ram)));
        report.push_str(&format!("RAM Usada: {} GB ({:.1}%)\n", 
            bytes_to_gb(ram.used_ram), ram.ram_usage_percent));
        report.push_str(&format!("RAM Livre: {} GB\n", bytes_to_gb(ram.free_ram)));
        report.push_str(&format!("Barra: {}\n", progress_bar(ram.ram_usage_percent, 20)));
        
        if ram.total_swap > 0 {
            report.push_str(&format!("\nSWAP Total: {} GB\n", bytes_to_gb(ram.total_swap)));
            report.push_str(&format!("SWAP Usado: {} GB ({:.1}%)\n", 
                bytes_to_gb(ram.used_swap), ram.swap_usage_percent));
        }
        report.push_str("\n");
        
        // Seção Discos
        report.push_str("=== INFORMACOES DE ARMAZENAMENTO ===\n");
        if disks.is_empty() {
            report.push_str("Nenhum disco encontrado.\n");
        } else {
            for (i, disk) in disks.iter().enumerate() {
                report.push_str(&format!("\nDisco {}:\n", i + 1));
                report.push_str(&format!("  Nome: {}\n", disk.name));
                report.push_str(&format!("  Ponto de montagem: {}\n", disk.mount_point));
                report.push_str(&format!("  Sistema de arquivos: {}\n", disk.file_system));
                report.push_str(&format!("  Tipo: {}\n", disk.disk_type));
                report.push_str(&format!("  Capacidade: {} GB\n", bytes_to_gb(disk.total_space)));
                report.push_str(&format!("  Usado: {} GB\n", bytes_to_gb(disk.used_space)));
                report.push_str(&format!("  Livre: {} GB\n", bytes_to_gb(disk.available_space)));
                report.push_str(&format!("  Uso: {:.1}%\n", disk.usage_percent));
                report.push_str(&format!("  Barra: {}\n", progress_bar(disk.usage_percent, 20)));
            }
        }
        
        report
    }
    
    /// Gera um relatório completo incluindo a pontuação de desempenho
    pub fn generate_complete_report() -> String {
        let mut report = generate_report(); // Relatório original
        report.push_str("\n");
        report.push_str(&display_performance_score(&calculate_performance_score()));
        report
    }
}