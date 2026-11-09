# Template Semua Request API dengan Postman

Panduan ini memuat semua endpoint yang didaftarkan di `src/routes.rs`: autentikasi, menu, pesanan, struk, dan daftar pembayaran. Ganti nilai contoh dengan data di database kamu.

## Persiapan

1. Pastikan server berjalan. Alamat default: `http://127.0.0.1:8080`.
2. Di Postman, buat atau pilih Environment dan tambahkan variabel berikut. Isi **Current value**.

| Variable | Contoh nilai | Keterangan |
|---|---|---|
| `baseUrl` | `http://127.0.0.1:8080` | Tanpa garis miring di akhir. |
| `accessToken` | kosong | Login akan mengisinya otomatis. Simpan sebagai nilai lokal/secret. |
| `userId` | `1` | ID akun untuk request ubah role. |
| `menuId` | `1` | ID menu untuk detail, ubah, hapus, atau pembelian. |
| `orderId` | kosong | ID order untuk request detail, ubah, hapus, dan struk. |
| `paymentId` | kosong | Diisi otomatis setelah order berhasil dibuat. |

Pilih Environment aktif di kanan atas. Pada Authorization collection, gunakan **Bearer Token** dengan Token `{{accessToken}}`. Untuk request `Register`, `Login`, dan `Logout`, atur Authorization request itu menjadi **No Auth**. Endpoint lainnya menggunakan bearer token.

Semua body JSON di bawah memakai nilai literal, tanpa placeholder. Contoh akun demo untuk `Register` dan `Login` adalah `demo_pembeli_20261109` dengan password `ContohPassword2026!`. Jalankan register sekali; jika username sudah dipakai, ganti username pada body Register dan Login dengan nilai yang belum dipakai. Jika akun sudah ada, lewati Register dan login dengan kredensial akun tersebut. Sebelum membuat order, cek `GET /api/menus` dan ganti angka `menu_id` di JSON dengan ID menu yang benar serta stok yang cukup.

Role yang tersedia: `admin`, `kasir`, `pembeli`. Register selalu membuat akun `pembeli`. Pilih Environment dengan token dari akun yang rolenya sesuai; Postman tidak mengganti role token hanya dengan mengubah request. Request dan body terperinci ada di bagian Autentikasi, Menu, dan Pesanan di bawah.

## Request Publik

Endpoint berikut tidak memerlukan bearer token dan dapat digunakan untuk semua role:

| Method | URL | Kegunaan |
|---|---|---|
| `POST` | `{{baseUrl}}/api/auth/register` | Membuat akun baru dengan role `pembeli`. |
| `POST` | `{{baseUrl}}/api/auth/login` | Login dan menyimpan token ke Environment. |
| `POST` | `{{baseUrl}}/api/auth/logout` | Memberi tahu klien untuk menghapus token lokal. |

## Akses Pembeli

Pakai token dari akun dengan role `pembeli`. Pembeli dapat melihat menu, membuat pembelian, lalu hanya membaca order dan struk miliknya sendiri.

| Method | URL | Catatan |
|---|---|---|
| `GET` | `{{baseUrl}}/api/auth/me` | Profil akun aktif. |
| `GET` | `{{baseUrl}}/api/menus` | Semua menu; filter `?field=id`, `name`, `price`, `category`, atau `stock` juga tersedia. |
| `GET` | `{{baseUrl}}/api/menus/{{menuId}}` | Detail menu. |
| `GET` | `{{baseUrl}}/api/orders` | Riwayat order milik pembeli; filter `id`, `customer_name`, `item_name`, `quantity`, `status`. |
| `GET` | `{{baseUrl}}/api/orders/{{orderId}}` | Detail order milik pembeli. |
| `POST` | `{{baseUrl}}/api/orders` | Membuat order dan payment `lunas` sekaligus. |
| `GET` | `{{baseUrl}}/api/orders/{{orderId}}/receipt` | Struk order milik pembeli. |

Body JSON untuk membuat order ada di bagian **Pesanan > Buat pesanan sekaligus mencatat pembayaran**. Pembeli tidak boleh mengubah/menghapus menu atau order dan tidak dapat melihat daftar semua payment.

## Akses Kasir

Pakai token dari akun dengan role `kasir`. Kasir dapat membaca menu dan mengelola menu/stok, tetapi tidak dapat mengakses route pesanan atau payment.

