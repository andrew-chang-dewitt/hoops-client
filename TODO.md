TO DO
===

- [x] experiment w/ axum-auth-sessions
- [ ] roll own cookie-auth context util
  - [/] learn how to set headers using axum in response sent by server fn
        - get response object from context in server fn:
          https://github.com/Indrazar/auth-sessions-example/blob/14817a048995a96ef1105abf502ad3e2b923b302/src/cookies.rs#L38
  - [ ] stuff token & user id into cookie & set as httponly in response sent by successful `login` fn
        - this approach keeps the server stateless and lets the API still manage token validity time?
        - set cookie header on response in server fn:
          https://github.com/Indrazar/auth-sessions-example/blob/14817a048995a96ef1105abf502ad3e2b923b302/src/cookies.rs#L49
  - [ ] learn how to wrap cookie auth util in leptos context so it's retrievable with `leptos::use_context`
      tips on managing route conntext and setting headers/cookies:
      - https://discord.com/channels/1031524867910148188/1072251332448223343/1076666884612689920
      - https://github.com/leptos-rs/leptos/blob/6c31d09eb2cb471eebfc1b6199bfd2616bd4df3a/integrations/axum/src/lib.rs#L1020
      -  learn how to guard routes behind auth with leptos:
      https://discord.com/channels/1031524867910148188/1076688518014832640/1080282063455932427

