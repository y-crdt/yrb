use rutie::{Array, Fixnum, Object};

pub(crate) fn convert_vecu8_to_array(vec: Vec<u8>) -> Array {
  let mut array = Array::new();

  for i in vec {
    array.push(Fixnum::new(i64::from(i)));
  }

  array
}

pub(crate) fn convert_array_to_vecu8(arr: Array) -> Vec<u8> {
  arr
    .into_iter()
    .map(|val| val
      .try_convert_to::<Fixnum>()
      .unwrap()
      .to_u32()
    )
    .map(|val| u8::try_from(val).unwrap())
    .collect()
}