| Method | URL | Catatan |
|---|---|---|
| `GET` | `{{baseUrl}}/api/auth/me` | Profil akun aktif. |
| `GET` | `{{baseUrl}}/api/menus` | Semua menu atau filter `field`: `id`, `name`, `price`, `category`, `stock`. |
| `GET` | `{{baseUrl}}/api/menus/{{menuId}}` | Detail menu. |
| `POST` | `{{baseUrl}}/api/menus` | Membuat menu dan stok. |
| `PUT` | `{{baseUrl}}/api/menus/{{menuId}}` | Mengganti seluruh data menu. |
| `PATCH` | `{{baseUrl}}/api/menus/{{menuId}}` | Mengganti sebagian atau seluruh field menu. |
| `DELETE` | `{{baseUrl}}/api/menus/{{menuId}}` | Menghapus menu. |

Body JSON menu ada di bagian **Menu**. Request apa pun ke `/api/orders` atau `/api/payments` ditolak untuk kasir.

## Akses Admin

Pakai token dari akun dengan role `admin`. Admin boleh memakai seluruh request menu dan pesanan, termasuk membaca data semua pengguna.

| Method | URL | Catatan |
|---|---|---|
| `GET` | `{{baseUrl}}/api/auth/me` | Profil akun aktif. |
| `PUT` | `{{baseUrl}}/api/auth/users/{{userId}}/role` | Mengubah role ke `admin`, `kasir`, atau `pembeli`. |
| `GET` | `{{baseUrl}}/api/menus` atau `{{baseUrl}}/api/menus/{{menuId}}` | Membaca menu; filter `field` sama seperti bagian Pembeli. |
| `POST`, `PUT`, `PATCH`, `DELETE` | `/api/menus` atau `/api/menus/{{menuId}}` | Mengelola menu dan stok; gunakan body di bagian **Menu**. |
| `GET` | `{{baseUrl}}/api/orders` atau `{{baseUrl}}/api/orders/{{orderId}}` | Melihat semua order; filter `field` sama seperti bagian Pembeli. |
| `POST` | `{{baseUrl}}/api/orders` | Admin juga boleh membuat order; payment `lunas` tercatat otomatis. |
| `PUT`, `PATCH`, `DELETE` | `{{baseUrl}}/api/orders/{{orderId}}` | Mengubah atau menghapus order. |
| `GET` | `{{baseUrl}}/api/orders/{{orderId}}/receipt` | Struk order mana pun. |
| `GET` | `{{baseUrl}}/api/payments` | Daftar semua payment. |

Body JSON untuk perubahan role, menu, dan order ada pada bagian Autentikasi, Menu, dan Pesanan. Tidak tersedia endpoint untuk membuat atau mengubah payment secara terpisah.

## Autentikasi

### Register

- Method: `POST`
- URL: `{{baseUrl}}/api/auth/register`
- Authorization: `No Auth`
- Header: `Content-Type: application/json`
- Body (`raw`, format `JSON`):

```json
{
  "username": "demo_pembeli_20261109",
  "password": "ContohPassword2026!"
}
```

Username wajib terisi; password 8-128 byte; username harus belum digunakan. Sukses: `201 Created`. Register selalu memberi role `pembeli`.

### Login

- Method: `POST`
- URL: `{{baseUrl}}/api/auth/login`
- Authorization: `No Auth`
- Header: `Content-Type: application/json`
- Body (`raw`, format `JSON`):

```json
{
  "username": "demo_pembeli_20261109",
  "password": "ContohPassword2026!"
}
```

Di **Scripts > Post-response** (atau **Tests** pada Postman lama), tempel script ini untuk menyimpan token:

```javascript
const body = pm.response.json();

if (pm.response.code === 200 && body.access_token) {
  pm.environment.set("accessToken", body.access_token);
} else {
  pm.environment.unset("accessToken");
}
```

Sukses: `200 OK`. Request terlindungi berikutnya memakai `{{accessToken}}`.

### Profil akun aktif

- Method: `GET`
- URL: `{{baseUrl}}/api/auth/me`
- Authorization: Bearer Token `{{accessToken}}`
- Body: tidak ada.

### Logout

- Method: `POST`
- URL: `{{baseUrl}}/api/auth/logout`
- Authorization: `No Auth`
- Body: tidak ada.

Endpoint ini hanya mengirim pesan agar klien membuang token; JWT tidak dicabut server. Setelah request, hapus `accessToken` dari Environment secara manual.

