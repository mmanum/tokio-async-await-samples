mod exercise_00;
mod exercise_01;
mod exercise_02;
mod exercise_03;
mod exercise_04;
mod exercise_05;
mod exercise_06;
mod exercise_07a;
mod exercise_07b;
mod exercise_08;
mod exercise_09a;
mod exercise_09b;

use tokio;

use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;

// #[tokio::main]
// async fn main() -> Result<(), Box<dyn std::error::Error>> {

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let rt = tokio::runtime::Runtime::new().unwrap();
    rt.block_on(async {
        #[cfg(feature = "ex00")]
        {
            exercise_00::run().await;
            println!("Done!");
        }

        #[cfg(feature = "ex01")]
        {
            exercise_01::run().await;
            println!("Done!");
        }

        #[cfg(feature = "ex02")]
        {
            exercise_02::run().await;
            println!("Done!");
        }

        #[cfg(feature = "ex03")]
        {
            exercise_03::run().await;
            println!("Done!");
        }

        #[cfg(feature = "ex04")]
        {
            exercise_04::run().await;
            println!("Done!");
        }

        #[cfg(feature = "ex05")]
        {
            exercise_05::run().await;
            println!("Done!");
        }

        #[cfg(feature = "ex06")]
        {
            exercise_06::run().await;
            println!("Done!");
        }

        #[cfg(feature = "ex07a")]
        {
            exercise_07a::run().await;
            println!("Done!");
        }

        #[cfg(feature = "ex07b")]
        {
            exercise_07b::run().await;
            println!("Done!");
        }

        #[cfg(feature = "ex08")]
        {
            exercise_08::run().await;
            println!("Done!");
        }

        #[cfg(feature = "ex09a")]
        {
            exercise_09a::run().await;
            println!("Done!");
        }

        #[cfg(feature = "ex09b")]
        {
            exercise_09b::run().await;
            println!("Done!");
        }

        #[cfg(feature = "default")]
        {
            // println!("Choose your feature (exercise).\n $ cargo build --feature ex01");
            return Err("Choose your feature (exercise).\n $ cargo build --feature ex01".into());
        }
    });
    Ok(())
}
