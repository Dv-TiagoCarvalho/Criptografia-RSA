# 🔐 Criptografia RSA: Resumo Executivo

A Criptografia RSA (Rivest, Shamir, Adleman), desenvolvida em 1977/1978, é o sistema de **chave pública (assimétrica) mais conhecido e amplamente utilizado no mundo**.

Sua característica fundamental é o uso de um **par de chaves**:

* **Chave Pública:** Usada para **codificar (criptografar)** mensagens. Pode ser distribuída livremente.
* **Chave Privada:** Usada para **decodificar (descriptografar)** mensagens. Deve ser mantida em **segredo absoluto**.

---

## 1. ⚙️ Princípios Matemáticos (Fundamentos)

A segurança do RSA está firmemente ancorada na **Teoria dos Números** e na **Aritmética Modular**.

### Números Primos e Fatoração

O sistema se baseia na escolha de dois primos distintos e muito grandes, $p$ e $q$. A segurança é garantida pela **dificuldade computacional de fatorar** o produto $n = p \cdot q$.

| Componente | Cálculo | Função |
| :--- | :--- | :--- |
| **Módulo ($n$)** | $n = p \cdot q$ | Base da operação (público). |
| **Função Totiente de Euler ($\varphi(n)$)** | $\varphi(n) = (p-1)(q-1)$ | Essencial para o cálculo da chave privada. |

### Geração de Chaves

O expoente público $e$ deve ser coprimo com $\varphi(n)$. O expoente privado $d$ é o inverso modular de $e$ módulo $\varphi(n)$, satisfazendo a congruência:

$$e \cdot d \equiv 1 \pmod{\varphi(n)}$$

> **Nota:** O valor de $d$ é determinado utilizando o **Algoritmo Euclidiano Estendido**.

---

## 2. 🔢 Funcionamento (Algoritmo Principal)

Após a escolha dos primos $p$ e $q$, são geradas a **Chave Pública** $(n, e)$ e a **Chave Privada** $(n, d)$.

### Criptografia (Codificação)

O texto cifrado $C$ é obtido a partir da mensagem $M$ (convertida em blocos numéricos) usando a **Chave Pública $(n, e)$**:

$$C \equiv M^e \pmod n$$

### Descriptografia (Decodificação)

A mensagem original $M$ é recuperada a partir do texto cifrado $C$ usando a **Chave Privada $(n, d)$**:

$$M \equiv C^d \pmod n$$

> **Validação:** O método é validado pelo **Teorema de Euler**, que garante a recuperação da mensagem original: $M^{ed} \equiv M \pmod n$.

---

## 3. 🛡️ Aplicações Práticas e Segurança

O RSA é vital, embora seja mais lento que algoritmos simétricos, o que o torna ideal para uso em **sistemas híbridos**.

* **Protocolos de Uso:** É o pilar de segurança em protocolos como **HTTPS** e **SSH**.
* **Privacidade e Confidencialidade:** Protege a mensagem, garantindo que somente o destinatário legítimo (que possui a chave privada) possa interpretá-la.
* **Autenticação e Assinatura Digital:** Permite a certificação da origem da mensagem. O remetente assina usando sua chave privada, e o destinatário verifica usando a chave pública correspondente.
* **Nível de Segurança Atual:** A segurança depende da inviabilidade da fatoração de $N$. Chaves de **2048 bits** são consideradas seguras.

### Vulnerabilidade Quântica (Ameaça Futura)

O RSA é vulnerável ao **Algoritmo de Shor**. Essa ameaça exige a futura migração para soluções de **Criptografia Pós-Quântica**.

                                                                Registro de execução
<img width="1900" height="905" alt="Captura de tela 2025-10-28 191534" src="https://github.com/user-attachments/assets/7918e773-7300-47fe-bc73-7ced2daea40f" />

