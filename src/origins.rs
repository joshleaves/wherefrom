use objc2_core_foundation::{CFArray, CFString, CFType};

/// Extract string origins from the `kMDItemWhereFroms` value.
///
/// Spotlight documents this attribute as an array of strings, but we still
/// downcast each entry defensively and skip any unexpected value.
pub fn wherefrom_origins(origins: &CFArray) -> Vec<String> {
  // SAFETY: `kMDItemWhereFroms` is a CFArray of CFType-backed objects.
  // We still validate each element with a checked downcast below.
  let values = unsafe { origins.cast_unchecked::<CFType>() };
  values
    .iter()
    .filter_map(|value| value.downcast::<CFString>().ok().map(|s| s.to_string()))
    .collect()
}

#[cfg(test)]
mod tests {
  use super::wherefrom_origins;
  use objc2_core_foundation::{CFArray, CFString, CFType};
  use uuid::Uuid;

  #[test]
  fn skips_non_string_values() {
    let first_value = format!("https://example.com/{}", Uuid::new_v4());
    let second_value = format!("https://example.com/{}", Uuid::new_v4());
    let first = CFString::from_str(&first_value);
    let not_a_string = CFArray::<CFType>::empty();
    let second = CFString::from_str(&second_value);

    let values: [&CFType; 3] = [
      first.as_ref(),
      not_a_string.as_ref(),
      second.as_ref(),
    ];
    let origins = CFArray::<CFType>::from_objects(&values);

    assert_eq!(
      wherefrom_origins(origins.as_opaque()),
      vec![
        first_value,
        second_value,
      ]
    );
  }
}
