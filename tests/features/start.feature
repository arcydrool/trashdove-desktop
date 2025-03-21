Feature: Starting a trashdove server
    
  Scenario: First starting and logging in
    Given the nest is empty
    When I visit the index
    Then I should see a login