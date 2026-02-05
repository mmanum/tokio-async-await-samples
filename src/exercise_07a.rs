/*
Ejercicio 7 (Nivel 7): FuturesUnordered (concurrencia dinámica)

Objetivo
    Lanzar N tareas/futures y recoger resultados según vayan terminando (no en orden de creación).

Enunciado
    Dado un vector de delays [300, 50, 200, 100], crea un FuturesUnordered con wait_return(delay, etiqueta) y ve imprimiendo cada resultado conforme llegue.

Restricción
    Usa futures::stream::FuturesUnordered y StreamExt::next().
*/

pub async fn run() -> Result<(), Box<dyn std::error::Error>> {
    println!("--- Exercise_07 !!! ");

    let delay_array = [3000, 2500, 2000, 5000];

    // delay_array.iter().map(async |delay| sleeping(*delay).await);

    let my_futures = delay_array.iter().map(|d| sleeping(*d));
    let _result = futures::future::join_all(my_futures).await;

    println!("\n\n--- Exercise_07!!!: Fin");
    Ok(())
}

use tokio::time::Duration;
use tokio::time::sleep;

async fn sleeping(delay: u64) -> String {
    println!("Sleep: {delay}");
    sleep(Duration::from_secs(delay)).await;
    println!("Task one completed (delay): {delay}");
    "Task one completed".to_owned()
}
