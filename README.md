# egov-document-push
> Kleines Rust Programm, das den aktuellen Status eines Ausweisdokuments abruft und eine Push-message über SimplePush schickt.

## Nutzung
Zunächst muss eine ``.env`` Datei erstellt werden
```bash
cp src/.env.example src/.env
```
und die Schlüssel entsprechend gesetzt werden. Im Anschluss kann das Programm kompiliert und ausgeführt werden
```bash
cargo run # Testen - vermutlich wird die .env Datei nicht gefunden, weil das Programm in einem anderen Ordner ausgeführt wird

cargo build --release #
./target/release/egov-document-push
```

Wurde für den Potsdamer EGOV Service (https://egov.potsdam.de) entwickelt und ausschließlich damit getestet.
