# Unversioned runtime types for Kubernetes
# These types are used across multiple API versions

{
  # IntOrString is a type that can hold either an integer or a string
  # In Nickel, we represent this as a String type with a contract
  IntOrString = String,

  # RawExtension is used to hold arbitrary JSON data
  # It can contain any valid JSON/Nickel value
  RawExtension = {
    ..
  },

  # Type contracts for validation
  contracts = {
    # Contract for IntOrString - accepts both numbers (as strings) and strings
    intOrString = fun value =>
      std.is_string value,

    # Contract for RawExtension - accepts any record
    rawExtension = fun value =>
      std.is_record value || value == null,
  },
}
