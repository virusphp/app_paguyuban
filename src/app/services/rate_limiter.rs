use std::collections::HashMap;
use std::sync::{Mutex, OnceLock};
use std::time::{Duration, Instant};

/// Struktur pelacak percobaan rate limit berbasis sliding window
pub struct RateLimiter {
    attempts: Mutex<HashMap<String, Vec<Instant>>>,
}

impl Default for RateLimiter {
    fn default() -> Self {
        Self::new()
    }
}

impl RateLimiter {
    pub fn new() -> Self {
        Self {
            attempts: Mutex::new(HashMap::new()),
        }
    }

    /// Memeriksa apakah suatu identifier (misal nomor HP / IP) masih diizinkan melakukan percobaan.
    /// Mengembalikan `Ok(())` jika belum melampaui batas, atau `Err(remaining_secs)` jika terkunci.
    pub fn check_limit(&self, key: &str, max_attempts: usize, window: Duration) -> Result<(), u64> {
        let mut map = self.attempts.lock().unwrap();
        let now = Instant::now();

        if let Some(timestamps) = map.get_mut(key) {
            // Bersihkan entri yang sudah melewati durasi jendela waktu
            timestamps.retain(|&t| now.duration_since(t) < window);

            if timestamps.len() >= max_attempts {
                // Hitung sisa waktu tunggu hingga entri tertua kedaluwarsa
                if let Some(oldest) = timestamps.first() {
                    let elapsed = now.duration_since(*oldest);
                    let remaining = if window > elapsed {
                        (window - elapsed).as_secs().max(1)
                    } else {
                        1
                    };
                    return Err(remaining);
                }
                return Err(window.as_secs().max(1));
            }
        }

        Ok(())
    }

    /// Mencatat 1 kegagalan percobaan untuk key tertentu
    pub fn record_failure(&self, key: &str, window: Duration) {
        let mut map = self.attempts.lock().unwrap();
        let now = Instant::now();

        let timestamps = map.entry(key.to_string()).or_default();
        timestamps.retain(|&t| now.duration_since(t) < window);
        timestamps.push(now);
    }

    /// Mereset catatan kegagalan (misalnya setelah login berhasil)
    pub fn reset(&self, key: &str) {
        let mut map = self.attempts.lock().unwrap();
        map.remove(key);
    }
}

/// Global singleton RateLimiter untuk proteksi endpoint login
pub fn login_rate_limiter() -> &'static RateLimiter {
    static INSTANCE: OnceLock<RateLimiter> = OnceLock::new();
    INSTANCE.get_or_init(RateLimiter::new)
}
