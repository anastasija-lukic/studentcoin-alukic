# StudentCoin

Predmet: Programiranje Blockchaina, Tehnički fakultet, SII, 2026/2027.

- Ed25519 novčanici čija je adresa hex(SHA-256(public key)).
- Potpisane transakcije čekaju u mempool-u.
- Proof of work hash bloka počinje sa 0000.
- Miner dobija nagradu od 50 StudentCoina.
- Nonce sprečava ponavljanje transakcija.
- HTTP API je napravljen u Axumu.

Preduslovi (PowerShell kao Administrator):\
Svaka komanda se pokreće posebno.

Git:

```powershell
winget install --id Git.Git -e --accept-source-agreements --accept-package-agreements
```

C++ Build Tools (traje oko 10 minuta):

```powershell
winget install --id Microsoft.VisualStudio.2022.BuildTools -e --accept-source-agreements --accept-package-agreements --override "--quiet --wait --norestart --add Microsoft.VisualStudio.Workload.VCTools --includeRecommended"
```

Rust i Cargo:

```powershell
curl.exe -sSfL -o "$env:TEMP\rustup-init.exe" https://win.rustup.rs/x86_64
```

```powershell
& "$env:TEMP\rustup-init.exe" -y --default-toolchain stable
```

Osvežavanje PATH-a:

```powershell
$env:Path = [Environment]::GetEnvironmentVariable("Path", "Machine") + ";" + [Environment]::GetEnvironmentVariable("Path", "User")
```

Provera:

```powershell
git --version; rustc --version; cargo --version; curl.exe --version
```

Pokretanje:

```bash
git clone https://github.com/anastasija-lukic/studentcoin-alukic.git
cd studentcoin-alukic
cargo run --release
```

Nalazi se na adresi <http://127.0.0.1:3000>.\
Primeri su za Windows sisteme.

Novi novčanik (u drugom terminalu):

```bash
cargo run --release -- wallet
```

```
secret:     <secret key>
public key: <public key>
address:    <adresa>
```

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

Potpisivanje transakcije:

```bash
cargo run --release -- sign <secret> <adresa primaoca> <iznos> <nonce>
```

Pravi se fajl podaci/transakcija.json.\
Nonce je next_nonce iz stanja adrese.

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

1. Alice i Bob prave novčanike.
2. Alice minuje blok i dobija 50.
3. Alice potpisuje transakciju od 20 za Boba sa nonce 0.
4. Alice šalje 20 Bobu i dobija 201.
5. Ista transakcija poslata ponovo vraća 400 Wrong nonce.
6. Novi blok se minuje i transakcija ide u njega.
7. Na kraju Alice ima 30, a Bob 20.

Library: ed25519-dalek, sha2, rand, serde, axum, tokio, hex.\
Verzije se nalaze u Cargo.locku, dolaze sa crates.io, a proverava ih Cargo.
