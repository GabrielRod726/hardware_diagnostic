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

//! # Hardware Diagnostic Crate
//! 
//! Uma biblioteca Rust para diagnóstico de hardware em sistemas Windows
//! com sistema de pontuação de desempenho inteligente.
//! 
//! ## 📦 Instalação
//! 
//! Adicione ao seu `Cargo.toml`:
//! 
//! ```toml
//! [dependencies]
//! hardware-diagnostic = "1.0"
//! ```
//! 
//! ## 🚀 Uso Rápido
//! 
//! ```rust
//! use hardware_diagnostic::{cpu_info, calculate_performance_score};
//! 
//! fn main() {
//!     // Informações da CPU
//!     let cpu = cpu_info();
//!     println!("CPU: {}", cpu.name);
//!     
//!     // Pontuação completa
//!     let score = calculate_performance_score();
//!     println!("Pontuação: {:.1}/10", score.overall_score);
//!     println!("Categoria: {:?}", score.category);
//! }
//! ```
//! 
//! ## 📚 Módulos
//! 
//! - [`engine`](engine/index.html) - Funcionalidades principais de diagnóstico
//!   - [`CpuInfo`](engine/struct.CpuInfo.html) - Informações da CPU
//!   - [`RamInfo`](engine/struct.RamInfo.html) - Informações de memória
//!   - [`DiskInfo`](engine/struct.DiskInfo.html) - Informações de discos
//!   - [`PerformanceScore`](engine/struct.PerformanceScore.html) - Pontuação de desempenho
//!   - [`PerformanceCategory`](engine/enum.PerformanceCategory.html) - Categorias
//!   - [`utils`](engine/utils/index.html) - Funções utilitárias
//! 
//! ## 🔧 Funções Principais
//! 
//! - [`cpu_info()`](engine/fn.cpu_info.html) - Coleta informações da CPU
//! - [`ram_info()`](engine/fn.ram_info.html) - Coleta informações de RAM
//! - [`disk_info()`](engine/fn.disk_info.html) - Coleta informações de discos
//! - [`calculate_performance_score()`](engine/fn.calculate_performance_score.html) - Calcula pontuação
//! - [`display_performance_score()`](engine/fn.display_performance_score.html) - Exibe pontuação formatada
//! 
//! ## 🎯 Sistema de Pontuação
//! 
//! | Pontuação | Categoria | Ação Recomendada |
//! |-----------|-----------|------------------|
//! | 1-2 | 🚨 Descarte | Upgrade completo |
//! | 3-4 | ⚠️ Manutenção | Intervenção urgente |
//! | 5-6 | 🔶 Precaução | Monitoramento |
//! | 7-10 | ✅ Bom Estado | Uso normal |
//! 
//! ## 📖 Documentação
//! 
//! Para gerar documentação local:
//! 
//! ```bash
//! cargo doc --open
//! ```
//! 
//! ## 🤝 Contribuindo
//! 
//! Issues e Pull Requests são bem-vindos no [GitHub](https://github.com/seuusuario/hardware-diagnostic).
//! 
//! ## 📄 Licença
//! 
//! MIT OR Apache-2.0

#![doc(html_logo_url = "https://placehold.co/100x100/0366d6/ffffff?text=HD")]
#![doc(html_favicon_url = "https://placehold.co/64x64/0366d6/ffffff?text=H")]
#![warn(missing_docs)]
#![warn(rustdoc::missing_crate_level_docs)]

/// Módulo principal contendo todas as funcionalidades de diagnóstico
pub mod engine;

// Re-exportações para fácil acesso
pub use engine::{
    CpuInfo, RamInfo, DiskInfo, PerformanceScore, PerformanceCategory,
    cpu_info, ram_info, disk_info, calculate_performance_score, display_performance_score
};

/// Versão da crate
pub const VERSION: &str = env!("CARGO_PKG_VERSION");