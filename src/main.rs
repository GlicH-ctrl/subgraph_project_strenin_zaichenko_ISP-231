// Система поиска частых подграфов
// Студенты: Стренин Денис, Заиченко Андрей, ИСП-231

use petgraph::graph::Graph;
use petgraph::Directed;
use std::collections::HashMap;
use std::io::{self, BufRead, Write};
use clap::Parser;

/// Метка вершины (буквы, цифры, символы)
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct VertexLabel(pub String);

/// Метка ребра (автоматически присваивается)
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct EdgeLabel(pub String);

/// Тип графа: ориентированный, с метками
type LabeledGraph = Graph<VertexLabel, EdgeLabel, Directed>;

/// Аргументы командной строки
#[derive(Parser, Debug)]
#[command(
    author = "Стренин Денис, Заиченко Андрей, ИСП-231", 
    version = "0.4.0", 
    about = "Поиск частых подграфов (интерактивный ввод)"
)]
struct Args {
    /// Минимальное количество вхождений подграфа
    #[arg(short, long, default_value_t = 2)]
    min_support: usize,
}

/// Создаёт граф из списка вершин, соединяя их цепочкой: v0→v1→v2...
fn create_graph_from_vertices(vertices: Vec<String>) -> LabeledGraph {
    let mut g = LabeledGraph::new();
    
    // Для поиска частых рёбер нужно минимум 2 вершины
    if vertices.len() < 2 {
        return g;
    }
    
    // 1. Добавляем вершины
    let mut node_indices = Vec::with_capacity(vertices.len());
    for label in &vertices {
        node_indices.push(g.add_node(VertexLabel(label.clone())));
    }
    
    // 2. Соединяем последовательно: 0→1, 1→2, 2→3...
    for i in 0..(node_indices.len() - 1) {
        g.add_edge(node_indices[i], node_indices[i + 1], EdgeLabel("link".into()));
    }
    
    g
}

/// Интерактивный ввод с лимитами (10 графов, по 5 вершин)
fn read_graphs_interactively() -> Vec<LabeledGraph> {
    let stdin = io::stdin();
    let mut graphs = Vec::new();
    
    //  Вывод предупреждений и правил ввода
    println!("\n ВВОД ГРАФОВ");
    println!("ОГРАНИЧЕНИЯ СИСТЕМЫ:");
    println!("   • Максимум графов: 10");
    println!("   • Максимум вершин в одном графе: 5");
    println!("   • Вводите вершины через пробел (пример: A1 B2 C3)");
    println!("   • Программа автоматически соединит их в цепочку");
    println!("   • Чтобы закончить ввод раньше → оставьте строку ПУСТОЙ и нажмите Enter\n");
    
    // Цикл строго на 10 попыток ввода
    for graph_num in 1..=10 {
        println!("--- Граф #{} из 10 ---", graph_num);
        print!("Введите вершины (макс. 5): ");
        let _ = io::stdout().flush(); // Сброс буфера вывода
        
        let mut line = String::new();
        if stdin.lock().read_line(&mut line).is_err() {
            break; // Ошибка чтения (Ctrl+C / EOF)
        }
        
        let line = line.trim();
        
        //  Пустая строка = завершить ввод
        if line.is_empty() {
            println!("\n Ввод завершён по запросу пользователя. Всего графов: {}", graphs.len());
            break;
        }
        
        // Парсим вершины
        let vertices: Vec<String> = line.split_whitespace().map(|s| s.to_string()).collect();
        
        // 🔹 Валидация: максимум 5 вершин
        if vertices.len() > 5 {
            println!("  Ошибка: введено {} вершин, но лимит — 5! Попробуйте снова.\n", vertices.len());
            continue; // Переходим к следующей итерации (граф # не меняется в счётчике цикла)
        }
        
        // 🔹 Валидация: минимум 2 вершины (иначе не будет рёбер)
        if vertices.len() < 2 {
            println!("  Ошибка: нужно минимум 2 вершины для создания графа.\n");
            continue;
        }
        
        // Всё ок => создаём граф и сохраняем
        println!("   ↪ Принято вершин: {}", vertices.len());
        let graph = create_graph_from_vertices(vertices);
        graphs.push(graph);
        println!();
    }
    
    graphs
}

/// Находит частые рёбра в наборе графов
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
    
    println!(" Система поиска частых подграфов v0.4");
    println!(" Авторы: Стренин Денис, Заиченко Андрей, группа ИСП-231");
    println!(" Учебный проект, 2026 г.\n");
    
    // Запуск интерактивного ввода
    let graphs = read_graphs_interactively();
    
    // Если графов нет — выходим
    if graphs.is_empty() {
        println!("\n  Не введено ни одного графа. Завершение работы.");
        return;
    }
    
    // Анализ
    println!("\n  Запуск анализа...");
    println!(" Графов для обработки: {}", graphs.len());
    println!(" Порог поддержки (min_support): {}", args.min_support);
    
    let frequent = find_frequent_edges(&graphs, args.min_support);
    
    // Результаты
    println!("\n РЕЗУЛЬТАТЫ ПОИСКА:");
    println!("   {}", "=".repeat(45));
    
    if frequent.is_empty() {
        println!("    Частые паттерны не найдены");
        println!("    Совет: уменьшите min_support или введите больше похожих графов");
    } else {
        println!("    Найдено частых паттернов: {}", frequent.len());
        println!();
        for ((from, to), count) in &frequent {
            println!("    {} → {} | вхождений: {}", from, to, count);
        }
    }
    
    println!("   {}", "=".repeat(45));
    println!("\n Работа завершена успешно");
}