# Test with Rust 🦀

*Bu belgeyi diğer dillerde okuyun: [English](README.md), [Türkçe](README.tr.md)*

Bu şablon, Rust kullanarak C projeleri için sağlam, son derece otomatikleştirilmiş ve bellek güvenliğine sahip bir test altyapısı sağlar. Ceedling (Unity/CMock) gibi araçlara modern bir alternatif olarak işlev görür ve Rust'ın güçlü ekosistemini, sıkı tip kontrolünü ve paralel çalışma yeteneklerini eski (legacy) C kod tabanlarına getirir.

## ✨ Özellikler

- **Otomatik Mock Üretimi:** `bindgen` ve `mockall` kullanarak tüm C başlık dosyaları (header) için otomatik olarak mock modülleri üretir.
- **Sıfır Boilerplate (Gereksiz Kod):** Mock'ları, basit `.toml` yapılandırma dosyaları üzerinden diziler kullanarak tanımlar.
- **Hiyerarşik Çakışma Koruması:** C projesinin klasör yapısını Rust ortamında aynen yansıtarak isimlendirme çakışmalarını önler.
- **Test İzolasyonu:** Mükemmel test izolasyonu ve sembol çözümlemesi sağlamak için GNU Linker bayraklarını (`-ffunction-sections`, `-fdata-sections`) kullanır.
- **Coverage (Kod Kapsamı) Desteği:** Derleyici ve bağlayıcı (linker) argümanlarınıza `--coverage` veya `-fprofile-arcs` geçirerek `gcovr` ile sorunsuz entegrasyonu destekler.

---

## 🚀 Başlarken

### 1. Önkoşullar
Standart Rust toolchain'e ve ( `bindgen` için) Clang'e ihtiyacınız vardır:
```bash
# Ubuntu/Debian
sudo apt install build-essential clang llvm
```

### 2. Yapılandırma (`Cargo.toml` & `.cargo/config.toml`)
Tüm framework ayarları `Cargo.toml` dosyasında `[package.metadata.foreigntest]` altında tanımlanır:
* `project_path`: C projesi kök dizininin yolu.
* `compile_args`: C derleyicisine ve `bindgen`'e gönderilen argümanlar (örn: `-std=c99`, `-m32`, `-DDEBUG=1`, `-I../custom_lib`).
* `linker_args`: GNU Linker'a gönderilen argümanlar (örn: `-lm`).
* `compile_commands_path`: `compile_commands.json` dosyanızın yolu (örn. CMake tarafından üretilen). `build.rs` bu JSON'ı otomatik olarak ayrıştırarak tüm `-I`, `-isystem` ve `-D` bayraklarını çıkaracaktır.

**Önemli:** Eğer `compile_commands.json` başarıyla ayrıştırılırsa, `**/*.h` dizinlerinin yavaş ve kaba kuvvetle aranıp dahil edilmesi işlemi otomatik olarak atlanır!

> **💡 Cargo.lock Hakkında Not:** `Cargo.lock` Rust paket versiyonlarını yönetir. Güvenle silebilir veya `.gitignore`'a ekleyebilirsiniz; `cargo test` çalıştırıldığında yeniden üretilecektir.

---

## 🏗 Mimari: Birim (Unit) vs Entegrasyon Testleri

Bu şablon harici bir C kütüphanesini test etmek için tasarlandığından, **tüm testler teknik olarak Rust entegrasyon testleridir**. Ancak, bunları mantıksal olarak Birim (Unit) ve Entegrasyon testleri olarak ikiye ayırıyoruz:

* **Birim (Unit) Testleri:** Tek bir C modülünü izole ederek sadece onun kaynak dosyasını derler ve dışa bağımlılıklarını (çağırdığı diğer C modüllerini) mocklar.
* **Entegrasyon Testleri:** Birden fazla C modülünü mocklamadan birlikte derler, böylece modüllerin bir alt sistem olarak birbirleriyle nasıl çalıştıklarını test etmenizi sağlar.

Testlerinizi `tests/` dizini içinde istediğiniz gibi organize edebilirsiniz:
* **Modül Başına:** O modüle ait tüm test fonksiyonlarını içeren tek bir test dosyası (örn. `test_app_example.rs`).
* **Senaryo/Fonksiyon Başına:** Bir klasörde birden fazla test dosyası (örn. `tests/app_example/test_init.rs`, `tests/app_example/test_run.rs`). Cargo tümünü otomatik olarak çalıştırır.

