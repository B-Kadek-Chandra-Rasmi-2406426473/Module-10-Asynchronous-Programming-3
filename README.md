# Tutorial 10 Asynchronous Programming (Bagian 3)

### Experiment 3.1: Original code YewChat

#### Cara menjalankan:
1. Jalankan WebSocket server:
```
cd SimpleWebsocketServer
npm start
```
2. Jalankan YewChat frontend:
```
cd YewChat
trunk serve --port 8000
```

3. Buka browser ke http://localhost:8000

#### Hasil:

![login](login.png)

![room-chat](chat.png)

YewChat adalah aplikasi web chat yang dibangun menggunakan Rust dan Yew framework yang dikompilasi ke WebAssembly. Frontend berkomunikasi dengan `WebSocket` server menggunakan protokol `ws://`. Ketika user mengetik username dan klik Connect, aplikasi akan terhubung ke WebSocket server dan broadcast pesan ke semua client yang sedang terhubung. Setiap client yang terhubung akan terlihat di sidebar kiri sebagai daftar users aktif.

