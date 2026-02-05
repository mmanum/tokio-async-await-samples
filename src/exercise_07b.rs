use futures::{FutureExt, TryStreamExt};
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

    let delay_array = [3000, 2500, 2000, 5000];

    let futures_unordered = futures::stream::FuturesUnordered::new();

    delay_array
        .iter()
        .for_each(|&d| futures_unordered.push(sleeping(d)));

    let system_time = std::time::SystemTime::now();
    println!(
        "-----START ({})-----",
        system_time.elapsed().unwrap().as_millis()
    );

    // while let Some(result) = futures_unordered.next().await {
    //     println!("Result: {result}");
    // }

    futures_unordered
        .for_each(|f| async move { println!("Result: {f}") })
        .await;

    // use futures::stream::TryStreamExt;
    // futures_unordered
    //     .try_for_each(|result| async move {
    //         println!("Result: {result}");
    //         Ok::<(), std::convert::Infallible>(())
    //     })
    //     .await;

    println!(
        "-----END ({} ms)-----",
        system_time.elapsed().unwrap().as_millis()
    );

    println!("\n\n--- Exercise_07!!!: Fin");
    Ok(())
}

use tokio::time::Duration;
use tokio::time::sleep;

async fn sleeping(delay: u64) -> String {
    println!("Sleep: {delay}");
    sleep(Duration::from_millis(delay)).await;
    format!("Task {delay} (completed)").to_owned()
}
