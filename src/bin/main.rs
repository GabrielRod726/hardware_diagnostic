// hardware-diagnostic - Ferramenta de diagnóstico de hardware
// Copyright (C) 2023  Seu Nome
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

//! # Hardware Diagnostic - Aplicação CLI
//! 
//! Aplicação de linha de comando para diagnóstico de hardware.

use hardware_diagnostic::engine::utils;
use hardware_diagnostic::{calculate_performance_score, display_performance_score, PerformanceCategory};
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    
    println!("{}", "=".repeat(60));
    println!("           🖥️  DIAGNÓSTICO DE HARDWARE - WINDOWS           ");
    println!("{}", "=".repeat(60));
    
    // Pontuação de desempenho
    let performance_score = calculate_performance_score();
    println!("\n{}", display_performance_score(&performance_score));
    
    // Decisão recomendada
    println!("{}", "=".repeat(60));
    println!("           🎯 DECISÃO RECOMENDADA           ");
    println!("{}", "=".repeat(60));
    
    match performance_score.category {
        PerformanceCategory::Descarte => {
            println!("🚨 AÇÃO RECOMENDADA: DESCARTE/UPGRADE COMPLETO");
            println!("• Pontuação: {:.1}/10", performance_score.overall_score);
        }
        PerformanceCategory::Manutencao => {
            println!("⚠️ AÇÃO RECOMENDADA: MANUTENÇÃO URGENTE");
            println!("• Pontuação: {:.1}/10", performance_score.overall_score);
        }
        PerformanceCategory::Precaução => {
            println!("🔶 AÇÃO RECOMENDADA: USO COM PRECAUÇÃO");
            println!("• Pontuação: {:.1}/10", performance_score.overall_score);
        }
        PerformanceCategory::BomEstado => {
            println!("✅ AÇÃO RECOMENDADA: USO NORMAL");
            println!("• Pontuação: {:.1}/10", performance_score.overall_score);
        }
    }
    
    // Salvamento de relatório
    if args.len() > 1 && (args[1] == "--save" || args[1] == "-s") {
        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();
        
        let filename = format!("diagnostico_{}.txt", timestamp);
        let full_report = utils::generate_complete_report();
        
        if let Err(e) = std::fs::write(&filename, full_report) {
            eprintln!("❌ Erro ao salvar: {}", e);
        } else {
            println!("📄 Relatório salvo: {}", filename);
        }
    }
    
    // Ajuda
    if args.len() > 1 && (args[1] == "--help" || args[1] == "-h") {
        println!("\n{}", "=".repeat(60));
        println!("           📖 AJUDA           ");
        println!("{}", "=".repeat(60));
        println!("\nUso: hardware-diagnostic [OPÇÃO]");
        println!("\nOpções:");
        println!("  --save, -s    Salva relatório em arquivo");
        println!("  --full, -f    Exibe relatório completo");
        println!("  --help, -h    Mostra esta ajuda");
        println!("\nExemplos:");
        println!("  hardware-diagnostic");
        println!("  hardware-diagnostic --save");
        println!("  hardware-diagnostic --help");
    }
    
    println!("\n{}", "=".repeat(60));
}