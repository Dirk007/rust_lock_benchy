#[cfg(feature = "mutex")]
macro_rules! create_lock(
    ($a:expr) => {
        Arc::new(Mutex::new($a))
    }
);

#[cfg(feature = "mutex")]
macro_rules! lock_read(
    ($a:ident) => {
        $a.lock().unwrap()
    }
);

#[cfg(feature = "mutex")]
macro_rules! lock_write(
    ($a:ident) => {
        $a.lock().unwrap()
    }
);

#[cfg(feature = "rwlock")]
macro_rules! create_lock(
    ($a:expr) => {
        Arc::new(RwLock::new($a))
    }
);

#[cfg(feature = "rwlock")]
macro_rules! lock_read(
    ($a:ident) => {
        $a.read().unwrap()
    }
);

#[cfg(feature = "rwlock")]
macro_rules! lock_write(
    ($a:ident) => {
        $a.write().unwrap()
    }
);

#[cfg(feature = "tmutex")]
macro_rules! create_lock(
    ($a:expr) => {
        Arc::new(Mutex::new($a))
    }
);

#[cfg(feature = "tmutex")]
macro_rules! lock_read(
    ($a:ident) => {
        $a.lock().await
    }
);

#[cfg(feature = "tmutex")]
macro_rules! lock_write(
    ($a:ident) => {
        $a.lock().await
    }
);

#[cfg(feature = "trwlock")]
macro_rules! create_lock(
    ($a:expr) => {
        Arc::new(RwLock::new($a))
    }
);

#[cfg(feature = "trwlock")]
macro_rules! lock_read(
    ($a:ident) => {
        $a.read().await
    }
);

#[cfg(feature = "trwlock")]
macro_rules! lock_write(
    ($a:ident) => {
        $a.write().await
    }
);

#[cfg(feature = "plmutex")]
macro_rules! create_lock(
    ($a:expr) => {
        Arc::new(Mutex::new($a))
    }
);

#[cfg(feature = "plmutex")]
macro_rules! lock_read(
    ($a:ident) => {
        $a.lock()
    }
);

#[cfg(feature = "plmutex")]
macro_rules! lock_write(
    ($a:ident) => {
        $a.lock()
    }
);

#[cfg(feature = "plrwlock")]
macro_rules! create_lock(
    ($a:expr) => {
        Arc::new(RwLock::new($a))
    }
);

#[cfg(feature = "plrwlock")]
macro_rules! lock_read(
    ($a:ident) => {
        $a.read()
    }
);

#[cfg(feature = "plrwlock")]
macro_rules! lock_write(
    ($a:ident) => {
        $a.write()
    }
);

/// Just a DRY remover
macro_rules! to_usize(
    ($a:ident, $b: expr) => {
        $a
        .value_of($b)
        .unwrap()
        .parse::<usize>()
        .expect("Values must be numeric")
    }
);
