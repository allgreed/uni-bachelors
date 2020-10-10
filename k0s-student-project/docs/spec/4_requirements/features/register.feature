Funkcja: Utwórz konto użytkownika dla gościa który poda prawidłowe dane oraz ważne zaproszenie
  Odmów rejestracji użytkownikowi z nieważnym zaproszeniem lub nieunikatowymi danymi


  Scenariusz: Rejestracja zakończona sukcesem
    Zakładając, że gość jest na stronie rejestracji
    Kiedy poda dane użytkownika
    Oraz poda zaproszenie
    Oraz spróbuje się zarejestrować
    Oraz ich dane zostaną sprawdzone jako prawidłowe
    Wtedy powinni zostać poinformowani o udanej rejestracji
    Oraz zostać przeniesionym do strony logowania

  Szablon scenariusza: Nieudana rejestracja spowodowana nieważnym zaproszeniem
    Zakładając, że gość jest na stronie rejestracji
    Kiedy poda dane użytkownika
    Oraz poda zaproszenie
    Oraz spróbuje się zarejestrować
    Oraz ich <dane> zostaną sprawdzone jako nieprawidłowe
    Wtedy powinni otrzymać informacje o nieudanej próbie rejestracji
    Oraz informacje o powodzie którym były nieprawidłowe <dane>
    Oraz mieć możliwość ponownej rejestracji

    Przykłady:
      |              dane |
      |  dane użytkownika |
      |       zaproszenie |
