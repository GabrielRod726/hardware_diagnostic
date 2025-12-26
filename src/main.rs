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

//! Programa principal de diagnóstico de hardware
//! 
//! Coleta e exibe informações detalhadas sobre CPU, RAM e discos
//! em sistemas Windows, incluindo pontuação de desempenho.

pub mod engine;

use engine::utils;
use engine::{calculate_performance_score, display_performance_score, PerformanceCategory};
use std::{io, fs};

fn main() {
    println!("{}", "=".repeat(60));
    println!("           🖥️  DIAGNÓSTICO DE HARDWARE           ");
    println!("{}", "=".repeat(60));
    
    // Coletar informações
    let cpu = engine::cpu_info();
    let ram = engine::ram_info();
    let disks = engine::disk_info();
    
    // Calcular pontuação de desempenho
    let performance_score = calculate_performance_score();
    
    // Exibir informações básicas
    println!("\n📋 RESUMO DO SISTEMA:");
    println!("{}", "-".repeat(40));
    println!("• CPU: {} ({:.1}% uso)", cpu.name, cpu.cpu_usage);
    println!("• Núcleos: {} lógicos, {} físicos", 
        cpu.number_cpus, 
        cpu.physical_cores.unwrap_or(0)
    );
    
    println!("• RAM: {:.1} GB / {:.1} GB ({:.1}% usado)", 
        utils::bytes_to_gb_f64(ram.used_ram),
        utils::bytes_to_gb_f64(ram.total_ram),
        ram.ram_usage_percent
    );
    
    println!("• Discos: {} volume(s) encontrado(s)", disks.len());
    for disk in &disks {
        println!("  → {}: {:.1} GB livre ({:.1}% usado)", 
            disk.name,
            utils::bytes_to_gb_f64(disk.available_space),
            disk.usage_percent
        );
    }
    
    // Exibir pontuação de desempenho
    println!("\n{}", display_performance_score(&performance_score));
    
    // Decisão baseada na pontuação
    println!("{}", "=".repeat(60));
    println!("           🎯 DECISÃO RECOMENDADA           ");
    println!("{}", "=".repeat(60));
    
    match performance_score.category {
        PerformanceCategory::Descarte => {
            println!("🚨 AÇÃO RECOMENDADA: DESCARTE/UPGRADE COMPLETO");
            println!("• Justificativa: Pontuação muito baixa ({:.1}/10)", performance_score.overall_score);
            println!("• Risco: Alto risco de falhas e baixa produtividade");
            println!("• Prazo: Imediato");
        }
        PerformanceCategory::Manutencao => {
            println!("⚠️ AÇÃO RECOMENDADA: MANUTENÇÃO URGENTE");
            println!("• Justificativa: Pontuação baixa ({:.1}/10)", performance_score.overall_score);
            println!("• Risco: Problemas de desempenho frequentes");
            println!("• Prazo: Dentro de 1-2 semanas");
        }
        PerformanceCategory::Precaução => {
            println!("🔶 AÇÃO RECOMENDADA: USO COM PRECAUÇÃO");
            println!("• Justificativa: Pontuação moderada ({:.1}/10)", performance_score.overall_score);
            println!("• Risco: Possíveis problemas sob carga pesada");
            println!("• Prazo: Monitoramento constante");
        }
        PerformanceCategory::BomEstado => {
            println!("✅ AÇÃO RECOMENDADA: USO NORMAL");
            println!("• Justificativa: Pontuação boa ({:.1}/10)", performance_score.overall_score);
            println!("• Risco: Baixo, para uso padrão");
            println!("• Prazo: Manutenção preventiva regular");
        }
    }
    
    // Timestamp e opções de salvamento
    let timestamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs();
    
    println!("\n{}", "=".repeat(60));
    println!("Relatório gerado em: {}", timestamp);
    
    // Opção: Salvar relatório completo
    let args: Vec<String> = std::env::args().collect();
    if args.len() > 1 && (args[1] == "--save" || args[1] == "-s") {
        let filename = format!("diagnostico_{}.txt", timestamp);
        let full_report = utils::generate_complete_report();
        
        match std::fs::write(&filename, full_report) {
            Ok(_) => println!("📄 Relatório salvo em: {}", filename),
            Err(e) => eprintln!("❌ Erro ao salvar relatório: {}", e),
        }
    }
    
    println!("{}", "=".repeat(60));
    
    // Opção: Exibir relatório completo
    if args.len() > 1 && (args[1] == "--full" || args[1] == "-f") {
        println!("\n{}", "=".repeat(60));
        println!("           📄 RELATÓRIO COMPLETO           ");
        println!("{}", "=".repeat(60));
        println!("{}", utils::generate_complete_report());
        utils::write_report();
    }
}

