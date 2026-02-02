use futures::FutureExt;
/*
Ejercicio 7 (Nivel 7): FuturesUnordered (concurrencia dinámica)

Objetivo
    Lanzar N tareas/futures y recoger resultados según vayan terminando (no en orden de creación).

Enunciado
    Dado un vector de delays [300, 50, 200, 100], crea un FuturesUnordered con wait_return(delay, etiqueta) y ve imprimiendo cada resultado conforme llegue.

Restricción
    Usa futures::stream::FuturesUnordered y StreamExt::next().
*/
use futures::stream::{FuturesUnordered, StreamExt};

pub async fn run() -> Result<(), Box<dyn std::error::Error>> {
    println!("Exercise_07 !!! ");

    let delay_array = [300, 50, 200, 100];

    let futures = delay_array.map(|d| sleeping(d));

    let mut set_futures_unordered = futures::stream::FuturesUnordered::new();

    set_futures_unordered.push(futures.iter());

    set_futures_unordered.into_iter().await;

    println!("\n\n--- Exercise_07!!!: Fin");
    Ok(())
}

use tokio::time::Duration;
use tokio::time::sleep;

async fn sleeping(delay: u64) -> String {
    println!("Sleep: {delay}");
    sleep(Duration::from_secs(delay)).await;
    "Task one completed".to_owned()
}
