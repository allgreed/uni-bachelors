Funkcja: Zatrzymaj aktywny kontener

  Scenariusz: Zatrzymaj uruchomiony kontener
    Zakładając, że użytkownik jest zalogowany
    Oraz są jakieś aktywne kontenery
    Kiedy wybierze konkretny kontener
    Oraz spróbuje go zatrzymać
    Wtedy kontener powinien zniknąć z dashboard
    Oraz być widoczny w logach kontenerów
    Oraz użytkownik powinien być poinformowany o udanej deaktywacji kontenerta

  Scenariusz: Próba zatrzymania nieistniejącego kontenera
    Zakładając, że użytkownik jest zalogowany
    Kiedy wybierze kontener
    Oraz spróbuje go zatrzymać
    Oraz kontener zostanie uznany za nieistniejący
    Wtedy użytkownik powinien otrzymać informacje o tym
    Oraz zostać przekierowany do dashboard