### Ubah role akun (admin)

- Method: `PUT`
- URL: `{{baseUrl}}/api/auth/users/{{userId}}/role`
- Authorization: Bearer Token `{{accessToken}}` dari akun admin.
- Header: `Content-Type: application/json`
- Body (`raw`, format `JSON`):

```json
{
  "role": "kasir"
}
```

Nilai `role` harus `admin`, `kasir`, atau `pembeli`. Sukses: `200 OK`; setelah role akun berubah, akun tersebut perlu login ulang.

## Menu

Semua request menu memerlukan bearer token. Semua role boleh membaca menu; hanya admin/kasir boleh membuat, mengubah, dan menghapus.

### Daftar menu

- Method: `GET`
- URL: `{{baseUrl}}/api/menus`
- Authorization: Bearer Token `{{accessToken}}`
- Body: tidak ada.

Filter opsional `field` memilih satu kolom; gunakan salah satu URL berikut:

```text
{{baseUrl}}/api/menus?field=id
{{baseUrl}}/api/menus?field=name
{{baseUrl}}/api/menus?field=price
{{baseUrl}}/api/menus?field=category
{{baseUrl}}/api/menus?field=stock
```

### Detail menu

- Method: `GET`
- URL: `{{baseUrl}}/api/menus/{{menuId}}`
- Authorization: Bearer Token `{{accessToken}}`
- Body: tidak ada.

### Tambah menu (admin/kasir)

- Method: `POST`
- URL: `{{baseUrl}}/api/menus`
- Authorization: Bearer Token `{{accessToken}}`
- Header: `Content-Type: application/json`
- Body (`raw`, format `JSON`):

```json
{
  "name": "Nasi Goreng",
  "price": 25000,
  "category": "Makanan",
  "stock": 20
}
```

Nama/kategori wajib terisi, harga harus lebih dari nol, stok minimal nol. Sukses: `201 Created`. Simpan `id` dari response sebagai `menuId`.

### Ganti seluruh data menu (admin/kasir)

- Method: `PUT`
- URL: `{{baseUrl}}/api/menus/{{menuId}}`
- Authorization: Bearer Token `{{accessToken}}`
- Header: `Content-Type: application/json`
- Body (`raw`, format `JSON`):

```json
{
  "name": "Nasi Goreng Spesial",
  "price": 30000,
  "category": "Makanan",
  "stock": 20
}
```

`PUT` mengirim seluruh field; body di atas menggantikan data menu sebelumnya. Sukses: `200 OK`.

### Ganti sebagian data menu (admin/kasir)

- Method: `PATCH`
- URL: `{{baseUrl}}/api/menus/{{menuId}}`
- Authorization: Bearer Token `{{accessToken}}`
- Header: `Content-Type: application/json`
- Body (`raw`, format `JSON`):

```json
{
  "name": "Nasi Goreng Spesial",
  "price": 27500,
  "category": "Makanan",
  "stock": 18
}
```

Contoh ini mengirim semua field. `PATCH` juga menerima satu atau lebih field dari `name`, `price`, `category`, `stock`. Harga harus lebih dari nol, stok tidak negatif, dan nama/kategori jika dikirim tidak boleh kosong. Body `{}` ditolak. Sukses: `200 OK`.

### Hapus menu (admin/kasir)

- Method: `DELETE`
- URL: `{{baseUrl}}/api/menus/{{menuId}}`
- Authorization: Bearer Token `{{accessToken}}`
- Body: tidak ada.

Sukses: `200 OK`; ID yang tidak ditemukan: `404 Not Found`.

## Pesanan

Admin boleh membaca dan mengelola semua pesanan. Pembeli boleh membuat pesanan serta membaca pesanan miliknya. Kasir tidak dapat memakai endpoint pesanan.

### Daftar pesanan

- Method: `GET`
- URL: `{{baseUrl}}/api/orders`
- Authorization: Bearer Token `{{accessToken}}`
- Body: tidak ada.

Admin melihat semua pesanan; pembeli melihat pesanan sendiri. Filter opsional `field` menerima salah satu nilai berikut:

```text
{{baseUrl}}/api/orders?field=id
{{baseUrl}}/api/orders?field=customer_name
{{baseUrl}}/api/orders?field=item_name
{{baseUrl}}/api/orders?field=quantity
{{baseUrl}}/api/orders?field=status
```

### Detail pesanan

