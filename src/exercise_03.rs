/*
Ejercicio 3 (Nivel 3): concurrencia básica con join!
Objetivo

    Ejecutar dos futures a la vez y observar que el orden temporal cambia.

Enunciado

    Reutiliza esperar_y_devolver(ms, etiqueta) -> String del Ejercicio 2.

    En run():
        Imprime "Inicio".
        Lanza a la vez A(300ms) y B(100ms) usando tokio::join!.
        Imprime ambos resultados.
        Imprime "Fin".

Restricciones
    No uses spawn todavía, solo tokio::join!.

Pista mínima
    let (a, b) = tokio::join!(fut_a, fut_b);

*/
use tokio::time::{Duration, sleep};

pub async fn run() -> Result<(), Box<dyn std::error::Error>> {
    println!("Exercise_03 !!!: Inicio");
    let (a, b) = tokio::join!(
        esperar_y_devolver(300, "A".to_string()),
        esperar_y_devolver(100, "B".to_string())
    );
    println!("{a} -- {b}");

    println!("Exercise_02 !!!: Fin");
    Ok(())
}

async fn esperar_y_devolver(delay: u64, etiqueta: String) -> String {
    sleep(Duration::from_millis(delay)).await; // importante el .await, sino no se ejecuta la función.
    format!("{etiqueta} tras {delay}ms")
}
