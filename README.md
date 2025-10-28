# 🔒 Criptografia RSA: Resumo Executivo

A Criptografia **RSA** (Rivest, Shamir, Adleman), desenvolvida em 1977/1978, é o sistema de **chave pública (assimétrica)** mais conhecido e amplamente utilizado no mundo.

Sua característica fundamental é o uso de um **par de chaves**:

- 🟢 **Chave Pública**: Usada para **criptografar mensagens**. Pode ser distribuída livremente.  
- 🔒 **Chave Privada**: Usada para **descriptografar mensagens**. Deve ser mantida em segredo absoluto.

A segurança do RSA está firmemente ancorada na **Teoria dos Números** e na **Aritmética Modular**.

O sistema se baseia na escolha de **dois primos distintos e grandes**, `p` e `q`. A segurança é garantida pela dificuldade de fatorar o produto:

```text
n = p * q
O módulo n é o produto dos primos escolhidos:

text
Copiar código
n = p * q
A função totiente de Euler é essencial para o cálculo da chave privada:

text
Copiar código
φ(n) = (p - 1)(q - 1)
O RSA utiliza dois expoentes:

O expoente público e deve ser coprimo com φ(n).

O expoente privado d é o inverso modular de e módulo φ(n):

text
Copiar código
e * d ≡ 1 (mod φ(n))
💡 Obs: O valor de d é calculado usando o Algoritmo Euclidiano Estendido, que permite encontrar o inverso modular de forma eficiente.

Após a escolha dos primos p e q, geramos:

🟢 Chave Pública: (n, e)

🔒 Chave Privada: (n, d)

O texto cifrado C é obtido a partir da mensagem M usando a Chave Pública:

text
Copiar código
C ≡ M^e (mod n)
A mensagem original M é recuperada usando a Chave Privada:

text
Copiar código
M ≡ C^d (mod n)
O método é validado pelo Teorema de Euler:

text
Copiar código
M^(ed) ≡ M (mod n)
Isso garante que o destinatário com a chave privada possa restaurar a mensagem original com precisão.

Embora o RSA seja mais lento que algoritmos simétricos, ele é ideal para sistemas híbridos, combinando a segurança da chave pública com a velocidade da chave simétrica.

O RSA é a base de segurança de protocolos essenciais:

HTTPS: Protocolo seguro de comunicação na web

SSH: Acesso remoto seguro a servidores

O RSA garante que somente o destinatário legítimo, que possui a chave privada, consiga interpretar a mensagem.

O RSA permite certificar a origem de mensagens:

O remetente assina a mensagem usando sua chave privada

O destinatário verifica usando a chave pública correspondente

Assim, é possível comprovar que a mensagem não foi alterada e realmente veio do remetente.

A segurança do RSA depende da dificuldade de fatoração de n.
Atualmente, chaves de 2048 bits são consideradas seguras.

O RSA é vulnerável ao Algoritmo de Shor, que pode fatorar grandes números primos em computadores quânticos.
Isso exige futura migração para Criptografia Pós-Quântica.

📌 Este documento serve como referência completa para estudo, implementação ou documentação de projetos de segurança com RSA.

<img width="1900" height="905" alt="Captura de tela 2025-10-28 191534" src="https://github.com/user-attachments/assets/9d14a1cf-8ac4-49b7-a66b-86bf5244e2dd" />
