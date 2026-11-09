# 🍽️ Catatan Postman: Restoran API (Actix Web + Rust)

> Panduan memakai semua endpoint di Postman, mulai dari persiapan, login, sampai pesan dan bayar.
> Sumber: README dan catatan repo `backend-actixweb-rust`.
> Bagian bertanda ⚠️ belum terkonfirmasi dari kode. Cek dulu sebelum dipakai.

---

## 📑 Daftar Isi
1. [Gambaran besar](#1--gambaran-besar)
2. [Persiapan Postman](#2--persiapan-postman)
3. [Siapa boleh apa](#3--siapa-boleh-apa)
4. [Cara jadi admin](#4--cara-jadi-admin)
5. [Template request: Auth](#5--template-request-auth)
6. [Template request: Menu](#6--template-request-menu)
7. [Template request: Order](#7--template-request-order)
8. [Template request: Payment dan struk](#8--template-request-payment-dan-struk)
9. [Alur lengkap sebagai pelanggan](#9--alur-lengkap-sebagai-pelanggan)
10. [Alur lengkap sebagai admin](#10--alur-lengkap-sebagai-admin)
11. [Arti kode status](#11--arti-kode-status)
12. [Troubleshooting](#12--troubleshooting)
13. [Yang masih perlu dicek](#13--yang-masih-perlu-dicek)

---

## 1. 🗺️ Gambaran besar

```
   POSTMAN                      SERVER (Actix Web)                DATABASE
 ┌─────────┐   request    ┌──────────────────────────┐         ┌──────────┐
 │ method  │ ───────────► │ 1. Route (cocokkan URL)  │         │PostgreSQL│
 │ URL     │              │ 2. Middleware (cek token │ ◄─────► │          │
 │ header  │              │    + role)               │         └──────────┘
 │ body    │ ◄─────────── │ 3. Handler (kerjakan)    │
 └─────────┘   response   └──────────────────────────┘
   JSON + status code
```

**Kunci utamanya adalah token.** Setelah login, server memberi `access_token` (JWT, berlaku **1 jam**).
Token itu harus dikirim di setiap request yang dilindungi.

```
LOGIN ──► dapat access_token ──► tempel di Authorization (Bearer Token) ──► akses endpoint
```

---

## 2. 🛠️ Persiapan Postman

### 2.1 Alamat dasar
```
http://127.0.0.1:8080
```
Server dijalankan dengan `cargo run` dari folder yang berisi `Cargo.toml`.
Pastikan PostgreSQL aktif dan file `.env` terisi `DATABASE_URL` dan `JWT_SECRET` (minimal 32 byte).

### 2.2 Buat Environment (biar tidak copy-paste berulang)
Di Postman: **Environments → New**, beri nama `Restoran`, lalu isi:

| Variable   | Initial value             |
|------------|---------------------------|
| `base_url` | `http://127.0.0.1:8080`   |
| `token`    | *(kosongkan)*             |

Pilih environment `Restoran` di pojok kanan atas. Sekarang URL cukup ditulis:
```
{{base_url}}/api/menus
```

### 2.3 Aturan umum tiap request

**Request dengan body JSON:**
1. Tab **Body** → pilih **raw** → ubah tipe ke **JSON**.
2. Tulis JSON-nya. Header `Content-Type: application/json` ditambahkan otomatis.

**Request yang butuh login:**
1. Tab **Authorization** → Type: **Bearer Token**.
2. Isi kolom Token dengan `{{token}}`.
3. **Jangan** menulis kata `Bearer` sendiri. Postman yang menambahkannya.

### 2.4 Simpan token otomatis setelah login ✨
Di request **Login**, buka tab **Scripts** (atau **Tests**), lalu tempel:

```javascript
const res = pm.response.json();
pm.environment.set("token", res.access_token);
```

Sekarang setiap kali login berhasil, `{{token}}` terisi otomatis.
Token kedaluwarsa dalam 1 jam, jadi login ulang kalau muncul 401.

> 💡 Alternatif: impor `postman/Restoran-API.postman_collection.json` dari repo.
> Menurut README, koleksi itu menyimpan token otomatis setelah login.

---

## 3. 🔐 Siapa boleh apa

Ada 3 role: `admin`, `kasir`, `pembeli`.

| Aksi                                   | Admin | Kasir | Pembeli |
|----------------------------------------|:-----:|:-----:|:-------:|
| Register / Login                       |  ✅   |  ✅   |   ✅    |
| Lihat menu (GET)                       |  ✅   |  ✅   |   ✅    |
| Tambah / ubah / hapus menu & stok      |  ✅   |  ✅   |   ❌    |
| Buat pesanan                           |  ✅   |  ❌   |   ✅    |
| Lihat daftar pesanan                   | semua |  ❌   | miliknya|
| Lihat detail pesanan                   |  ✅   |  ❌   | miliknya|
| Ubah / hapus pesanan (PUT/PATCH/DELETE)|  ✅   |  ❌   |   ❌    |
| Ajukan payment, ambil struk            |  ✅   |  ❌   | miliknya|
| Lihat semua payment, konfirmasi payment|  ✅   |  ❌   |   ❌    |
| Ubah role user                         |  ✅   |  ❌   |   ❌    |

Aturan penting:
- Akun baru **selalu** berole `pembeli`. Role tidak bisa dipilih saat register.
- Kalau role akun diubah, token lama langsung ditolak (401). **Login ulang** untuk mendapat token baru.

---

## 4. 👑 Cara jadi admin

Admin pertama tidak bisa dibuat lewat API. Caranya:

```
① Register akun biasa ──► ② Ubah role lewat SQL ──► ③ Login ulang ──► ④ Pakai token baru
```

**① Register** (lihat template 5.1).

**② Jalankan SQL** di PostgreSQL (misalnya lewat `psql`):
```sql
UPDATE "user" SET role = 'admin' WHERE username = 'admin1';
```
Tanda kutip ganda pada `"user"` wajib, karena `user` kata khusus di PostgreSQL.

**③ Login ulang** (template 5.2). Token lama tidak berlaku lagi karena rolenya sudah berubah.

Setelah ada admin, admin bisa mengubah role akun lain lewat API (template 5.5).

---

## 5. 🔑 Template request: Auth

### 5.1 Register
```
POST {{base_url}}/api/auth/register
Auth : tidak perlu
```
Body (raw → JSON):
```json
{
  "username": "budi",
  "password": "rahasia123"
}
```
- Username tidak boleh kosong. Password minimal 8 byte (maksimal 128).
- ✅ `201` berhasil. ⚠️ `409` username sudah dipakai. ⚠️ `400` input salah.

### 5.2 Login
```
POST {{base_url}}/api/auth/login
Auth : tidak perlu
```
Body:
```json
{
  "username": "budi",
  "password": "rahasia123"
}
```
Response berisi: `access_token`, `token_type` (`Bearer`), `expires_in`, dan role akun.
Salin `access_token`, atau biarkan script di 2.4 menyimpannya otomatis.

### 5.3 Logout
```
POST {{base_url}}/api/auth/logout
Auth : tidak wajib
```
Tanpa body. Server **tidak** mencabut token. Logout hanya mengingatkan klien membuang token.
Token lama tetap berlaku sampai kedaluwarsa (1 jam).

### 5.4 Lihat akun sendiri
```
GET {{base_url}}/api/auth/me
Auth : Bearer {{token}}
```
Tanpa body. Response berisi `id`, `username`, `role`. Cocok untuk mengecek role aktif.

### 5.5 Ubah role user (khusus admin)
```
PUT {{base_url}}/api/auth/users/3/role
Auth : Bearer {{token}}   (token admin)
```
Ganti `3` dengan id user yang mau diubah.
Body, isi hanya salah satu dari tiga nilai:
```json
{ "role": "kasir" }
```
```json
{ "role": "pembeli" }
```
```json
{ "role": "admin" }
```
Nilai lain menghasilkan `400`.

---

## 6. 🍜 Template request: Menu

Semua endpoint menu butuh login. Semua role boleh **membaca**. Hanya admin dan kasir boleh **mengubah**.

### 6.1 Daftar menu
```
GET {{base_url}}/api/menus
Auth : Bearer {{token}}
```
Tanpa body. Harga tampil sebagai teks, misalnya `Rp25.000`.

**Ambil satu kolom saja**, tambahkan parameter `field` (tab **Params**: Key `field`, Value `name`):
```
GET {{base_url}}/api/menus?field=name
```
Nilai `field` yang didukung: `id`, `name`, `price`, `category`, `stock`.

### 6.2 Detail satu menu
```
GET {{base_url}}/api/menus/1
Auth : Bearer {{token}}
```

### 6.3 Tambah menu (admin/kasir)
```
POST {{base_url}}/api/menus
Auth : Bearer {{token}}
```
Body:
```json
{
  "name": "Nasi Goreng",
  "price": 25000,
  "category": "Makanan",
  "stock": 20
}
```
- Semua field wajib diisi.
- Teks harus berisi sesuatu setelah spasi tepi dibuang. Harga harus lebih dari 0.
- Harga disimpan sebagai `NUMERIC(12,2)`, bukan angka desimal biasa.

### 6.4 Ganti seluruh menu, PUT (admin/kasir)
```
PUT {{base_url}}/api/menus/1
Auth : Bearer {{token}}
```
Body **harus lengkap**:
```json
{
  "name": "Nasi Goreng Spesial",
  "price": 30000,
  "category": "Makanan",
  "stock": 25
}
```

### 6.5 Ubah sebagian menu, PATCH (admin/kasir)
```
PATCH {{base_url}}/api/menus/1
Auth : Bearer {{token}}
```
Kirim **hanya field yang mau diubah**:
```json
{ "price": 27000 }
```
```json
{ "stock": 50 }
```
```json
{ "price": 27000, "stock": 15 }
```

### 6.6 Hapus menu (admin/kasir)
```
DELETE {{base_url}}/api/menus/1
Auth : Bearer {{token}}
```
Tanpa body.

> 🧠 **PUT vs PATCH**: PUT = ganti semua field (harus lengkap). PATCH = ubah sebagian (cukup yang berubah).

---

## 7. 🧾 Template request: Order

### 7.1 Buat pesanan (admin/pembeli)
```
POST {{base_url}}/api/orders
Auth : Bearer {{token}}
```
Body:
```json
{
  "customer_name": "Budi",
  "menu_id": 1,
  "quantity": 2,
  "payment_method": "cash"
}
```
- `menu_id` adalah id menu dari `GET /api/menus`.
- `quantity` harus lebih dari 0.
- `payment_method` harus `cash` atau `bank_transfer`.
- Server menghitung total dari harga menu dan jumlah. Jika stok cukup, order serta payment simulasi berstatus `lunas` dicatat dalam satu transaksi.
- **Catat `id` pesanan dari response.** Id ini dipakai untuk mengambil struk.

### 7.2 Daftar pesanan (admin/pembeli)
```
GET {{base_url}}/api/orders
Auth : Bearer {{token}}
```
Admin melihat **semua** pesanan. Pembeli hanya melihat **miliknya sendiri**.
Parameter `field` yang didukung: `id`, `customer_name`, `item_name`, `quantity`, `status`.
```
GET {{base_url}}/api/orders?field=status
```

### 7.3 Detail pesanan (admin / pembeli pemilik)
```
GET {{base_url}}/api/orders/15
Auth : Bearer {{token}}
```

### 7.4 Ubah sebagian pesanan, PATCH (khusus admin)
```
PATCH {{base_url}}/api/orders/15
Auth : Bearer {{token}}   (token admin)
```
Contoh mengubah status:
```json
{ "status": "diproses" }
```

### 7.5 Ganti seluruh pesanan, PUT (khusus admin)
```
PUT {{base_url}}/api/orders/15
Auth : Bearer {{token}}   (token admin)
```
⚠️ Body harus berisi **semua field input** pesanan, tapi daftar persisnya belum terkonfirmasi.
Cek di `src/handlers/orders.rs` atau koleksi Postman di repo.

### 7.6 Hapus pesanan (khusus admin)
```
DELETE {{base_url}}/api/orders/15
Auth : Bearer {{token}}   (token admin)
```

---

## 8. 💳 Pembayaran dan struk

Pembayaran dicatat otomatis saat `POST /api/orders`; tidak ada request pembayaran kedua atau konfirmasi admin.

### 8.1 Lihat semua payment (khusus admin)
```
GET {{base_url}}/api/payments
Auth : Bearer {{token}}   (token admin)
```

### 8.2 Ambil struk (admin / pembeli pemilik)
```
GET {{base_url}}/api/orders/15/receipt
Auth : Bearer {{token}}
```
Struk langsung tersedia setelah order berhasil. Status `lunas` di proyek ini adalah simulasi pencatatan metode pembayaran, bukan bukti uang diterima atau transfer terverifikasi.

---

## 9. 🛒 Alur lengkap sebagai pelanggan

```
 1. Register ─► 2. Login ─► 3. Lihat menu ─► 4. Pesan + pilih metode
                                                │
                                  5. Payment tercatat ─► 6. Ambil struk
```

| # | Request | Isi |
|---|---------|-----|
| 1 | `POST /api/auth/register` | `{"username":"budi","password":"rahasia123"}` |
| 2 | `POST /api/auth/login`    | body sama, lalu simpan token |
| 3 | `GET /api/menus`          | catat `id` menu dan cek stok |
| 4 | `POST /api/orders`        | `{"customer_name":"Budi","menu_id":1,"quantity":2,"payment_method":"cash"}`, catat `id` pesanan |
| 5 | `GET /api/orders/{id}`    | cek order berstatus lunas |
| 6 | `GET /api/orders/{id}/receipt` | struk langsung tersedia |

---

## 10. 👑 Alur lengkap sebagai admin

```
 Login ─► cek /me ─► kelola menu ─► (pembeli pesan dan payment tercatat otomatis)
```

| # | Request | Keterangan |
|---|---------|------------|
| 1 | `POST /api/auth/login` | pakai akun yang sudah dijadikan admin (bagian 4) |
| 2 | `GET /api/auth/me` | pastikan `role` = `admin` |
| 3 | `POST /api/menus` | `{"name":"Nasi Goreng","price":25000,"category":"Makanan","stock":20}` |
| 4 | `PATCH /api/menus/1` | `{"stock": 50}` |
| 5 | `GET /api/orders` | lihat semua pesanan |
| 6 | `PATCH /api/orders/15` | `{"status":"diproses"}` |
| 7 | `GET /api/payments` | lihat daftar payment simulasi |
| 8 | `PUT /api/auth/users/2/role` | `{"role":"kasir"}` untuk mengubah role user lain |

---

## 11. 🚦 Arti kode status

| Kode | Arti | Contoh penyebab |
|:----:|------|-----------------|
| 200  | Berhasil | baca atau ubah data |
| 201  | Data baru dibuat | register, buat menu atau pesanan |
| 400  | Input salah | field kurang, harga 0, JSON rusak, role tidak dikenal |
| 401  | Belum login / token bermasalah | token hilang, rusak, kedaluwarsa, atau role berubah |
| 403  | Dilarang | token valid, tapi role tidak boleh mengakses endpoint |
| 404  | Tidak ditemukan | id tidak ada |
| 409  | Konflik | username sudah dipakai |
| 500  | Error server | masalah query atau konfigurasi server |

**Cara mudah mengingat 401 vs 403:**
```
401 = "Siapa kamu?"  (identitas tidak terbukti)
403 = "Kamu dikenal, tapi tidak boleh masuk ke sini."
```

---

## 12. 🧯 Troubleshooting

| Gejala | Kemungkinan penyebab | Solusi |
|--------|----------------------|--------|
| `401` pesan "Bearer token diperlukan" | Header Authorization kosong, atau tidak diawali `Bearer ` | Isi tab Authorization dengan tipe Bearer Token |
| `401` "Token tidak valid atau sudah kedaluwarsa" | Token lewat 1 jam, rusak, atau salah salin | Login ulang |
| `401` "Role akun berubah; silakan login kembali" | Role diubah setelah token dibuat | Login ulang |
| `403` "Role tidak memiliki akses..." | Role tidak diizinkan untuk endpoint itu | Pakai akun dengan role yang sesuai (lihat bagian 3) |
| `400` saat membuat pesanan | `quantity` 0 atau field kurang | Periksa isi JSON |
| Pesanan gagal | Stok tidak cukup | Cek stok lewat `GET /api/menus/{id}` |
| Struk belum bisa diambil | Order belum memiliki payment lunas | Periksa apakah order berhasil dibuat dengan metode pembayaran yang valid |
| Server tidak mau jalan | `DATABASE_URL` salah, PostgreSQL mati, atau `JWT_SECRET` < 32 byte | Baca error pertama di terminal |
| Tidak ada respons di Postman | Server belum `cargo run`, atau port salah | Pastikan server aktif di `127.0.0.1:8080` |

---

## 13. ⚠️ Yang masih perlu dicek

- [ ] Daftar field lengkap untuk `PUT /api/orders/{id}` (bagian 7.5).
- [ ] Nama tabel payment di database (cek dengan `\dt` di `psql`).

Sumber untuk mengecek: koleksi `postman/Restoran-API.postman_collection.json`,
`src/handlers/payments.rs`, `src/handlers/orders.rs`, dan `src/db.rs`.

---

## 🔒 Catatan keamanan
- Jangan membagikan `access_token` dan `JWT_SECRET`.
- Jangan commit file `.env` ke Git.
- Logout tidak mencabut token di server, jadi token hanya mati saat kedaluwarsa.
- Gunakan HTTPS kalau server dipasang di luar komputer lokal.