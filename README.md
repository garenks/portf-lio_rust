# 📈 Análise e Previsão de PIB com Regressão Linear em Rust

Este projeto implementa uma aplicação em Rust para realizar análise estatística de dados históricos do PIB per capita brasileiro, utilizando **regressão linear simples** para prever valores futuros.

---

## 📚 Descrição

A aplicação carrega dados de um arquivo `.json` contendo o PIB per capita por ano, realiza os cálculos estatísticos básicos e aplica regressão linear para:

- Calcular a **inclinação (coeficiente angular)** e o **intercepto** da reta de regressão.
- Estimar o **PIB per capita para o ano de 2030** (ou qualquer outro ano).
- Calcular as métricas de avaliação: **Erro Quadrático Médio (MSE)** e **Coeficiente de Determinação (R²)**.

---

## 🔧 Tecnologias Utilizadas

- 🦀 [Rust](https://www.rust-lang.org/)
- 📦 Crates: `serde`, `serde_json`

---

## 📁 Estrutura do Projeto


---

## 📈 Funcionalidades Principais

### `calcular_media(valores: &[f64]) -> f64`

Calcula a média aritmética de um vetor de valores `f64`.

---

### `calcular_inclinacao_pib(dados: &[RegistroPib]) -> f64`

Aplica a fórmula de regressão linear para encontrar a **inclinação (a)** da reta:

\[
a = \frac{\sum (x_i - \bar{x})(y_i - \bar{y})}{\sum (x_i - \bar{x})^2}
\]

---

### `calcular_intercepto_pib(dados: &[RegistroPib], inclinacao: f64) -> f64`

Calcula o **intercepto (b)** da reta com base na média dos anos e dos valores de PIB:

\[
b = \bar{y} - a \cdot \bar{x}
\]

---

### `prever_pib(intercepto: f64, inclinacao: f64, ano: f64) -> f64`

Utiliza a equação da reta para prever o PIB de um determinado ano:

\[
y = a \cdot x + b
\]

---

### `calcular_r_quadrado_pib(...) -> f64`

Calcula o **coeficiente de determinação (R²)**, que mede a qualidade da regressão:

- **1.0** = ajuste perfeito
- **0.0** = nenhum ajuste

---

### `calcular_erro_quadratico_medio_pib(...) -> f64`

Calcula o **Erro Quadrático Médio (MSE)** entre os valores reais e os previstos, útil para avaliar a precisão.

---

### `ler_dados_pib_de_json(...) -> Result<Vec<RegistroPib>>`

Lê os dados de PIB a partir de um arquivo `.json` estruturado da seguinte forma:

```json
{
  "PIB_per_capita_Brasil": {
    "1996": 5219.36,
    "1997": 5729.02,
    ...
  }
}

