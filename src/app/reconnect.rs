use std::time::Duration;

const BASE_MS: u64 = 1_000;
const MAX_MS: u64 = 30_000;
const MIN_MS: u64 = 500;

/// Bounded exponential backoff with ±25% jitter.
///
/// attempt=1 → ~1 s, attempt=2 → ~2 s, …, attempt=6+ → ~30 s (capped).
/// Jitter is derived from sub-second system-clock nanoseconds, so no PRNG
/// dependency is needed.
pub fn backoff_delay(attempt: u32) -> Duration {
    let exp = attempt.saturating_sub(1).min(5);
    let base = BASE_MS.saturating_mul(1u64 << exp);
    let capped = base.min(MAX_MS);
    let quarter = capped / 4;
    let nanos = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.subsec_nanos())
        .unwrap_or(0) as u64;
    let jitter: i64 = if quarter > 0 {
        (nanos % (quarter * 2)) as i64 - quarter as i64
    } else {
        0
    };
    let ms = ((capped as i64) + jitter).max(MIN_MS as i64) as u64;
    Duration::from_millis(ms)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn delay_increases_with_attempt() {
        let d1 = backoff_delay(1).as_millis();
        let d3 = backoff_delay(3).as_millis();
        let d6 = backoff_delay(6).as_millis();
        assert!(d1 < d3, "delay should grow: {d1} < {d3}");
        assert!(d3 < d6, "delay should grow: {d3} < {d6}");
    }

    #[test]
    fn delay_is_bounded() {
        for attempt in 1..=20 {
            let d = backoff_delay(attempt).as_millis();
            assert!(
                d <= MAX_MS as u128 + MAX_MS as u128 / 4,
                "delay exceeded max bound at attempt {attempt}: {d}ms"
            );
            assert!(
                d >= MIN_MS as u128,
                "delay below min at attempt {attempt}: {d}ms"
            );
        }
    }

    #[test]
    fn attempt_zero_is_safe() {
        let d = backoff_delay(0).as_millis();
        assert!(d >= MIN_MS as u128);
    }
}
