# Construindo uma Blockchain em Rust: Um Guia Passo a Passo

A blockchain é uma tecnologia revolucionária que tem o potencial de transformar muitos setores, desde finanças até cadeias de suprimentos. Neste artigo, vamos explorar como construir uma blockchain simples em Rust, uma linguagem de programação conhecida por sua segurança e desempenho. Vamos analisar cada componente e funcionalidade que compõem nossa blockchain.

## Componentes da Blockchain

### Header

O cabeçalho de um bloco contém informações essenciais, como o nonce (número usado apenas uma vez) e o hash do bloco. Em Rust, podemos representar o cabeçalho com a seguinte estrutura:

```rust
pub struct Header {
    pub nonce: i32,
    pub block_hash: String,
}
```

### Payload

Os dados específicos de um bloco, excluindo o cabeçalho, são armazenados no payload. Isso pode incluir informações como sequência, carimbo de data/hora, dados do bloco e o hash do bloco anterior. Em Rust, podemos definir o payload da seguinte forma:

```rust
pub struct Payload {
    pub sequence: i32,
    pub timestamp: Date,
    pub data: String,
    pub previous_hash: String,
}
```

### Bloco

Um bloco na blockchain é uma combinação do cabeçalho e do payload. Essa estrutura encapsula todas as informações necessárias para representar um bloco completo na cadeia de blocos. Aqui está a definição em Rust:

```rust
pub struct Block {
    pub header: Header,
    pub payload: Payload,
}
```

### Funções Hash e Prova de Trabalho

Para garantir a integridade e segurança da blockchain, utilizamos funções hash e prova de trabalho. A função hash é responsável por calcular o hash de uma sequência de bytes, enquanto a prova de trabalho verifica se um hash atende a determinados critérios de dificuldade. Aqui estão as implementações em Rust:

```rust
pub fn hash(data: impl AsRef<[u8]>) -> String {
    // Implementação da função hash
}

pub fn isHashProofed(hash: String, difficulty: i32, prefix: String) -> bool {
    // Implementação da prova de trabalho
}
```
### Como funciona a prova de trabalho(PoW) e mineração:

**Como funciona:**
- A função `mineblock` itera por diferentes valores de nonce (um número usado apenas uma vez) até encontrar um hash válido para um novo bloco.
- Em cada iteração, o hash do payload fornecido é concatenado com o nonce atual para calcular um novo hash.
- Esse processo continua até que um hash adequado seja encontrado, que atenda aos critérios de dificuldade especificados pela função `isHashProofed`.
**Propósito:**
- O propósito da função `mineblock` é adicionar novos blocos à blockchain de forma segura e confiável.
- Ela utiliza o mecanismo de prova de trabalho (Proof of Work - PoW) para garantir que apenas blocos válidos sejam adicionados à blockchain.
- Ao fazer isso, a função contribui para a segurança e integridade da blockchain, tornando-a resistente a fraudes e ataques maliciosos.

### Verificar prova:

**Como funciona:**
- A função verifica se um hash começa com um prefixo específico, repetido várias vezes de acordo com a dificuldade desejada.
- Se o hash atender a essa condição, a função retorna verdadeiro, indicando que o hash atende aos critérios de dificuldade. Caso contrário, retorna falso.
**Propósito:**
- A função `isHashProofed` é usada para garantir que um hash calculado atenda aos requisitos de dificuldade estabelecidos pela prova de trabalho.
- Isso é essencial para validar a autenticidade de um bloco minerado e garantir que apenas blocos válidos sejam adicionados à blockchain.


## Estrutura Blockchain

A estrutura `Blockchain` é o coração de nosso sistema blockchain. Ela gerencia a adição, validação e visualização de blocos na cadeia de blocos. Aqui estão os métodos principais:

- **Método `new`:** Inicializa uma nova instância da blockchain com os parâmetros especificados, como dificuldade e prefixo de prova de trabalho.
- **Método `create_genesis_block`:** Cria o bloco de gênese inicial da blockchain.
- **Método `show_blocks`:** Exibe os blocos existentes na blockchain.
- **Método `mine_block`:** Realiza o processo de mineração para adicionar um novo bloco à blockchain.
- **Método `verify_block`:** Verifica a validade de um bloco antes de adicioná-lo à blockchain.
- **Método `push_block`:** Adiciona um bloco à lista de blocos da blockchain.
- **Método `print_blockchain`:** Imprime a blockchain na saída padrão para visualização.

## New blockchain:

```rust
fn main() {
    let mut blockchain = blockchain::Blockchain::new(4, "0".to_string());
}
```
**Como funciona:**
- O método `new` é o construtor da estrutura Blockchain e é chamado para criar uma nova instância da blockchain.
- Ele recebe como parâmetros a dificuldade desejada e o prefixo para a prova de trabalho.
- No início do método, uma nova instância da blockchain é criada com uma lista de blocos vazia.
- Em seguida, o método chama a função `create_genesis_block` para gerar o bloco de gênese inicial da blockchain.
- Após a criação do bloco de gênese, a instância da blockchain é retornada como resultado.
**Propósito:**
- O propósito do método `new` é inicializar uma nova blockchain com os parâmetros especificados, como dificuldade e prefixo de prova de trabalho.
- Ele é responsável por criar a estrutura inicial da blockchain, incluindo a adição do bloco de gênese.
- Ao criar a blockchain, o método estabelece as bases para o funcionamento da rede, garantindo que o primeiro bloco seja criado corretamente e que a mineração possa começar a partir desse ponto.




## Conclusão

Construir uma blockchain nos permite explorar os conceitos fundamentais por trás dessa tecnologia. Compreender os componentes básicos, como cabeçalhos, payloads e blocos, e as funcionalidades essenciais, como funções hash e prova de trabalho, é fundamental para criar uma blockchain robusta e segura.

Espero que este guia tenha sido útil para entender como construir uma blockchain simples em Rust. Com esses fundamentos em mãos, você está pronto para explorar e experimentar novas ideias e aplicações.
