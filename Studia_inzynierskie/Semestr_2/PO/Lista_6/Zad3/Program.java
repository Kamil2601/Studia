// Kamil Michalak
// Pracownia PO, czwartek, s. 108
// L5, z4, Producent-Konsument
// Program
// Program.java
// 2018-04-05


class Program
{
    public static void main(String[] args)
    {
        Bufor<String> kolejka = new Bufor<String>(10);

        Producent p = new Producent(kolejka);
        Konsument k = new Konsument(kolejka);
        p.start();
        k.start();
    }
}