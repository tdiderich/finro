use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::{SystemTime, UNIX_EPOCH};

static COUNTER: AtomicU64 = AtomicU64::new(0);

pub fn generate() -> String {
    let mut hasher = DefaultHasher::new();
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_nanos()
        .hash(&mut hasher);
    std::process::id().hash(&mut hasher);
    COUNTER.fetch_add(1, Ordering::Relaxed).hash(&mut hasher);
    let hash = hasher.finish();
    format!("kz-{:04x}", hash & 0xFFFF)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ids_are_unique_across_rapid_calls() {
        let ids: Vec<String> = (0..100).map(|_| generate()).collect();
        let unique: std::collections::HashSet<&String> = ids.iter().collect();
        assert_eq!(ids.len(), unique.len(), "generated duplicate IDs");
    }

    #[test]
    fn id_format() {
        let id = generate();
        assert!(id.starts_with("kz-"), "bad prefix: {id}");
        assert_eq!(id.len(), 7, "bad length: {id}");
    }
}
