Funkcja: Zaloguj gościa uwierzytelniając jego dane
  Odrzuć logowanie nieistniejących użytkowników

  Założenia:
    Zakładając, że gość nie jest zalogowany

  Scenariusz: Udane logowanie
    Zakładając, że gość jest na stronie logowania
    Kiedy poda dane logowania
    Oraz podejmie próbę zalogowania
    Oraz ich dane logowania zostaną potwierdzone jako prawidłowe
    Wtedy powinni otrzymać dostęp do dashboard i opcji wylogowania

  Scenariusz: Odmów zalogowania niezarejestrowanemu gościowi
    Zakładając, że gość jest na stronie logowania
    Kiedy gość poda dane logowania
    Oraz spróbuje się zalogować
    But ich dane logowania zostaną potwierdzone jako nie prawidłowe
    Wtedy powinni otrzymać komunikat o nieudanym logowaniu
    Oraz mieć możliwość ponownego zalogowania się
