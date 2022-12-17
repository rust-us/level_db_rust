use std::ops::Deref;
use std::sync::{Arc, LockResult, Mutex, MutexGuard, TryLockResult};

pub struct Lock<T> {
    lock: Arc<Mutex<T>>,
}

/// 线程内使用的锁对象
impl<T> Lock<T> {
    #[inline]
    fn new(t: Arc<Mutex<T>>) -> Self {
        Self {
            lock: t
        }
    }

    #[inline]
    pub fn lock(&self) -> LockResult<MutexGuard<'_, T>> {
        self.lock.lock()
    }

    #[inline]
    pub fn try_lock(&self) -> TryLockResult<MutexGuard<'_, T>> {
        self.lock.try_lock()
    }
}

/// 互斥锁
pub struct MutexLock<T> {
    mutex_hold: Arc<Mutex<T>>,
}

impl<T> MutexLock<T> {
    /// 构建互斥锁
    ///
    /// # Arguments
    ///
    /// * `t`: 锁对象
    ///
    /// returns: MutexLock<T>
    ///
    /// # Examples
    ///
    /// ```
    /// let mutex_lock = MutexLock::new(0);
    /// ```
    #[inline]
    pub fn new(t: T) -> Self {
        Self {
            mutex_hold: Arc::new(Mutex::new(t))
        }
    }

    /// 获取用于move的锁对象
    ///
    /// # Arguments
    ///
    ///
    /// returns: MutexLock<T>
    ///
    /// # Examples
    ///
    /// ```
    /// let mutex_lock = MutexLock::new(0);
    /// let lock = mutex_lock.get_lock();    ///
    /// let handle = thread::spawn(move || {
    ///     let mut num = lock.lock().unwrap();
    ///     *num += 1;
    ///     });
    /// ```
    #[inline]
    pub fn get_lock(&self) -> Lock<T> {
        return Lock::new(Arc::clone(&self.mutex_hold));
    }
}
