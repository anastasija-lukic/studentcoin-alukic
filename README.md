# StudentCoin

Predmet: Programiranje Blockchaina, Tehnički fakultet, SII, 2026/2027.

- Ed25519 novčanici čija je adresa hex(SHA-256(public key)).
- Potpisane transakcije čekaju u mempool-u.
- Proof of work hash bloka počinje sa 0000.
- Miner dobija nagradu od 50 StudentCoina.
- Nonce sprečava ponavljanje transakcija.
- HTTP API je napravljen u Axumu.

Preduslovi (PowerShell kao Administrator):\
Instalira Git, C++ Build Tools, Rust i Cargo, pa ispisuje verzije.

```powershell
winget install --id Git.Git -e --accept-source-agreements --accept-package-agreements; winget install --id Microsoft.VisualStudio.2022.BuildTools -e --accept-source-agreements --accept-package-agreements --override "--quiet --wait --norestart --add Microsoft.VisualStudio.Workload.VCTools --includeRecommended"; curl.exe -sSfL -o "$env:TEMP\rustup-init.exe" https://win.rustup.rs/x86_64; & "$env:TEMP\rustup-init.exe" -y --default-toolchain stable; $env:Path = [Environment]::GetEnvironmentVariable("Path", "Machine") + ";" + [Environment]::GetEnvironmentVariable("Path", "User"); git --version; rustc --version; cargo --version; curl.exe --version
```

1. Server (Tab 1):

```powershell
cd ([Environment]::GetFolderPath("MyDocuments"))
git clone https://github.com/anastasija-lukic/studentcoin-alukic.git
cd studentcoin-alukic
cargo run --release
```

2. Novčanici (Tab 2 Alice, Tab 3 Bob):

```powershell
cd "$([Environment]::GetFolderPath('MyDocuments'))\studentcoin-alukic"
cargo run --release -- wallet
```

3. Lanac:

```powershell
curl.exe http://127.0.0.1:3000/chain
```

4. Alice minuje blok:

```powershell
curl.exe "http://127.0.0.1:3000/mine?miner=<ALICE_ADRESA>"
```

5. Stanje Alice:

```powershell
curl.exe http://127.0.0.1:3000/balance/<ALICE_ADRESA>
```

6. Alice potpisuje transakciju (Tab 2):

```powershell
cargo run --release -- sign <ALICE_SECRET> <BOB_ADRESA> 20 0
```

7. Slanje transakcije (Tab 2):

```powershell
curl.exe -X POST http://127.0.0.1:3000/transactions/new -H "Content-Type: application/json" --data-binary "@podaci/transakcija.json"
```

8. Novi blok:

```powershell
curl.exe http://127.0.0.1:3000/mine
```

9. Stanja:

```powershell
curl.exe http://127.0.0.1:3000/balance/<ALICE_ADRESA>
curl.exe http://127.0.0.1:3000/balance/<BOB_ADRESA>
```

10. Lanac:

```powershell
curl.exe http://127.0.0.1:3000/chain
```

11. Ponovno slanje iste transakcije, očekuje se Wrong nonce (Tab 2):

```powershell
curl.exe -X POST http://127.0.0.1:3000/transactions/new -H "Content-Type: application/json" --data-binary "@podaci/transakcija.json"
```

Library: ed25519-dalek, sha2, rand, serde, axum, tokio, hex.\
Verzije se nalaze u Cargo.locku, dolaze sa crates.io, a proverava ih Cargo.
