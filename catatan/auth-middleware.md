# `src/auth_middleware.rs`

File ini berisi middleware yang membatasi akses ke endpoint tertentu. Middleware adalah lapisan kecil yang dijalankan sebelum handler utama. Dalam proyek ini, middleware memvalidasi bearer JWT dan menaruh `user_id` terverifikasi pada request extensions.

Navigasi: [indeks](README.md) · [routing](routes.md) · [handler autentikasi](handlers/auth.md).

> **Inti bab:** middleware memvalidasi bearer JWT, mencocokkan role token dengan database, lalu menegakkan izin role pada route.

File sumber: [auth_middleware.rs](../src/auth_middleware.rs).

## Mengapa Middleware Dibutuhkan?

Tanpa pemeriksaan ini, setiap handler menu atau pesanan harus mengulang kode pengecekan login. Middleware mengumpulkan pemeriksaan tersebut di satu tempat. `routes.rs` memasang middleware hanya pada scope yang perlu dilindungi.

Middleware ini tidak memeriksa username atau password lagi. Tugas itu dilakukan saat login. Middleware memvalidasi JWT, memastikan role token masih sama dengan role database, lalu menyediakan `user_id` dan role lewat request extensions.

## Import

- `HttpMessage` menyediakan akses ke header dan extensions request.
- `ServiceRequest` adalah request Actix yang masih berada di pipeline middleware.
- `ServiceResponse` adalah response setelah request diteruskan.
- `MessageBody` adalah trait untuk tipe body HTTP.
- `Next` mewakili middleware atau handler berikutnya dalam rantai.
- `ErrorUnauthorized` membuat error HTTP 401; token tidak valid/kedaluwarsa atau role yang sudah berubah ditolak.
- `ErrorForbidden` membuat error HTTP 403 saat identitas valid tetapi role tidak mempunyai izin.

## Fungsi `require_login`

```rust
pub async fn require_login(
    request: ServiceRequest,
    next: Next<impl MessageBody>,
) -> Result<ServiceResponse<impl MessageBody>, Error>
```

Fungsi asynchronous menerima request dan objek `next`. Hasilnya `Ok(response)` jika token lolos validasi, atau `Err(error)` jika autentikasi gagal.

Bagian pemeriksaan:

```rust
Authorization: Bearer <access_token>
```

Middleware mengambil token dari header Authorization, memvalidasinya menggunakan algoritma HS256 yang eksplisit, memeriksa expiry, lalu mem-parsing subject menjadi ID positif. Role JWT dibandingkan dengan nilai terkini di database pada setiap request. Klaim yang tidak sah atau kedaluwarsa tidak diteruskan ke handler.

Jika header/token tidak ada atau tidak valid, middleware mengembalikan HTTP 401. Jika konfigurasi state autentikasi tidak ada, itu merupakan kesalahan server (500), bukan kesalahan kredensial klien.

```rust
if !authenticated {
    return Err(ErrorUnauthorized("Login diperlukan"));
}
```

Tanda `!` berarti NOT. Jadi blok dijalankan ketika `authenticated` bernilai `false`.

Jika bearer token dan policy role valid, `next.call(request).await` meneruskan request ke route/handler berikutnya. Hasil response diteruskan kembali ke client.

## Tempat Middleware Dipasang

Pada `routes.rs`, middleware khusus dipasang per scope. Semua role dapat membaca katalog; admin/kasir dapat mengubah menu/stok. Admin dapat mengakses semua operasi pesanan, sementara pembeli hanya dapat membuat pesanan dan meminta detail pesanan. Handler detail juga memeriksa kepemilikan pesanan di database. Kasir tidak dapat mengakses scope pesanan. Perubahan role hanya tersedia untuk admin.

## Batas Perlindungan Saat Ini

- Middleware memeriksa role, tetapi akses masih role-based dan belum mendukung permission khusus per pengguna.
- JWT stateless tidak dapat dicabut oleh endpoint logout; klien perlu menghapus token dan menunggu expiry untuk invalidasi server-side.
- Route yang tidak diberi middleware tetap dapat diakses tanpa login.
- Gunakan HTTPS agar bearer token tidak dapat dibaca saat transit melalui jaringan.

## Alur yang Perlu Diingat

![Diagram session lama; middleware saat ini memeriksa bearer JWT.](visual/session.svg)

```text
Client mengirim Authorization: Bearer <token>
    ↓
middleware memvalidasi signature, expiry, dan role database
    ↓
role memeriksa izin endpoint → user_id/role ditaruh pada request extensions
   ┌────┴────┐
 tidak ada   ada
    ↓          ↓
   401      handler
```

---

Kembali ke [indeks catatan](README.md) atau lanjut ke [handler autentikasi](handlers/auth.md).