---

## 🛠 Nasıl Birim (Unit) Testi Yazılır

### 1. Spec Tanımlama (`spec/`)
Test etmek istediğiniz C dosyasının yolunu yansıtan bir `.toml` dosyası oluşturun. (örn: `spec/source/app/app_example.toml`)

Başlık dosyalarını (headers), kaynak dosyaları (sources) ve mock'ları **Diziler (Arrays)** ve katı göreceli yollar kullanarak tanımlayın:
```toml
headers = ["source/app/app_example.h"]
sources = ["source/app/app_example.c"]
mocks   = ["lib/lib_example", "source/util/util_example"]
```
*(Eğer `headers` veya `sources` atlanırsa, `build.rs` bunları TOML dosyasının adı ve konumundan çıkarmaya çalışacaktır).*

### 2. Testi Yazma
Test dosyanızda (`tests/test_app_example.rs`), otomatik oluşturulan `bindings` ve `mocks` modüllerini `#[path]` kullanarak içe aktarın. Bunlar C projenizin klasör hiyerarşisini birebir takip eder!

```rust
#![feature(c_variadic)] // Yalnızca variadic C fonksiyonlarını test ediyorsanız

// 1. C tiplerini içe aktarın (Bindings)
#[allow(non_upper_case_globals, non_camel_case_types, non_snake_case, unused)]
#[path = "../bindings/source/app/app_example.rs"]
pub mod app_example;
use app_example::*;

// 2. Mock'ları içe aktarın
#[allow(non_snake_case, unused)]
#[path = "../mocks/source/app/app_example_mocks.rs"]
pub mod mocks;
use mocks::*;

#[cfg(test)]
mod app_tests {
    use super::*;

    #[test]
    fn test_initialization() {
        // 3. Mock beklentilerini ayarlayın
        let ctx = mock_lib_example::lib_example_init_context();
        ctx.expect().once().returning(|| true);
        
        // 4. Gerçek C fonksiyonunu çağırın
        unsafe {
            app_example_init();
        }
    }
}
```

> **Uyarı:** C'deki global `static` değişkenler, aynı çalıştırılabilir (executable) dosya içinde çalışıyorlarsa birden fazla `#[test]` fonksiyonu arasında kalıcı olur. Durum (state) bulaşmasını önlemek için, bu framework `.cargo/config.toml` dosyasında `RUST_TEST_THREADS = "1"` ayarlayarak testlerin sırayla çalışmasını zorunlu kılar. Ayrıca her testin başında C durumunu manuel olarak sıfırlamanız gerekebilir.

---

## 🤝 Nasıl Entegrasyon Testi Yazılır

Entegrasyon testleri, birden fazla C modülünün birbirleriyle etkileşimlerini mocklamadan doğru bir şekilde çalışıp çalışmadığını doğrular.

### 1. Spec Tanımlama
Bir entegrasyon testi oluşturmak için, bir `.toml` dosyası tanımlayın (örn. `spec/integration/app_subsystem.toml`) ve `sources` dizisine **birden fazla** kaynak dosyasını ekleyin. Gerçek uygulamalarının derlenip birbirine bağlanması için bu modülleri `mocks` dizininden çıkarın.

```toml
headers = ["source/app/app_example.h", "source/util/util_example.h"]
sources = ["source/app/app_example.c", "source/util/util_example.c"]
# Dikkat: util_example'ı burada mocklamıyoruz!
mocks   = ["lib/lib_example"] 
```

### 2. Testi Yazma
Rust testinizde (`tests/test_app_subsystem.rs`), artık `app_example.c`'den fonksiyonlar çağırabilir ve bunların gerçek `util_example.c` mantığıyla doğru etkileşime girdiğini doğrulayabilirsiniz; sadece harici `lib_example` sınırını mocklarsınız.

```rust
// 1. Her iki modül için de C tiplerini içe aktarın
#[allow(non_upper_case_globals, non_camel_case_types, non_snake_case, unused)]
#[path = "../bindings/integration/app_subsystem.rs"]
pub mod app_subsystem;
use app_subsystem::*;

// 2. Mock'ları içe aktarın (sadece lib_example mocklanmıştır)
#[allow(non_snake_case, unused)]
#[path = "../mocks/integration/app_subsystem_mocks.rs"]
pub mod mocks;
use mocks::*;

#[cfg(test)]
mod subsystem_tests {
    use super::*;

    #[test]
    fn test_subsystem_integration() {
        // Sadece daha alt seviyedeki kütüphaneyi mockluyoruz
        let ctx = mock_lib_example::lib_example_init_context();
        ctx.expect().once().returning(|| true);
        
        unsafe {
            // Bu çağrı dahili olarak gerçek util_example fonksiyonlarını çağıracaktır!
            app_example_init(); 
        }
    }
}
```

