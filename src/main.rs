// Система поиска частых подграфов

use petgraph::graph::Graph;
use petgraph::Directed; 
use std::collections::HashMap;
use clap::Parser;

/// Метка вершины (например, "Человек", "Фильм")
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct VertexLabel(pub String);

/// Метка ребра (например, "дружит", "смотрел")
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct EdgeLabel(pub String);

/// Тип нашего графа: 
/// VertexLabel - данные в вершинах,
/// EdgeLabel - данные в рёбрах, 
/// Directed - граф ориентированный
type LabeledGraph = Graph<VertexLabel, EdgeLabel, Directed>;

/// Параметры запуска программы из терминала
#[derive(Parser, Debug)]
#[command(author = "Стренин Денис, ИСП-231", version = "0.1.0", about = "Поиск частых подграфов")]
struct Args {
    /// Минимальное количество вхождений подграфа
    #[arg(short, long, default_value_t = 2)]
    min_support: usize,
}

/// Создаёт первый тестовый граф: A -> B -> C
fn create_graph_1() -> LabeledGraph {
    let mut g = LabeledGraph::new();  // <-- ИСПРАВЛЕНО: было new_directed()
    let a = g.add_node(VertexLabel("User".into()));
    let b = g.add_node(VertexLabel("Product".into()));
    let c = g.add_node(VertexLabel("Category".into()));
    
    g.add_edge(a, b, EdgeLabel("bought".into()));
    g.add_edge(b, c, EdgeLabel("belongs_to".into()));
    g
}

/// Создаёт второй тестовый граф: X -> Y -> Z
fn create_graph_2() -> LabeledGraph {
    let mut g = LabeledGraph::new();  // <-- ИСПРАВЛЕНО: было new_directed()
    let x = g.add_node(VertexLabel("User".into()));
    let y = g.add_node(VertexLabel("Product".into()));
    let z = g.add_node(VertexLabel("Brand".into()));
    
    g.add_edge(x, y, EdgeLabel("bought".into()));
    g.add_edge(y, z, EdgeLabel("made_by".into()));
    g
}

/// Находит рёбра, которые встречаются в наборах графов чаще заданного порога
fn find_frequent_edges(
    graphs: &[LabeledGraph], 
    min_support: usize
) -> Vec<((String, String), usize)> {
    
    let mut counts: HashMap<(String, String), usize> = HashMap::new();

    for graph in graphs {
        for edge_idx in graph.edge_indices() {
            let (src, dst) = graph.edge_endpoints(edge_idx).unwrap();
            let src_name = &graph[src].0;
            let dst_name = &graph[dst].0;
            
            let key = (src_name.clone(), dst_name.clone());
            *counts.entry(key).or_insert(0) += 1;
        }
    }

    counts.into_iter()
          .filter(|(_, count)| *count >= min_support)
          .collect()
}

fn main() {
    // Заголовок программы
    println!("Система поиска частых подграфов v0.1");
    println!("Автор: Стренин Денис и Заиченко Андрей, группа ИСП-231");
    println!();
    
    // Читаем аргументы командной строки
    let args = Args::parse();
    
    // Создаём тестовые графы для анализа
    let graphs = vec![create_graph_1(), create_graph_2()];
    
    println!("  Запуск анализа... min_support = {}", args.min_support);
    println!("  Обрабатывается графов: {}", graphs.len());
    
    // Запускаем алгоритм поиска
    let frequent = find_frequent_edges(&graphs, args.min_support);
    
    // Вывод результатов
    println!("\n Результаты поиска:");
    if frequent.is_empty() {
        println!("     Частые паттерны не найдены при заданном пороге");
    } else {
        println!("     Найдено паттернов: {}", frequent.len());
        println!();
        for ((from, to), count) in &frequent {
            println!("    {} → {} | вхождений: {}", from, to, count);
        }
    }
    
    println!("\n✨ Работа завершена успешно");
}