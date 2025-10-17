use std::fmt;

fn main() {
    // “Newtype” = membuat type baru dari type lama.
    // Disebut newtype pattern karena konsep ini diambil dari Haskell.
    println!("Newtype pattern");

    /*
    Gunakan newtype pattern ketika kamu ingin:
    - Mengimplementasikan trait eksternal untuk type eksternal.
    - Membuat pembatasan perilaku di atas type yang sudah ada.
    - Membuat pemisahan semantik (misalnya UserId(String) vs SessionId(String)).
     */

    let w = Wrapper(vec![String::from("hello"), String::from("world")]);
    println!("w = {w}");

    let user = UserId("12345".into());
    let session = SessionId("ses123".into());
    // logout(user); // tidak valid
    logout(session);

    // Kamu bisa pakai newtype untuk keamanan tipe atau pemisahan logika, bukan cuma untuk trait saja.
}

struct Wrapper(Vec<String>);

impl fmt::Display for Wrapper {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}]", self.0.join(", "))
    }
}

struct UserId(String);
struct SessionId(String);

fn logout(session: SessionId) {
    println!("Logging out: {}", session.0);
}