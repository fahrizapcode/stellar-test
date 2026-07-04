# Stellar Outlet Governance Smart Contract



Smart contract sederhana berbasis **Soroban (Stellar)** yang digunakan sebagai MVP untuk platform tata kelola ekspansi outlet UMKM.



Contract ini berfungsi sebagai **audit trail** untuk mencatat data outlet dan laporan revenue sharing secara transparan di blockchain Stellar.



## Fitur



- Mendaftarkan outlet baru

- Menyimpan informasi brand, operator, dan investor

- Mencatat laporan pendapatan outlet

- Mencatat pembagian revenue sharing

- Melihat daftar outlet

- Melihat histori laporan revenue

- Emit event setiap transaksi untuk audit trail



## Struktur Data



### Outlet



```rust

Outlet {

    id: u64,

    brand_name: String,

    operator: Address,

    investor: Address,

}

```



### Revenue Report



```rust

RevenueReport {

    id: u64,

    outlet_id: u64,

    total_revenue: i128,

    brand_share: i128,

    investor_share: i128,

    operator_share: i128,

    platform_share: i128,

    timestamp: u64,

}

```



---



## Fungsi Smart Contract



### 1. Register Outlet



Digunakan untuk mendaftarkan outlet baru.



```rust

register_outlet(

    brand_name,

    operator,

    investor

)

```



Output:



```

"Outlet berhasil didaftarkan"

```



---



### 2. Get Outlets



Mengambil seluruh data outlet yang telah terdaftar.



```rust

get_outlets()

```



---



### 3. Submit Revenue



Mencatat laporan pendapatan outlet beserta pembagian revenue sharing.



```rust

submit_revenue(

    outlet_id,

    total_revenue,

    brand_share,

    investor_share,

    operator_share,

    platform_share

)

```



Output:



```

"Revenue report berhasil dicatat"

```



---



### 4. Get Reports



Mengambil seluruh histori laporan revenue.



```rust

get_reports()

```



---



## Alur MVP



```text

Register Outlet

        │

        ▼

Outlet Tersimpan di Blockchain

        │

        ▼

Outlet Beroperasi

        │

        ▼

Submit Revenue Report

        │

        ▼

Revenue Sharing Tercatat

        │

        ▼

Data Dapat Dibaca oleh Semua Pihak

```



---



## Arsitektur



```

Brand Owner

      │

      ▼

Register Outlet

      │

      ▼

Soroban Smart Contract

      │

      ├── Storage Outlet

      ├── Storage Revenue Report

      └── Event Audit Trail

      │

      ▼

Investor / Operator / Dashboard

```



---



## Teknologi



- Rust

- Soroban SDK

- Stellar Blockchain



---



## Tujuan MVP



Smart contract ini hanya digunakan sebagai **proof of concept** untuk menunjukkan bagaimana blockchain Stellar dapat digunakan sebagai **lapisan transparansi (trust layer)** dalam tata kelola ekspansi bisnis.



Versi ini **belum mencakup**:



- Tokenisasi RWA

- Pembayaran on-chain

- Smart contract escrow

- NFT kepemilikan

- Revenue sharing otomatis

- Integrasi frontend

- Integrasi database

- Authentication



Fitur-fitur tersebut direncanakan pada tahap pengembangan berikutnya.



---



## Roadmap



### Phase 1

- ✅ Register Outlet

- ✅ Revenue Report

- ✅ Audit Trail



### Phase 2

- Revenue Sharing Otomatis

- Dashboard Investor



### Phase 3

- Tokenisasi RWA

- Smart Contract Distribution



### Phase 4

- Public Investment

- Secondary Market

- Integrasi Ekosistem Stellar

![Wallet Connected](./contracts/notes/Screenshot%202026-07-04%20165754.png)



