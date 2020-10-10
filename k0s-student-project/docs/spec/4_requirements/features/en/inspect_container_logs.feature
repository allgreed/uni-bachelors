Feature: Inspect the statistics of a specific container

  Scenario: See the list of used containers that can be inspected further
    Given the user is logged in
    When they choose to see the containers logs
    They should see a list of all containers ever used
    And be able to further choose a specific one

  Scenario: See the details of a chosen container
    Given the user is logged in
    And have ever started a container before
    And is on the container logs page
    When they choose to see the specific container logs
    They should see statistics of that container
    And its state being active or inactive

  Scenario: Trying to access a non-existent container
    Given the user is logged in
    And is on the container logs page
    When they choose to see the specific container logs
    And that container is checked to have never existed
    Then the user should be informed about that
    And be transferred to the container logs list
