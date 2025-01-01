# Cifra de Vigenère em Rust

Este projeto implementa a Cifra de Vigenère em Rust, permitindo a criptografia e descriptografia de mensagens usando uma chave.

## Funcionalidades

*   Criptografa um texto plano usando a Cifra de Vigenère.
*   Descriptografa um texto cifrado usando a mesma chave.
*   Utiliza um alfabeto customizável.
*   Lida com letras maiúsculas e minúsculas, convertendo tudo para maiúsculas internamente.

## Como funciona

A Cifra de Vigenère é um método de criptografia polialfabética que usa uma série de cifras de César diferentes com base nas letras de uma palavra-chave. Para criptografar, cada letra do texto plano é deslocada no alfabeto por um número correspondente à posição da letra da chave. A chave se repete se for menor que o texto plano. A descriptografia inverte esse processo.

## Código (Pseudo-código)

```
função vigenere_criptografar(texto_plano, alfabeto, chave):
  texto_plano_maiusculo = texto_plano em maiúsculas
  chave_maiuscula = chave em maiúsculas
  texto_cifrado = string vazia
  indice_chave = 0

  para cada caractere em texto_plano_maiusculo:
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
  texto_plano = string vazia
  indice_chave = 0

  para cada caractere em texto_cifrado_maiusculo:
    xi = índice do caractere no alfabeto
    ki = índice do caractere em chave_maiuscula no índice_chave no alfabeto
    indice = (xi - ki) módulo comprimento do alfabeto (tratando underflow)
    adiciona caractere do alfabeto no índice ao texto_plano
    indice_chave = (indice_chave + 1) módulo comprimento da chave

  retorna texto_plano
```

## Execução

`cargo run`
