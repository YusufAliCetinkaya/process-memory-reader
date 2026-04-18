# Process Memory Mapper

Çalışan bir sürecin yüklü modüllerini ve bu modüllerin bellek adreslerini listeleme yöntemi.

## Özellikler
- Hedef süreci isme göre tarar ve PID numarasını tespit eder.
- Süreç modüllerinin (DLL ve EXE) anlık görüntüsünü oluşturur.
- Her modülün bellek üzerindeki başlangıç adresini (Base Address) raporlar.

## Teknik Detaylar
- WinAPI doğrudan `windows-sys` crate üzerinden çağrılır.
- CreateToolhelp32Snapshot ile süreç ve modül listeleme işlemleri yapılır.
- Bellek erişimi için PROCESS_QUERY_INFORMATION ve PROCESS_VM_READ yetkileri kullanılır.

```bash
cargo run
