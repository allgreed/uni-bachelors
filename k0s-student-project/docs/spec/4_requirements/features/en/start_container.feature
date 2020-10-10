Feature: Start a new container

  Scenario: Start a container
    Given the user is logged in
    When they provide a valid container image
    And choose to run a new container
    Then the new container should be seen as active on the dashboard
