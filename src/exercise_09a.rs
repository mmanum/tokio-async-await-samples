/*
Ejercicio (Nivel 9): cancelación por “deadline global”

    Ejecuta la misma cola de trabajos de ejemplo anteiror con MAX_CONCURRENCY = 3,
    pero con un límite total de tiempo (p. ej. 4 segundos).

    Todo lo que no haya terminado antes del deadline debe considerarse cancelado y debes imprimir cuántos acabaron y cuántos se cancelaron.
*/

use futures::future;
use futures::stream::{self, FuturesUnordered, StreamExt};

use tokio::time::{Duration, sleep};

static MAX_CONCURRENCY: usize = 3usize;

pub async fn run() -> Result<(), Box<dyn std::error::Error>> {
    let delays = [5000u64, 1200, 800, 3000, 9000, 2000, 500, 4000];

    let system_time = std::time::SystemTime::now();
    println!(
        "-----START ({})-----",
        system_time.elapsed().unwrap().as_millis()
    );

    let mut deadline = sleep(Duration::from_millis(4_000));
    tokio::pin!(deadline);

    let mut finished = stream::iter(delays)
        .map(|d| job(d))
        .buffer_unordered(MAX_CONCURRENCY)
        .take_until(deadline)
        .fold(1u64, |count, f| async move {
            println!("\t\t Result {f}");
            count + 1
        })
        .await;

    println!(
        "-- END ({} ms) --",
        system_time.elapsed().unwrap().as_millis()
    );

    println!("Futures completed: {finished}/{}", delays.len());
    Ok(())
}

async fn job(ms: u64) -> String {
    println!("Start job({ms})");
    tokio::time::sleep(tokio::time::Duration::from_millis(ms)).await;
    format!("Done job({ms})")
}
