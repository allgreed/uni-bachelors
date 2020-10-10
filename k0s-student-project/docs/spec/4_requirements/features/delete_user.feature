Funkcja: Administrator usuwa użytkownika

  Założenia:
    Zakładając, że użytkownik ma uprawnienia administratora
    Oraz jest zalogowany

  Scenariusz: Wstępne usunięcie użytkownika
    Zakładając, że administrator jest w panelu ustawień konkretnego użytkownika
    Kiedy wybierze opcję usunięcia użytkownika
    Wtedy powinien zostać poproszony o podanie hasła
    Oraz potwierdzenie usunięcia

  Scenariusz: Pomyślne usunięcie użytkownika
    Zakładając, że administrator jest w panelu ustawień konkretnego użytkownika
    Kiedy podane hasło zostanie zaakceptowane jako prawidłowe
    Oraz administrator potwierdził usunięcie
    Wtedy admnistrator powinien być przeniesiony do listy użytkowników
    Oraz usunięty użytkownik powinien nie być widoczny w na liście użytkowników
