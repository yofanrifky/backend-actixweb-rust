# Template Semua Request API dengan curl

Dokumen ini berisi contoh `curl` untuk semua endpoint yang didaftarkan di `src/routes.rs`: autentikasi, menu, pesanan, struk, dan daftar pembayaran. Jalankan command dari terminal kedua setelah server API aktif.

## Persiapan

Server default memakai `http://127.0.0.1:8080`. Jalankan sekali pada terminal yang akan dipakai untuk request:

```bash
BASE_URL="http://127.0.0.1:8080"
```

Jika server memakai host/port lain, ganti nilai `BASE_URL`. Request terlindungi memakai variabel `TOKEN` yang diisi setelah login.

Register menghasilkan role `pembeli`. Role yang tersedia adalah `admin`, `kasir`, dan `pembeli`. Login dengan akun role yang sesuai; bearer token menentukan akses request. Body lengkap tiap request ada di bagian Autentikasi, Menu, Pesanan, dan Struk/Pembayaran di bawah.

## Request Publik

Tidak perlu bearer token untuk register, login, dan logout:

| Method | URL | Kegunaan |
|---|---|---|
| `POST` | `$BASE_URL/api/auth/register` | Membuat akun baru; role awal selalu `pembeli`. |
| `POST` | `$BASE_URL/api/auth/login` | Menerbitkan token JWT. |
| `POST` | `$BASE_URL/api/auth/logout` | Memberi tahu klien untuk menghapus token lokal. |

## Akses Pembeli

Login menggunakan akun `pembeli`, lalu set `TOKEN` dari response login. Pembeli dapat membaca menu, membuat pembelian, dan melihat order/struk miliknya sendiri.

| Method | URL | Catatan |
|---|---|---|
| `GET` | `$BASE_URL/api/auth/me` | Profil akun aktif. |
| `GET` | `$BASE_URL/api/menus` | Semua menu; filter `field=id`, `name`, `price`, `category`, atau `stock`. |
| `GET` | `$BASE_URL/api/menus/$MENU_ID` | Detail menu. |
| `GET` | `$BASE_URL/api/orders` | Riwayat order milik pembeli; filter `field=id`, `customer_name`, `item_name`, `quantity`, `status`. |
| `GET` | `$BASE_URL/api/orders/$ORDER_ID` | Detail order milik pembeli. |
| `POST` | `$BASE_URL/api/orders` | Membuat order dan payment `lunas` sekaligus. |
| `GET` | `$BASE_URL/api/orders/$ORDER_ID/receipt` | Struk order milik pembeli. |

Contoh body order ada di bagian **Pesanan**. Pembeli tidak dapat mengubah/menghapus menu atau order, dan tidak dapat melihat daftar semua payment.

## Akses Kasir

Login menggunakan akun `kasir`, lalu set `TOKEN` dari response login. Kasir hanya mengelola menu/stok dan membaca menu.

| Method | URL | Catatan |
|---|---|---|
| `GET` | `$BASE_URL/api/auth/me` | Profil akun aktif. |
| `GET` | `$BASE_URL/api/menus` | Semua menu atau filter `field=id`, `name`, `price`, `category`, `stock`. |
| `GET` | `$BASE_URL/api/menus/$MENU_ID` | Detail menu. |
| `POST` | `$BASE_URL/api/menus` | Membuat menu dan stok. |
| `PUT` | `$BASE_URL/api/menus/$MENU_ID` | Mengganti seluruh data menu. |
| `PATCH` | `$BASE_URL/api/menus/$MENU_ID` | Mengganti sebagian atau seluruh field menu. |
| `DELETE` | `$BASE_URL/api/menus/$MENU_ID` | Menghapus menu. |

Body JSON menu ada di bagian **Menu**. Request kasir ke `/api/orders` atau `/api/payments` akan mendapat `403 Forbidden`.

## Akses Admin

Login menggunakan akun `admin`, lalu set `TOKEN` dari response login. Admin dapat mengelola menu dan semua order, mengubah role akun, membaca struk siapa pun, serta melihat daftar payment.

