// tests/integration_test.rs

use hardware_diagnostic::*;
use std::process::Command;

#[test]
fn test_cli_arguments() {
    // Testa execução com diferentes argumentos
    let output = Command::new("cargo")
        .args(["run", "--", "--help"])
        .output()
        .expect("Falha ao executar comando");
    
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("diag"));
}

#[test]
fn test_save_report() {
    // Testa a geração de arquivo
    let temp_file = "test_report_temp.txt";
    
    // Remove arquivo se existir
    let _ = std::fs::remove_file(temp_file);
    
    // Executa comando de salvamento
    let output = Command::new("cargo")
        .args(["run", "--", "--save"])
        .output()
        .expect("Falha ao executar comando");
    
    // Verifica se algum arquivo foi criado
    let files: Vec<_> = std::fs::read_dir(".")
        .unwrap()
        .filter_map(Result::ok)
        .filter(|f| f.file_name().to_string_lossy().contains("diagnostico_"))
        .collect();
    
    assert!(!files.is_empty(), "Nenhum arquivo de relatório criado");
    
    // Limpeza
    for file in files {
        std::fs::remove_file(file.path()).ok();
    }
}