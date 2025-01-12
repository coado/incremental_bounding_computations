
# Incremental Calculation of the Objective Function in Local Search Algorithms


<p align="center" width="100%">
  <img src="https://github.com/user-attachments/assets/ab1a9070-6544-49fe-87e9-5beeae1824d9" alt="Computation Graph 1" title="Computation Graph 1" width="50%" />
</p>
<p align="center" width="100%">
<em>Figure 1: Computation Graph for Initial State</em>
</p>

<p align="center" width="100%">
  <img src="https://github.com/user-attachments/assets/d8ed063b-f362-4c27-942c-a7f11674ef89" alt="Computation Graph 1" title="Computation Graph 1" width="50%" />
</p>

<p align="center" width="100%">
<em>Figure 2: Computation Graph after the change in the input vertex "a" presenting the propagation of the dirty state</em>
</p>


## Abstract

The thesis investigates the application of incremental calculation through computational graphs in local search methods to tackle NP-hard problems, specifically focusing on the Traveling Salesman Problem (TSP) and the Graph Coloring Problem. Incremental computation, a strategy where computations are updated based on changes in input data, potentially reduces processing time by minimizing redundant calculations. The study implements and benchmarks Directed Computational Graphs (DCGs) to incrementally calculate objective functions. It describes libraries that enable straightforward creation and discusses various designs of computational graphs. Results reveal significant variance in performance, with computational graphs often introducing overhead that may not necessarily translate to improved efficiency. In some cases, such as with TSP and Graph Coloring, traditional methods outperformed the incremental approach, indicating that the utility of computational graphs is highly context-dependent. Although computational graphs demonstrated potential under specific conditions, the research underscores the importance of meticulously considering when and how to integrate them into problem-solving frameworks, particularly in computationally complex scenarios.


