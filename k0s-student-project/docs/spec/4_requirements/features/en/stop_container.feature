Feature: Stop an active container

  Scenario: Stop a running container
    Given the user is logged in
    And there are active containers
    When they choose a specific container
    And choose to stop it
    Then the container should disappear from the dashboard
    And the user should see a message about successful deactivation
