# Merge Sort

Este é um projeto que mostra o funcionamento dos algoritmos de ordenação, em especial o Merge Sort.

## Merge Sort: Funcionamento, Vantagens e Desvantagens

O **Merge Sort** é um algoritmo de ordenação baseado na estratégia _divide and conquer_ (dividir e conquistar). Ele divide recursivamente o conjunto de dados em metades menores até que cada sub-conjunto tenha apenas um elemento. Em seguida, essas partes são combinadas (merge) de forma ordenada, resultando na lista final ordenada. Este algoritmo é conhecido por sua estabilidade, o que significa que ele mantém a ordem relativa dos elementos iguais. Sua principal vantagem é o desempenho consistente, com complexidade de tempo de \(O(n log n)\), independentemente da ordem inicial dos dados. Outra vantagem é a eficiência para ordenar grandes conjuntos de dados. No entanto, uma desvantagem significativa é o alto uso de memória, já que ele requer espaço adicional proporcional ao tamanho do conjunto de dados, o que pode ser um problema em sistemas com memória limitada.

## Comparação com Outros Algoritmos

Com base nos resultados obtidos na tabela, o **Merge Sort** se destaca em relação ao **Bubble Sort**, **Selection Sort** e **Insertion Sort**, especialmente para entradas maiores. Enquanto esses algoritmos têm tempos de execução muito elevados devido à sua complexidade \(O(n²)\), o Merge Sort mantém tempos significativamente mais baixos, mesmo para \(n = 100,000\), com apenas 103,72 ms, em comparação com os exorbitantes 235236,73 ms do Bubble Sort.

Comparado ao **Quick Sort**, o Merge Sort apresenta um desempenho inferior em tempo de execução (103,72 ms vs. 80,82 ms para \(n = 100,000\)), mas compensa com sua estabilidade, algo que o Quick Sort não oferece. Além disso, o Merge Sort tem uma maior quantidade de trocas (1.668.928 contra 1.134.811), destacando seu custo adicional de memória.

Em relação ao **Heap Sort**, o Merge Sort é geralmente mais rápido (103,72 ms vs. 132,78 ms para \(n = 100,000\)), embora ambos tenham complexidade \(O(n log n)\). Contudo, o Heap Sort é mais eficiente em termos de uso de memória, já que não requer espaço adicional significativo como o Merge Sort.

Em resumo, o Merge Sort é uma escolha ideal para situações que demandam estabilidade e onde o uso de memória extra não é uma restrição. Para máxima eficiência em tempo, o Quick Sort pode ser preferível, enquanto o Heap Sort é vantajoso em cenários de restrição de memória.

# Resultados obtidos

| Algoritmo      | Tamanho (n) | Tempo (ms) | Trocas     | Comparações |
| -------------- | ----------- | ---------- | ---------- | ----------- |
| Bubble Sort    | 1000        | 39.37      | 243685     | 499500      |
| Selection Sort | 1000        | 22.53      | 996        | 499500      |
| Insertion Sort | 1000        | 26.12      | 243685     | 244675      |
| Merge Sort     | 1000        | 1.45       | 9976       | 8689        |
| Quick Sort     | 1000        | 0.70       | 5025       | 10623       |
| Heap Sort      | 1000        | 1.26       | 9074       | 16840       |
| Bubble Sort    | 10000       | 2650.16    | 24974255   | 49995000    |
| Selection Sort | 10000       | 1458.83    | 9989       | 49995000    |
| Insertion Sort | 10000       | 1601.24    | 24974255   | 24984248    |
| Merge Sort     | 10000       | 10.46      | 133616     | 120504      |
| Quick Sort     | 10000       | 7.83       | 88523      | 156023      |
| Heap Sort      | 10000       | 12.87      | 124187     | 235344      |
| Bubble Sort    | 100000      | 235236.73  | 2505752754 | 4999950000  |
| Selection Sort | 100000      | 139922.73  | 99982      | 4999950000  |
| Insertion Sort | 100000      | 159365.85  | 2505752754 | 2505852741  |
| Merge Sort     | 100000      | 103.72     | 1668928    | 1536159     |
| Quick Sort     | 100000      | 80.82      | 1134811    | 1997654     |
| Heap Sort      | 100000      | 132.78     | 1574736    | 3019291     |
