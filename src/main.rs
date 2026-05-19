// Система поиска частых подграфов

use petgraph::graph::Graph;
use petgraph::Directed;

/// Метка вершины (например, "Человек", "Фильм")
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct VertexLabel(pub String);

/// Метка ребра (например, "дружит", "смотрел")
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct EdgeLabel(pub String);

/// Тип нашего графа: ориентированный, с метками на вершинах и рёбрах
type LabeledGraph = Graph<VertexLabel, EdgeLabel, Directed>;

fn main() {
    println!("Структуры данных инициализированы");
}