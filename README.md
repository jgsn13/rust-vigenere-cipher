# Cifra de Vigenère em Rust

Este projeto implementa a Cifra de Vigenère em Rust, permitindo a criptografia e descriptografia de mensagens usando uma chave.

## Funcionalidades

*   Criptografa um texto usando a Cifra de Vigenère.
*   Descriptografa um texto cifrado usando a mesma chave.
*   Utiliza um alfabeto customizável.
*   Lida com letras maiúsculas e minúsculas, convertendo tudo para maiúsculas internamente.

## Como funciona

A Cifra de Vigenère é um método de criptografia polialfabética que utiliza múltiplas cifras de César, com deslocamentos definidos por uma palavra-chave.

**Criptografia:**

A fórmula de criptografia é:

`Eᵢ(xᵢ) = (xᵢ + Kᵢ) mod 26`

Onde:

*   `Eᵢ(xᵢ)`: letra cifrada.
*   `xᵢ`: letra do texto.
*   `Kᵢ`: letra da chave.
*   `mod 26`: operação de módulo 26 (resto da divisão por 26).

Essa fórmula desloca cada letra do texto (`xᵢ`) pela letra correspondente da chave (`Kᵢ`), "dando a volta" no alfabeto se a soma ultrapassar 25 ('Z').

**Descriptografia:**

A fórmula de descriptografia é:

`Dᵢ(xᵢ) = (xᵢ - Kᵢ) mod 26`

Onde:

*   `Dᵢ(xᵢ)`: letra descriptografada.
*   `xᵢ`: letra do texto cifrado.

Essa fórmula inverte o processo de criptografia, subtraindo o valor da chave (`Kᵢ`) da letra cifrada (`xᵢ`). Para evitar números negativos no módulo, algumas implementações usam `(xᵢ - Kᵢ + 26) mod 26`, que é equivalente.

**Repetição da chave:**

Se a chave for menor que o texto, ela é repetida.

## Código (Pseudo-código)

```
função vigenere_criptografar(texto, alfabeto, chave):
  texto_maiusculo = texto em maiúsculas
  chave_maiuscula = chave em maiúsculas
  texto_cifrado = string vazia
  indice_chave = 0

  para cada caractere em texto_maiusculo:
    xi = índice do caractere no alfabeto
    ki = índice do caractere em chave_maiuscula no índice_chave no alfabeto
    indice = (xi + ki) módulo comprimento do alfabeto
    adiciona caractere do alfabeto no índice ao texto_cifrado
    indice_chave = (indice_chave + 1) módulo comprimento da chave

  retorna texto_cifrado

função vigenere_descriptografar(texto_cifrado, alfabeto, chave):
  texto_cifrado_maiusculo = texto_cifrado em maiúsculas
  chave_maiuscula = chave em maiúsculas
  comprimento_alfabeto = comprimento do alfabeto
  texto = string vazia
  indice_chave = 0

  para cada caractere em texto_cifrado_maiusculo:
    xi = índice do caractere no alfabeto
    ki = índice do caractere em chave_maiuscula no índice_chave no alfabeto
    indice = (xi - ki) módulo comprimento do alfabeto (tratando underflow)
    adiciona caractere do alfabeto no índice ao texto
    indice_chave = (indice_chave + 1) módulo comprimento da chave

  retorna texto
```

## Execução

`cargo run`
