Feature: Log in a pre-registered potential user authenticating their credentials
  Reject non-registered guests

  Background:
    Given the guest is not logged in

  Scenario: Allow the guest to log in
    Given the guest is on the login page
    When the guest provides login details
    And attempts to log in
    And their credentials are checked to be valid
    Then they should be given access to the dashboard and a log out option

  Scenario: Deny access to a unregistered guest
    Given the guest is on the login page
    When the guest provides login details
    And attempts to log in
    But their credentials are checked to be invalid
    Then they should see a message about unsuccessful log in attempt
    And allowed to try to log in again
