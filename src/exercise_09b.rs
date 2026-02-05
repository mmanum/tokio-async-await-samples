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

    let jobs = delays.iter().map(|&d| job(d));

    let mut results = stream::iter(jobs).buffer_unordered(MAX_CONCURRENCY);

    let mut finished = 0usize;

    let system_time = std::time::SystemTime::now();
    println!(
        "-----START ({})-----",
        system_time.elapsed().unwrap().as_millis()
    );

    let mut deadline = sleep(Duration::from_millis(4_000));
    tokio::pin!(deadline);

    loop {
        tokio::select! {
            _ = &mut deadline => {
                println!("\t\t\tTimeout! marchen!");
                break;
            }

            maybe = results.next() => {
                match maybe {
                    Some(msg) => { finished += 1; println!("\t\t Result {msg}"); }
                    None => break, // ya no quedan jobs
                }
            }
        }
    }
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
