Feature: Regions API
  As a frontend application
  I want to be able to retrieve region data from the API
  So that I can display Japanese city names to users

  Background:
    Given the regions database is populated with city data
    And the API server is running

  Scenario: Retrieve all regions
    When I send a GET request to "/regions"
    Then the response status code should be 200
    And the response should include the following regions:
      | slug       | katakana     | english    |
      | chicago    | シカゴ        | Chicago    |
      | columbus   | コロンバス     | Columbus   |
      | san-diego  | サンディエゴ   | San Diego  |
      | los-angeles| ロサンゼルス   | Los Angeles|
      | new-york   | ニューヨーク   | New York   |
      | boston     | ボストン       | Boston     |
      | nashville  | ナッシュビル   | Nashville  |

  Scenario: Retrieve a specific region by slug
    When I send a GET request to "/regions/chicago"
    Then the response status code should be 200
    And the response should include a region with:
      | field     | value     |
      | slug      | chicago   |
      | katakana  | シカゴ     |
      | english   | Chicago   |

  Scenario: Retrieve regions filtered by name
    When I send a GET request to "/regions?filter=n"
    Then the response status code should be 200
    And the response should include regions with slugs:
      | nashville |
      | new-york  |
      | san-diego |

  Scenario: Attempt to retrieve a non-existent region
    When I send a GET request to "/regions/nonexistent"
    Then the response status code should be 404
    And the response should include an error message "Region not found"

  Scenario: Create a new region
    When I send a POST request to "/regions" with the following data:
      | slug      | seattle   |
      | katakana  | シアトル    |
      | english   | Seattle   |
    Then the response status code should be 201
    And the response should include a region with:
      | field     | value     |
      | slug      | seattle   |
      | katakana  | シアトル    |
      | english   | Seattle   |
    When I send a GET request to "/regions/seattle"
    Then the response status code should be 200

  Scenario: Update an existing region
    When I send a PUT request to "/regions/boston" with the following data:
      | katakana  | ボストンシ    |
      | english   | Boston City |
    Then the response status code should be 200
    And the response should include a region with:
      | field     | value        |
      | slug      | boston       |
      | katakana  | ボストンシ     |
      | english   | Boston City  |

  Scenario: Delete a region
    When I send a DELETE request to "/regions/nashville"
    Then the response status code should be 204
    When I send a GET request to "/regions/nashville"
    Then the response status code should be 404