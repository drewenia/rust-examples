fn main() {
    //'{}' otomatik olarak herhangi bir argümanla değiştirilecektir. Bunlar stringified edilecektir
    println!("{} days", 31);

    //'{0}' yerine ilk arguman olan 21, '{1}' yerine 11 gelecek
    println!("{0} this is {1}. {1} this is {0}", 21, 11);

    //named arguman kullanimi
    println!(
        "{subject} {verb} {object}",
        object = "the lazy dog",
        subject = "the quick brown fox",
        verb = "jumps over"
    );

    // ':' operatorunden sonra farkli bicimlendirmeler kullanilabilir
    println!("Base 10 :                  {}", 69420); //69420
    println!("Base 2 (binary) :          {:b}", 69420); //10000111100101100
    println!("Base 8 (octal) :           {:o}", 69420); //207454
    println!("Base 16 (hexadecimal) :    {:x}", 69420); //10f2c

    // metni belli bir genislikle saga yaslayabilirsiniz. Toplam 5 genisliginde alan ayirir 4 bosluk birakir 1'i yazar
    println!("{number:>5}", number = 1);

    // sayilari fazladan sifir ile doldurabilirsiniz. '00004' output'u uretilir
    println!("{number:0>5}", number = 4);

    // isareti ters cevirerek sola ayarlamak. '50000' output'u uretilir
    println!("{number:0<5}", number = 5);

    // '$' kullanarak adlandirilmis bagimsiz degiskenler kullanilabilir. '00001' output'u uretilir
    println!("{number:0>width$}", number = 1, width = 5);

    /*
       Yalnızca fmt::Display implement eden türler `{}` ile biçimlendirilebilir. Kullanıcı tanımlı tipler
       varsayılan olarak fmt::Display'i implement edemezler.
    */

    let number: f64 = 1.0;
    let width: usize = 5;
    println!("{number:>width$}"); // cikti olarak gene 4 bosluktan sonra 1 output'u uretecektir
}
