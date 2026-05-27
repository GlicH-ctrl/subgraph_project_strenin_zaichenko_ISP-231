// Система поиска частых подграфов v0.5
// Студенты: Стренин Денис, Заиченко Андрей, ИСП-231

use petgraph::graph::Graph;
use petgraph::Directed;
use std::collections::HashMap;
use std::io::{self, BufRead, Write};
use std::fs;
use std::error::Error;
use clap::Parser;

/// Метка вершины
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct VertexLabel(pub String);

/// Метка ребра
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct EdgeLabel(pub String);

/// Тип графа
type LabeledGraph = Graph<VertexLabel, EdgeLabel, Directed>;

/// Аргументы командной строки
#[derive(Parser, Debug)]
#[command(
    author = "Стренин Денис, Заиченко Андрей, ИСП-231", 
    version = "0.5.0", 
    about = "Поиск частых подграфов (CLI + файл)"
)]
struct Args {
    #[arg(short, long, default_value_t = 2)]
    min_support: usize,

    /// Максимальное количество графов для анализа
    #[arg(long, default_value_t = 10)]
    max_graphs: usize,

    /// Максимальное количество вершин в одном графе
    #[arg(long, default_value_t = 5)]
    max_vertices: usize,

    /// Путь к .txt файлу с графами (каждая строка = один граф)
    #[arg(short, long)]
    input_file: Option<String>,
}

/// Создаёт граф из вершин, соединяя их цепочкой: v0→v1→v2...
fn create_graph_from_vertices(vertices: Vec<String>) -> LabeledGraph {
    let mut g = LabeledGraph::new();
    if vertices.len() < 2 { return g; }

    let mut node_indices = Vec::with_capacity(vertices.len());
    for label in &vertices {
        node_indices.push(g.add_node(VertexLabel(label.clone())));
    }

    for i in 0..(node_indices.len() - 1) {
        g.add_edge(node_indices[i], node_indices[i + 1], EdgeLabel("link".into()));
    }
    g
}

///Загрузка графов из .txt файла
fn load_graphs_from_txt(
    path: &str, 
    max_graphs: usize, 
    max_vertices: usize
) -> Result<Vec<LabeledGraph>, Box<dyn Error>> {
    let content = fs::read_to_string(path)?;
    let mut graphs = Vec::new();

    for line in content.lines() {
        let line = line.trim();
        if line.is_empty() { continue; }

        let vertices: Vec<String> = line.split_whitespace().map(|s| s.to_string()).collect();
        
        if vertices.len() > max_vertices {
            eprintln!("  Пропуск строки: {} вершин (лимит: {})", vertices.len(), max_vertices);
            continue;
        }
        if vertices.len() < 2 { continue; }

        graphs.push(create_graph_from_vertices(vertices));
        if graphs.len() >= max_graphs { break; }
    }
    Ok(graphs)
}

///Интерактивный ввод с параметризованными лимитами
fn read_graphs_interactively(max_graphs: usize, max_vertices: usize) -> Vec<LabeledGraph> {
    let stdin = io::stdin();
    let mut graphs = Vec::new();

    println!("\n ВВОД ГРАФОВ (лимиты: {} графов, {} вершин)", max_graphs, max_vertices);
    println!("   • Вводите вершины через пробел (пример: A1 B2 C3)");
    println!("   • Пустая строка = завершить ввод\n");

    for graph_num in 1..=max_graphs {
        println!("--- Граф #{} из {} ---", graph_num, max_graphs);
        print!("Введите вершины (макс. {}): ", max_vertices);
        let _ = io::stdout().flush();

        let mut line = String::new();
        if stdin.lock().read_line(&mut line).is_err() { break; }

        let line = line.trim();
        if line.is_empty() {
            println!("\n Ввод завершён. Всего графов: {}", graphs.len());
            break;
        }

        let vertices: Vec<String> = line.split_whitespace().map(|s| s.to_string()).collect();
        if vertices.len() > max_vertices {
            println!("  Ошибка: введено {} вершин (лимит: {}). Попробуйте снова.\n", vertices.len(), max_vertices);
            continue;
        }
        if vertices.len() < 2 {
            println!("  Ошибка: нужно минимум 2 вершины.\n");
            continue;
        }

        println!("   ↪ Принято вершин: {}", vertices.len());
        graphs.push(create_graph_from_vertices(vertices));
        println!();
    }
    graphs
}

/// Алгоритм поиска частых рёбер
fn find_frequent_edges(
    graphs: &[LabeledGraph], 
    min_support: usize
) -> Vec<((String, String), usize)> {
    let mut counts: HashMap<(String, String), usize> = HashMap::new();
    for graph in graphs {
        for edge_idx in graph.edge_indices() {
            let (src, dst) = graph.edge_endpoints(edge_idx).unwrap();
            let key = (graph[src].0.clone(), graph[dst].0.clone());
            *counts.entry(key).or_insert(0) += 1;
        }
    }
    counts.into_iter()
          .filter(|(_, count)| *count >= min_support)
          .collect()
}

fn main() {
    let args = Args::parse();

    println!(" Система поиска частых подграфов v0.5");
    println!(" Авторы: Стренин Денис, Заиченко Андрей, группа ИСП-231");
    println!(" Учебный проект, 2026 г.\n");

    //Выбор источника данных
    let graphs = if let Some(path) = &args.input_file {
        println!(" Загрузка из файла: {}", path);
        match load_graphs_from_txt(path, args.max_graphs, args.max_vertices) {
            Ok(g) => g,
            Err(e) => {
                eprintln!(" Ошибка чтения файла: {}", e);
                std::process::exit(1);
            }
        }
    } else {
        read_graphs_interactively(args.max_graphs, args.max_vertices)
    };

    if graphs.is_empty() {
        println!("\n  Нет графов для анализа. Завершение работы.");
        return;
    }

    println!("\n  Запуск анализа...");
    println!(" Графов: {} | min_support: {}", graphs.len(), args.min_support);

    let frequent = find_frequent_edges(&graphs, args.min_support);

    println!("\n РЕЗУЛЬТАТЫ:");
    println!("   {}", "=".repeat(40));
    if frequent.is_empty() {
        println!("    Частые паттерны не найдены");
    } else {
        println!("    Найдено паттернов: {}", frequent.len());
        for ((from, to), count) in &frequent {
            println!("    {} → {} | вхождений: {}", from, to, count);
        }
    }
    println!("   {}", "=".repeat(40));
    println!("\n Работа завершена");
}