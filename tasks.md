# Tasks

## Side Tasks

- Create a custom script in cargo to run the bootstrapper

- Lookup for try catch

- [X] Remove oNode camelCase warning

## Important Tasks

## Etapa 1

- [X] Create `oNode` app (with UDP)

- [X] Definir uma estratégia de construção da rede overlay: Abordagem baseada num controlador

- [ ] Manter as ligações com os vizinhos ativas, começando por monitorar o seu estado: Manda-se um ping periódico com um
  pacote UDP, numa thread numa porta diferente a ouvir "pings". (**Marco**)

## Etapa 2

- Dividir o video em blocos (por causa do UDP)

- [ ] Fazer um servidor que envie o video dividido em pacotes **numerados**. (**Gonçalo**)

- [ ] Fazer um client que receba o video dividido em pacotes **numerados**. (**Diogo**)

- Setup ambiente de teste no core (**all**)

## Etapa 3

- Mensagem de ping: a identificação do servidor, o nº de saltos que a mensagem dá e o instante temporal, interface que
  conduz à melhor rote de volta à fonte. (**Marco**)

## Etapa 4

- [ ] Escolher uma métrica mais favoravel: Sugestão: Latência

- Estratégia 1: Por iniciativa do servidor de streaming, com anúncios periódicos.

## Nodos

- Saber que subscritores existem à sua volta

- Reencaminhar pacotes

- Descobrir o caminho + curto