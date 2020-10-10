Feature: Create a user account for a guest that provides a login details and an invite
  Do not register without a valid invite


  Scenario: Successful registration
    Given the guest is on the register page
    When the guest fills in their details
    And provides an invite
    And attempts to register
    And their credentials are checked to be valid
    Then they should see a message about successful registration
    And be transferred to the log in page

  Scenario outline: Unsuccessful registration attempt cause by invalid invite
    Given the guest is on the register page
    When the guest fills in their details
    And provides an invite
    And attempts to register
    And their <detail> is checked to be valid
    Then they should see a message about a failed registration
    And see the reason being invalid <detail>
    And be given a chance to attempt registration again

    Examples:
      | detail |
      |  login |
      | invite |
