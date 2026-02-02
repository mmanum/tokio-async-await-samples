/*
Ejercicio 1 (Nivel 1): “Hola async”

Objetivo
    Entender qué significa async fn y cómo se ejecuta una función async con un runtime.

Enunciado

Crea un programa que:
    Defina una función async fn saludar(nombre: &str) -> String que devuelva "Hola, <nombre>!".
    En main, llame a saludar("AL") usando .await y lo imprima por pantalla.

Restricciones
    Debes usar #[tokio::main] para poder usar .await en main.
    La función saludar no debe imprimir nada; solo devolver el String.

Pista (mínima)
    Si una función es async fn, al llamarla obtienes un future; para obtener el resultado necesitas .await. */

pub async fn run() -> Result<(), Box<dyn std::error::Error>> {
    println!("Exercise_01 !!!");
    calculate().await;
    Ok(())
}
use std::io::{self, Write};
async fn calculate() {
    let stdout = io::stdout();
    let mut out = stdout.lock();
    for index in 1..50 {
        print!(".");
        out.flush().unwrap();
        use tokio::time::{Duration, sleep};
        sleep(Duration::from_millis(100)).await; // importante el .await, sino no se ejecuta la función.
    }
}
