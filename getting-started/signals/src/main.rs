use signal_hook::{ consts::SIGINT, iterator::Signals };
use std::{ sync::{ atomic::{ AtomicBool, Ordering }, Arc }, thread, error::Error, time::Duration };

fn main() -> Result<(), Box<dyn Error>> {
  // This is a crate for signals handling. The input of the constructor is
  // signals that we will pick up
  let mut signals = Signals::new(&[SIGINT])?;

  // This instantiates a shared variable between the thread listening to
  // signals and the main thread
  let shutdown = Arc::new(AtomicBool::new(false));

  // But actually, we need to pass a reference to the signal listener,
  // I don't still get the borrowing thing of Rust, but I guess I will over time
  let shutdown_reference = shutdown.clone();

  // Now this is quite the thing.
  // thread::spawn receives a function to execute
  // but if I just pass the function, it won't have access to the outer context
  // i.e. the signals instance and shutdown_reference instance
  // So we achieve that context awareness by using move || . This makes
  // the variables available inside the function
  thread::spawn(move || {
    // Here we just loop forever and wait for signals to come
    for signal in signals.forever() {
      println!("Received signal {:?}", signal);

      // This is actually not needed, since we are only listening to SIGINT
      // in the constructor
      if signal == SIGINT {
        println!("Now shutting down...");
        shutdown_reference.store(true, Ordering::Relaxed);
      }
    }
  });

  // This is the rest of the program. We now just sleep 1 second 30 times.
  // Every time we sleep 1 second and wake up, we check the value of the
  // shutdown variable, and if so, break the loop and head to the exit
  let seconds = 30;

  println!("Now you have {} seconds to Ctrl+C or send any kind of signal", seconds);
  for _i in 0..seconds {
    thread::sleep(Duration::from_secs(1));

    if shutdown.load(Ordering::Relaxed) {
      break;
    }
  }

  Ok(())
}
