# StudentCoin

Predmet: Programiranje Blockchaina, Tehnički fakultet, SII, 2026/2027.

- Ed25519 novčanici čija je adresa hex(SHA-256(public key)).
- Potpisane transakcije čekaju u mempool-u.
- Proof of work hash bloka počinje sa 0000.
- Miner dobija nagradu od 50 StudentCoina.
- Nonce sprečava ponavljanje transakcija.
- HTTP API je napravljen u Axumu.

Pokretanje (Rust 2024):

```bash
git clone https://github.com/anastasija-lukic/studentcoin-alukic.git
cd studentcoin-alukic
cargo run --release
```

Nalazi se na adresi http://127.0.0.1:3000.\
Primeri su za Windows sisteme.

Ceo lanac:

```bash
curl.exe http://127.0.0.1:3000/chain
```

Mining bloka:

```bash
curl.exe "http://127.0.0.1:3000/mine?miner=<adresa>"
```

Stanje adrese:

```bash
curl.exe http://127.0.0.1:3000/balance/<adresa>
```

```json
{ "address": "...", "balance": 50, "next_nonce": 0 }
```

Nova transakcija:

```bash
curl.exe -X POST http://127.0.0.1:3000/transactions/new -H "Content-Type: application/json" --data-binary "@podaci/transakcija.json"
```

```json
{
  "sender": "<adresa>",
  "receiver": "<adresa>",
  "amount": 20,
  "nonce": 0,
  "public_key": "<public key>",
  "signature": "<128 hex karaktera>"
}
```

- 201 za validnu transakciju.
- 400 za loš signature, nedovoljno sredstava ili pogrešan nonce.
- 422 ako nedostaju određena polja.

Demo:

1. Alice minuje blok i dobija 50.
2. Alice šalje 20 Bobu i dobija 201.
3. Ista transakcija poslata ponovo vraća 400 Wrong nonce.
4. Novi blok se minuje i transakcija ide u njega.
5. Na kraju Alice ima 30, a Bob 20.

Library: ed25519-dalek, sha2, rand, serde, axum, tokio, hex.\
Verzije se nalaze u Cargo.locku.
