use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Document {
    pub id: String,
    pub content: String,
}

impl Document {
    pub fn new(id: &str, content: &str) -> Self {
        Self {
            id: id.to_string(),
            content: content.to_string(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Vector(pub Vec<f32>);

impl From<Vec<f32>> for Vector {
    fn from(v: Vec<f32>) -> Self {
        Vector(v)
    }
}

pub struct AuraEngine {
    dimension: usize,
    storage: HashMap<String, (Document, Vector)>,
}

impl AuraEngine {
    pub fn new(dimension: usize) -> Self {
        Self {
            dimension,
            storage: HashMap::new(),
        }
    }

    pub fn insert(&mut self, doc: Document, vector: Vector) -> Result<(), &'static str> {
        if vector.0.len() != self.dimension {
            return Err("Vector dimension mismatch");
        }
        self.storage.insert(doc.id.clone(), (doc, vector));
        Ok(())
    }

    pub fn query(&self, query_vec: &Vector, limit: usize) -> Result<Vec<&Document>, &'static str> {
        if query_vec.0.len() != self.dimension {
            return Err("Query vector dimension mismatch");
        }

        let mut distances: Vec<(&Document, f32)> = self.storage.values()
            .map(|(doc, vec)| {
                let dist = cosine_similarity(&query_vec.0, &vec.0);
                (doc, dist)
            })
            .collect();

        // Sort by highest similarity
        distances.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));

        Ok(distances.into_iter().take(limit).map(|(doc, _)| doc).collect())
    }
}

fn cosine_similarity(a: &[f32], b: &[f32]) -> f32 {
    let dot_product: f32 = a.iter().zip(b.iter()).map(|(x, y)| x * y).sum();
    let norm_a: f32 = a.iter().map(|x| x * x).sum::<f32>().sqrt();
    let norm_b: f32 = b.iter().map(|x| x * x).sum::<f32>().sqrt();
    
    if norm_a == 0.0 || norm_b == 0.0 {
        0.0
    } else {
        dot_product / (norm_a * norm_b)
    }
}
