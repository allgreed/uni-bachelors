Feature: Log out a user

  Scenario: Allow the user to log out
    Given the user is logged in
    When they attempt to log out
    Then they are should be transferred to a log in page
    And be unable to access the dashboard