---

## 🎭 İleri Düzey Test Özellikleri

### Donanım Makrolarını Ezmek (`support/` dizini)
Gömülü (embedded) projelerde genellikle `*(int *)60 = 124;` gibi PC'de Segmentation Fault'a (Bölümlendirme Hatası) neden olacak donanım makroları bulunur.
Bunu atlatmak için sorunlu başlık dosyasını `support/` klasörüne kopyalayın (örn. `support/header_mxu.h`) ve makroyu düzenleyin (örn. `int x; #define REG_A (&x)`).
**`build.rs` include'lar (`-I`) için her zaman `support/` dizinine öncelik verir.** Hem `cc` hem de `bindgen` orijinal C dosyası yerine sizin ezdiğiniz başlık dosyasını okuyacaktır!

### C `main` Fonksiyonunu Test Etmek
C programınızın `main()` fonksiyonunu, Rust'ın dahili test koşucusu (runner) `main` ile çakışmadan test etmek için, `Cargo.toml` veya `compile_commands.json` içindeki `compile_args` kısmına bir tanımlama (define) ekleyin:
* `-Dmain=app_main`
Bu, C'nin giriş noktasını (entry point) `app_main` olarak yeniden adlandırır. Böylece Rust testlerinizden `unsafe { app_main(); }` çağrısı yapabilirsiniz!

---

## 🔧 32-Bit Toolchain'ler ve Sysroot'lar

Eğer gömülü C kodunuz kesinlikle 32-bit ise, `Cargo.toml` içindeki `compile_args` kısmına `-m32` geçmeniz zorunludur.

**KRİTİK:** `-m32` geçmek C kütüphanesini 32-bit olarak derler, ancak modern işletim sistemlerinde Rust varsayılan olarak 64-bit kullanır. Bağlayıcı (linker) *uyumsuz mimari (incompatible architecture)* hatası ile çökecektir.

Bunu çözmek için `.cargo/config.toml` dosyası `build.target` değerini varsayılan olarak `i686-unknown-linux-gnu` (32-bit) yapar:
```toml
[build]
target = "i686-unknown-linux-gnu"
rustflags = ["-C", "instrument-coverage"]

[env]
RUST_TEST_THREADS = "1"
CARGO_PROFILE_TEST_INCREMENTAL = "0"
LLVM_PROFILE_FILE = "target/profraw/cargo-test-%p-%m.profraw"
```

1. **Eğer bu 32-bit hedef (target) sisteminizde kurulu değilse, önce onu eklemelisiniz:**
   ```bash
   rustup target add i686-unknown-linux-gnu
   ```
2. **32-bit Sistem Kütüphanelerini Kurun (Ubuntu):**
   ```bash
   sudo apt install gcc-multilib g++-multilib
   ```
3. **Testleri Çalıştırın:**
   Artık `--target` belirtmenize gerek yok, sadece `cargo test` çalıştırmak yeterlidir. Eğer 64-bit bir C projesini test etmek isterseniz, `.cargo/config.toml` dosyasındaki `target` yapılandırmasını silebilir veya istediğiniz 64-bit hedef ile değiştirebilirsiniz.

---

## 🧪 Kod Kapsamı (Code Coverage)

Kod kapsamı raporları oluşturmak için, `Cargo.toml` içindeki `compile_args` dizisinde `-fprofile-arcs` ve `-ftest-coverage` (veya `--coverage`) olduğundan ve `linker_args` dizisine `--coverage` eklendiğinden emin olun.

Testleri çalıştırın ve (GCC kapsamını kusursuz şekilde yöneten) `gcovr` kullanarak bir HTML raporu oluşturun:
```bash
# 1. Henüz kurmadıysanız gcovr'yi kurun
pip install --user gcovr --break-system-packages

# 2. Coverage verilerini üretmek için temizleyip testleri çalıştırın
cargo clean
cargo test

# 3. HTML raporunu oluşturun
mkdir -p coverage
~/.local/bin/gcovr -r ../test-c-project-for-rust --object-directory target/i686-unknown-linux-gnu/debug/build/ --html-details -o coverage/index.html
```
