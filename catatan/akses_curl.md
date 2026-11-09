# Mengakses API dengan `curl`

Panduan ini menunjukkan cara mengirim request ke backend menggunakan `curl` dari terminal Linux. `curl` adalah program untuk mengirim request HTTP dan menampilkan response server.

Navigasi: [indeks catatan belajar](README.md).

> **Inti bab:** `curl` mengirim request HTTP dari terminal. Untuk endpoint terlindungi, kirim JWT login melalui header Authorization.

### Isi panduan

1. [Persiapan](#sebelum-mulai)
2. [Membaca perintah `curl`](#memahami-bentuk-perintah)
3. [Register dan login](#1-membuat-akun)
4. [Membaca dan mengubah menu/pesanan](#4-membaca-menu-dan-pesanan)
5. [Logout](#8-logout)
6. [Status dan pemecahan masalah](#arti-status-yang-sering-terlihat)

## Sebelum Mulai

1. Pastikan PostgreSQL berjalan dan database sudah tersedia.
2. Pastikan `.env` sudah memiliki `DATABASE_URL` dan `JWT_SECRET` yang valid.
3. Jalankan server dari folder backend dengan `cargo run`.
4. Biarkan terminal server tetap berjalan. Buka terminal kedua untuk mengetik perintah `curl`.

Secara default, server memakai alamat `http://127.0.0.1:8080`. Berikut variabel Bash agar alamat tidak perlu diketik berulang kali:

```bash
BASE_URL="http://127.0.0.1:8080"
```

Salin perintah tersebut ke terminal yang akan dipakai untuk `curl`. Jika kamu mengubah `HOST` atau `PORT` di `.env`, sesuaikan `BASE_URL`.

## Memahami Bentuk Perintah

Contoh berikut membuat menu. Bagian sesudah `curl` adalah opsi dan nilai yang mengatur request:

```bash
curl -i -H "Authorization: Bearer $TOKEN" -X POST "$BASE_URL/api/menus" \
  -H "Content-Type: application/json" \
  -d '{"name":"Nasi Goreng","price":25000,"category":"Makanan","stock":20}'
```

Arti setiap bagian:

<details>
<summary>🔎 Penjelasan opsi curl dan tanda kutip (buka jika ingin tahu detailnya)</summary>

- `curl` menjalankan program klien HTTP dari terminal. Program ini mengirim request ke URL dan menampilkan hasilnya.
- `-i` (sama dengan `--include`) menyertakan header response di keluaran, sebelum body response. Dari sini kamu dapat melihat status seperti `HTTP/1.1 201 Created`. Opsi ini hanya menampilkan header response; bukan untuk mengirim header request.
- `-H "Authorization: Bearer $TOKEN"` mengirim access token pada header Authorization. Ganti `$TOKEN` dengan nilai `access_token` dari response login.
- `-X POST` (sama dengan `--request POST`) menentukan metode HTTP sebagai `POST`. Metode memberi tahu server jenis operasi yang diminta. `GET` adalah default untuk request tanpa body; `-X GET` biasanya tidak perlu ditulis. `-X` hanya menetapkan nama metode, bukan membuat atau memformat body.
- `"$BASE_URL/api/menus"` adalah URL tujuan. `$BASE_URL` diganti oleh Bash dengan nilai variabel yang sebelumnya diatur. Tanda petik ganda menjaga URL tetap menjadi satu argumen dan membantu mencegah karakter khusus shell ditafsirkan secara tidak sengaja.
- `-H "Content-Type: application/json"` (sama dengan `--header`) mengirim header request. `Content-Type` menyatakan format body yang dikirim; `application/json` berarti server harus membaca body sebagai JSON. Ini bukan header response.
- `-d '{...}'` (sama dengan `--data`) mengirim data sebagai body request. Penggunaan `-d` membuat `curl` memakai metode `POST` secara default, sehingga `-X POST` pada contoh menegaskan maksudnya. Tanda petik tunggal Bash di luar JSON menjaga tanda petik ganda di dalam JSON tetap utuh. Nilai teks JSON memakai tanda petik ganda, sedangkan angka seperti `25000` tidak.
- `\` di ujung baris adalah cara Bash menyambung perintah ke baris berikutnya agar mudah dibaca. Harus menjadi karakter terakhir pada baris tersebut; jangan menaruh spasi sesudahnya. Tanpa `\`, tulis seluruh perintah dalam satu baris.

Opsi lain yang dipakai dalam panduan:

- `-H` dapat dipakai berulang untuk mengirim beberapa header. Header Authorization harus memakai skema `Bearer` diikuti satu spasi dan token.
- `-X PUT`, `-X PATCH`, dan `-X DELETE` menetapkan metode untuk mengganti data, mengubah sebagian data, dan menghapus data. `-X` selalu menentukan metode; endpoint server yang menentukan operasi dan format data yang diterima.
- `?field=price` pada URL memulai query string. `field` adalah nama parameter dan `price` nilainya. `?` merupakan pemisah antara path endpoint dan parameter, bukan opsi `curl`.
- `/1` pada URL adalah bagian path yang digunakan endpoint ini sebagai ID data. Ganti `1` dengan ID menu atau pesanan yang ingin dituju.
- `-i` tidak wajib untuk mengirim request. Hapus opsi ini bila hanya ingin melihat body response, bukan header-nya.

Tanda petik dan perintah shell yang juga muncul:

- `"..."` mengelompokkan teks sebagai satu argumen sambil tetap memungkinkan Bash mengganti variabel seperti `$BASE_URL`.
- `'...'` mempertahankan isi secara literal. Karena itu, JSON di contoh ditulis dengan petik tunggal di luar dan petik ganda untuk nama field serta teks JSON.
- `BASE_URL="http://127.0.0.1:8080"` membuat variabel Bash bernama `BASE_URL`. Variabel ini hanya tersedia pada terminal/sesi shell tempat kamu mengaturnya.
- `unset TOKEN` menghapus token dari environment shell saat selesai. Ini tidak membatalkan token yang masih berlaku di server.

Ringkasnya, `-H` dan `-d` menjelaskan data yang dikirim, `-X` memilih metode, sedangkan `-i` membantu memeriksa response. Opsi dapat digabung; urutan opsi di contoh-contoh ini dibuat agar mudah dibaca.

</details>

## 1. Membuat Akun

Kirim request register. Username harus unik dan password minimal 8 byte menurut pemeriksaan kode saat ini. Untuk huruf ASCII, batas tersebut setara dengan delapan karakter; karakter Unicode bisa menggunakan lebih dari satu byte.

```bash
curl -i -X POST "$BASE_URL/api/auth/register" \
  -H "Content-Type: application/json" \
  -d '{"username":"yofan","password":"contoh-password-123"}'
```

Jika berhasil, response biasanya berstatus `201 Created`:

```json
{"message":"Akun berhasil dibuat"}
```

Jika username tersebut sudah pernah didaftarkan, server merespons `409 Conflict`. Gunakan username lain atau login dengan akun yang sudah ada. Contoh password di panduan ini hanya untuk pengujian lokal; jangan pakai sebagai password akun sungguhan.

## 2. Login dan Simpan Token

Login menghasilkan JWT pada body response. Tampilkan body agar nilai `access_token` dapat disalin.

```bash
curl -i -X POST "$BASE_URL/api/auth/login" \
  -H "Content-Type: application/json" \
  -d '{"username":"yofan","password":"contoh-password-123"}'
```

Jika berhasil, body response berisi token dan waktu berlaku:

```json
{"message":"Login berhasil","access_token":"...","token_type":"Bearer","expires_in":3600}
```

Salin nilai `access_token` (tanpa tanda kutip) ke variabel shell. Token adalah kredensial rahasia; jangan bagikan atau masukkan ke Git.

```bash
TOKEN="tempel_access_token_di_sini"
```

Gunakan HTTPS di jaringan selain komputer lokal, karena bearer token dapat digunakan siapa pun yang memilikinya.

## 3. Mengirim Bearer Token pada Request Berikutnya

Semua endpoint menu dan pesanan, serta `/api/auth/me`, memerlukan header `Authorization: Bearer $TOKEN`.

Cek akun yang sedang login:

```bash
curl -i -H "Authorization: Bearer $TOKEN" "$BASE_URL/api/auth/me"
```

Contoh response sukses:

```json
{"id":1,"username":"yofan"}
```

Jika menjalankan request tanpa token yang valid, hasilnya `401 Unauthorized`.

## 4. Membaca Menu dan Pesanan

Ambil semua menu:

```bash
curl -i -H "Authorization: Bearer $TOKEN" "$BASE_URL/api/menus"
```

Ambil satu menu berdasarkan ID:

```bash
curl -i -H "Authorization: Bearer $TOKEN" "$BASE_URL/api/menus/1"
```

Ambil hanya harga menu. Tanda `?` memulai query string; `field=price` adalah parameter query:

```bash
curl -i -H "Authorization: Bearer $TOKEN" "$BASE_URL/api/menus?field=price"
```

Ambil semua pesanan atau satu kolom pesanan:

```bash
curl -i -H "Authorization: Bearer $TOKEN" "$BASE_URL/api/orders"
curl -i -H "Authorization: Bearer $TOKEN" "$BASE_URL/api/orders?field=status"
```

## 5. Membuat Data

Request `POST` memerlukan header JSON dan body dengan field yang sesuai.

Membuat menu:

```bash
curl -i -H "Authorization: Bearer $TOKEN" -X POST "$BASE_URL/api/menus" \
  -H "Content-Type: application/json" \
  -d '{"name":"Nasi Goreng","price":25000,"category":"Makanan"}'
```

Membuat pesanan:

```bash
curl -i -H "Authorization: Bearer $TOKEN" -X POST "$BASE_URL/api/orders" \
  -H "Content-Type: application/json" \
  -d '{"customer_name":"Budi","menu_id":1,"quantity":2}'
```

Jika berhasil, operasi pembuatan mengembalikan `201 Created` dan data yang baru dibuat. Sesuaikan menu dan pesanan dengan kebutuhanmu.

## 6. Mengubah Data

`PUT` mengganti semua field yang diminta oleh input lengkap. Ganti angka `1` dengan ID data yang benar.

```bash
curl -i -H "Authorization: Bearer $TOKEN" -X PUT "$BASE_URL/api/menus/1" \
  -H "Content-Type: application/json" \
  -d '{"name":"Nasi Goreng Spesial","price":30000,"category":"Makanan"}'
```

`PATCH` hanya mengubah field yang dikirim. Field lain tetap memakai nilainya sekarang.

```bash
curl -i -H "Authorization: Bearer $TOKEN" -X PATCH "$BASE_URL/api/menus/1" \
  -H "Content-Type: application/json" \
  -d '{"price":27500}'
```

Contoh mengubah status pesanan:

```bash
curl -i -H "Authorization: Bearer $TOKEN" -X PATCH "$BASE_URL/api/orders/1" \
  -H "Content-Type: application/json" \
  -d '{"status":"diproses"}'
```

## 7. Menghapus Data

Hati-hati: `DELETE` menghapus data dengan ID tersebut dari database.

```bash
curl -i -H "Authorization: Bearer $TOKEN" -X DELETE "$BASE_URL/api/menus/1"
curl -i -H "Authorization: Bearer $TOKEN" -X DELETE "$BASE_URL/api/orders/1"
```

## 8. Pembelian, Pembayaran, dan Struk

Pilih metode pembayaran saat membuat order. Server menghitung total dari harga menu dan jumlah, lalu mencatat order serta payment `lunas` secara otomatis dalam satu transaksi.

```bash
curl -i -H "Authorization: Bearer $TOKEN" -X POST "$BASE_URL/api/orders" \
  -H "Content-Type: application/json" \
  -d '{"customer_name":"Budi","menu_id":1,"quantity":2,"payment_method":"bank_transfer"}'
```

Response berisi order, ID payment, metode, total, dan status. Pemilik order dapat langsung mengambil struk dengan ID order dari response:

```bash
curl -i -H "Authorization: Bearer $TOKEN" "$BASE_URL/api/orders/1/receipt"
```

Ini simulasi: status lunas hanya berarti pembayaran tercatat oleh aplikasi, bukan bukti uang tunai diterima atau transfer bank berhasil.

## 9. Logout

Logout endpoint mengingatkan klien untuk membuang token. JWT yang sudah diterbitkan tetap berlaku sampai kedaluwarsa, sehingga hapus juga token dari variabel lokal:

```bash
curl -i -X POST "$BASE_URL/api/auth/logout"
unset TOKEN
```

Untuk menguji penolakan setelah logout, login lagi, simpan token, panggil logout, lalu ingat bahwa server stateless tetap menerima token itu sampai expiry. Menghapus token dari klien adalah tindakan logout pada implementasi ini.

```bash
curl -i -H "Authorization: Bearer $TOKEN" "$BASE_URL/api/auth/me"
```

Untuk mulai lagi, lakukan login dan simpan token baru ke variabel `TOKEN`.

## Arti Status yang Sering Terlihat

| Status | Makna umum dalam API ini |
|---|---|
| `200 OK` | Request berhasil, misalnya membaca data atau login. |
| `201 Created` | Akun, menu, atau pesanan berhasil dibuat. |
| `400 Bad Request` | JSON tidak valid atau nilai input tidak memenuhi aturan. |
| `401 Unauthorized` | Belum login, bearer token tidak dikirim/tidak valid, atau password salah. |
| `403 Forbidden` | Token valid, tetapi role tidak memiliki izin untuk operasi. |
| `404 Not Found` | ID menu/pesanan tidak ditemukan. |
| `409 Conflict` | Username sudah digunakan. |
| `500 Internal Server Error` | Server atau database mengalami kegagalan. |

## Masalah Umum

- **`curl: (7) Failed to connect`**: server belum berjalan, atau `BASE_URL` memakai host/port yang salah.
- **`401 Unauthorized` pada menu/pesanan**: login dulu, lalu kirim `Authorization: Bearer $TOKEN`.
- **`403 Forbidden`**: operasi tidak diizinkan untuk role saat ini. Kasir tidak dapat melihat pesanan; pembeli tidak dapat mengubah menu atau mengelola order.
- **Login selalu gagal**: pastikan akun sudah dibuat dan username/password sama persis.
- **Register menghasilkan 409**: username sudah ada di tabel `"user"`; gunakan username berbeda.
- **`400 Bad Request`**: periksa ejaan nama field JSON, tanda kutip, koma, serta tipe nilai seperti `price` dan `quantity`.
- **Token kedaluwarsa**: login kembali untuk memperoleh token baru.

## Keamanan

Bearer token adalah kredensial: siapa pun yang memilikinya dapat bertindak sebagai akun sampai token kedaluwarsa. Jangan masukkan token ke URL atau log, gunakan HTTPS, dan hapus dari shell setelah selesai:

```bash
unset TOKEN
```

Perintah tersebut menghapus variabel shell, tetapi tidak mencabut token dari server. Jangan kirim token melalui HTTP di jaringan publik; deployment seharusnya memakai HTTPS.
