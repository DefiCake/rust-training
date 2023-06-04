fn main() {
  let progress_bar = indicatif::ProgressBar::new(10);

  for i in 0..10 {
    sleep();
    progress_bar.println(format!("[+] finished #{}", i));
    progress_bar.inc(1);
  }
}

fn sleep() {
  std::thread::sleep(std::time::Duration::from_millis(100))
}
