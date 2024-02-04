
### const_itoa
pequeno exemplo demonstrando uma função que transforma um número evaludado em contexto genérico em uma string em tempo de compilação

parece que transformar um número obtido de forma constante fora de contexto generico em uma string estática é impossível já que é impossível fazer o compilador alocar espaço no `.text` dependendo da execução de uma função constante

mas esse exemplo usa genéricos que fazem o compilador gerar funções para cada instanciação do genérico, e essas funções podem sim ter `const`

isso pode ser útil caso você tenha uma struct como `MeuArrayStatico<const NUM: usize>` e precisa gerar uma string constante que descreve o struct,

por exemplo, você poderia adicionar uma constante `NOME` ao struct que teria o valor `const NOME: &'static str = "MeuArrayStatico<123>"` quando NUM fosse 123

fazer algo assim é impossível sem a técnica demonstrada aqui