- Method: `GET`
- URL: `{{baseUrl}}/api/orders/{{orderId}}`
- Authorization: Bearer Token `{{accessToken}}`
- Body: tidak ada.

Pembeli hanya dapat melihat order miliknya; admin dapat melihat semua.

### Buat pesanan sekaligus mencatat pembayaran

- Method: `POST`
- URL: `{{baseUrl}}/api/orders`
- Authorization: Bearer Token `{{accessToken}}`
- Header: `Content-Type: application/json`
- Body (`raw`, format `JSON`):

```json
{
  "customer_name": "Budi",
  "menu_id": 1,
  "quantity": 1,
  "payment_method": "cash"
}
```

Ganti angka `menu_id` jika ID menu yang tersedia bukan `1`. `quantity` harus positif dan stok cukup. `payment_method` hanya `cash` atau `bank_transfer`. Request ini membuat order dan payment berstatus `lunas` dalam satu transaksi; tidak ada request `POST /api/payments`. Total dihitung server dari harga menu. Sukses: `201 Created`.

Di **Scripts > Post-response** / **Tests**, simpan ID order dan payment:

```javascript
if (pm.response.code === 201) {
  const body = pm.response.json();
  pm.environment.set("orderId", String(body.id));
  pm.environment.set("paymentId", String(body.payment.id));
} else {
  pm.environment.unset("orderId");
  pm.environment.unset("paymentId");
}
```

Jangan kirim ulang POST yang sudah sukses: setiap request sukses membuat order dan mengurangi stok lagi.

### Ganti seluruh data pesanan (admin)

- Method: `PUT`
- URL: `{{baseUrl}}/api/orders/{{orderId}}`
- Authorization: Bearer Token `{{accessToken}}` dari akun admin.
- Header: `Content-Type: application/json`
- Body (`raw`, format `JSON`):

```json
{
  "customer_name": "Budi",
  "item_name": "Nasi Goreng Spesial",
  "quantity": 3,
  "status": "diproses"
}
```

Semua field wajib dikirim; quantity positif dan teks tidak boleh kosong. Status `paid` ditolak. Sukses: `200 OK`.

### Ganti sebagian data pesanan (admin)

- Method: `PATCH`
- URL: `{{baseUrl}}/api/orders/{{orderId}}`
- Authorization: Bearer Token `{{accessToken}}` dari akun admin.
- Header: `Content-Type: application/json`
- Body (`raw`, format `JSON`):

```json
{
  "customer_name": "Budi",
  "item_name": "Nasi Goreng Spesial",
  "quantity": 3,
  "status": "diproses"
}
```

Contoh ini mengirim semua field opsional. `PATCH` juga menerima satu atau lebih field dari `customer_name`, `item_name`, `quantity`, `status`; field yang tidak dikirim tidak berubah. Quantity harus positif, teks yang dikirim tidak boleh kosong, dan status `paid` ditolak. Sukses: `200 OK`.

### Hapus pesanan (admin)

- Method: `DELETE`
- URL: `{{baseUrl}}/api/orders/{{orderId}}`
- Authorization: Bearer Token `{{accessToken}}` dari akun admin.
- Body: tidak ada.

Sukses: `200 OK`; ID yang tidak ditemukan: `404 Not Found`.

## Struk dan Pembayaran

### Ambil struk

- Method: `GET`
- URL: `{{baseUrl}}/api/orders/{{orderId}}/receipt`
- Authorization: Bearer Token `{{accessToken}}`
- Body: tidak ada.

Hanya pemilik order atau admin dapat membaca struk. Request order sukses membuat payment `lunas` secara otomatis.

### Daftar pembayaran (admin)

- Method: `GET`
- URL: `{{baseUrl}}/api/payments`
- Authorization: Bearer Token `{{accessToken}}` dari akun admin.
- Body: tidak ada.

Endpoint ini hanya untuk melihat daftar pembayaran; tidak tersedia route tambah, ubah, atau hapus pembayaran secara terpisah.

## Catatan Status

- `401 Unauthorized`: token tidak ada, tidak valid, atau kedaluwarsa. Login lagi.
- `403 Forbidden`: role akun tidak diizinkan untuk request tersebut.
- `404 Not Found`: ID tidak ditemukan atau pembeli bukan pemilik data.
- `409 Conflict` saat membuat order: menu tidak ditemukan atau stok tidak cukup.
- Logout tidak mencabut token JWT di server; hapus token di Environment Postman.