/*
Ejercicio Nivel 8: “Ventana de concurrencia” (máximo N tareas a la vez)
Objetivo

    Aprender a lanzar muchas operaciones async y limitar la concurrencia (no ejecutar todas a la vez),
    recoger resultados conforme terminan.

Enunciado

    Tienes una lista de “trabajos” representados por milisegundos:
    let jobs = [500, 120, 80, 300, 900, 200, 50, 400];


Debes implementar:
    pub async fn run() -> Result<(), Box<dyn std::error::Error>>


que haga lo siguiente:

Define MAX_CONCURRENCY: usize = 3.
Implementa async fn job(ms: u64) -> String que:
    duerma ms milisegundos,

    devuelva format!("job({ms}) done").

Ejecuta todos los jobs pero asegurando que:
    nunca haya más de MAX_CONCURRENCY jobs “en vuelo” simultáneamente.
    Imprime cada resultado en el momento en que termina (orden de finalización, no de entrada).

Al final imprime:

el número total de jobs procesados y el tiempo total transcurrido (usa tokio::time::Instant).

Restricciones
No uses spawn (hazlo solo con futures/streams).

Usa futures (por ejemplo stream::iter(...) + .buffer_unordered(MAX_CONCURRENCY)), o una solución equivalente basada en FuturesUnordered gestionando la “ventana”.

*/

use futures::stream::{self, StreamExt};

static MAX_CONCURRENCY: usize = 3usize;

pub async fn runa() -> Result<(), Box<dyn std::error::Error>> {
    let delays = [5000u64, 1200, 800, 3000, 9000, 2000, 500, 4000];

    let jobs = delays.iter().map(|&n| job(n));

    let system_time = std::time::SystemTime::now();
    println!(
        "-----START ({})-----",
        system_time.elapsed().unwrap().as_millis()
    );

    stream::iter(jobs)
        .buffer_unordered(MAX_CONCURRENCY)
        .for_each(|f| async move {
            println!("\t\t{f}. Result");
        }) // llama al trait stream.
        .await;

    println!(
        "-- END ({} ms) --",
        system_time.elapsed().unwrap().as_millis()
    );

    Ok(())
}

pub async fn run() -> Result<(), Box<dyn std::error::Error>> {
    let delays = [5000u64, 1200, 800, 3000, 9000, 2000, 500, 4000];
    let delays_len = delays.len();
    let completed = stream::iter(delays)
        .map(|d| job(d))
        .buffer_unordered(MAX_CONCURRENCY)
        .fold(1usize, |count, f| async move {
            println!("\t\t{f}. Furutes Resolved = {count}/{}", delays_len);
            count + 1
        })
        .await;

    println!("\n\nCompleted: {completed}");
    Ok(())
}

async fn job(ms: u64) -> String {
    println!("Start job({ms})");
    tokio::time::sleep(tokio::time::Duration::from_millis(ms)).await;
    format!("Done job({ms})")
}
