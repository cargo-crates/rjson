use rjson::reactive;
fn main() {
  let state = reactive(serde_json::json!({
      "name": "John Doe",
      "age": 43,
      "phones": [
          "+44 1234567",
          "+44 2345678"
      ],
      "address": {
          "province": "山东"
      }
  }));

  let mut state = state.lock().unwrap();
  state.pset("name", "zhangsan".into());
  state.pset("age", 18.into());
  state.pset("age2", serde_json::json!(null));
  state["phones"][0] = "0529".into();
  state.pset("phones.1", "0539".into());
  println!("name: {}, age: {}, age2: {}", state.pget("name"), state.pget("age"), state.pget("age2"));
  println!("phones: {:?}", state.pget("phones.0"));
  println!("state: {:?}", state);

  // rjson::effect(move || {
  //     println!("hello effect, age: {}", state.pget("age"));
  // });
}