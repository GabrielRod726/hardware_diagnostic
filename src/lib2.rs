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

//! Biblioteca de diagnóstico de hardware
//! 
//! Esta biblioteca fornece funcionalidades para coletar e analisar
//! informações do sistema em computadores Windows.

pub mod engine;

// Re-exporta os tipos principais para fácil acesso
pub use engine::{CpuInfo, RamInfo, DiskInfo, utils, write_report};