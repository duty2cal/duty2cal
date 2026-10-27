Feature: State the public legal notices

  @e2e @browser
  Scenario: Reader opens the Impressum from the site footer and reads who publishes the site
    Given  the converter page served from the built Site output
    When  the reader follows the footer's "Impressum" link
    Then  the page states the operator's name and a postal address
    And  the page offers an electronic mail address as a link
    And  the page states that the employer address is a service address for this private project and not the employer's offering
    And  the page states that the project is not affiliated with Lufthansa, Jeppesen, or any duty-plan provider or airline
    And  the page states the source repository and the MPL-2.0 licence
    And  the page states the as-is sentence
    And  the page carries no placeholder and no unresolved marker
    And  the page sends no request to another origin since it loaded
    And  the page sets no cookie and stores no key in "localStorage" or "sessionStorage"
    And  the reader saw the site root's heading before activation and the notice's heading after it
    And  the page meets the visual baseline at both viewports

  @e2e @browser
  Scenario: Reader opens the Datenschutzerklärung from the footer and reads what a page request processes
    Given  the converter page served from the built Site output
    When  the reader follows the footer's "Datenschutz" link
    Then  the page names the controller and how to reach it
    And  the page states that GitHub, Inc. hosts the site in the United States and logs the visitor's IP address for security purposes
    And  the page names the time of the request, the page, the referrer, and the browser and device information that GitHub processes
    And  the page states that GitHub publishes no fixed retention period and that the operator has no access to those logs
    And  the page states the legal basis of that processing
    And  the page states the third-country transfer and its basis
    And  the page states that the host uses sub-processors to deliver the pages
    And  the page names the mail-forwarding provider, its role and its location, and the legal basis of the forwarding
    And  the page states that the conversion runs in the browser without upload, storage or a request after it loads
    And  the page states that no cookie is set and no third-party content is loaded
    And  the page states the reader's rights and the right to complain to a supervisory authority
    And  the page carries a standing date
    And  the page sends no request to another origin since it loaded
    And  the page sets no cookie and stores no key in "localStorage" or "sessionStorage"
    And  the reader saw the site root's heading before activation and the notice's heading after it
    And  the page meets the visual baseline at both viewports

  Scenario: A page set that serves only one of the two notices is refused and the missing notice is named
    Given  a page set whose converter page links both notices
    And  the page set carries the Impressum and no Datenschutzerklärung
    When  the notices are checked before publication
    Then  the check is refused and names the Datenschutzerklärung
    And  the previously published site stays

  Scenario: A page that does not link both notices is refused and the page is named
    Given  a page set that carries both notices
    And  the Datenschutzerklärung does not link the Impressum
    When  the notices are checked before publication
    Then  the check is refused and names the Datenschutzerklärung and the missing "Impressum" link
    And  the previously published site stays

  Scenario: An Impressum that still carries a placeholder is refused and the marker is named
    Given  a page set that carries both notices and links both
    And  the Impressum still carries the marker "TODO"
    When  the notices are checked before publication
    Then  the check is refused and names the Impressum and the marker
    And  the previously published site stays

  Scenario: An Impressum without a postal address or without an electronic mail address is refused
    Given  a page set that carries both notices and links both
    And  the Impressum states a name and no postal address
    When  the notices are checked before publication
    Then  the check is refused and names the Impressum and the missing postal address

  Scenario: A Datenschutzerklärung without a standing date is refused and the date is named
    Given  a page set that carries both notices and links both
    And  the Datenschutzerklärung carries no standing date
    When  the notices are checked before publication
    Then  the check is refused and names the Datenschutzerklärung and the missing standing date

  Scenario: A Datenschutzerklärung without an English summary is refused and the summary is named
    Given  a page set that carries both notices and links both
    And  the Datenschutzerklärung carries no English summary
    When  the notices are checked before publication
    Then  the check is refused and names the Datenschutzerklärung and the missing English summary

  Scenario: An Impressum that carries the discontinued dispute-resolution reference is refused
    Given  a page set that carries both notices and links both
    And  the Impressum links "ec.europa.eu/consumers/odr"
    When  the notices are checked before publication
    Then  the check is refused and names the Impressum and the withdrawn reference

  Scenario: A notice that loads an asset from another origin is refused and the asset is named
    Given  a page set that carries both notices and links both
    And  the Datenschutzerklärung loads an image from "https://cdn.analytics.invalid/pixel.png"
    When  the notices are checked before publication
    Then  the check is refused and names the Datenschutzerklärung and the foreign asset

  Scenario: Both notices are German with an English summary and name the German version as binding
    Given  a page set that carries both notices and links both
    When  a reader reads either notice
    Then  the notice's body is written in German
    And  the notice carries a short English summary below the German text
    And  the summary states that the German version is the binding one

  Scenario: A carrier that copies the postal address instead of linking the notices is refused
    Given  the site serves both notices and links both
    And  the organization profile copies the operator's postal address
    When  the notices are checked across the carriers
    Then  the check is refused and names the organization profile and the copied address
    And  the organization profile and the README link the notices instead
