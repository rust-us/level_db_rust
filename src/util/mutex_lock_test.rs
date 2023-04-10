mod test {
    use std::thread;
    use crate::util::mutex_lock::MutexLock;

    #[test]
    fn test() {
        let mutex_lock = MutexLock::new(0);
        let mut handles = vec![];

        for _ in 0..10 {
            let lock = mutex_lock.get_lock();
            let handle = thread::spawn(move || {
                let mut num = lock.lock().unwrap();

                *num += 1;
            });
            handles.push(handle);
        }

        for handle in handles {
            handle.join().unwrap();
        }

        println!("Result: {}", mutex_lock.get_lock().lock().unwrap());
    }
}