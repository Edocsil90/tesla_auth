# tesla_auth — versão revista (0.16.0)

Gera os tokens da API da Tesla (access token e **refresh token**) através do login oficial da Tesla, numa janela no teu computador. O refresh token é o que se cola na integração **Tesla Custom Integration (alandtse/tesla)** do Home Assistant.

Esta versão parte do projeto original de Adrian Kumpf (licença MIT) e acrescenta:

| Alteração | Porquê |
|---|---|
| Dependências de rede atualizadas (`rustls`, `rustls-webpki`, `webpki-roots`, `quinn-proto`, `tokio`) | Corrige 12 avisos de segurança da RustSec (TLS e certificados) presentes na versão 0.15.0 |
| Sessão privada por defeito | Os cookies do login Tesla deixam de ficar guardados no disco; quem usar o PC a seguir não abre a app já autenticado. `--keep-session` repõe o comportamento antigo |
| Tokens já não são escritos no terminal | Evita que fiquem no histórico da consola ou em logs. `--print` volta a escrevê-los |
| Só navega para `https:` (e `about:`/`data:`/`blob:`) | Bloqueia `http:`, `file:`, `javascript:` e esquemas que abrem outras apps |
| Botões **Copy** na página dos tokens | Copiar sem ter de selecionar à mão |
| Auditoria de segurança automática no CI | Avisa se aparecer uma vulnerabilidade nova nas dependências |

## Como usar (Windows)

1. Em **Releases** deste repositório, descarrega `tesla_auth-x86_64-pc-windows-msvc.zip` e extrai.
2. Executa `tesla_auth.exe` com duplo clique (no Windows 10/11 o WebView2 já vem instalado).
3. Faz login na página da Tesla (com o código de verificação, se tiveres).
4. Carrega em **Copy** ao lado de **Refresh Token**.
5. No Home Assistant: Definições → Dispositivos e serviços → Adicionar integração → **Tesla Custom Integration** → cola o refresh token.

## Cuidados

- O refresh token dá acesso à tua conta Tesla: trata-o como uma palavra-passe e não o envies a ninguém.
- Mudar a palavra-passe da conta Tesla invalida o token.
- O modo `--debug` mostra mais informação; não partilhes a saída dele.

## Avisos que ficam (não corrigíveis aqui)

- `glib 0.18.5` (RUSTSEC-2024-0429, *unsound*) e `proc-macro-error` (*unmaintained*): só existem na versão para **Linux**, vêm da pilha GTK3 usada por `tao`/`wry` e só desaparecem quando esses projetos mudarem para GTK4. Não afetam o Windows nem o macOS.
