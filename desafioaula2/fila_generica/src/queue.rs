// Define a estrutura `Node` genérica, que será usada como o elemento da fila.
// O tipo `T` permite que o nó armazene qualquer tipo de dado.
pub struct Node<T> {
    value: T,  // Armazena o valor do nó, que pode ser de qualquer tipo `T`.
    next: Option<Box<Node<T>>>, // Aponta para o próximo nó na lista, se houver. Usamos `Box` para alocar a memória dinamicamente.
}

// Define a estrutura `Queue` genérica, que representa uma fila.
// A fila também é genérica e pode armazenar dados de qualquer tipo `T`.
pub struct Queue<T> {
    head: Option<Box<Node<T>>>, // Aponta para o primeiro nó da fila.
    tail: *mut Node<T>, // Aponta para o último nó da fila (usando ponteiro bruto para evitar custos de segurança).
    len: usize, // Armazena o tamanho da fila.
}

// Implementa os métodos para a estrutura `Queue`.
impl<T> Queue<T> {
    // Cria e retorna uma nova instância da fila.
    // Inicializa a fila com `head` e `tail` como nulos e o tamanho como 0.
    pub fn new() -> Self {
        Queue {
            head: None, // Não há elementos inicialmente.
            tail: std::ptr::null_mut(), // O último nó também é nulo no início.
            len: 0, // O tamanho da fila começa em 0.
        }
    }

    // Adiciona um novo elemento `elem` à fila.
    pub fn enqueue(&mut self, elem: T) {
        // Cria um novo nó para o elemento e inicializa o próximo nó como `None`.
        let new_node = Box::new(Node {
            value: elem,
            next: None,
        });

        // Converte o nó em um ponteiro bruto para manipulação com segurança no bloco `unsafe`.
        let raw_node = Box::into_raw(new_node);

        unsafe {
            // Se a fila está vazia (não há elementos), o novo nó será tanto o primeiro quanto o último.
            if self.tail.is_null() {
                self.head = Some(Box::from_raw(raw_node)); // O `head` agora aponta para o novo nó.
                self.tail = raw_node; // O `tail` também aponta para o novo nó.
            } else {
                // Se a fila não está vazia, adicionamos o novo nó após o nó atual no `tail`.
                (*self.tail).next = Some(Box::from_raw(raw_node)); // O próximo nó de `tail` aponta para o novo nó.
                self.tail = raw_node; // Atualizamos o `tail` para o novo nó.
            }
        }
        self.len += 1; // Aumenta o tamanho da fila.
    }

    // Remove o primeiro elemento da fila e o retorna, se houver.
    pub fn dequeue(&mut self) -> Option<T> {
        // Tenta pegar o primeiro nó da fila. Se existir, a operação é realizada.
        self.head.take().map(|mut node| {
            // Atualiza o `head` para o próximo nó.
            self.head = node.next.take();
            
            // Se a fila ficou vazia após a remoção, o `tail` também deve ser nulo.
            if self.head.is_none() {
                self.tail = std::ptr::null_mut();
            }
            self.len -= 1; // Diminui o tamanho da fila.
            node.value // Retorna o valor do nó removido.
        })
    }

    // Retorna uma referência ao primeiro elemento da fila, sem removê-lo.
    pub fn peek(&self) -> Option<&T> {
        // Se houver um nó na frente, retorna uma referência ao seu valor.
        self.head.as_ref().map(|node| &node.value)
    }

    // Retorna o número atual de elementos na fila.
    pub fn len(&self) -> usize {
        self.len // Retorna o tamanho armazenado.
    }
}
