# Hardware Diagnostic

[![Crates.io](https://img.shields.io/crates/v/hardware-diagnostic.svg)](https://crates.io/crates/hardware-diagnostic)
[![Docs.rs](https://docs.rs/hardware-diagnostic/badge.svg)](https://docs.rs/hardware-diagnostic)
[![License: GPLv3](https://img.shields.io/crates/l/hardware-diagnostic.svg)](LICENSE)

Uma ferramenta de diagnóstico de hardware para Windows escrita em Rust, com sistema de pontuação de desempenho inteligente.

## ✨ Funcionalidades

- ✅ **Coleta completa de informações** de CPU, RAM e discos
- ✅ **Sistema de pontuação** automática (0-10 pontos)
- ✅ **Categorização inteligente** baseada no desempenho
- ✅ **Recomendações personalizadas** para cada situação
- ✅ **Relatórios formatados** em texto e JSON
- ✅ **Interface CLI** simples e intuitiva
- ✅ **Documentação completa** com `cargo doc`

## 📊 Sistema de Pontuação

| Pontuação | Categoria | Significado |
|-----------|-----------|-------------|
| 1-2 | 🚨 **Descarte** | Upgrade completo necessário |
| 3-4 | ⚠️ **Manutenção** | Intervenção urgente necessária |
| 5-6 | 🔶 **Precaução** | Monitoramento constante |
| 7-10 | ✅ **Bom Estado** | Adequado para uso normal |

## 🚀 Instalação

### Via Cargo
```bash
cargo install hardware-diagnostic
```
### 📄 Licença

Este programa é software livre: você pode redistribuí-lo e/ou modificá-lo
sob os termos da GNU General Public License conforme publicada pela
Free Software Foundation, seja a versão 3 da Licença, ou
(a seu critério) qualquer versão posterior.

Este programa é distribuído na esperança de que seja útil,
mas SEM QUALQUER GARANTIA; sem mesmo a garantia implícita de
COMERCIALIZAÇÃO ou ADEQUAÇÃO A UM DETERMINADO FIM. Veja a
GNU General Public License para mais detalhes.

Você deve ter recebido uma cópia da GNU General Public License
junto com este programa. Caso contrário, veja <https://www.gnu.org/licenses/>.