| Method | URL | Catatan |
|---|---|---|
| `GET` | `$BASE_URL/api/auth/me` | Profil akun aktif. |
| `PUT` | `$BASE_URL/api/auth/users/$USER_ID/role` | Mengubah role ke `admin`, `kasir`, atau `pembeli`. |
| `GET` | `$BASE_URL/api/menus` atau `$BASE_URL/api/menus/$MENU_ID` | Membaca menu; filter sama seperti bagian Pembeli. |
| `POST`, `PUT`, `PATCH`, `DELETE` | `/api/menus` atau `/api/menus/$MENU_ID` | Mengelola menu/stok; body ada di bagian **Menu**. |
| `GET` | `$BASE_URL/api/orders` atau `$BASE_URL/api/orders/$ORDER_ID` | Melihat semua order; filter sama seperti bagian Pembeli. |
| `POST` | `$BASE_URL/api/orders` | Admin juga boleh membuat order; payment dicatat otomatis. |
| `PUT`, `PATCH`, `DELETE` | `$BASE_URL/api/orders/$ORDER_ID` | Mengubah atau menghapus order. |
| `GET` | `$BASE_URL/api/orders/$ORDER_ID/receipt` | Struk order mana pun. |
| `GET` | `$BASE_URL/api/payments` | Daftar semua payment. |

Body JSON perubahan role, menu, dan order ada di bagian Autentikasi, Menu, dan Pesanan. Tidak tersedia endpoint untuk membuat atau mengubah payment secara terpisah.

## Autentikasi

### Register

Contoh username/password di bawah dapat langsung dipakai untuk membuat akun demo jika username belum ada. Password harus 8-128 byte; username harus unik. Jika username sudah dipakai, ganti nilainya pada request Register dan Login. Jika akun sudah ada, lewati Register dan gunakan kredensial akun itu pada Login.

```bash
curl -i -X POST "$BASE_URL/api/auth/register" \
  -H "Content-Type: application/json" \
  -d '{"username":"demo_pembeli_20261109","password":"ContohPassword2026!"}'
```

Sukses: `201 Created`. Role akun baru otomatis `pembeli`.

### Login

Gunakan akun demo yang baru dibuat atau akun yang sudah terdaftar. Nilai di JSON harus sama dengan akun tersebut. Salin `access_token` dari response JSON.

```bash
curl -i -X POST "$BASE_URL/api/auth/login" \
  -H "Content-Type: application/json" \
  -d '{"username":"demo_pembeli_20261109","password":"ContohPassword2026!"}'
```

Login sukses: `200 OK`. Simpan token di sesi shell yang sama, tanpa awalan `Bearer `:

```bash
TOKEN="TEMPEL_NILAI_access_token_DI_SINI"
```

Untuk semua request terlindungi, header yang digunakan adalah `Authorization: Bearer $TOKEN`.

### Profil akun aktif

```bash
curl -i -H "Authorization: Bearer $TOKEN" "$BASE_URL/api/auth/me"
```

### Logout

```bash
curl -i -X POST "$BASE_URL/api/auth/logout"
```

Logout hanya memberitahu klien agar membuang token; JWT tidak dicabut server. Hapus token dari terminal setelah selesai:

```bash
unset TOKEN
```

### Ubah role akun (admin)

Login menggunakan akun admin dan gunakan token admin. Ganti ID akun dan nilai role bila perlu. Nilai yang diterima: `admin`, `kasir`, `pembeli`.

```bash
USER_ID=1

curl -i -H "Authorization: Bearer $TOKEN" -X PUT "$BASE_URL/api/auth/users/$USER_ID/role" \
  -H "Content-Type: application/json" \
  -d '{"role":"kasir"}'
```

Setelah role berubah, akun tersebut harus login lagi. Route ini hanya tersedia untuk admin; pembuatan akun admin pertama tidak disediakan oleh endpoint register.

## Menu

Semua request menu perlu token. Semua role boleh membaca; hanya admin/kasir yang boleh membuat, mengubah, atau menghapus menu.

