use unicode_normalization::UnicodeNormalization;
use std::time::Instant;
use std::collections::HashMap;
use std::io::{self, Write};

#[derive(Debug)]

struct Product {
    id: u32,
    name: String,
    category: String,

}

fn preprocess(text: &str) -> String {
    text.nfkd() 
        .filter(|c| c.is_ascii() && c.is_alphanumeric() || *c == ' ')
        .collect::<String>()
        .to_lowercase()
}

fn main() {
    let mut product_table: HashMap<u32, Product> = HashMap::new();
    product_table.insert(
        1000,
        Product {
            id: 1000,
            name: "Notebook Acer".to_string(),
            category: "Eletrônicos".to_string(),
        },
    );
    product_table.insert(
        1015,
        Product {
            id: 1015,
            name: "cabo-usb".to_string(),
            category: "Eletrônico".to_string(),
        },
    );

    product_table.insert(
        1020,
        Product {
            id: 1020,
            name: "Churrasqueira a gás".to_string(),
            category: "Eletrodoméstico".to_string(),
        },
    );

    product_table.insert(
        1060,
        Product {
            id: 1060,
            name: "TV 80 polegadas".to_string(),
            category: "Eletrônico".to_string(),
        },
    );

    let mut cache: HashMap<String, Vec<u32>> = HashMap::new();
    let start = Instant::now();

    loop {
        print!("\nDigite um termo de busca (ou 'sair' para encerrar): ");
        io::stdout().flush().unwrap();

        let mut search_term = String::new();
        io::stdin().read_line(&mut search_term).expect("Erro na leitura");
        let search_term = search_term.trim();

        if search_term.eq_ignore_ascii_case("sair") {
            break;
        }

        let normalized_term = preprocess(search_term);

        if let Some(cached_ids) = cache.get(&normalized_term) {
            println!("[CACHE] Resultados encontrados:");
            let duration = start.elapsed();
            println!("Tempo de resposta: {:.2?}", duration);
            for id in cached_ids {
                if let Some(product) = product_table.get(id) {

                    println!("-> {:?}", product);

                }

            }

        } else {

            let result: Vec<&Product> = product_table
                .values()
                .filter(|product| preprocess(&product.name).contains(&normalized_term))
                .collect();

            if result.is_empty() {
                println!("Nenhum produto encontrado para esse termo");
            } else {
                println!("Resultado da busca:");
                let duration = start.elapsed();

                println!("Tempo de resposta: {:.2?}", duration);

                for p in &result {
                    println!("-> {:?}", p);

                }

                let ids: Vec<u32> = result.iter().map(|p| p.id).collect();
                cache.insert(normalized_term, ids);
            }
        }
    }

    println!("\nCatálogo completo:");

    for (id, product) in &product_table {
        println!("ID {}: {} - {}", id, product.name, product.category);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_preprocess_removes_accents_and_special_chars() {
        let input = "Churrasqueira à gás!";
        let expected = "churrasqueira a gas";
        assert_eq!(preprocess(input), expected);
    }

    #[test]
    fn test_preprocess_keeps_alphanumeric_and_space() {
        let input = "Notebook Acer 2024";
        let expected = "notebook acer 2024";
        assert_eq!(preprocess(input), expected);
    }

    #[test]
    fn test_search_finds_product_by_name() {
        let mut product_table: HashMap<u32, Product> = HashMap::new();
        product_table.insert(1, Product {
            id: 1,
            name: "Smartphone Samsung".to_string(),
            category: "Eletrônicos".to_string(),
        });

        let term = "Samsung";
        let normalized = preprocess(term);
        let results: Vec<&Product> = product_table
            .values()
            .filter(|p| preprocess(&p.name).contains(&normalized))
            .collect();

        assert_eq!(results.len(), 1);
        assert_eq!(results[0].id, 1);
    }

    #[test]
    fn test_search_cache_works_correctly() {
        let mut cache: HashMap<String, Vec<u32>> = HashMap::new();
        let key = "notebook acer".to_string();
        let ids = vec![1000];
        cache.insert(key.clone(), ids.clone());

        assert!(cache.contains_key(&key));
        assert_eq!(cache.get(&key), Some(&ids));
    }

    #[test]
    fn test_search_returns_empty_for_not_found() {
        let mut product_table: HashMap<u32, Product> = HashMap::new();
        product_table.insert(1, Product {
            id: 1,
            name: "Fogão Elétrico".to_string(),
            category: "Cozinha".to_string(),
        });

        let term = "XBox";
        let normalized = preprocess(term);
        let results: Vec<&Product> = product_table
            .values()
            .filter(|p| preprocess(&p.name).contains(&normalized))
            .collect();

        assert!(results.is_empty());
    }
}
