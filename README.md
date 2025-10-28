<!DOCTYPE html>
<html lang="pt-BR">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Criptografia RSA: Resumo Executivo</title>
    <style>
        body { font-family: Arial, sans-serif; line-height: 1.6; margin: 20px; }
        h1 { color: #2c3e50; border-bottom: 2px solid #3498db; padding-bottom: 10px; }
        h2 { color: #2980b9; margin-top: 30px; }
        h3 { color: #34495e; }
        .key-pair strong { color: #e74c3c; }
        .math-formula { background-color: #ecf0f1; padding: 10px; border-radius: 5px; overflow-x: auto; margin: 15px 0; }
        .table-container { overflow-x: auto; }
        table { width: 100%; border-collapse: collapse; margin: 20px 0; }
        th, td { border: 1px solid #bdc3c7; padding: 12px; text-align: left; }
        th { background-color: #3498db; color: white; }
        .note { border-left: 4px solid #f39c12; background-color: #fffaf4; padding: 10px; margin: 15px 0; }
    </style>
</head>
<body>

    <h1>🔒 Criptografia RSA: Resumo Executivo</h1>

    <p>A Criptografia RSA (Rivest, Shamir, Adleman), desenvolvida em 1977/1978, é o sistema de **chave pública (assimétrica) mais conhecido e amplamente utilizado no mundo**.</p>

    <p>Sua característica fundamental é o uso de um par de chaves:</p>
    <ul>
        <li class="key-pair"><strong>Chave Pública:</strong> Usada para **codificar (criptografar)** mensagens. Pode ser distribuída livremente.</li>
        <li class="key-pair"><strong>Chave Privada:</strong> Usada para **decodificar (descriptografar)** mensagens. Deve ser mantida em segredo absoluto.</li>
    </ul>

    <hr>

    <h2>1. ⚙️ Princípios Matemáticos (Fundamentos)</h2>

    <p>A segurança do RSA está firmemente ancorada na **Teoria dos Números** e na **Aritmética Modular**.</p>

    <h3>Números Primos e Fatoração</h3>
    <p>O sistema se baseia na escolha de dois primos distintos e muito grandes, $p$ e $q$. A segurança é garantida pela **dificuldade computacional de fatorar** o produto $n = p \cdot q$.</p>

    <div class="table-container">
        <table>
            <thead>
                <tr>
                    <th>Componente</th>
                    <th>Cálculo</th>
                    <th>Função</th>
                </tr>
            </thead>
            <tbody>
                <tr>
                    <td><strong>Módulo ($n$)</strong></td>
                    <td>$n = p \cdot q$</td>
                    <td>Base da operação (público).</td>
                </tr>
                <tr>
                    <td><strong>Função Totiente de Euler ($\varphi(n)$)</strong></td>
                    <td>
                        <div class="math-formula">
                            $$\varphi(n) = (p-1)(q-1)$$
                        </div>
                    </td>
                    <td>Essencial para o cálculo da chave privada.</td>
                </tr>
            </tbody>
        </table>
    </div>

    <h3>Chaves Públicas e Privadas ($e$ e $d$)</h3>
    <p>O expoente público $e$ deve ser coprimo com $\varphi(n)$. O expoente privado $d$ é o inverso modular de $e$ módulo $\varphi(n)$, satisfazendo a congruência:</p>
    <div class="math-formula">
        $$e \cdot d \equiv 1 \pmod{\varphi(n)}$$
    </div>

    <div class="note">
        <strong>Algoritmos Auxiliares:</strong> O valor de $d$ é determinado utilizando o **Algoritmo Euclidiano Estendido**.
    </div>

    <hr>

    <h2>2. 🔢 Funcionamento (Algoritmo Principal)</h2>
    <p>Após a escolha dos primos $p$ e $q$, são geradas a Chave Pública $(n, e)$ e a Chave Privada $(n, d)$.</p>

    <h3>Criptografia (Codificação)</h3>
    <p>O texto cifrado $C$ é obtido a partir da mensagem $M$ (convertida em blocos numéricos) usando a Chave Pública $(n, e)$:</p>
    <div class="math-formula">
        $$C \equiv M^e \pmod n$$
    </div>

    <h3>Descriptografia (Decodificação)</h3>
    <p>A mensagem original $M$ é recuperada a partir do texto cifrado $C$ usando a Chave Privada $(n, d)$:</p>
    <div class="math-formula">
        $$M \equiv C^d \pmod n$$
    </div>

    <p><strong>Por que funciona?</strong> O método é validado pelo **Teorema de Euler**, que garante a recuperação da mensagem original: $M^{ed} \equiv M \pmod n$.</p>

    <hr>

    <h2>3. 🛡️ Aplicações Práticas e Segurança</h2>

    <p>O RSA é vital, embora seja mais lento que algoritmos simétricos, o que o torna ideal para uso em **sistemas híbridos**.</p>

    <ul>
        <li><strong>Protocolos de Uso:</strong> É o pilar de segurança em protocolos como **HTTPS e SSH**.</li>
        <li><strong>Privacidade e Confidencialidade:</strong> Protege a mensagem, garantindo que somente o destinatário legítimo (que possui a chave privada) possa interpretá-la.</li>
        <li><strong>Autenticação e Assinatura Digital:</strong> Permite a certificação da origem da mensagem. O remetente assina usando sua chave privada, e o destinatário verifica usando a chave pública correspondente.</li>
        <li><strong>Nível de Segurança Atual:</strong> A segurança depende da inviabilidade da fatoração de $N$. Atualmente, chaves de **2048 bits** são consideradas seguras.</li>
    </ul>

    <h3>Vulnerabilidade Quântica</h3>
    <p>O RSA é vulnerável ao **Algoritmo de Shor**. Essa ameaça exige a futura migração para soluções de **Criptografia Pós-Quântica**.</p>

</body>
</html>
