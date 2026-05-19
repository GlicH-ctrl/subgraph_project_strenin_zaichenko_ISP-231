// Система поиска частых подграфов
use std::collections::HashMap;
use petgraph::graph::Graph;
use petgraph::Directed;
use clap::Parser;

/// Параметры запуска программы из терминала
#[derive(Parser, Debug)]
#[command(author = "Стренин Денис Заиченко Андрей, ИСП-231", version = "0.1.0", about = "Поиск частых подграфов")]
struct Args {
    /// Минимальное количество вхождений подграфа
    #[arg(short, long, default_value_t = 2)]
    min_support: usize,
}

/// Метка вершины (например, "Человек", "Фильм")
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct VertexLabel(pub String);

/// Метка ребра (например, "дружит", "смотрел")
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct EdgeLabel(pub String);

/// Тип нашего графа: ориентированный, с метками на вершинах и рёбрах
type LabeledGraph = Graph<VertexLabel, EdgeLabel, Directed>;

/// Создаёт первый тестовый граф: A -> B -> C
fn create_graph_1() -> LabeledGraph {
    let mut g = LabeledGraph::new_directed();
    let a = g.add_node(VertexLabel("User".into()));
    let b = g.add_node(VertexLabel("Product".into()));
    let c = g.add_node(VertexLabel("Category".into()));
    
    g.add_edge(a, b, EdgeLabel("bought".into()));
    g.add_edge(b, c, EdgeLabel("belongs_to".into()));
    g
}

/// Создаёт второй тестовый граф: X -> Y -> Z
fn create_graph_2() -> LabeledGraph {
    let mut g = LabeledGraph::new_directed();
    let x = g.add_node(VertexLabel("User".into()));
    let y = g.add_node(VertexLabel("Product".into()));
    let z = g.add_node(VertexLabel("Brand".into()));
    
    g.add_edge(x, y, EdgeLabel("bought".into()));  // совпадает с первым
    g.add_edge(y, z, EdgeLabel("made_by".into()));
    g
}

/// Находит рёбра, которые встречаются в наборах графов чаще заданного порога
/// Возвращает: ((метка_откуда, метка_куда), количество_вхождений)
fn find_frequent_edges(graphs: &[LabeledGraph], min_support: usize) -> Vec<((String, String), usize)> {
    let mut counts: HashMap<(String, String), usize> = HashMap::new();

    // Проходим по всем графам
    for graph in graphs {
        // Проходим по каждому ребру
        for edge_idx in graph.edge_indices() {
            let (src, dst) = graph.edge_endpoints(edge_idx).unwrap();
            let src_name = &graph[src].0;
            let dst_name = &graph[dst].0;
            
            // Считаем встречи этой пары меток
            let key = (src_name.clone(), dst_name.clone());
            *counts.entry(key).or_insert(0) += 1;
        }
    }

    // Оставляем только те, что встретились >= min_support раз
    counts.into_iter()
          .filter(|(_, count)| *count >= min_support)
          .collect()
}

fn main() {
    let args = Args::parse(); // Читаем аргументы
    let graphs = vec![create_graph_1(), create_graph_2()];
    
    let frequent = find_frequent_edges(&graphs, args.min_support);
    
    println!("🔍 Поиск частых рёбер (min_support = {}):", args.min_support);
    if frequent.is_empty() {
        println!("   (ничего не найдено)");
    } else {
        for ((from, to), count) in frequent {
            println!("   {} → {} : {} раз", from, to, count);
        }
    }
}