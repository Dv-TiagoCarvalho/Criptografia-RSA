# Criptografia-RSA - registro de execução

Criptografia RSA: Resumo Executivo
A Criptografia RSA (Rivest, Shamir, Adleman), desenvolvida em 1977 ou 1978, é o sistema de chave pública (assimétrica) mais conhecido e amplamente utilizado. Sua característica principal é o uso de um par de chaves diferentes: a Chave Pública, usada para codificar, e a Chave Privada, usada para decodificar, que deve ser mantida em segredo.

1. Princípios Matemáticos (Fundamentos)
A segurança do RSA está ancorada em conceitos de Teoria dos Números e Aritmética Modular.
1. Números Primos e Fatoração: O sistema exige a escolha de dois primos distintos e grandes, p e q. A segurança é garantida pela dificuldade computacional de fatorar o produto n=p⋅q.
2. Módulo (n): É calculado como o produto dos primos: n=p⋅q.
3. Função Totiente de Euler (φ(n)): É essencial para calcular a chave privada e deve ser mantida em segredo. É calculada por: $$\varphi(n) = (p-1)(q-1) \text{$$$$}$$
4. Chaves e e d: O expoente público e deve ser coprimo com φ(n). O expoente privado d é o inverso modular de e módulo φ(n), satisfazendo a congruência: $$e \cdot d \equiv 1 \pmod{\varphi(n)} \text{$$$$}$$
5. Algoritmos Auxiliares: O valor de d é determinado utilizando o Algoritmo Euclidiano Estendido.

2. Funcionamento (Algoritmo Passo-a-Passo)
Após a escolha dos primos p e q, são geradas a Chave Pública (n,e) e a Chave Privada (n,d).
1. Pré-codificação: A mensagem M é convertida em números (blocos b) tais que 1≤b<n.
2. Codificação (Criptografia): O texto cifrado C é obtido usando a chave pública: $$C \equiv M^e \pmod n \text{$$$$}$$

3. Decodificação (Descriptografia): A mensagem original M é recuperada usando a chave privada: $$M \equiv C^d \pmod n \text{$$$$}$$
4. Por que funciona? O método funciona em virtude do Teorema de Euler, garantindo que M ed ≡M(modn), recuperando a mensagem original.

3. Aplicações Práticas e Segurança
O RSA é vital, apesar de ser mais lento que algoritmos simétricos, o que o torna ideal para uso em sistemas híbridos. É amplamente utilizado em protocolos como HTTPS e SSH.
1. Privacidade e Confidencialidade: O objetivo principal é proteger a mensagem para que somente o destinatário legítimo possa interpretá-la.
2. Autenticação e Assinatura Digital: O RSA também é usado para certificação de mensagens. O remetente usa sua chave privada para criar uma assinatura, que o destinatário verifica usando a chave pública correspondente.
3. Segurança: A segurança depende da inviabilidade da fatoração de N. Para ser seguro, os primos p e q devem ser muito grandes (chaves de 2048 bits são atualmente seguras).
4. Vulnerabilidade Quântica: O RSA é vulnerável ao Algoritmo de Shor, o que exige a futura migração para criptografia pós-quântica

<img width="1900" height="905" alt="Captura de tela 2025-10-28 191534" src="https://github.com/user-attachments/assets/9d14a1cf-8ac4-49b7-a66b-86bf5244e2dd" />