### Daftar semua menu

```bash
curl -i -H "Authorization: Bearer $TOKEN" "$BASE_URL/api/menus"
```

### Filter satu field menu

`field` menerima `id`, `name`, `price`, `category`, atau `stock`:

```bash
curl -i -H "Authorization: Bearer $TOKEN" "$BASE_URL/api/menus?field=id"
curl -i -H "Authorization: Bearer $TOKEN" "$BASE_URL/api/menus?field=name"
curl -i -H "Authorization: Bearer $TOKEN" "$BASE_URL/api/menus?field=price"
curl -i -H "Authorization: Bearer $TOKEN" "$BASE_URL/api/menus?field=category"
curl -i -H "Authorization: Bearer $TOKEN" "$BASE_URL/api/menus?field=stock"
```

### Detail menu

Ganti `1` dengan ID menu:

```bash
MENU_ID=1
curl -i -H "Authorization: Bearer $TOKEN" "$BASE_URL/api/menus/$MENU_ID"
```

### Tambah menu (admin/kasir)

Nama dan kategori wajib terisi, harga harus lebih dari nol, stok minimal nol.

```bash
curl -i -H "Authorization: Bearer $TOKEN" -X POST "$BASE_URL/api/menus" \
  -H "Content-Type: application/json" \
  -d '{"name":"Nasi Goreng","price":25000,"category":"Makanan","stock":20}'
```

Sukses: `201 Created`; simpan nilai `id` response untuk request detail/ubah/hapus.

### Ganti seluruh data menu (admin/kasir)

```bash
curl -i -H "Authorization: Bearer $TOKEN" -X PUT "$BASE_URL/api/menus/$MENU_ID" \
  -H "Content-Type: application/json" \
  -d '{"name":"Nasi Goreng Spesial","price":30000,"category":"Makanan","stock":20}'
```

`PUT` mengganti semua field menu; body harus lengkap.

### Ganti sebagian data menu (admin/kasir)

Contoh ini mengirim semua field opsional. `PATCH` juga menerima satu atau lebih field dari `name`, `price`, `category`, `stock`:

```bash
curl -i -H "Authorization: Bearer $TOKEN" -X PATCH "$BASE_URL/api/menus/$MENU_ID" \
  -H "Content-Type: application/json" \
  -d '{"name":"Nasi Goreng Spesial","price":27500,"category":"Makanan","stock":18}'
```

Body `{}` ditolak; harga harus lebih dari nol; stok tidak boleh negatif; nama/kategori jika dikirim tidak boleh kosong.

### Hapus menu (admin/kasir)

```bash
curl -i -H "Authorization: Bearer $TOKEN" -X DELETE "$BASE_URL/api/menus/$MENU_ID"
```

## Pesanan

Admin boleh melihat dan mengelola semua pesanan. Pembeli boleh membuat pesanan dan melihat pesanan miliknya. Kasir ditolak pada endpoint order.

### Daftar semua pesanan yang dapat dilihat akun

Admin melihat semua data; pembeli hanya data miliknya.

```bash
curl -i -H "Authorization: Bearer $TOKEN" "$BASE_URL/api/orders"
```

### Filter satu field pesanan

`field` menerima `id`, `customer_name`, `item_name`, `quantity`, atau `status`:

```bash
curl -i -H "Authorization: Bearer $TOKEN" "$BASE_URL/api/orders?field=id"
curl -i -H "Authorization: Bearer $TOKEN" "$BASE_URL/api/orders?field=customer_name"
curl -i -H "Authorization: Bearer $TOKEN" "$BASE_URL/api/orders?field=item_name"
curl -i -H "Authorization: Bearer $TOKEN" "$BASE_URL/api/orders?field=quantity"
curl -i -H "Authorization: Bearer $TOKEN" "$BASE_URL/api/orders?field=status"
```

### Detail pesanan

```bash
ORDER_ID=1
curl -i -H "Authorization: Bearer $TOKEN" "$BASE_URL/api/orders/$ORDER_ID"
```

