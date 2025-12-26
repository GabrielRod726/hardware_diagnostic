#!/bin/bash
# Script de debug para hardware-diagnostic

echo "🔧 Modo Debug Ativado"

# Compila com símbolos de debug
export RUSTFLAGS="-C debuginfo=2"
export RUST_BACKTRACE=full

# Executa com diferentes níveis de log
RUST_LOG=debug cargo run -- --full

# Executa testes com cobertura
echo "🧪 Executando testes..."
cargo test -- --nocapture

# Análise estática
echo "📊 Rodando Clippy..."
cargo clippy -- -D warnings

echo "🔍 Rodando MIRI (análise de UB)..."
cargo +nightly miri test