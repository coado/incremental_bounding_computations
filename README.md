
# Incremental Calculation of the Objective Function in Local Search Algorithms

<img width="753" alt="graph-coloring-comp-1" src="https://github.com/user-attachments/assets/5c2f4a8f-e0fb-4a52-8605-aec90bbeffa3" />


## Abstract

The thesis investigates the application of incremental calculation through computational graphs in local search methods to tackle NP-hard problems, specifically focusing on the Traveling Salesman Problem (TSP) and the Graph Coloring Problem. Incremental computation, a strategy where computations are updated based on changes in input data, potentially reduces processing time by minimizing redundant calculations. The study implements and benchmarks Directed Computational Graphs (DCGs) to incrementally calculate objective functions. It describes libraries that enable straightforward creation and discusses various designs of computational graphs. Results reveal significant variance in performance, with computational graphs often introducing overhead that may not necessarily translate to improved efficiency. In some cases, such as with TSP and Graph Coloring, traditional methods outperformed the incremental approach, indicating that the utility of computational graphs is highly context-dependent. Although computational graphs demonstrated potential under specific conditions, the research underscores the importance of meticulously considering when and how to integrate them into problem-solving frameworks, particularly in computationally complex scenarios.