Pembeli hanya dapat mengambil pesanan miliknya; admin dapat mengambil semua pesanan.

### Buat pesanan sekaligus mencatat pembayaran

Contoh JSON lengkap memakai `menu_id` `1` dan `quantity` `2`. Sebelum mengirim, cek `GET /api/menus`; jika ID `1` tidak ada atau stoknya kurang dari `2`, ganti angka `menu_id` dan/atau `quantity` dengan nilai yang tersedia. Metode hanya `cash` atau `bank_transfer`.

```bash
curl -i -H "Authorization: Bearer $TOKEN" -X POST "$BASE_URL/api/orders" \
  -H "Content-Type: application/json" \
  -d '{"customer_name":"Budi","menu_id":1,"quantity":2,"payment_method":"cash"}'
```

Contoh jika memakai transfer bank:

```bash
curl -i -H "Authorization: Bearer $TOKEN" -X POST "$BASE_URL/api/orders" \
  -H "Content-Type: application/json" \
  -d '{"customer_name":"Budi","menu_id":1,"quantity":2,"payment_method":"bank_transfer"}'
```

Order dan payment `lunas` dibuat bersamaan; total dihitung server berdasarkan harga menu. Tidak ada endpoint `POST /api/payments`. Sukses: `201 Created`; simpan `id` order untuk request berikutnya. Jangan ulangi POST yang sudah sukses karena akan membuat order baru dan mengurangi stok lagi.

### Ganti seluruh data pesanan (admin)

`PUT` butuh semua field. `status` tidak boleh kosong atau bernilai `paid`.

```bash
curl -i -H "Authorization: Bearer $TOKEN" -X PUT "$BASE_URL/api/orders/$ORDER_ID" \
  -H "Content-Type: application/json" \
  -d '{"customer_name":"Budi","item_name":"Nasi Goreng Spesial","quantity":3,"status":"diproses"}'
```

### Ganti sebagian data pesanan (admin)

Contoh ini mengirim semua field opsional. `PATCH` juga menerima satu atau beberapa field dari `customer_name`, `item_name`, `quantity`, dan `status`.

```bash
curl -i -H "Authorization: Bearer $TOKEN" -X PATCH "$BASE_URL/api/orders/$ORDER_ID" \
  -H "Content-Type: application/json" \
  -d '{"customer_name":"Budi","item_name":"Nasi Goreng Spesial","quantity":3,"status":"diproses"}'
```

Quantity harus positif, teks yang dikirim tidak boleh kosong, dan status `paid` ditolak.

### Hapus pesanan (admin)

```bash
curl -i -H "Authorization: Bearer $TOKEN" -X DELETE "$BASE_URL/api/orders/$ORDER_ID"
```

## Struk dan Pembayaran

### Ambil struk pesanan

Gunakan ID order dari response `POST /api/orders`. Hanya pemilik order atau admin yang bisa melihat struk.

```bash
curl -i -H "Authorization: Bearer $TOKEN" "$BASE_URL/api/orders/$ORDER_ID/receipt"
```

### Daftar pembayaran (admin)

Login sebagai admin terlebih dahulu. Endpoint ini hanya membaca pembayaran yang sudah tercatat.

```bash
curl -i -H "Authorization: Bearer $TOKEN" "$BASE_URL/api/payments"
```

Tidak ada route untuk menambah, mengubah, atau menghapus payment terpisah. Payment dibuat oleh request pembuatan order.

## Perhatian

- Jangan memasukkan token asli ke file proyek atau membagikannya.
- `401 Unauthorized` berarti token tidak ada/tidak valid/kedaluwarsa; login ulang.
- `403 Forbidden` berarti role tidak diizinkan. Menu edit perlu admin/kasir; order edit dan daftar payment perlu admin.
- `404 Not Found` berarti ID tidak ada atau akun pembeli bukan pemilik data.
- `409 Conflict` saat membuat order berarti menu tidak ditemukan atau stok kurang.
- Semua transaksi pembayaran pada API ini adalah simulasi, bukan pemrosesan uang sungguhan.