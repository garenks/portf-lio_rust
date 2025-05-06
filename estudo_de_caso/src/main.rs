use unicode_normalization::UnicodeNormalization;

use std::collections::HashMap;

use std::io::{self, Write};

use std::time::Instant;





#[derive(Debug)]

struct Product {

    id: u32,

    name: String,

    category: String,

}



fn preprocess(text: &str) -> String {

    text.nfkd() // Normaliza acentos (ex: "á" -> "a" + "~")

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

                println!("Nenhum produto encontrado para esse termo.");

            } else {

                println!("Resultado da busca:");

                let duration = start.elapsed();

                println!("Tempo de resposta: {:.2?}", duration);

                for p in &result {

                    println!("-> {:?}", p);

                }



                // Armazena no cache

                let ids: Vec<u32> = result.iter().map(|p| p.id).collect();

                cache.insert(normalized_term, ids);

            }

        }

    }



    // Itera sobre todos os produtos

    println!("\nCatálogo completo:");

    for (id, product) in &product_table {

        println!("ID {}: {} - {}", id, product.name, product.category);

    }

}