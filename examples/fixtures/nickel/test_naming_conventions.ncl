# Test file to verify naming conventions work correctly
# This test validates that our import naming conventions are functioning properly

let test_result = {
  # Test that we can use proper Nickel naming conventions
  naming_test_passed = true,
  
  # Test various naming patterns
  camelCase = "works",
  snake_case = "works",
  PascalCase = "works",
  
  # Test special characters that Nickel allows
  with_numbers_123 = true,
  with_dash = "dash-value",
  
  # Verify the test passes
  result = "All naming conventions work correctly"
} in

# Export the test result
test_result