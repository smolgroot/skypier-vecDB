use anyhow::{anyhow, Result};

pub fn cosine_similarity(a: &[f32], b: &[f32]) -> Result<f32> {
    if a.len() != b.len() {
        return Err(anyhow!("Vector dimensions must match"));
    }

    let dot_product: f32 = a.iter().zip(b.iter()).map(|(x, y)| x * y).sum();
    let norm_a: f32 = a.iter().map(|x| x * x).sum::<f32>().sqrt();
    let norm_b: f32 = b.iter().map(|x| x * x).sum::<f32>().sqrt();

    if norm_a == 0.0 || norm_b == 0.0 {
        return Ok(0.0);
    }

    Ok(dot_product / (norm_a * norm_b))
}

pub fn euclidean_distance(a: &[f32], b: &[f32]) -> Result<f32> {
    if a.len() != b.len() {
        return Err(anyhow!("Vector dimensions must match"));
    }

    let distance = a
        .iter()
        .zip(b.iter())
        .map(|(x, y)| (x - y).powi(2))
        .sum::<f32>()
        .sqrt();

    Ok(distance)
}

pub fn dot_product(a: &[f32], b: &[f32]) -> Result<f32> {
    if a.len() != b.len() {
        return Err(anyhow!("Vector dimensions must match"));
    }

    Ok(a.iter().zip(b.iter()).map(|(x, y)| x * y).sum())
}

pub fn manhattan_distance(a: &[f32], b: &[f32]) -> Result<f32> {
    if a.len() != b.len() {
        return Err(anyhow!("Vector dimensions must match"));
    }

    Ok(a.iter().zip(b.iter()).map(|(x, y)| (x - y).abs()).sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cosine_similarity() {
        let a = vec![1.0, 0.0, 0.0];
        let b = vec![1.0, 0.0, 0.0];
        assert!((cosine_similarity(&a, &b).unwrap() - 1.0).abs() < 1e-6);

        let a = vec![1.0, 0.0, 0.0];
        let b = vec![0.0, 1.0, 0.0];
        assert!((cosine_similarity(&a, &b).unwrap() - 0.0).abs() < 1e-6);
    }

    #[test]
    fn test_euclidean_distance() {
        let a = vec![0.0, 0.0, 0.0];
        let b = vec![1.0, 1.0, 1.0];
        assert!((euclidean_distance(&a, &b).unwrap() - 3.0_f32.sqrt()).abs() < 1e-6);
    }

    #[test]
    fn test_dot_product() {
        let a = vec![1.0, 2.0, 3.0];
        let b = vec![4.0, 5.0, 6.0];
        assert!((dot_product(&a, &b).unwrap() - 32.0).abs() < 1e-6);
    }
}
